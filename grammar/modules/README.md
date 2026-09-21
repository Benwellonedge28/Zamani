Zamani Module System

Production Architecture, Ownership, Integration, Scalability, and Conformance Contract

Path: "grammar/modules/"
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust safety: "unsafe" is prohibited
Portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Status

This document is the architectural and integration contract for the Zamani module subsystem.

It defines:

- what "grammar/modules/" owns;
- what every file in the directory owns;
- what every file explicitly does not own;
- how module syntax integrates with the canonical lexer;
- how module syntax integrates with "grammar/Zamani.g4";
- how modules map into the frontend AST;
- how module semantics are resolved;
- how packages, namespaces, imports, exports, aliases, dependencies, and versions interact;
- how modules remain independent of machine scale;
- how modules integrate with classical, quantum, HDL, hardware, AI, distributed, networking, and future domains;
- how module information reaches the compiler and runtime;
- how compatibility and diagnostics work;
- what constitutes completion;
- what tests are required before production status.

This document is not itself another grammar authority.

The actual syntax is defined by the canonical grammar files and their aggregate composition.

---

2. Architectural Principle

A Zamani module is a language-level source organization and compilation abstraction.

A module is not inherently:

- a filesystem directory;
- a filesystem file;
- a package;
- a process;
- a thread;
- a CPU;
- a GPU;
- an FPGA;
- an ASIC;
- a QPU;
- a physical qubit;
- a memory bank;
- a network node;
- a cluster;
- a container;
- a deployment;
- a runtime instance;
- a hardware resource.

For example:

module quantum::algorithms {
    ...
}

defines a module named "quantum::algorithms".

It does not itself mean:

use a QPU
allocate N qubits
select QPU #0
use GPU #1
use CPU core #7
deploy to node #3
use a particular memory bank
use a particular network topology

Those decisions belong to downstream semantic, resource, compilation, scheduling, routing, deployment, runtime, HAL, and target infrastructure.

---

3. Module-System Mission

The module system must allow one language to organize all Zamani computational domains without turning each domain into a separate language.

A module may contain or expose constructs associated with:

- classical computing;
- systems programming;
- embedded computing;
- parallel computing;
- distributed computing;
- HPC;
- quantum computing;
- hybrid quantum/classical computing;
- HDL;
- hardware/software co-design;
- accelerators;
- AI/ML;
- data processing;
- scientific computing;
- networking;
- cryptography;
- security;
- temporal computation;
- Sankofa concepts;
- nano computation;
- future computational domains.

The module subsystem must not need to understand the implementation semantics of each domain.

Its responsibility is to provide the organizational language infrastructure through which those constructs can coexist.

---

4. Canonical Compilation Position

The module subsystem participates in the frontend pipeline:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
canonical parser / Zamani.g4
    │
    ├── core
    ├── names
    ├── types
    ├── expressions
    ├── statements
    ├── declarations
    ├── functions
    ├── modules
    │     ├── modules.g4
    │     ├── imports.g4
    │     ├── exports.g4
    │     ├── visibility.g4
    │     ├── namespaces.g4
    │     ├── aliases.g4
    │     ├── packages.g4
    │     ├── dependencies.g4
    │     ├── versioning.g4
    │     └── module-attributes.g4
    └── other domains
    │
    ▼
frontend AST
    │
    ▼
module/package/namespace/name resolution
    │
    ▼
semantic analysis
    │
    ├── type system
    ├── effects
    ├── capabilities
    ├── resources
    ├── ownership
    ├── visibility
    ├── dependencies
    └── domain semantics
    │
    ▼
canonical semantic representations
    │
    ├── classical representation
    ├── quantum::ir
    ├── HDL/hardware representation
    └── other canonical domain representations
    │
    ▼
optimization
    │
    ▼
routing / scheduling / resilience / QEC / ZQN where applicable
    │
    ▼
HAL / backend lowering
    │
    ▼
target realization
    │
    ├── CPU
    ├── multicore
    ├── GPU
    ├── FPGA
    ├── ASIC
    ├── QPU
    ├── simulator
    ├── accelerator
    ├── cluster
    ├── HPC
    ├── distributed infrastructure
    ├── cloud
    └── future targets

The module grammar stops at syntax.

It does not perform the downstream operations.

---

5. Authority Model

The module subsystem participates in the following repository-wide authority hierarchy:

grammar/DESIGN.md
        │
        ▼
language specification
        │
        ▼
canonical grammar
        │
        ▼
lexer/parser implementation
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR / domain IR
        │
        ▼
compiler / runtime / backend

The following files have distinct purposes:

"grammar/DESIGN.md"

Normative architecture for the entire grammar subsystem.

"grammar/Zamani.g4"

Canonical ANTLR composition root.

"grammar/modules/*.g4"

Independent module-system grammar components.

"grammar/grammar.md"

Implementation-conformance reference.

It must not silently define syntax that is absent from the canonical grammar/specification.

"grammar/Zamani-Grammar.md"

Historical/extended/design reference.

It must not silently promote proposed syntax into the implemented language.

"src/lexer.rs"

Actual Rust lexical implementation.

"src/parser.rs"

Actual Rust parser implementation.

"src/ast/"

Frontend AST representation.

Semantic analysis

Owns the meaning of modules, names, imports, exports, packages, dependencies, versions, visibility, and related constructs.

Canonical IR

Owns executable/compilable semantic representations.

The module grammar must not become an additional IR.

---

6. Directory Contents

The current module directory contains:

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

These filenames are retained.

No rename is required.

Each file has one primary responsibility.

---

7. "modules.g4"

Owns

"modules.g4" owns the syntax of module declarations.

It owns:

- module declaration syntax;
- module declaration headers;
- module names as wrappers around canonical qualified names;
- optional module attributes;
- optional module visibility;
- inline module-body boundaries;
- declaration terminators;
- syntactic module-body composition.

Does not own

It does not own:

- identifier syntax;
- qualified-name syntax;
- imports;
- exports;
- aliases;
- namespaces;
- packages;
- dependencies;
- versions;
- general attributes;
- declarations;
- functions;
- types;
- expressions;
- statements;
- quantum operations;
- HDL;
- hardware;
- AI;
- resources;
- compiler behavior;
- runtime behavior.

Required integration

"modules.g4" consumes canonical name, visibility, and module-attribute grammar.

The module body must ultimately contain the aggregate language's canonical item/declaration structure.

It must not create a second declaration language.

---

8. "imports.g4"

Owns

"imports.g4" owns import syntax.

It covers the syntactic representation of:

- imported module paths;
- imported names;
- import groups;
- wildcard imports;
- aliases;
- source qualifiers;
- import attributes where specified by the language.

Does not own

It does not:

- read files;
- access registries;
- access networks;
- download packages;
- resolve symbols;
- solve dependency graphs;
- verify artifacts;
- load runtime modules.

Those are downstream operations.

Integration

imports.g4
    ↓
AST import declaration
    ↓
module/package/name resolver
    ↓
dependency graph
    ↓
semantic validation

---

9. "exports.g4"

Owns

"exports.g4" owns:

- export declarations;
- exported names;
- export lists;
- re-export syntax;
- export aliases;
- wildcard export syntax if part of the accepted language.

Does not own

It does not own:

- symbol resolution;
- visibility semantics;
- ABI generation;
- package publication;
- runtime loading;
- deployment.

Important distinction

Visibility and export are related but not identical.

Visibility answers:

«Who may access a declaration?»

Export answers:

«Which declarations form this module's public module interface?»

The semantic layer validates their relationship.

---

10. "visibility.g4"

Owns

"visibility.g4" is the canonical owner of visibility syntax used throughout the language.

The vocabulary must be reusable by:

- modules;
- functions;
- types;
- declarations;
- interfaces;
- traits;
- classical declarations;
- quantum declarations;
- HDL declarations;
- hardware declarations;
- distributed declarations;
- future domains.

Critical rule

No sibling grammar may create an independent visibility vocabulary.

There must not be competing definitions such as:

moduleVisibility
functionVisibility
quantumVisibility
hardwareVisibility

if they are merely duplicates of the same language concept.

---

11. "namespaces.g4"

Owns

"namespaces.g4" owns namespace declaration/reference syntax.

Does not own

It does not own:

- module identity;
- package identity;
- filesystem identity;
- symbol resolution;
- deployment identity.

A namespace is a naming/scope concept.

A module is a source organization/compilation concept.

A package is a distribution/dependency concept.

These relationships are established semantically.

---

12. "aliases.g4"

Owns

"aliases.g4" owns syntactic alias declarations and alias clauses that are explicitly part of the module system.

It may support aliases for:

- imported modules;
- imported names;
- exported names;
- module references;
- package references;
- namespace references;

where those constructs are specified.

Does not own

It does not determine:

- whether two names are semantically equivalent;
- whether an alias creates a new symbol;
- whether an alias is legal under visibility rules;
- whether an alias introduces ambiguity;
- whether a dependency is valid.

Those are semantic responsibilities.

Integration

Aliases must lower to the canonical AST name/reference representation rather than introducing a second name model.

---

13. "packages.g4"

Owns

"packages.g4" owns package-related source syntax.

A package may provide:

- distribution identity;
- package metadata;
- package declarations;
- package-level source organization;
- package-facing dependency information.

Does not own

It does not:

- access a registry;
- download packages;
- install packages;
- resolve dependencies;
- verify signatures;
- manage filesystem paths;
- select execution hardware.

A package must remain independent from target hardware.

---

14. "dependencies.g4"

Owns

"dependencies.g4" owns source-level dependency declarations.

It may represent:

- dependency identity;
- dependency requirement;
- dependency source;
- version requirement;
- dependency alias;
- dependency features/options where specified by the language.

Does not own

It does not perform:

- dependency solving;
- network access;
- package retrieval;
- artifact verification;
- lockfile generation;
- installation;
- compilation;
- runtime loading.

The dependency subsystem downstream constructs the actual dependency graph.

---

15. "versioning.g4"

Owns

"versioning.g4" owns the syntax necessary to express language/package/module/dependency version information where that syntax is part of Zamani source.

It must distinguish, where applicable:

language version
module/package version
dependency requirement
compatibility declaration
feature version
dialect version

These are different semantic concepts even if they share version syntax.

Does not own

It does not:

- decide compatibility;
- resolve dependency versions;
- perform migration;
- select package versions;
- enforce registry policy.

Those belong to compatibility, package, dependency, and semantic infrastructure.

---

16. "module-attributes.g4"

Owns

"module-attributes.g4" owns syntax specifically associated with module declarations.

Examples may include attributes describing:

- module status;
- module role;
- experimental/stable status;
- module-level semantic properties;
- explicit language features associated with a module.

Does not own

It does not own the meaning of those attributes.

It does not:

- execute attributes;
- select hardware;
- alter runtime behavior directly;
- bypass semantic validation.

General-purpose attributes remain under the canonical attribute infrastructure.

---

17. Canonical Names

Module grammar must never independently redefine identifier or qualified-name syntax.

The intended architecture is:

grammar/lexer/
        │
        ▼
canonical identifiers/tokens
        │
        ▼
grammar/core/names.g4
        │
        ▼
canonical qualified names
        │
        ├── modules
        ├── imports
        ├── exports
        ├── aliases
        ├── namespaces
        ├── declarations
        ├── functions
        ├── types
        ├── quantum
        ├── hardware
        ├── HDL
        ├── distributed
        └── future domains

For example:

quantum::algorithms::optimization

must be represented using the canonical name infrastructure.

"modules.g4" may wrap a qualified name as a module name, but must not redefine qualified-name syntax.

---

18. Module Names Are Not Paths by Default

The syntax:

module quantum::algorithms;

does not inherently mean:

filesystem/quantum/algorithms

It does not inherently mean:

package://quantum/algorithms

It does not inherently mean:

hardware://quantum/algorithms

It is a language-level name.

A source provider may later map that name to:

- files;
- generated sources;
- packages;
- archives;
- registries;
- virtual sources;
- embedded sources;
- remote sources;

according to separate semantic/toolchain rules.

---

19. Module Identity

The semantic layer must distinguish at least:

module identity
namespace identity
package identity
source identity
artifact identity
deployment identity
runtime identity

The parser records syntax.

It must not collapse these identities prematurely.

---

20. Module Nesting

Module nesting must be unbounded by language design.

The grammar must not define:

MAX_MODULE_DEPTH

or equivalent.

Valid structural forms may include:

module a;

module a::b;

module a::b::c;

and arbitrarily deeper qualified structures supported by the parser/resource implementation.

No finite language-level maximum is permitted.

---

21. Module Contents

A module body must use the canonical Zamani item/declaration architecture.

Conceptually:

module
    ↓
module body
    ↓
canonical items

rather than:

module
    ↓
special module-only declarations

This is essential because modules must be able to contain future domains without repeatedly modifying the module subsystem.

A module can therefore contain, subject to semantic rules:

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
future declarations

The module grammar does not need to know the implementation semantics of those constructs.

---

22. Aggregate Grammar Responsibility

"grammar/Zamani.g4" is the composition root.

The aggregate grammar is responsible for connecting the independent grammar components.

Conceptually:

Zamani.g4
│
├── core
├── lexer vocabulary
├── names
├── types
├── expressions
├── statements
├── declarations
├── functions
├── effects
├── memory
├── concurrency
├── modules
│   ├── modules.g4
│   ├── imports.g4
│   ├── exports.g4
│   ├── visibility.g4
│   ├── namespaces.g4
│   ├── aliases.g4
│   ├── packages.g4
│   ├── dependencies.g4
│   ├── versioning.g4
│   └── module-attributes.g4
├── classical
├── quantum
├── hybrid
├── HDL
├── hardware
├── resources
├── distributed
├── AI
├── data
├── networking
├── security
├── compile
├── execution
├── interoperability
├── dialects
├── macros
└── metaprogramming

The module files must not independently create competing complete Zamani parsers.

---

23. Lexer Integration

The module grammar is parser syntax.

Lexical ownership belongs to the canonical lexer subsystem.

The module subsystem consumes canonical tokens for concepts such as:

module
import
export
package
namespace
dependency
version
visibility
attributes
identifiers
qualified names
strings
punctuation
braces
parentheses
commas
terminators

Exact token names are determined by the canonical lexer/token contract.

Module grammar files must not silently introduce duplicate lexical authorities.

---

24. Token-Vocabulary Rule

The repository must have one canonical parser/lexer token vocabulary.

The module subsystem must not create a competing token universe.

Any historical discrepancy such as different "tokenVocab" declarations must be resolved at the aggregate grammar architecture level.

The production target is:

one canonical lexical vocabulary
        ↓
all parser components

not:

modules → token vocabulary A
quantum → token vocabulary B
HDL → token vocabulary C

unless those are explicitly generated views of one authoritative vocabulary.

---

25. Frontend AST Contract

The parser must provide enough structure for the frontend AST to represent module declarations without losing semantic information.

At minimum, the conceptual module AST must preserve:

ModuleDeclaration
├── attributes
├── visibility
├── name
├── body
└── source span

The AST should preserve:

- source order;
- source spans;
- canonical name segments;
- explicit visibility;
- attributes;
- body presence;
- child items;
- provenance required for diagnostics;
- information necessary for tooling.

The grammar must not define Rust AST structures.

---

26. AST Boundary

The module parser must lower into the existing domain-neutral frontend AST architecture.

It must not introduce:

ModuleIR
QuantumModuleIR
HardwareModuleIR
PackageIR

merely because a module contains a particular domain.

The module AST represents source organization.

Domain IR represents computation.

---

27. Semantic Ownership

Semantic analysis owns:

- module identity;
- module uniqueness;
- module nesting semantics;
- namespace relationships;
- package relationships;
- visibility;
- import resolution;
- export validation;
- alias resolution;
- dependency graph construction;
- dependency-cycle detection;
- package compatibility;
- version compatibility;
- symbol resolution;
- declaration accessibility;
- module attribute interpretation;
- source-provider resolution;
- compilation-unit semantics.

None of these should be encoded as parser-side semantic execution.

---

28. Dependency Graph Semantics

The module system must support arbitrarily large dependency graphs subject only to actual implementation resources.

The language must not impose:

MAX_DEPENDENCIES
MAX_IMPORTS
MAX_MODULES
MAX_PACKAGES
MAX_GRAPH_DEPTH
MAX_GRAPH_WIDTH

The dependency graph may contain:

A → B
A → C
B → D
C → D

and more complex structures.

The semantic layer must detect invalid cycles where the language/package model prohibits them.

Graph size is a resource concern, not a language semantic limit.

---

29. Cycles

The parser must not attempt to solve dependency cycles.

It merely parses the source.

Semantic/package resolution must determine whether cycles are:

- legal;
- illegal;
- conditionally legal;
- permitted only through interfaces;
- permitted for certain dependency classes.

Diagnostics must identify the actual semantic cycle rather than reporting a syntax error.

---

30. Imports and Exports

The conceptual relationship is:

module
 │
 ├── imports
 │      ↓
 │   name/module resolution
 │
 └── exports
        ↓
     public interface

Import and export syntax remain separate owners.

Visibility remains a separate owner.

Aliases remain a separate owner.

---

31. Visibility Versus Export

These concepts must never be silently collapsed.

For example:

pub fn compute() {
    ...
}

and:

export compute;

may have related effects, but they represent different source-level concepts unless the language specification explicitly defines them as equivalent.

The grammar preserves the distinction.

Semantic analysis determines the actual relationship.

---

32. Package Versus Module

The language must preserve:

package ≠ module

A package may contain many modules.

A package may contain multiple computational domains.

A package is not automatically:

- one machine;
- one executable;
- one process;
- one deployment;
- one hardware target.

---

33. Namespace Versus Module

The language must preserve:

namespace ≠ module

A namespace is primarily a naming/scope abstraction.

A module is a source organization/compilation abstraction.

They may be associated semantically, but the grammar must not assume they are identical.

---

34. Module Versus Filesystem

The language must preserve:

module ≠ file
module ≠ directory

A toolchain may map modules to source files or directories, but that is a source-provider/tooling decision.

This allows future source providers without changing the language.

---

35. Module Versus Deployment

A module must not imply deployment.

One module may eventually be:

- inlined;
- linked;
- statically compiled;
- dynamically loaded;
- replicated;
- partitioned;
- distributed;
- synthesized into hardware;
- lowered into quantum execution;
- embedded into firmware;
- executed on an accelerator.

Those are compiler/runtime/deployment decisions.

---

36. POCO-REAF

The module system is foundational to:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

Module organization must remain valid when the program scales across:

tiny embedded system
        ↓
single CPU
        ↓
multicore CPU
        ↓
GPU
        ↓
FPGA
        ↓
ASIC
        ↓
QPU
        ↓
heterogeneous accelerator
        ↓
cluster
        ↓
HPC system
        ↓
distributed system
        ↓
cloud
        ↓
future computational architecture

Changing the target must not require rewriting module structure merely because the target has different:

- processor counts;
- memory capacity;
- vector widths;
- GPU counts;
- FPGA resources;
- qubit counts;
- communication topology;
- accelerator counts;
- node counts.

---

37. No Artificial Resource Limits

The module grammar must never encode language-level limits such as:

MAX_MODULES
MAX_MODULE_DEPTH
MAX_IMPORTS
MAX_EXPORTS
MAX_DEPENDENCIES
MAX_PACKAGES
MAX_NAMESPACES
MAX_TARGETS
MAX_DEVICES
MAX_NODES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_QUBITS
MAX_MEMORY

There must be no grammar production equivalent to:

moduleCount <= 1024

or:

moduleDepth <= 64

or:

imports <= 256

unless such a limit is explicitly part of an external implementation/resource policy rather than the language definition.

---

38. Tiny-to-Large Scalability

The module system must support programs ranging from:

one module

to:

many modules
many packages
many namespaces
many dependency edges
many computational domains

subject only to available implementation resources.

A tiny program must not pay for unnecessary module machinery semantically.

A large program must not require a different module language.

---

39. Compile-Time Resource Limits

Implementations may impose operational limits for:

- memory;
- parsing time;
- dependency-resolution time;
- recursion depth;
- compiler workers;
- cache size;
- filesystem capacity.

Those are implementation policies.

They must be:

- explicit;
- diagnosable;
- configurable where appropriate;
- distinguishable from language semantics.

A resource exhaustion error must not be reported as a grammar incompatibility.

---

40. Resource Requirements Are Not Module Semantics

If a module contains:

requires capability("quantum.measurement");

that requirement belongs to the resource/capability subsystem.

The module grammar should not interpret it as:

this module requires QPU #0

Likewise:

requires memory(required_memory);

must not imply a fixed physical memory device.

The distinction is:

semantic requirement
        ≠
physical realization

---

41. Module Placement

Module placement is not grammar semantics.

A module may later be:

host
device
accelerator
QPU
FPGA
distributed worker
cloud service
embedded target

but the module syntax itself remains target-independent.

Placement is determined downstream using:

- capabilities;
- requirements;
- constraints;
- preferences;
- compiler decisions;
- resource availability.

---

42. Classical Integration

A module may contain classical constructs.

Those constructs ultimately participate in the classical semantic/IR pipeline.

The module layer must not know whether a classical function will eventually run on:

- one CPU;
- many CPUs;
- GPU;
- accelerator;
- embedded processor;
- future processor.

---

43. Quantum Integration

A module may contain quantum constructs.

The module layer must not:

- enumerate quantum gates;
- allocate qubits;
- select physical qubits;
- select QPU topology;
- perform routing;
- perform scheduling;
- perform QEC;
- perform calibration.

Quantum constructs eventually reach the canonical:

quantum::ir

boundary.

The module system must not create a second quantum IR.

---

44. HDL and Hardware Integration

A module may contain HDL and hardware/software co-design constructs.

The module system must not encode:

fixed FPGA size
fixed bus width
fixed register count
fixed number of logic elements
fixed number of hardware modules
fixed device address

Hardware requirements and capabilities belong to the hardware/resource subsystems.

---

45. AI and Data Integration

Modules may contain AI/ML/data constructs.

The module grammar must not depend on:

- PyTorch;
- TensorFlow;
- JAX;
- CUDA;
- vendor-specific AI runtimes;
- a fixed tensor accelerator.

Framework interoperability belongs under interoperability/dialect/toolchain infrastructure.

---

46. Distributed Integration

A module may participate in distributed computation.

The module system must not encode:

MAX_NODES
NODE_0
NODE_1
NODE_2

or assume one module equals one distributed process.

Deployment and placement are downstream.

---

47. Networking Integration

Module imports are language dependencies.

They must not be confused with runtime network communication.

The same module may be compiled:

locally

or:

distributed

without changing its fundamental module identity.

---

48. Security Integration

Module visibility and exports interact with security, but the module grammar must not become the complete security model.

Semantic/security layers may later validate:

- capability access;
- authority;
- package trust;
- signed dependencies;
- sandboxing;
- FFI permissions;
- external resource permissions.

The parser only recognizes the language syntax.

---

49. Interoperability

Foreign modules and formats must enter through the interoperability boundary.

Examples may include:

Rust
C
C++
Python
WASM
OpenQASM
QIR
HDL
other future formats

These are interoperability concerns.

They must not create a second Zamani module language.

---

50. Dialects

A dialect may extend module syntax only through an explicit dialect mechanism.

A dialect must identify:

name
version
syntax extension
semantic extension
AST mapping
IR mapping
compatibility
capabilities
portability classification

A dialect must never silently change the meaning of stable core module syntax.

---

51. Versioning

Module/package/dependency version syntax belongs to "versioning.g4".

Compatibility semantics belong downstream.

Version handling must support:

language compatibility
module compatibility
package compatibility
dependency compatibility
dialect compatibility

without imposing artificial counts or limits.

---

52. Backward Compatibility

Existing valid Zamani module syntax must not be broken merely because the module grammar is being modularized.

Migration must follow:

existing syntax
    ↓
inventory
    ↓
canonical ownership
    ↓
modular grammar
    ↓
AST compatibility
    ↓
semantic compatibility
    ↓
regression tests
    ↓
remove duplicate authority

A syntax change must document:

- old form;
- new form;
- language version;
- compatibility status;
- migration;
- diagnostics;
- AST effect;
- semantic effect;
- tooling effect.

---

53. Legacy Monolithic Grammar

The current "grammar/Zamani.g4" contains historical/module-related grammar.

The migration must not simply delete those rules.

For every legacy module construct:

1. identify its current syntax;
2. identify its consumers;
3. identify duplicate definitions;
4. compare against the modular owner;
5. preserve valid behavior;
6. correct ambiguity;
7. establish one owner;
8. update aggregate composition;
9. update parser implementation;
10. update AST mapping;
11. update semantic resolution;
12. add regression tests;
13. remove the duplicate only after migration is verified.

The end state must have one authoritative syntax owner.

---

54. Parser Determinism

Module parsing must be deterministic.

The module grammar must not depend on:

- filesystem state;
- package registry state;
- environment variables;
- current time;
- network responses;
- random values;
- hardware discovery;
- runtime state.

The same source and lexical input must produce the same syntactic interpretation.

---

55. Semantic Determinism

Where the language promises deterministic semantic resolution, the resolver must define deterministic rules for:

- module identity;
- import resolution;
- alias resolution;
- export resolution;
- dependency selection;
- namespace lookup;
- ambiguity diagnostics.

The grammar itself does not perform these operations.

---

56. Source Spans and Diagnostics

Every module-system AST node must preserve source location information sufficient to diagnose:

- malformed module declarations;
- invalid imports;
- invalid exports;
- invalid aliases;
- invalid visibility;
- unresolved modules;
- ambiguous names;
- dependency conflicts;
- version incompatibility;
- visibility violations.

Syntax errors and semantic errors must remain distinguishable.

For example:

invalid module syntax

must not be reported as:

module dependency unavailable

and:

required package not found

must not be reported as:

invalid Zamani syntax

---

57. Error-Recovery Requirements

The parser should support useful recovery where the underlying parser architecture permits it.

A malformed module must not unnecessarily destroy diagnostics for unrelated declarations.

Recovery must not alter accepted semantics.

Examples worth testing include:

missing module name
missing terminator
missing closing brace
malformed qualified name
malformed import
malformed export
malformed alias
malformed dependency
malformed version

---

58. Security Boundary

The module grammar must contain no execution behavior.

It must never:

- execute shell commands;
- read arbitrary files;
- access secrets;
- access hardware;
- access a network;
- mutate global compiler state;
- invoke package registries;
- perform dependency downloads.

The parser parses.

The semantic/toolchain infrastructure decides what the parsed constructs mean and what operations are permitted.

---

59. Rust Safety Contract

The Zamani implementation must use safe Rust.

Production code must contain no:

unsafe

including:

unsafe fn
unsafe impl
unsafe trait
unsafe {
    ...
}

unless a future repository-wide policy explicitly creates a narrowly documented exception; the current module-system requirement is no unsafe Rust.

The implementation baseline is:

Rust 1.97
Rust 1.97.1

The grammar files themselves contain no embedded Rust actions.

---

60. No Embedded Semantic Actions

Module ".g4" files should not perform semantic work through embedded actions or predicates that duplicate Rust semantic infrastructure.

Avoid embedding:

- filesystem logic;
- symbol resolution;
- dependency resolution;
- resource discovery;
- target discovery;
- hardware selection;
- IR construction;
- runtime behavior.

The grammar remains declarative.

---

61. Canonical IR Boundary

The module subsystem does not create executable IR.

Its information is preserved in the AST and semantic model until the compiler determines which parts affect generated computation.

For example:

module declaration
    ↓
AST module declaration
    ↓
semantic module environment
    ↓
resolved declarations
    ↓
canonical computation IR

The module container itself should not become an artificial runtime IR object unless a downstream subsystem genuinely requires such a semantic representation.

---

62. Quantum IR Boundary

Quantum declarations inside modules must eventually reach the existing canonical:

quantum::ir

pipeline.

The architecture remains:

Zamani source
    ↓
module syntax
    ↓
domain-neutral AST
    ↓
semantic analysis
    ↓
quantum semantic lowering
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
QEC/resilience
    ↓
ZQN
    ↓
HAL
    ↓
target

The module grammar must never create a competing quantum IR.

---

63. Resource and Capability Boundary

Modules may contain declarations that ultimately require capabilities.

The distinction must remain:

module
    ≠
resource requirement
    ≠
capability
    ≠
hardware
    ≠
deployment

For example:

requires capability("quantum.measurement");

expresses a capability requirement.

It does not encode:

QPU = device 3

Similarly:

requires capability("gpu.compute");

does not select a particular GPU.

---

64. Target Independence

Module syntax must remain meaningful without knowing whether the eventual target is:

- x86;
- ARM;
- RISC-V;
- GPU;
- FPGA;
- ASIC;
- QPU;
- simulator;
- accelerator;
- cluster;
- cloud;
- future hardware.

Target-specific behavior belongs downstream.

---

65. Module Metadata

Module metadata must be represented structurally and semantically.

It may eventually include information concerning:

- stability;
- version;
- compatibility;
- capabilities;
- effects;
- resource requirements;
- provenance;
- security;
- domain classification.

However, metadata must not silently become executable compiler behavior merely because it is syntactically present.

---

66. Feature Manifest Integration

Where the repository adopts machine-readable feature manifests, the module feature must have a complete contract covering:

feature ID
feature name
status
language version
grammar owner
lexer requirements
AST mapping
semantic rules
IR mapping
compiler consumers
runtime consumers
diagnostics
compatibility
positive tests
negative tests
boundary tests
scalability tests
determinism tests
hard-coding audit

This ensures a module-system feature can be completed independently rather than requiring redesign after another subsystem is changed.

---

67. Independent Completion Contract

Every file under "grammar/modules/" must be independently completable.

Before a file is marked complete, its contract must answer:

Purpose

What exact language concept does this file define?

Owns

Which syntax belongs exclusively to this file?

Does not own

Which nearby concepts must remain elsewhere?

Inputs

Which canonical tokens/rules does it consume?

Outputs

Which parse-tree structures does it expose?

Dependencies

Which grammar components must already exist?

AST

What AST representation consumes the parse-tree result?

Semantics

What semantic subsystem interprets it?

IR

Does it affect canonical IR, and if so how?

Compiler

Which compiler subsystem consumes its semantic result?

Runtime

Which runtime subsystem may eventually consume its effects?

Cross-domain

How does it interact with classical, quantum, HDL, hardware, AI, distributed, and future domains?

Diagnostics

What errors can it produce?

Compatibility

What existing syntax must remain supported?

Determinism

What deterministic behavior is required?

Scalability

What happens from tiny programs to extremely large programs?

Hard-coding audit

Does it introduce any artificial physical/resource limit?

Tests

What positive, negative, boundary, scalability, determinism, and compatibility tests prove completion?

---

68. Dependency Order

The module subsystem should be completed in dependency order.

Recommended order:

canonical lexical vocabulary
        ↓
canonical identifiers/names
        ↓
qualified names
        ↓
visibility
        ↓
module attributes
        ↓
modules
        ↓
aliases
        ↓
namespaces
        ↓
imports
        ↓
exports
        ↓
versioning
        ↓
packages
        ↓
dependencies
        ↓
aggregate Zamani composition
        ↓
AST
        ↓
semantic resolution
        ↓
compiler/toolchain integration

This ordering minimizes later re-editing.

---

69. File Completion Order

For the current directory, the recommended independent-first order is:

1. "visibility.g4"
2. "module-attributes.g4"
3. "modules.g4"
4. "aliases.g4"
5. "namespaces.g4"
6. "imports.g4"
7. "exports.g4"
8. "versioning.g4"
9. "packages.g4"
10. "dependencies.g4"
11. "README.md" final conformance pass

However, each file must first be checked against already-existing repository contracts rather than assuming these files are blank.

---

70. "visibility.g4" Completion Gate

Complete only when:

- one visibility vocabulary exists;
- canonical tokens are used;
- no duplicate visibility grammar exists elsewhere;
- AST representation is known;
- semantic accessibility rules are defined;
- declaration consumers are identified;
- module consumers are identified;
- diagnostics are defined;
- compatibility is defined;
- tests exist;
- no hardware limits exist.

---

71. "module-attributes.g4" Completion Gate

Complete only when:

- module-specific attribute syntax is defined;
- general attributes are delegated correctly;
- AST representation is defined;
- semantic interpretation is identified;
- diagnostics are defined;
- unsupported attributes are distinguishable from malformed syntax;
- compatibility is documented;
- tests exist.

---

72. "modules.g4" Completion Gate

Complete only when:

- module declaration syntax is unambiguous;
- module names use canonical names;
- visibility is delegated;
- attributes are delegated;
- module bodies use canonical items;
- module nesting is scalable;
- no fixed module limits exist;
- source spans are preserved;
- AST mapping exists;
- semantic ownership is defined;
- parser integration exists;
- regression tests exist;
- scalability tests exist;
- hard-coding audit passes.

---

73. "aliases.g4" Completion Gate

Complete only when:

- alias syntax is canonical;
- alias targets use canonical names/references;
- alias semantics are externalized;
- ambiguity rules are defined;
- AST mapping exists;
- import/export integration is defined;
- diagnostics exist;
- compatibility tests exist.

---

74. "namespaces.g4" Completion Gate

Complete only when:

- namespace syntax is distinct from modules;
- canonical names are reused;
- nesting is scalable;
- namespace identity is semantic;
- AST mapping exists;
- resolution ownership is defined;
- module/package interaction is defined;
- tests exist.

---

75. "imports.g4" Completion Gate

Complete only when:

- all supported import forms are specified;
- aliases are integrated;
- canonical names are used;
- wildcard behavior is specified;
- import resolution is explicitly downstream;
- dependency graph integration is defined;
- AST mapping exists;
- diagnostics exist;
- compatibility tests exist.

---

76. "exports.g4" Completion Gate

Complete only when:

- export syntax is canonical;
- re-export semantics are defined downstream;
- aliases integrate correctly;
- visibility remains distinct;
- AST mapping exists;
- diagnostics exist;
- compatibility tests exist.

---

77. "versioning.g4" Completion Gate

Complete only when:

- version syntax is canonical;
- language/package/dependency versions are distinguishable;
- semantic compatibility ownership is explicit;
- migration ownership is explicit;
- diagnostics are defined;
- version limits are not artificial;
- tests exist.

---

78. "packages.g4" Completion Gate

Complete only when:

- package identity syntax is defined;
- package/module distinction is preserved;
- package metadata is structurally represented;
- dependency integration is defined;
- registry access remains downstream;
- package size/count limits are absent from syntax;
- tests exist.

---

79. "dependencies.g4" Completion Gate

Complete only when:

- dependency declarations are complete;
- dependency aliases integrate;
- version requirements integrate;
- dependency source information is defined;
- graph construction is downstream;
- cycle detection is downstream;
- package retrieval is downstream;
- security verification is downstream;
- tests exist.

---

80. Production Tests

The module subsystem requires more than happy-path parsing.

The test matrix must contain:

tests/
├── modules/
│   ├── positive/
│   ├── negative/
│   ├── boundary/
│   ├── scalability/
│   ├── determinism/
│   └── compatibility/

---

81. Positive Tests

At minimum, test:

module math;

module math::linear;

module math::linear::matrix;

pub module quantum::algorithms {
    ...
}

and modules containing different domain constructs.

---

82. Negative Tests

Test:

- missing module name;
- invalid name;
- invalid qualified name;
- malformed visibility;
- malformed attributes;
- malformed import;
- malformed export;
- malformed alias;
- malformed dependency;
- malformed version;
- unterminated module body;
- unexpected tokens.

---

83. Boundary Tests

Test:

- empty modules;
- one-item modules;
- deeply nested qualified names;
- many imports;
- many exports;
- many aliases;
- many dependencies;
- nested modules;
- mixed-domain modules;
- large module bodies.

No test may establish a language-level maximum merely because the test fixture uses a particular size.

---

84. Scalability Tests

Scalability tests must verify that module semantics remain structurally valid as the source grows.

Test progressively larger:

module counts
qualified-name depths
imports
exports
aliases
dependencies
nested declarations
package graphs
cross-domain declarations

The purpose is to verify correctness and resource behavior, not to establish artificial language limits.

---

85. Determinism Tests

Repeatedly parse identical source and verify stable:

- tokenization;
- parse structure;
- source spans;
- diagnostics where ordering is specified;
- AST structure.

Semantic resolution should likewise have deterministic rules where required by the language.

---

86. Compatibility Tests

The module subsystem must test:

legacy module syntax
current syntax
deprecated syntax
versioned syntax
package/module interaction
import/export compatibility
alias compatibility
namespace compatibility

The compatibility matrix must identify whether each feature is:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

in accordance with the repository-wide "grammar.md" model.

---

87. Cross-Domain Tests

A module should be able to organize constructs from different domains without the module subsystem needing domain-specific semantics.

Test combinations such as:

classical + quantum
classical + HDL
quantum + hardware
AI + data
distributed + networking
quantum + classical + hardware
AI + accelerator
embedded + hardware
future dialect + core language

The module grammar must remain unchanged merely because new domains are added.

---

88. Hard-Coding Audit

Every module grammar file must be checked for accidental limits.

Reject universal language constructs equivalent to:

MAX_MODULES
MAX_IMPORTS
MAX_EXPORTS
MAX_DEPENDENCIES
MAX_PACKAGES
MAX_NAMESPACES
MAX_MODULE_DEPTH
MAX_MODULE_ITEMS

Also reject target-specific assumptions such as:

CPU_COUNT
GPU_COUNT
FPGA_COUNT
QPU_COUNT
QUBIT_COUNT
NODE_COUNT
MEMORY_SIZE
DEVICE_ID
DEVICE_ADDRESS

when they are presented as universal language limits.

---

89. Valid Constants Versus Invalid Limits

Not every numeric value is forbidden.

This is valid:

const package_count = 1000;

because it is program data.

This is not valid as a universal language constraint:

MAX_MODULES = 1000

Likewise, a compiler implementation may have a configurable resource budget.

That does not make the budget part of Zamani language semantics.

---

90. No Physical Topology in Module Syntax

Module syntax must not encode:

GPU 0
CPU 7
QPU 3
qubit 42
node 12
memory bank 2
device address 0x...

unless explicitly represented as a target-specific interoperability/dialect construct.

Portable module syntax must remain independent of physical topology.

---

91. Module Resource Independence

The same module graph must be usable when:

resources are scarce

and when:

resources are abundant

The compiler may choose different realizations.

The source module organization remains the same.

---

92. Compilation and Deployment Independence

A module should not have to be rewritten merely because compilation changes from:

debug → release
CPU → GPU
single-target → cross-target
local → distributed
simulator → hardware

Compilation profiles and deployment policies belong downstream.

---

93. Reproducibility

Module resolution must support reproducible builds where the package/toolchain contract requires it.

Reproducibility belongs to:

- package metadata;
- dependency resolution;
- version selection;
- lock information;
- source provenance;
- compiler configuration.

The parser itself must remain deterministic and side-effect free.

---

94. Provenance

The semantic/toolchain layer should preserve provenance sufficient to identify:

- source module;
- package;
- version;
- dependency;
- dialect;
- source span;
- generated artifact;
- external source where applicable.

The module grammar only needs to preserve the source structures required to establish that provenance.

---

95. Tooling Integration

The module AST must support tooling such as:

- diagnostics;
- go-to-definition;
- find references;
- import management;
- export inspection;
- dependency visualization;
- module graph visualization;
- refactoring;
- formatting;
- documentation generation;
- build planning;
- IDE/LSP integration.

Tooling must consume canonical AST/semantic information rather than reparsing module syntax through a second implementation.

---

96. Documentation Integration

The module system must be represented consistently in:

grammar/DESIGN.md
grammar/README.md
grammar/specification/
grammar/spec/
grammar/reference/
grammar/grammar.md

Documentation must not invent syntax that the canonical grammar does not support.

---

97. "grammar.md" Integration

"grammar/grammar.md" should eventually report module features using the repository-wide status model:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

The module README does not override implementation status.

The actual status must be derived from repository evidence.

---

98. "Zamani-Grammar.md" Integration

"Zamani-Grammar.md" may contain historical or proposed module concepts.

Those concepts must not automatically become legal syntax.

Promotion must follow:

proposal
   ↓
language specification
   ↓
AST contract
   ↓
canonical grammar
   ↓
semantic implementation
   ↓
IR/compiler integration where required
   ↓
tests
   ↓
stable feature

---

99. No Duplicate Module Language

The repository must not eventually contain:

Zamani.g4 module syntax
modules.g4 module syntax
parser.rs module syntax
some other module parser

as independent authorities.

There may be multiple implementations or generated representations, but they must implement one canonical language contract.

---

100. Parser Implementation Integration

The actual Rust frontend currently has:

src/lexer.rs
src/parser.rs
src/ast/

The module grammar must be reconciled against those implementations.

The integration process is:

module grammar
    ↓
ANTLR parse contract
    ↓
Rust lexer token compatibility
    ↓
Rust parser compatibility
    ↓
frontend AST mapping
    ↓
semantic module resolution

A grammar change is not production-complete until the Rust frontend agrees with it.

---

101. AST Integration With Existing Architecture

The module subsystem must use the repository's existing AST architecture rather than introducing a parallel AST.

The existing AST already provides generic concepts such as identifiers, spans, expressions, types, declarations, and program structure.

Module integration should extend or reuse those concepts rather than creating unrelated representations.

---

102. No Domain-Specific Module ASTs

Do not create:

QuantumModule
GpuModule
HdlModule
AiModule
DistributedModule

merely because a module contains a domain.

The module is still a module.

Its contents determine its domain semantics.

---

103. Quantum Example

A module may contain quantum computation:

module quantum::algorithms {
    ...
}

The module system provides:

name
scope
organization
imports
exports
visibility
dependencies

The quantum subsystem provides:

quantum semantics
quantum operations
quantum types
measurement
resilience
QEC intent

The canonical quantum compiler pipeline provides:

quantum::ir
optimization
routing
scheduling
ZQN
HAL
target realization

These responsibilities must remain separate.

---

104. HDL Example

A module may contain HDL/hardware constructs:

module hardware::accelerators {
    ...
}

The module system does not decide:

FPGA size
register count
physical pin numbers
clock frequency
device ID

unless explicitly expressed through a target-specific downstream construct.

---

105. Distributed Example

A module may contain distributed computation:

module distributed::services {
    ...
}

This does not mean:

N nodes

or:

one module = one node

Placement and deployment are downstream.

---

106. AI Example

A module may contain AI computation:

module ai::models {
    ...
}

The module grammar remains independent of the AI framework or accelerator.

---

107. Future Domain Rule

Adding a new computational domain must not require redesigning the fundamental module abstraction.

The required integration should normally be:

new domain grammar
        ↓
canonical item/declaration integration
        ↓
AST
        ↓
semantic domain model
        ↓
domain IR

rather than:

rewrite modules.g4
rewrite imports.g4
rewrite exports.g4

This is a central extensibility requirement.

---

108. Maintainability Rule

Do not create files merely to make the directory tree look complete.

Every module grammar file must have:

- one clear owner;
- a documented interface;
- a known upstream dependency;
- known downstream consumers;
- tests;
- completion criteria.

If a new concept does not need a separate grammar component, it should not receive one merely for organizational symmetry.

---

109. Circular Dependency Rule

The module grammar must not create circular grammar ownership such as:

modules → imports → modules → imports

where both files redefine each other's syntax.

Instead:

modules
imports
exports
aliases
namespaces
packages
dependencies
versioning

are independent components composed by the aggregate grammar.

Semantic relationships may be cyclic at the model level where the language permits them, but grammar ownership must remain clear.

---

110. Grammar Dependency Direction

The preferred dependency direction is:

lexer
  ↓
core names/tokens
  ↓
module components
  ↓
aggregate parser
  ↓
AST
  ↓
semantic analysis
  ↓
IR
  ↓
compiler/runtime

Never:

grammar/modules
        ↓
runtime
        ↓
hardware
        ↓
lexer

The grammar must remain upstream.

---

111. Runtime Independence

No module grammar construct should require the parser to know:

- runtime scheduler state;
- runtime resource availability;
- current hardware;
- current calibration;
- current cluster state;
- current network state.

Those are runtime concerns.

---

112. Compiler Independence From Physical Topology

A module graph must remain valid if the compiler changes its realization strategy.

For example:

one module

could be:

inlined

or:

compiled separately

or:

distributed

or:

lowered into accelerator code

depending on compiler decisions.

The module syntax does not prescribe the realization.

---

113. Module Graph and POCO-REAF

The module graph is part of the portable semantic program.

The physical deployment graph is not necessarily identical to the module graph.

Therefore:

module graph
    ≠
process graph
    ≠
hardware topology
    ≠
network topology

This distinction is essential for true scalability.

---

114. Optimization Independence

Compiler optimization may:

- inline modules;
- eliminate unused declarations;
- specialize generic code;
- partition code;
- fuse operations;
- distribute computation;
- place computations on accelerators.

None of those transformations should require changing the source module syntax.

---

115. Resource Negotiation

The intended architecture is:

source module
    ↓
semantic requirements
    ↓
capability/resource model
    ↓
target capability discovery
    ↓
resource negotiation
    ↓
planning
    ↓
lowering

The module grammar does not perform negotiation.

---

116. Error Categories

The implementation should distinguish at least:

lexical error
syntax error
AST construction error
name-resolution error
module-resolution error
visibility error
import error
export error
alias error
namespace error
package error
dependency error
version error
capability error
resource error
target compatibility error
runtime error

This prevents unrelated failures from being reported as grammar failures.

---

117. Production Validation

The module subsystem is production-ready only when all required layers agree:

specification
    ✓
grammar
    ✓
lexer
    ✓
parser
    ✓
AST
    ✓
semantic analysis
    ✓
IR where applicable
    ✓
compiler
    ✓
tooling
    ✓
tests
    ✓
documentation
    ✓
compatibility
    ✓

A grammar file passing ANTLR generation alone is insufficient.

---

118. Required Repository Checks

The final repository validation must include the actual project commands for:

formatting
compilation
tests
linting
grammar generation/validation
frontend conformance
integration tests

For the Rust implementation, the intended baseline is Rust 1.97/1.97.1 with no unsafe code.

The repository's actual "Cargo.toml" and CI configuration determine the exact commands and version declaration.

---

119. No-Unsafe Validation

The production pipeline must include a repository-level no-unsafe check.

At minimum, production Rust must reject accidental introduction of:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {

The preferred Rust-level enforcement is:

#![forbid(unsafe_code)]

at applicable crate boundaries, supplemented by repository CI auditing.

---

120. Definition of Complete Module Grammar

The module subsystem is complete only when:

- every module grammar file has one owner;
- duplicate ownership is removed;
- canonical names are reused;
- visibility is canonical;
- attributes are canonical;
- imports are independent;
- exports are independent;
- aliases are independent;
- namespaces are independent;
- packages are independent;
- dependencies are independent;
- versioning is independent;
- module bodies use canonical Zamani items;
- aggregate grammar composition is defined;
- lexer integration is defined;
- AST integration is defined;
- semantic integration is defined;
- compiler integration is defined;
- runtime integration is defined;
- tooling integration is defined;
- compatibility is defined;
- diagnostics are defined;
- deterministic behavior is defined;
- scalability is tested;
- no hard-coded resource limits exist;
- no physical topology is embedded into portable module semantics;
- no second module language exists;
- no second AST authority exists;
- no second IR authority exists.

---

121. What This Directory Must Never Own

"grammar/modules/" must never become responsible for:

filesystem access
package downloading
dependency solving
symbol resolution
runtime loading
hardware discovery
resource allocation
device selection
qubit allocation
GPU selection
CPU selection
FPGA selection
routing
scheduling
QEC implementation
ZQN implementation
HAL implementation
optimization
execution

It describes source organization.

---

122. What Downstream Systems Must Own

Concern| Owner
Tokens| "grammar/lexer/"
Identifiers| canonical core/name grammar
Module syntax| "grammar/modules/modules.g4"
Import syntax| "grammar/modules/imports.g4"
Export syntax| "grammar/modules/exports.g4"
Visibility syntax| "grammar/modules/visibility.g4"
Namespace syntax| "grammar/modules/namespaces.g4"
Alias syntax| "grammar/modules/aliases.g4"
Package syntax| "grammar/modules/packages.g4"
Dependency syntax| "grammar/modules/dependencies.g4"
Version syntax| "grammar/modules/versioning.g4"
Module attributes| "grammar/modules/module-attributes.g4"
Grammar composition| "grammar/Zamani.g4"
Lexical implementation| "src/lexer.rs"
Parsing implementation| "src/parser.rs"
AST| "src/ast/"
Name/module resolution| semantic layer
Dependency resolution| package/toolchain layer
Type semantics| type/semantic layer
Classical IR| canonical classical IR
Quantum IR| "quantum::ir"
HDL/hardware representation| corresponding canonical semantic/IR layer
Optimization| compiler
Routing| compiler/quantum infrastructure
Scheduling| compiler/runtime infrastructure
QEC| quantum resilience/QEC subsystem
ZQN| ZQN subsystem
HAL| hardware abstraction layer
Deployment| deployment infrastructure
Runtime execution| runtime

---

123. Integration Contract With "grammar/Zamani.g4"

The root grammar must eventually expose module constructs through one canonical composition.

Conceptually:

program
    → items
        → moduleDeclaration
        → importDeclaration
        → exportDeclaration
        → package/dependency constructs
        → other canonical items

The exact root-rule names must follow the actual canonical "Zamani.g4".

The module README must not force a duplicate root grammar.

---

124. Integration Contract With "src/parser.rs"

The Rust parser must recognize the same accepted module language as the canonical grammar.

Differences must be classified as:

SPECIFICATION GAP
GRAMMAR GAP
PARSER GAP
LEGACY COMPATIBILITY
PLANNED FEATURE

They must not remain undocumented.

---

125. Integration Contract With "src/ast/"

The AST must represent module syntax without losing:

name
attributes
visibility
body
source spans
source order

Imports, exports, aliases, namespaces, packages, dependencies, and versions must similarly map into existing canonical AST abstractions or explicitly defined extensions.

---

126. Integration Contract With Semantic Analysis

The semantic layer must build a module environment capable of resolving:

module
namespace
package
import
export
alias
dependency
version

without relying on physical deployment.

---

127. Integration Contract With Compiler

The compiler may use module information for:

- compilation-unit boundaries;
- dependency ordering;
- visibility;
- symbol linkage;
- optimization;
- specialization;
- incremental compilation;
- caching;
- artifact generation;
- provenance.

It must not assume a module corresponds to a physical machine component.

---

128. Integration Contract With Runtime

Runtime systems may consume module-derived metadata for:

- loading;
- service registration;
- observability;
- provenance;
- dynamic linking;
- deployment.

Runtime behavior must not be required for parsing.

---

129. Integration Contract With Future Hardware

A new hardware target must not require changing module syntax simply because the target has a different:

CPU count
GPU count
FPGA capacity
QPU capacity
memory capacity
network topology
accelerator topology

This is a direct POCO-REAF requirement.

---

130. Production Architecture Summary

The final module architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                     CANONICAL LEXER
                              │
                              ▼
                       Zamani.g4
                     composition root
                              │
              ┌───────────────┴───────────────┐
              │                               │
              ▼                               ▼
        CORE NAME SYSTEM               MODULE SYSTEM
                                      │
                 ┌────────────────────┼────────────────────┐
                 │                    │                    │
                 ▼                    ▼                    ▼
             modules              imports              exports
                 │                    │                    │
                 ├── visibility       ├── aliases         │
                 ├── attributes      ├── names            │
                 ├── namespaces      └── dependencies    │
                 ├── packages                             │
                 └── versioning                            │
                              │
                              ▼
                         FRONTEND AST
                              │
                              ▼
                       SEMANTIC MODEL
                              │
             ┌────────────────┼─────────────────┐
             │                │                 │
             ▼                ▼                 ▼
          Classical       Quantum           HDL/Hardware
             │                │                 │
             │          quantum::ir             │
             └────────────────┼─────────────────┘
                              │
                              ▼
                         CANONICAL IR
                              │
                              ▼
                     OPTIMIZATION / LOWERING
                              │
             ┌────────────────┼─────────────────┐
             │                │                 │
             ▼                ▼                 ▼
          Routing         Scheduling        Resilience
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       TARGET REALIZATION

The important invariant is:

MODULE ORGANIZATION
        ≠
PHYSICAL DEPLOYMENT

and:

SOURCE INTENT
        ≠
RESOURCE REALIZATION

and:

GRAMMAR
        ≠
SEMANTIC MODEL
        ≠
IR
        ≠
HARDWARE

---

131. Governing POCO-REAF Rule

The module subsystem exists to make source organization portable across computational scale.

Therefore:

«Zamani modules describe how a program is logically organized, not the accidental limits or topology of the machine on which the program eventually runs.»

The same module graph must remain meaningful across:

atom
→ embedded
→ single processor
→ multicore
→ GPU
→ FPGA
→ ASIC
→ QPU
→ accelerator
→ workstation
→ server
→ cluster
→ HPC
→ distributed system
→ cloud
→ future architectures

provided the available target resources and capabilities can satisfy the program's semantic requirements or an explicitly supported alternative realization exists.

---

132. Final Non-Negotiable Rules

1. Preserve existing filenames unless a genuine architectural conflict requires a rename.
2. "grammar/Zamani.g4" remains the composition root.
3. "grammar/modules/*.g4" remains modular syntax infrastructure.
4. "grammar/grammar.md" remains implementation-conformance documentation.
5. "grammar/Zamani-Grammar.md" cannot silently introduce implemented syntax.
6. Module grammar must reuse canonical identifiers and qualified names.
7. Visibility has one canonical owner.
8. General attributes have one canonical owner.
9. Import syntax belongs to "imports.g4".
10. Export syntax belongs to "exports.g4".
11. Alias syntax belongs to "aliases.g4".
12. Namespace syntax belongs to "namespaces.g4".
13. Package syntax belongs to "packages.g4".
14. Dependency syntax belongs to "dependencies.g4".
15. Version syntax belongs to "versioning.g4".
16. Module attributes belong to "module-attributes.g4".
17. Module grammar must not resolve names.
18. Module grammar must not access files.
19. Module grammar must not access networks.
20. Module grammar must not access package registries.
21. Module grammar must not select hardware.
22. Module grammar must not allocate resources.
23. Module grammar must not perform routing.
24. Module grammar must not perform scheduling.
25. Module grammar must not implement QEC.
26. Module grammar must not implement ZQN.
27. Module grammar must not implement HAL behavior.
28. Module grammar must not construct a second quantum IR.
29. "quantum::ir" remains the canonical quantum semantic/IR boundary.
30. Modules must remain independent of physical hardware topology.
31. No fixed CPU/core/thread/GPU/FPGA/ASIC/QPU/qubit/node/memory limits may be encoded as universal language limits.
32. No fixed module/import/export/dependency/package/nesting counts may be encoded as language limits.
33. Resource constraints belong to resource/capability infrastructure.
34. Target realization belongs downstream.
35. Package identity must remain distinct from module identity.
36. Namespace identity must remain distinct from module identity.
37. Filesystem identity must remain distinct from module identity.
38. Deployment identity must remain distinct from module identity.
39. Every completed grammar feature must have an AST contract.
40. Every completed grammar feature must have a semantic contract.
41. Every completed grammar feature must have downstream integration defined.
42. Every completed feature must have positive tests.
43. Every completed feature must have negative tests.
44. Every completed feature must have boundary tests.
45. Every completed feature must have scalability tests.
46. Compatibility must be tested.
47. Determinism must be tested where required.
48. Hard-coding audits must pass.
49. Production Rust must use no "unsafe".
50. Rust 1.97/1.97.1 is the implementation baseline.
51. No empty files or directories should be created merely to make the architecture look complete.
52. A feature is not production-ready merely because its ".g4" file parses.
53. A feature is production-ready only when its complete specification → lexer → parser → AST → semantic → IR/compiler/tooling/test chain is accounted for.
54. Adding a new computational domain must not require turning the module subsystem into a domain-specific grammar.
55. The module system must remain scalable from the smallest useful program to programs constrained only by available implementation resources.

---

133. Definition of Done

"grammar/modules/" is production-ready when:

one module language
        ↓
one canonical grammar authority
        ↓
one canonical lexical vocabulary
        ↓
one canonical name system
        ↓
one domain-neutral AST
        ↓
one semantic module/dependency model
        ↓
canonical domain semantic/IR boundaries
        ↓
compiler/runtime/tooling integration
        ↓
complete conformance tests

all agree.

The decisive test is not whether the directory contains many grammar files.

The decisive test is whether a module feature can be implemented once, traced completely through the repository, validated independently, and then remain valid when the same Zamani program is compiled for a radically different machine, accelerator, quantum processor, distributed environment, or future architecture.

That is the module-system foundation required for Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
Zamani Modules Grammar

Production Architecture and Integration Contract

Path: "grammar/modules/"

Purpose: Canonical source-level grammar components for Zamani's module system.

Language: Zamani

Grammar technology: ANTLR4

Rust implementation baseline: Rust 1.97 / Rust 1.97.1

Safety requirement: Safe Rust only. No "unsafe" Rust.

Primary portability objective:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

---

1. Purpose

The "grammar/modules/" directory defines the syntax required for organizing Zamani programs into reusable, composable, independently compilable source-level units.

The module system must support Zamani as a universal computational language spanning:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL;
- hardware/software co-design;
- embedded computing;
- systems programming;
- parallel computing;
- distributed computing;
- HPC;
- AI/ML;
- scientific computing;
- data computing;
- accelerators;
- networking;
- cryptography;
- future computational domains.

The module system is deliberately domain-neutral.

A module can contain or expose declarations belonging to any supported computational domain without the module grammar needing to know whether a declaration represents:

- a classical function;
- a quantum operation;
- a logical-qubit abstraction;
- an HDL component;
- a hardware interface;
- an accelerator;
- an AI model;
- a distributed service;
- a data structure;
- or a future computational abstraction.

The module system provides organization and namespace boundaries.

It does not provide machine implementation semantics.

---

2. Architectural Position

The module grammar belongs near the beginning of the Zamani compilation pipeline:

Zamani source
     |
     v
canonical lexer
     |
     v
module parser components
     |
     +--> names
     +--> visibility
     +--> attributes
     +--> imports
     +--> exports
     +--> namespaces
     +--> packages
     +--> dependencies
     |
     v
frontend AST
     |
     v
module / package / namespace resolution
     |
     v
semantic analysis
     |
     +------------------------------+
     |                              |
     v                              v
classical semantic model       quantum semantic model
                                     |
                                     v
                                quantum::ir
     |
     +--> HDL / hardware semantic model
     +--> distributed semantic model
     +--> accelerator semantic model
     +--> future semantic models
     |
     v
canonical IR / semantic representations
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
target lowering
     |
     v
hardware / runtime / deployment

The module grammar is therefore a syntax-layer component.

It must never become a semantic execution layer.

---

3. Directory Ownership

The directory owns the syntax of the Zamani module system as a coordinated collection of independent grammar components.

grammar/modules/
├── README.md
├── modules.g4
├── imports.g4
├── exports.g4
├── visibility.g4
├── namespaces.g4
├── packages.g4
├── dependencies.g4
└── module-attributes.g4

Each file has one primary responsibility.

"modules.g4"

Owns:

- module declarations;
- module declaration headers;
- module names as wrappers around canonical qualified names;
- inline module bodies;
- module declaration boundaries;
- module-body composition.

Does not own:

- identifier syntax;
- qualified-name syntax;
- imports;
- exports;
- package syntax;
- dependency syntax;
- namespace syntax;
- visibility vocabulary;
- attributes;
- declarations belonging to other domains.

---

"imports.g4"

Owns:

- import declarations;
- import clauses;
- named imports;
- wildcard imports;
- aliases;
- import source syntax.

Does not own:

- module resolution;
- filesystem access;
- package downloading;
- dependency solving;
- symbol resolution;
- visibility semantics;
- runtime loading.

---

"exports.g4"

Owns:

- export declarations;
- export lists;
- export aliases;
- re-export syntax;
- wildcard export syntax where specified.

Does not own:

- visibility semantics;
- symbol resolution;
- ABI;
- package publication;
- runtime deployment.

---

"visibility.g4"

Owns the canonical visibility vocabulary.

It is reusable by:

- modules;
- functions;
- types;
- declarations;
- traits;
- interfaces;
- quantum declarations;
- hardware declarations;
- HDL declarations;
- distributed declarations;
- future domains.

No other module-system file may duplicate its visibility alternatives.

---

"namespaces.g4"

Owns:

- namespace declaration syntax;
- namespace aliases;
- namespace-specific wrappers around canonical qualified names.

It does not own module semantics.

A namespace is a naming/scope abstraction.

A module is a source organization/compilation abstraction.

They may interact semantically but must remain distinct concepts.

---

"packages.g4"

Owns:

- package declaration syntax;
- package metadata syntax;
- package identity syntax where applicable;
- package-level source declarations.

It does not own:

- package downloading;
- package registry access;
- dependency solving;
- cryptographic verification;
- installation;
- filesystem management.

---

"dependencies.g4"

Owns:

- dependency declaration syntax;
- dependency requirement syntax;
- dependency aliases or source qualifiers where specified.

It does not own:

- dependency resolution;
- version solving;
- network access;
- package retrieval;
- registry interaction;
- lockfile generation;
- artifact verification.

---

"module-attributes.g4"

Owns:

- attributes specifically attached to modules;
- module-specific attribute structure.

It does not own:

- general-purpose attribute vocabulary unless explicitly delegated to the canonical core attribute grammar;
- semantic interpretation of attributes;
- compiler behavior;
- runtime behavior.

---

4. Fundamental Principle

A Zamani module is a source-level language construct.

It is not inherently:

- a file;
- a directory;
- a filesystem path;
- a package;
- a process;
- a thread;
- a CPU;
- a GPU;
- an FPGA;
- an ASIC;
- a QPU;
- a physical qubit;
- a network node;
- a cluster;
- a container;
- a deployment;
- a runtime;
- a hardware resource.

For example:

module quantum::algorithms {
    ...
}

means that the source program contains a module named:

quantum::algorithms

It does not mean:

- select a QPU;
- select a quantum backend;
- allocate a fixed number of qubits;
- select a topology;
- allocate a device;
- select a calibration;
- reserve hardware;
- schedule execution.

Those decisions belong downstream.

---

5. POCO-REAF Requirement

The module system is a foundational part of POCO-REAF.

The same module syntax must remain valid when the program eventually executes on:

- one processor;
- many processors;
- an embedded processor;
- a GPU;
- an FPGA;
- an ASIC;
- a QPU;
- a simulator;
- a heterogeneous accelerator;
- a cluster;
- a supercomputer;
- a distributed deployment;
- a cloud environment;
- a future architecture.

Changing execution hardware must not require changing module organization merely because the machine has different:

- processor counts;
- memory capacity;
- qubit capacity;
- accelerator counts;
- topology;
- communication characteristics;
- scheduling capabilities.

---

6. Absolute Scalability Rule

The module grammar must contain no artificial finite limits.

Do not introduce grammar constants or productions equivalent to:

MAX_MODULES
MAX_MODULE_DEPTH
MAX_MODULE_SEGMENTS
MAX_MODULE_ITEMS
MAX_IMPORTS
MAX_EXPORTS
MAX_PACKAGES
MAX_DEPENDENCIES
MAX_NAMESPACES
MAX_TARGETS

The grammar must use structural repetition:

*
+
?

where appropriate.

For example:

moduleBody
    : LBRACE item* RBRACE
    ;

means that the grammar does not impose a language-level declaration count.

Actual resource limitations are implementation concerns.

They may be enforced by:

- compiler resource policies;
- parser configuration;
- memory policies;
- execution policies;
- build policies;
- deployment policies.

Those limits must never silently become language semantics.

---

7. No Machine-Specific Module Semantics

Module syntax must never encode:

CPU count
core count
thread count
GPU count
FPGA count
ASIC count
QPU count
qubit count
memory size
register count
vector width
device ID
device address
topology
network size
cluster size
accelerator count

A module may express a semantic requirement through another language subsystem, but module syntax itself must remain independent.

For example, these concepts are different:

module organization
resource requirement
hardware capability
target selection
deployment placement
runtime allocation

They must never be conflated.

---

8. Canonical Name Ownership

Names are not owned by the module grammar.

The canonical ownership hierarchy is:

grammar/lexer/
        |
        v
grammar/core/names.g4
        |
        v
grammar/core/qualified-names.g4
        |
        +--> modules
        +--> namespaces
        +--> imports
        +--> exports
        +--> declarations
        +--> functions
        +--> types
        +--> quantum
        +--> hardware
        +--> HDL
        +--> distributed
        +--> future domains

Module grammar must therefore reuse the canonical:

identifier
qualifiedName

rules.

It must not independently redefine:

IDENTIFIER (DOUBLE_COLON IDENTIFIER)*

or an equivalent qualified-name grammar.

This prevents syntax divergence between:

module names
namespace names
type names
function names
quantum names
hardware names
resource names

---

9. Visibility Ownership

Visibility belongs to:

grammar/modules/visibility.g4

Module declarations may consume the canonical visibility rule:

visibilityModifier?

The module grammar must not redefine:

moduleVisibility

with its own copy of:

pub
public
private
protected
internal

The same principle applies to:

- functions;
- declarations;
- types;
- quantum declarations;
- hardware declarations;
- HDL declarations;
- future declaration categories.

One vocabulary must have one owner.

---

10. Attributes Ownership

Module-specific attributes belong to:

grammar/modules/module-attributes.g4

General attributes belong to the appropriate canonical core attribute grammar.

The module grammar must not create a second annotation language.

For example:

@experimental
module quantum::future {
    ...
}

is syntactically divided into:

@experimental
    |
    v
attribute grammar
    |
    v
module
    |
    v
module name/body

The grammar records syntax.

Semantic analysis determines what "experimental" means.

---

11. Modules Versus Namespaces

Modules and namespaces must remain distinct.

Module

A source organization and compilation unit concept.

Namespace

A logical naming and scope concept.

A module may establish, inhabit, or correspond to a namespace according to semantic rules.

However:

module == namespace

must not be assumed by the grammar.

Similarly:

module == package

must not be assumed.

And:

module == filesystem directory

must not be assumed.

These relationships belong to semantic and toolchain layers.

---

12. Modules Versus Packages

A package is a distribution/dependency concept.

A module is a source organization concept.

A package may contain many modules.

A module may belong to a package.

The grammar must preserve that distinction.

The module grammar must not infer:

first module-name segment == package name

or:

module path == filesystem path

or:

module path == package registry coordinate

unless a separate semantic specification explicitly defines such a relationship.

---

13. Module Body Ownership

A module body must contain the canonical Zamani item/declaration language.

The module grammar must not create an isolated declaration universe.

Conceptually:

moduleBody
    : LBRACE item* RBRACE
    ;

where "item" is supplied by the aggregate parser architecture.

This allows modules to contain future and existing computational domains without changing the fundamental module grammar.

A module may therefore eventually contain:

classical declarations
quantum declarations
HDL declarations
hardware declarations
AI declarations
data declarations
distributed declarations
network declarations
security declarations
future declarations

without "modules.g4" importing every domain grammar individually.

This is important for extensibility.

---

14. Aggregate Parser Responsibility

The aggregate parser is responsible for composing the individual grammar components.

Conceptually:

Zamani aggregate grammar
        |
        +--> Lexer
        |
        +--> Core
        |
        +--> Types
        |
        +--> Expressions
        |
        +--> Statements
        |
        +--> Declarations
        |
        +--> Functions
        |
        +--> Modules
        |      |
        |      +--> Imports
        |      +--> Exports
        |      +--> Visibility
        |      +--> Namespaces
        |      +--> Packages
        |      +--> Dependencies
        |      +--> Module attributes
        |
        +--> Effects
        +--> Classical
        +--> Quantum
        +--> Hybrid
        +--> HDL
        +--> Hardware
        +--> Distributed
        +--> AI
        +--> Data
        +--> Networking
        +--> Security
        +--> Resources
        +--> Compilation
        +--> Execution
        +--> Interoperability
        +--> Dialects
        +--> Macros
        +--> Metaprogramming

Individual module grammar components must not independently construct an alternative complete Zamani parser.

---

15. Lexer Contract

The module grammar is parser-only.

Lexer ownership belongs to:

grammar/lexer/

The parser grammar must consume the canonical lexical vocabulary.

The aggregate lexer must provide the tokens required by the module components, including the language's canonical spellings for:

module
import
export
from
as
package
namespace
dependency
visibility
attributes
qualification
braces
parentheses
commas
semicolons
strings
identifiers

Exact token names must be determined by the canonical lexer.

No module grammar should silently create competing lexer token names.

---

16. Token Vocabulary Consistency

All modular parser grammars must converge on one canonical token vocabulary.

A repository-wide audit must resolve any difference between parser components that use different token vocabulary declarations.

For example, if one grammar component uses:

tokenVocab = ZamaniTokens;

while another uses:

tokenVocab = ZamaniLexer;

the aggregate grammar architecture must explicitly establish which generated lexer/parser vocabulary is canonical.

This is an integration requirement, not something to hide inside "modules/".

The final architecture must not contain accidental parallel lexical authorities.

---

17. Legacy Monolithic Grammar Migration

The existing monolithic:

grammar/Zamani.g4

contains older inline module-system productions such as:

moduleDecl
importDecl
exportDecl
visibilityModifier

The modular grammar architecture must progressively replace those duplicated definitions with the canonical modular components.

The migration must not silently remove supported language features.

For every legacy rule:

1. Identify the existing syntax.
2. Identify its consumers.
3. Compare it with the modular grammar.
4. Preserve valid syntax.
5. Correct invalid or ambiguous syntax.
6. Record compatibility implications.
7. Migrate consumers.
8. Remove duplicate ownership only after migration.
9. Add regression tests.

The final state must have one authoritative owner for each construct.

---

18. Import Integration

"imports.g4" owns import syntax.

"modules.g4" must not duplicate import grammar.

A module body may contain imports because imports are canonical Zamani items.

Conceptually:

module
  |
  +--> import
  +--> export
  +--> declaration
  +--> function
  +--> type
  +--> quantum declaration
  +--> hardware declaration
  +--> future declaration

Import resolution occurs later.

The parser must never:

- open files;
- access package registries;
- contact networks;
- resolve symbols;
- download modules;
- select targets.

---

19. Export Integration

"exports.g4" owns export syntax.

Visibility and export must remain separate concepts.

For example:

pub fn compute() {
    ...
}

and:

export compute;

are not identical concepts.

Visibility answers:

«Who can access the declaration?»

Export answers:

«Which declaration is intentionally exposed through this module interface?»

Semantic analysis may later validate their interaction.

The grammar must preserve the distinction.

---

20. Dependency Integration

"dependencies.g4" owns dependency declaration syntax.

Dependency resolution belongs downstream.

The grammar must not perform:

version solving
registry access
network access
package downloading
signature verification
dependency graph solving

The parser only records what the source says.

Semantic/toolchain infrastructure later constructs the dependency graph.

---

21. Package Integration

"packages.g4" owns package syntax.

A package may contain many modules and computational domains.

Package syntax must not impose machine-specific limits.

For example, package semantics must not imply:

one package == one machine
one module == one process
one package == one target

Such assumptions violate POCO-REAF.

---

22. Namespace Integration

"namespaces.g4" owns namespace syntax.

Module names may use canonical qualified names.

For example:

module quantum::algorithms;

does not require "modules.g4" to redefine:

quantum
::
algorithms

The canonical name grammar owns those structures.

Namespace resolution remains semantic.

---

23. Frontend AST Contract

The grammar must provide sufficient parse-tree structure for the frontend AST to preserve at least:

ModuleDeclaration
    attributes
    visibility
    name
    body
    source span

The AST should preserve:

- source ordering;
- source spans;
- module name segments;
- explicit versus absent visibility;
- attributes;
- whether a body exists;
- child items;
- source provenance required by diagnostics and tooling.

The exact Rust AST structures do not belong in ".g4" files.

No Rust structures should be embedded into this grammar.

---

24. Semantic Contract

After parsing, semantic analysis owns:

- module identity;
- module uniqueness;
- module nesting;
- module relationships;
- namespace relationships;
- package relationships;
- visibility checking;
- import resolution;
- export validation;
- dependency resolution;
- dependency cycle detection;
- symbol resolution;
- accessibility;
- module attributes' meaning;
- package compatibility;
- source-provider resolution;
- compilation-unit semantics.

The grammar must not attempt to perform these operations.

---

25. Canonical IR Boundary

The module grammar must never create an IR.

The pipeline is:

module syntax
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
    +--> classical IR
    +--> quantum::ir
    +--> HDL/hardware IR
    +--> distributed IR
    +--> accelerator IR

In particular, module syntax must never introduce:

ModuleIR
QuantumModuleIR
HardwareModuleIR
QubitModuleIR

as duplicate semantic representations merely because the module contains those domains.

For quantum computation, semantic lowering eventually reaches the repository's canonical:

quantum::ir

boundary.

"modules/" remains completely independent of QEC and ZQN.

---

26. Quantum Integration

Quantum modules may contain:

- quantum functions;
- circuits;
- gates;
- operations;
- measurements;
- logical-qubit abstractions;
- quantum-classical interactions;
- QEC-related declarations;
- quantum resource requirements.

However, "modules/" does not own those constructs.

For example:

module quantum::algorithms {
    ...
}

does not imply:

a fixed QPU
a fixed qubit count
a physical topology
a calibration
a backend
a scheduling policy
a routing policy

The module system remains independent of:

- "quantum::ir";
- QEC;
- ZQN;
- routing;
- scheduling;
- hardware discovery;
- calibration.

---

27. Classical Integration

Classical modules may contain:

- functions;
- types;
- numerical operations;
- data structures;
- concurrency;
- parallel computation;
- systems abstractions.

The module grammar does not need separate syntax for:

CPU module
GPU module
HPC module
embedded module

unless the language specification establishes genuinely different source semantics.

Target-specific implementation belongs downstream.

---

28. HDL and Hardware Integration

HDL and hardware declarations may be organized into modules.

For example:

module hardware::accelerators {
    ...
}

does not itself select a physical accelerator.

The module layer must remain separate from:

- hardware topology;
- physical placement;
- device IDs;
- addresses;
- clock hardware;
- physical resource allocation;
- routing;
- synthesis;
- scheduling.

Hardware semantics belong to HDL/hardware grammar and later compiler layers.

---

29. Distributed Integration

Modules may contain distributed declarations and services.

A module does not inherently represent:

one node
one process
one service instance
one cluster
one container

Deployment semantics belong to the distributed execution and deployment layers.

This preserves scalability from a single machine to arbitrarily large deployments subject to available resources.

---

30. Resource Independence

Module organization must not encode resource requirements.

These are separate concepts:

module
requirement
capability
constraint
preference
hint
resource
target
placement
deployment

For example:

module quantum::simulation {
    ...
}

must not implicitly mean:

requires 100 qubits
requires GPU
requires N cores
requires device X

If the program has a genuine resource requirement, it must be represented through the resource/capability system.

---

31. Determinism

Module grammar must be deterministic.

It must perform no:

- filesystem access;
- network access;
- environment inspection;
- hardware inspection;
- package lookup;
- dependency resolution;
- symbol lookup;
- randomness;
- clock access;
- runtime execution.

Given the same token stream and grammar version, syntactic interpretation must be deterministic.

Semantic resolution may depend on explicit compiler inputs, but that must occur outside the grammar.

---

32. Security Boundary

The parser must be incapable of causing external side effects.

The grammar must not contain:

- embedded Rust actions;
- filesystem operations;
- network operations;
- shell commands;
- environment-variable access;
- dynamic code execution;
- package downloads;
- registry requests;
- hardware access;
- runtime callbacks.

The grammar must remain a pure syntax specification.

---

33. Rust Safety Contract

".g4" files contain grammar definitions only.

They must contain no embedded Rust implementation.

Generated and handwritten Zamani Rust code must comply with:

Rust 1.97
Rust 1.97.1
safe Rust only
no unsafe

The module grammar must not require unsafe functionality.

Parser correctness must not depend on unsafe code.

---

34. Error Boundary

The grammar reports syntax errors.

Semantic analysis reports semantic errors.

Syntax errors

Examples:

module ;

module ::foo;

module foo::;

module foo {

Semantic errors

Examples:

duplicate module identity
unresolved import
dependency cycle
inaccessible symbol
invalid package relationship
invalid visibility
module/namespace conflict

The parser must not encode semantic checks through arbitrary predicates.

---

35. Module Declaration Forms

The canonical module syntax should support the language-approved forms:

module math;

and:

module math {
    ...
}

and qualified names:

module math::linear;

and:

module math::linear {
    ...
}

Visibility may be composed where allowed:

pub module math {
    ...
}

Attributes may be composed where allowed:

@experimental
pub module quantum::algorithms {
    ...
}

Exact accepted combinations are governed by the language specification and semantic validation.

---

36. Empty Modules

The grammar may accept an empty module body when the language specification permits it:

module empty {}

Whether an empty module is useful, deprecated, or semantically invalid is not a parser concern.

If the language decides to reject it, that policy should be represented explicitly in semantic validation unless there is a compelling syntactic reason.

---

37. Module Declaration Without a Body

A body-less module declaration:

module math;

must have an explicitly defined semantic interpretation.

Possible semantic meanings include:

- declaration of an externally defined module;
- declaration of a separately supplied compilation unit;
- forward declaration;
- source-unit association.

The grammar records the syntax.

The semantic specification determines the meaning.

The parser must not invent filesystem behavior.

---

38. Nested Modules

Nested modules must not have a fixed depth.

Examples:

module a {
    ...
}

module a::b {
    ...
}

module a::b::c {
    ...
}

and arbitrarily longer qualified names are syntactically governed by the canonical qualified-name grammar.

No:

MAX_MODULE_DEPTH

may exist in the grammar.

---

39. Source Ordering

The AST must preserve source ordering.

This is important for:

- diagnostics;
- source mapping;
- tooling;
- documentation;
- formatting;
- semantic provenance;
- reproducible compilation;
- deterministic diagnostics.

Semantic layers may reorder declarations internally where allowed, but that is not the responsibility of the grammar.

---

40. Source Provenance

Module parse nodes should retain source-span information through the frontend architecture.

At minimum, tooling should be able to identify:

module declaration start
module name span
each name segment span
visibility span
attribute spans
body span
declaration/item spans

This enables:

- precise diagnostics;
- IDE navigation;
- refactoring;
- formatting;
- source-to-source transformations;
- dependency visualization.

---

41. Tooling Integration

The module grammar must support tooling such as:

- language servers;
- IDEs;
- formatters;
- documentation generators;
- dependency analyzers;
- symbol browsers;
- refactoring tools;
- go-to-definition;
- rename operations;
- import management;
- module graph visualization.

Tooling must consume parser/AST information rather than implement its own incompatible module parser.

---

42. Compilation Integration

The compiler should conceptually process:

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
module resolution
  |
  v
name resolution
  |
  v
semantic validation
  |
  v
canonical semantic representation
  |
  v
domain-specific IR
  |
  v
optimization
  |
  v
routing / scheduling / lowering
  |
  v
target realization

"modules/" participates only in the first parser stage.

---

43. Runtime Integration

There must be no direct runtime dependency on this directory.

The runtime must not call the grammar to:

- resolve modules;
- load packages;
- discover hardware;
- select devices;
- execute imports.

Any runtime module loading mechanism must operate on compiler-produced metadata or explicitly defined runtime abstractions.

---

44. Dependency Graph

The module grammar architecture should follow this direction:

Lexer
  |
  v
Core names / paths / metadata
  |
  +--> Visibility
  |
  +--> Module attributes
  |
  +--> Namespaces
  |
  v
Modules
  |
  +--> Imports
  +--> Exports
  +--> Packages
  +--> Dependencies
  |
  v
Aggregate declaration/item grammar
  |
  v
Frontend AST
  |
  v
Semantic analysis

The reverse direction must not occur.

In particular:

modules -> IR
modules -> runtime
modules -> hardware
modules -> scheduling
modules -> routing
modules -> optimization

must not be direct grammar dependencies.

---

45. No Circular Dependencies

The module grammar must not establish cycles such as:

modules -> AST -> modules

or:

modules -> semantic analysis -> modules

or:

modules -> quantum -> modules

or:

modules -> hardware -> modules

or:

modules -> runtime -> modules

The dependency direction is:

syntax
  ↓
AST
  ↓
semantic analysis
  ↓
IR
  ↓
compiler
  ↓
runtime

---

46. Compatibility Contract

Module-system evolution must be versioned.

Changes must be classified as:

- additive;
- compatible;
- deprecated;
- migration-required;
- breaking.

A grammar change must not silently redefine existing valid programs.

Before removing a module feature:

1. identify existing uses;
2. determine intended semantics;
3. determine compatibility requirements;
4. add migration rules;
5. update documentation;
6. update tests;
7. only then remove obsolete syntax.

---

47. Legacy Compatibility

The current repository contains an older monolithic module system alongside the modular grammar architecture.

The final architecture must converge on one source-level module specification.

The modular files become the maintainable ownership units.

The monolithic grammar must not remain a second independent authority indefinitely.

Compatibility tests must ensure that valid legacy programs remain valid unless a deliberate language-version change says otherwise.

---

48. Grammar Authority

The repository must establish:

canonical grammar
      |
      +--> modular grammar components
      |
      +--> generated parser
      |
      +--> tests
      |
      +--> documentation

Documentation must describe the canonical grammar.

Generated parser artifacts must be derived from it.

No generated artifact should become a hand-maintained competing grammar authority.

---

49. Testing Requirements

The module grammar requires dedicated tests.

Tests belong under:

grammar/tests/modules/

and relevant cross-domain suites.

---

50. Positive Tests

At minimum test:

module math;

module math {}

module math::linear;

module math::linear {}

pub module math {}

@experimental
module quantum::algorithms {}

module quantum::algorithms {
    ...
}

module hardware::accelerators {
    ...
}

module distributed::services {
    ...
}

and modules containing mixed computational domains.

---

51. Negative Tests

Test malformed module syntax including:

module;

module ;

module ::math;

module math::;

module ::;

module math {

module math {}
extra-invalid-token

and other malformed combinations defined by the canonical grammar.

---

52. Semantic Negative Tests

Separate parser failures from semantic failures.

Examples:

duplicate module
unresolved module
duplicate namespace
invalid visibility
invalid import
cyclic dependency
invalid package relationship
invalid export

must be tested in semantic-analysis suites rather than encoded into grammar predicates.

---

53. Cross-Domain Tests

Modules must be tested with:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security
accelerators
future dialects

Examples should include:

classical module + quantum declaration
quantum module + classical function
hardware module + HDL declarations
distributed module + quantum computation
AI module + accelerator declarations
classical + quantum + distributed
classical + quantum + HDL + hardware

This proves that modules remain domain-neutral.

---

54. Scalability Tests

Tests must verify that the grammar imposes no accidental finite limits.

Generate module names and module structures containing:

- many modules;
- many declarations;
- many qualified-name segments;
- many imports;
- many exports;
- many dependencies;
- deeply nested source structures;
- very large compilation units.

The tests must distinguish:

grammar acceptance

from:

implementation resource exhaustion

A parser implementation may have explicit operational resource policies.

Those policies must not be interpreted as language-level module limits.

---

55. Hard-Coding Audit

Every module grammar review must search for:

MAX_MODULE
MAX_IMPORT
MAX_EXPORT
MAX_PACKAGE
MAX_DEPENDENCY
MAX_NAMESPACE
MAX_DEPTH
MAX_SEGMENTS
MAX_ITEMS

and equivalent hard-coded restrictions.

Also search for:

q[0]
q[1]
CPU
GPU
QPU
device
address
topology
cores
threads
qubits
memory

inside module-specific grammar.

Any occurrence must be classified as:

1. legitimate language syntax;
2. documentation;
3. test fixture;
4. diagnostic text;
5. accidental machine coupling.

Accidental machine coupling must be removed.

---

56. Resource Exhaustion Policy

"Unlimited" at the language level does not mean physically infinite execution.

The correct distinction is:

Language semantics
        !=
Implementation capacity

A parser may encounter:

available memory
stack limits
input-size policies
execution timeouts
compiler budgets

but those are implementation policies.

They must not alter the meaning of valid Zamani source.

Thus:

scale from atom to everywhere

means the language does not impose arbitrary machine-size ceilings.

Actual execution remains bounded by available resources.

---

57. Deterministic Builds

Module syntax must support reproducible compilation.

Module parsing must not depend on:

- current time;
- random values;
- machine identity;
- environment variables;
- local filesystem state;
- network state.

Any external module resolution must be represented as an explicit compiler input and handled outside grammar parsing.

---

58. Reproducibility

The module AST and semantic module graph should preserve enough provenance to reproduce compilation decisions.

Important provenance includes:

- module source identity;
- module name;
- source span;
- dependency declarations;
- import/export syntax;
- language version;
- applicable attributes.

The grammar itself does not perform reproducibility enforcement.

---

59. Security

Module syntax must not become an arbitrary code-execution mechanism.

The module grammar must not allow syntax that implicitly means:

execute shell command
download code
execute downloaded code
inspect host filesystem
inspect hardware
open arbitrary network connection
access secrets

Any explicit interoperability or package mechanism must pass through separately defined security and compiler policies.

---

60. Future Extensibility

The module grammar must support future computational domains without redesigning its fundamental structure.

A future declaration such as:

new-domain::declaration

should be able to inhabit a module through the canonical item/declaration composition.

The module grammar should not require a new module syntax merely because Zamani gains:

- neuromorphic computing;
- photonic computing;
- molecular computing;
- biological computing;
- optical accelerators;
- new quantum models;
- new AI architectures;
- new hardware technologies;
- future execution models.

The module abstraction must remain stable.

---

61. Dialect Integration

Future dialects may introduce additional declarations.

They must integrate through the aggregate parser and dialect registration mechanism.

They must not modify the fundamental module abstraction merely to become module members.

The relationship is:

module
  |
  +--> canonical item
          |
          +--> core declaration
          +--> quantum declaration
          +--> HDL declaration
          +--> hardware declaration
          +--> dialect declaration
          +--> future declaration

---

62. Macro Integration

Macros may appear inside modules where the language permits them.

Macro syntax remains owned by:

grammar/macros/

The module grammar must not duplicate macro syntax.

Macro expansion happens after parsing according to the language's compilation model.

The module parser must preserve macro source structure sufficiently for diagnostics and tooling.

---

63. Metaprogramming Integration

Compile-time and metaprogramming constructs may be organized by modules.

Module syntax remains unchanged.

The module grammar does not execute compile-time code.

Execution belongs to the appropriate compiler stage.

---

64. Interoperability

A module may expose:

- C interoperability;
- C++ interoperability;
- Python interoperability;
- OpenQASM interoperability;
- Verilog/HDL interoperability;
- system interfaces;
- ABI definitions.

Those constructs belong to:

grammar/interoperability/

The module system only provides the organizational boundary.

---

65. Quantum "quantum::ir" Boundary

A module containing quantum computation does not create a quantum IR.

The flow remains:

module syntax
     |
     v
frontend AST
     |
     v
semantic quantum analysis
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
hardware / runtime

The module system must never redefine:

QubitId
PhysicalQubitId
QuantumGate
QuantumOperation
QuantumCircuit

or equivalent semantic representations.

Those belong to the canonical quantum architecture.

---

66. QEC Boundary

QEC is not owned by modules.

A module may contain a declaration that refers to an error-correction abstraction if the quantum language supports it.

The meaning is handled downstream.

The module grammar must not implement:

- syndrome extraction;
- correction;
- decoder behavior;
- logical qubit management;
- code selection.

---

67. ZQN Boundary

ZQN is not owned by modules.

Module syntax must not describe:

- noise models;
- fault injection;
- noise channels;
- correlated faults;
- leakage;
- loss;
- erasure;
- calibration noise.

Those remain ZQN concerns.

A module can organize source declarations that eventually use ZQN semantics, but the module grammar remains unaware of those implementation details.

---

68. Scheduling and Routing Boundary

Module structure does not determine:

- operation order;
- timing;
- resource scheduling;
- qubit placement;
- physical routing;
- pulse timing;
- hardware topology.

These belong to later compilation stages.

Therefore:

module

must never mean:

schedule
route
place
allocate
dispatch

---

69. Hardware Abstraction Boundary

Hardware capabilities are supplied by the hardware abstraction layer and compilation context.

Module syntax may identify a semantic requirement through a separate resource/capability language, but:

module name

must not encode:

device identity
hardware address
physical topology

This distinction is necessary for POCO-REAF.

---

70. Resource/Capability Boundary

The module system must compose with:

grammar/resources/

for explicit source-level requirements where appropriate.

Conceptually:

module
   |
   +--> requirements
   +--> constraints
   +--> preferences
   +--> hints

But these remain separate semantic concepts.

A module name itself is not a resource requirement.

---

71. Documentation Contract

The module grammar documentation must remain synchronized with:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/specification/
grammar/modules/

Documentation changes must not silently introduce syntax absent from the authoritative grammar.

Likewise, new grammar syntax must have corresponding specification coverage.

---

72. Required File-Level Completion Contracts

Each module grammar file is complete only when all of the following have been established:

Purpose
Ownership
Non-ownership
Lexical dependencies
Grammar dependencies
AST expectations
Semantic boundary
Compiler integration
Runtime non-dependency
Tooling integration
Cross-domain integration
Compatibility behavior
Scalability behavior
Hard-coding audit
Positive tests
Negative tests
Boundary tests
Determinism tests
Documentation

A file is not complete merely because ANTLR accepts it.

---

73. "modules.g4" Completion Criteria

"modules.g4" is complete only when:

- module syntax has one authoritative owner;
- module names reuse canonical names;
- visibility uses canonical visibility syntax;
- attributes use canonical attribute syntax;
- module bodies use canonical item composition;
- no module-specific declaration language is duplicated;
- no machine limits exist;
- no filesystem semantics are embedded;
- no package resolution is performed;
- no runtime dependency exists;
- no IR is created;
- no quantum representation is duplicated;
- parser integration is deterministic;
- positive tests pass;
- negative tests pass;
- boundary tests pass;
- cross-domain tests pass;
- compatibility tests pass.

---

74. "imports.g4" Completion Criteria

"imports.g4" is complete only when:

- import syntax has one authoritative owner;
- qualified names are canonical;
- aliases are canonical;
- source literals are syntactically opaque;
- no filesystem access is implied;
- no package resolution is performed;
- no dependency solving is performed;
- no machine target is encoded;
- semantic resolution is delegated;
- import tests pass;
- legacy import syntax has been audited.

---

75. "exports.g4" Completion Criteria

"exports.g4" is complete only when:

- export syntax has one authoritative owner;
- export and visibility remain distinct;
- re-export syntax is explicitly defined;
- wildcard behavior is specified;
- aliases are canonical;
- semantic accessibility is delegated;
- no runtime export behavior is embedded;
- tests cover all supported export forms.

---

76. "visibility.g4" Completion Criteria

"visibility.g4" is complete only when:

- there is one visibility vocabulary;
- all declaration grammars consume it;
- no duplicate visibility rules remain;
- explicit and absent visibility are distinguishable where required;
- semantic access checking is delegated;
- no domain-specific visibility duplication exists;
- compatibility behavior is documented;
- tests cover every supported spelling.

---

77. "namespaces.g4" Completion Criteria

"namespaces.g4" is complete only when:

- canonical qualified names are reused;
- namespace syntax is distinct from module syntax;
- namespace aliases are explicitly defined;
- no filesystem assumptions exist;
- no package assumptions exist;
- namespace resolution is semantic;
- no namespace-depth limit exists;
- tests cover aliases, nesting, and invalid syntax.

---

78. "packages.g4" Completion Criteria

"packages.g4" is complete only when:

- package syntax is authoritative;
- package identity is separated from module identity;
- dependency syntax is delegated;
- package resolution is external to grammar;
- no registry/network behavior is embedded;
- package metadata syntax is versioned;
- tests cover valid and invalid package declarations.

---

79. "dependencies.g4" Completion Criteria

"dependencies.g4" is complete only when:

- dependency syntax is authoritative;
- requirements are represented structurally;
- resolution is semantic/toolchain responsibility;
- dependency count is unlimited at the grammar level;
- no registry/network access exists;
- no version-solving algorithm is embedded;
- dependency-cycle detection remains semantic;
- tests cover dependency forms and boundaries.

---

80. "module-attributes.g4" Completion Criteria

"module-attributes.g4" is complete only when:

- module-specific attributes have clear ownership;
- generic attributes are delegated appropriately;
- attribute values use canonical expressions/literals where required;
- no attribute executes code;
- no hardware is selected;
- no runtime action occurs;
- attribute semantics are documented elsewhere;
- tests cover valid, invalid, repeated, and conflicting syntax.

---

81. Production Test Matrix

The module system must ultimately be covered by:

grammar/tests/
├── modules/
│   ├── modules_positive
│   ├── modules_negative
│   ├── imports_positive
│   ├── imports_negative
│   ├── exports_positive
│   ├── exports_negative
│   ├── namespaces_positive
│   ├── namespaces_negative
│   ├── packages_positive
│   ├── packages_negative
│   ├── dependencies_positive
│   ├── dependencies_negative
│   ├── visibility_positive
│   └── visibility_negative
│
├── cross-domain/
├── scalability/
├── determinism/
├── compatibility/
└── roundtrip/

The exact physical test-file layout may be consolidated where that improves maintainability, but ownership and coverage must remain explicit.

---

82. Round-Trip Requirement

Where a source printer exists:

Zamani source
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
Zamani source

must preserve semantics.

Module tests should verify:

- module names;
- qualified-name structure;
- visibility;
- attributes;
- body boundaries;
- imports;
- exports;
- package declarations;
- dependency declarations.

Source formatting may change.

Meaning must not.

---

83. Determinism Test

Given identical source:

source A

multiple parser executions must produce equivalent parse structures.

No parser output may depend on:

machine
time
randomness
filesystem
network
hardware
environment

---

84. Repository-Wide Integration Test

The final grammar integration must prove:

grammar
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
semantic analysis
   |
   v
canonical IR

with modules participating without creating a circular dependency.

The integration suite must include at least one complete program containing:

module
imports
exports
namespace
package/dependency metadata
classical code
quantum code
hardware/HDL code
resource declarations

where each domain is represented according to its canonical grammar.

---

85. POCO-REAF Acceptance Test

A module program must remain semantically identical when compiled for different target environments.

The source:

module application::core {
    ...
}

must not need rewriting merely because compilation targets change from:

embedded

to:

CPU

to:

GPU

to:

FPGA

to:

ASIC

to:

QPU

to:

cluster

to:

cloud

provided the target satisfies the program's semantic requirements.

---

86. What the Module Grammar Must Never Do

The module grammar must never:

- discover hardware;
- select hardware;
- allocate hardware;
- select a QPU;
- allocate qubits;
- select a topology;
- schedule operations;
- route operations;
- optimize programs;
- execute code;
- resolve packages;
- access a registry;
- access the filesystem;
- access the network;
- construct IR;
- construct "quantum::ir";
- perform QEC;
- model ZQN noise;
- create runtime state;
- impose machine-size limits.

---

87. Implementation Order

The module subsystem should be completed in dependency-first order.

Recommended order:

1. Canonical lexer/token contract
        |
2. core/names.g4
        |
3. core/qualified-names.g4
        |
4. core/attributes.g4
        |
5. modules/visibility.g4
        |
6. modules/module-attributes.g4
        |
7. modules/namespaces.g4
        |
8. modules/modules.g4
        |
9. modules/imports.g4
        |
10. modules/exports.g4
        |
11. modules/packages.g4
        |
12. modules/dependencies.g4
        |
13. aggregate parser integration
        |
14. frontend AST integration
        |
15. semantic module resolution
        |
16. repository-wide compatibility tests

The exact order must follow the actual dependency graph if repository inspection reveals a different prerequisite.

---

88. Definition of Done

The "grammar/modules/" subsystem is production-ready only when:

- every grammar component has one clear owner;
- no duplicate grammar authority remains;
- canonical names are reused;
- canonical visibility is reused;
- attributes are correctly delegated;
- module bodies compose the canonical item grammar;
- imports are separate from exports;
- packages are separate from modules;
- namespaces are separate from modules;
- dependencies are separate from packages;
- syntax and semantics are clearly separated;
- no runtime behavior exists in grammar;
- no filesystem/network behavior exists in grammar;
- no hardware assumptions exist in grammar;
- no fixed scalability limits exist;
- no quantum-machine assumptions exist;
- no IR is duplicated;
- "quantum::ir" remains the canonical quantum semantic boundary;
- QEC remains outside module grammar;
- ZQN remains outside module grammar;
- optimization remains outside module grammar;
- routing remains outside module grammar;
- scheduling remains outside module grammar;
- hardware discovery remains outside module grammar;
- compiler integration is deterministic;
- Rust 1.97/1.97.1 compatibility is maintained;
- no unsafe Rust is required;
- parser diagnostics have stable source boundaries;
- AST provenance is preserved;
- tooling integration is possible;
- cross-domain programs parse correctly;
- scalability tests pass;
- determinism tests pass;
- compatibility tests pass;
- round-trip tests pass;
- documentation matches the authoritative grammar.

---

89. Final Architectural Rule

The module system must embody:

«A Zamani module organizes computation; it does not determine the machine on which that computation must execute.»

Therefore:

Module
   ≠
Machine

Module
   ≠
Device

Module
   ≠
QPU

Module
   ≠
Qubit set

Module
   ≠
CPU count

Module
   ≠
GPU count

Module
   ≠
Filesystem directory

Module
   ≠
Package

Module
   ≠
Process

Module
   ≠
Deployment

The stable abstraction is:

Zamani Source
      |
      v
Module Organization
      |
      v
Semantic Meaning
      |
      v
Canonical IR
      |
      v
Target-independent compilation
      |
      v
Target realization
      |
      v
Available hardware/resources

This separation is essential to:

«Zamani — From Atom to Everywhere»

and:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

The module grammar must therefore remain small in semantic responsibility, strong in syntactic structure, extensible across computational domains, deterministic, safe, versionable, and completely independent of arbitrary hardware scale.
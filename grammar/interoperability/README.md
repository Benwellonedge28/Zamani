Zamani Interoperability Grammar

Path: "grammar/interoperability/"
Primary document: "grammar/interoperability/README.md"
Role: Production interoperability architecture, ownership registry, integration contract, dependency map, conformance contract, and completion gate
Language: Zamani
Grammar technology: ANTLR 4
Implementation baseline: Rust 2021, Rust 1.97.1 or later
Rust safety: Safe Rust only; Rust "unsafe" is prohibited
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Status: Normative subsystem orchestration document

---

1. Purpose

"grammar/interoperability/" defines the language boundary between Zamani and external computational ecosystems.

It provides the syntax and source-level contracts required to describe, import, export, expose, consume, exchange, translate, serialize, deserialize, and interoperate with:

- foreign languages;
- foreign functions;
- foreign types;
- foreign modules;
- foreign symbols;
- foreign runtimes;
- ABIs;
- calling conventions;
- linkage models;
- system interfaces;
- operating-system interfaces;
- APIs;
- serialization systems;
- data interchange formats;
- C;
- C++;
- Rust;
- Python;
- Zig;
- assembly representations;
- WebAssembly;
- OpenQASM;
- QIR;
- Verilog;
- future HDL formats;
- quantum runtimes;
- accelerator runtimes;
- device interfaces;
- distributed services;
- future programming languages;
- future computational formats;
- future execution substrates.

The directory exists to make external integration extensible without making external technology the semantic authority of Zamani.

The fundamental rule is:

«Interoperability describes an external contract. It does not become the owner of the external system's complete semantics, and it does not determine how the contract is physically realized.»

The intended pipeline is:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
canonical parser
    │
    ▼
interoperability syntax
    │
    ▼
domain-neutral AST
    │
    ▼
structural validation
    │
    ▼
semantic validation
    │
    ├── types
    ├── effects
    ├── capabilities
    ├── resources
    ├── contracts
    ├── policies
    ├── security
    ├── compatibility
    └── provenance
    │
    ▼
canonical semantic model
    │
    ├── classical representation
    ├── quantum::ir
    └── HDL/hardware semantic representation
    │
    ▼
target-independent optimization
    │
    ▼
lowering
    │
    ├── ABI
    ├── linkage
    ├── routing
    ├── scheduling
    ├── resilience
    ├── QEC
    ├── ZQN
    └── HAL
    │
    ▼
target realization

Interoperability is therefore an upstream contract layer, not an execution engine.

---

2. Architectural Position

"grammar/interoperability/" is subordinate to the repository-wide language architecture.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/Zamani.g4
        │
        ▼
grammar/antlr/ZamaniLexer.g4
grammar/antlr/ZamaniParser.g4
        │
        ▼
grammar domain composition
        │
        ▼
Rust lexer/parser
        │
        ▼
domain-neutral AST
        │
        ▼
structural validation
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic model
        │
        ├── classical representation
        ├── quantum::ir
        └── HDL/hardware representation
        │
        ▼
compiler / lowering
        │
        ▼
runtime / HAL / target

This README does not supersede:

- "grammar/DESIGN.md";
- "grammar/README.md";
- "grammar/specification/";
- "grammar/spec/";
- canonical lexer authority;
- canonical parser authority;
- AST authority;
- semantic authority;
- canonical IR authority.

It orchestrates how interoperability participates in those systems.

---

3. Core Interoperability Principle

The subsystem separates:

WHAT a foreign boundary means

from:

HOW the boundary is realized

A Zamani source program may declare:

language identity
format identity
ABI identity
calling-convention intent
linkage intent
foreign symbol
foreign type
foreign function
conversion
marshaling
ownership
lifetime
nullability
effect
capability
resource requirement
security policy
compatibility requirement
provenance

It must not thereby select:

CPU
GPU
FPGA
ASIC
QPU
device
physical address
register
core
thread
node
machine
runtime instance
library location
network endpoint
process

unless those properties are explicitly part of a separately owned target/resource/runtime contract.

---

4. Ownership

4.1 This directory owns

The interoperability subsystem owns source-level syntax and composition for:

- foreign boundaries;
- foreign identities;
- foreign declarations;
- foreign callable contracts;
- foreign types;
- FFI declarations;
- ABI declarations;
- calling-convention intent;
- linkage intent;
- external symbols;
- conversion intent;
- marshaling intent;
- callbacks;
- serialization declarations;
- deserialization declarations;
- API boundaries;
- system-interface declarations;
- external language adapters;
- external format adapters;
- interoperability metadata;
- interoperability compatibility metadata;
- interoperability security metadata;
- interoperability resource requirements;
- interoperability capabilities;
- interoperability effects;
- interoperability policies;
- interoperability provenance.

4.2 This directory does not own

It does not own:

- universal identifiers;
- universal names;
- ordinary expressions;
- ordinary statements;
- ordinary functions;
- ordinary modules;
- the canonical type system;
- the canonical ownership model;
- the canonical effect system;
- the canonical capability system;
- the canonical resource system;
- general contracts;
- general policies;
- general security semantics;
- semantic analysis;
- physical ABI realization;
- linker implementation;
- object-file generation;
- library loading;
- runtime execution;
- hardware discovery;
- target selection;
- device allocation;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL.

Those responsibilities remain downstream or belong to their canonical repository owners.

---

5. The Single-Authority Rule

Every interoperability concept MUST have one source-of-truth owner.

The following ownership model is mandatory:

Concept| Owner
Shared tokens| "grammar/lexer/" / canonical lexer
Names| "grammar/core/names.g4"
Attributes| "grammar/core/attributes.g4"
Types| "grammar/types/"
Expressions| "grammar/expressions/"
Functions| "grammar/functions/"
Modules| "grammar/modules/"
Effects| "grammar/effects/"
Resources| "grammar/resources/"
Capabilities| "grammar/resources/" / canonical capability model
Contracts| "grammar/validation/"
Policies| "grammar/policies/" where established
Security| "grammar/security/"
Provenance| canonical provenance subsystem
Generic FFI| "grammar/interoperability/ffi.g4"
ABI| "grammar/interoperability/abi.g4"
Calling conventions| "grammar/interoperability/calling-conventions.g4"
Linkage| "grammar/interoperability/linkage.g4"
Foreign functions| "grammar/interoperability/foreign-functions.g4"
Foreign types| "grammar/interoperability/foreign-types.g4"
External functions| "grammar/interoperability/external-functions.g4"
External types| "grammar/interoperability/external-types.g4"
Foreign-language composition| "grammar/interoperability/foreign.g4"
Serialization| "grammar/interoperability/serialization.g4"
Deserialization| "grammar/interoperability/deserialization.g4"
Data layout intent| "grammar/interoperability/data-layout.g4"
API boundary| "grammar/interoperability/api.g4"
System interfaces| canonical system-interface grammar after reconciliation
C| "grammar/interoperability/c.g4"
C++| "grammar/interoperability/cpp.g4"
Rust| "grammar/interoperability/rust.g4"
Python| "grammar/interoperability/python.g4"
Zig| "grammar/interoperability/zig.g4"
Assembly| "grammar/interoperability/AssemblyLanguage.g4"
OpenQASM| "grammar/interoperability/openqasm.g4"
QIR| "grammar/interoperability/qir.g4"
QASM compatibility| "grammar/interoperability/qasm.g4" only where its distinct responsibility is established
WebAssembly| "grammar/interoperability/wasm.g4"
Verilog| "grammar/interoperability/verilog.g4"
HDL interoperability| "grammar/interoperability/hdl.g4"
Interoperability composition| "grammar/interoperability/interoperability.g4"
Quantum semantic IR| "quantum::ir"
Classical semantic IR| canonical classical IR
Hardware semantic representation| HDL/hardware subsystem

No file may silently create a second owner for a concept already assigned above.

---

6. The Composition Root Correction

The repository currently contains:

grammar/interoperability/interoperability.g4 

where the filename has a trailing space.

That is not acceptable as a production composition-root filename.

The canonical intended filename is:

grammar/interoperability/interoperability.g4

The trailing-space filename MUST be removed through an explicit repository migration.

The migration MUST:

1. preserve the current file contents;
2. create the canonical path without the trailing space;
3. reconcile imports;
4. reconcile ANTLR grammar names;
5. update references;
6. update build/test manifests;
7. verify generated parser inputs;
8. verify no reference still points to the trailing-space path;
9. remove the trailing-space artifact;
10. record the migration in compatibility/provenance documentation.

This README treats:

grammar/interoperability/interoperability.g4

as the intended canonical composition file.

---

7. "interoperability.g4"

Purpose

The composition grammar orchestrates the interoperability subsystem.

It MUST be the only interoperability composition grammar.

Owns

It owns:

- interoperability dispatch;
- composition;
- feature-family entry points;
- interoperability item classification;
- routing from generic interoperability syntax to specialized grammars.

Does not own

It must not redefine:

- identifiers;
- qualified names;
- types;
- expressions;
- functions;
- modules;
- attributes;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- foreign-language syntax;
- ABI internals;
- serialization internals;
- OpenQASM syntax;
- HDL syntax.

Integration

ZamaniParser
      │
      ▼
interoperability entry point
      │
      ├── FFI
      ├── ABI
      ├── foreign functions
      ├── foreign types
      ├── external functions
      ├── external types
      ├── serialization
      ├── deserialization
      ├── API
      ├── system interfaces
      ├── C/C++/Rust/Python/Zig
      ├── assembly
      ├── WebAssembly
      ├── OpenQASM/QASM
      ├── QIR
      ├── HDL
      └── Verilog

Completion

It is complete only when every imported rule has:

- one owner;
- a stable public entry point;
- an AST mapping;
- semantic ownership;
- diagnostics;
- tests;
- source-span preservation;
- deterministic parsing.

---

8. "ffi.g4"

Purpose

Defines generic source-level foreign-function boundary syntax.

Owns

- foreign interfaces;
- foreign callable bindings;
- foreign callable references;
- foreign calls;
- callback declarations;
- callback calls;
- marshaling intent;
- ownership intent;
- lifetime intent;
- nullability intent;
- representation intent;
- encoding intent;
- boundary effects;
- boundary capabilities;
- boundary resource requirements;
- security requirements;
- compatibility requirements;
- provenance metadata.

Does not own

- ABI implementation;
- linker behavior;
- loader behavior;
- library discovery;
- runtime dispatch;
- physical addresses;
- registers;
- hardware selection;
- routing;
- scheduling.

Integration

ffi.g4
  ↓
domain-neutral AST
  ↓
canonical types
  ↓
effects
  ↓
capabilities
  ↓
resources
  ↓
contracts
  ↓
policies/security
  ↓
ABI/lowering

---

9. "abi.g4"

Purpose

Defines ABI contracts.

Owns

- ABI identity;
- ABI profiles;
- ABI compatibility metadata;
- ABI boundary contracts;
- ABI-specific callable boundary declarations;
- ABI representation intent;
- ABI ownership/lifetime/nullability intent;
- ABI adaptation intent.

Does not own

- actual stack layout;
- register allocation;
- instruction selection;
- object generation;
- linker execution;
- loader execution;
- physical memory layout.

Integration

abi.g4
  ↓
ABI semantic contract
  ↓
target ABI resolver
  ↓
lowering
  ↓
linker/runtime

ABI identity MUST remain extensible.

No permanent closed enumeration of ABIs is permitted.

---

10. "calling-conventions.g4"

Purpose

Defines symbolic calling-convention intent.

Owns

- calling-convention identity;
- convention attributes;
- compatibility metadata;
- convention selection intent.

Does not own

- physical registers;
- register counts;
- stack slots;
- instruction sequences;
- processor-specific machine behavior.

A calling convention describes a contract, not a machine.

---

11. "linkage.g4"

Purpose

Defines linkage intent.

Owns

- external linkage;
- imported linkage;
- exported linkage;
- internal linkage;
- weak/linkage modifiers where semantically supported;
- symbol visibility intent;
- versioned linkage intent;
- symbolic linkage metadata.

Does not own

- linking;
- object-file generation;
- relocation execution;
- dynamic loading.

The compiler/linker subsystem realizes linkage.

---

12. Foreign Function Ownership

The repository contains both:

grammar/functions/foreign-functions.g4

and:

grammar/interoperability/foreign-functions.g4

These MUST NOT become competing function systems.

The ownership boundary is:

grammar/functions/foreign-functions.g4
    =
generic function-level foreign declaration integration

grammar/interoperability/foreign-functions.g4
    =
interoperability-specific external callable contract

grammar/interoperability/ffi.g4
    =
generic FFI boundary

grammar/interoperability/abi.g4
    =
ABI contract

grammar/interoperability/calling-conventions.g4
    =
calling convention

grammar/interoperability/linkage.g4
    =
linkage

Any overlapping rule MUST be reconciled so that one file owns the syntax and the other consumes it.

---

13. "foreign-functions.g4"

Purpose

Declares externally implemented callable contracts.

Owns

- external callable declarations;
- foreign callable metadata;
- external symbol references;
- callable compatibility metadata;
- external callable requirements.

Integration

foreign function
      ↓
canonical function model
      ↓
FFI
      ↓
ABI
      ↓
calling convention
      ↓
linkage
      ↓
compiler/lowering

It must never become a second general function grammar.

---

14. "external-functions.g4"

This file must be treated as a compatibility layer for externally implemented callable declarations.

Its role must be explicitly distinguished from "foreign-functions.g4".

If semantic overlap exists, it MUST be reduced to:

external-functions.g4
    compatibility/composition layer

foreign-functions.g4
    canonical foreign callable contract

Duplicate AST representations are prohibited.

---

15. "external-foreign-functions.g4"

This file MUST NOT create a third foreign-function semantic model.

Its role must be one of:

- compatibility adapter;
- migration layer;
- specialized external-boundary syntax.

Its final ownership MUST be recorded in this README and the grammar's own feature contract.

If it contains duplicate callable rules, those rules MUST delegate to the canonical foreign-function model.

---

16. "foreign-types.g4"

Purpose

Defines foreign type boundary syntax.

Owns

- foreign type declarations;
- opaque types;
- handles;
- foreign representations;
- nullability intent;
- ownership intent;
- lifetime intent;
- conversion metadata.

Does not own

- canonical Zamani type semantics;
- concrete machine layout;
- pointer width;
- register width;
- physical addresses.

A foreign type MUST become a canonical semantic type or opaque boundary type.

It must never create a second type system.

---

17. "external-types.g4"

This file provides compatibility/composition support for external type boundaries.

It MUST NOT create a competing foreign-type semantic model.

Canonical semantic ownership remains with:

foreign-types.g4

and:

grammar/types/

---

18. "data-layout.g4"

Purpose

Defines source-level representation/layout intent required for interoperability.

Owns

- representation attributes;
- alignment intent;
- packing intent;
- field-order intent;
- external representation metadata;
- layout compatibility declarations.

Does not own

- concrete target layout calculation;
- register allocation;
- stack layout;
- machine layout;
- physical memory placement.

No fixed universal:

pointer width
address width
word width
register width
alignment
endianness

may be assumed.

---

19. "foreign.g4"

"foreign.g4" is the broad foreign-boundary abstraction.

It must orchestrate, not duplicate:

foreign language identity
foreign format identity
foreign declarations
foreign modules
foreign symbols
foreign types
foreign functions
foreign metadata

It must delegate specialized semantics to the appropriate leaf grammar.

---

20. "c.g4"

"c.g4" owns C-specific interoperability syntax.

It MUST NOT implement the C language.

It MUST NOT become:

- a C compiler;
- a C semantic universe;
- an ABI implementation;
- a linker;
- a library loader.

Its output must normalize into:

FFI
+
foreign types/functions
+
ABI
+
calling convention
+
linkage
+
canonical semantic model

---

21. "cpp.g4"

"cpp.g4" owns C++ interoperability-specific declarations.

It must preserve C++-specific concepts required for the boundary without importing the entire C++ language into Zamani's universal grammar.

C++ templates, overloads, namespaces, ABI names, object models, exceptions, ownership conventions, and representation requirements must be represented only where needed for interoperability.

---

22. "rust.g4"

"rust.g4" owns Rust interoperability syntax.

It must not duplicate Zamani's Rust implementation model.

Rust-specific declarations normalize into:

foreign type
foreign function
ABI
ownership
lifetime
effects
capabilities
conversion
canonical semantic representation

Rust's source language is an external ecosystem, not a replacement for Zamani semantics.

---

23. "python.g4"

"python.g4" owns Python interoperability boundaries.

It must not imply that the Zamani parser executes Python.

It must not:

- start a Python interpreter;
- import Python modules;
- inspect the filesystem;
- inspect Python packages;
- resolve runtime objects.

Those operations belong to toolchain/runtime components under explicit capability and security policy.

---

24. "zig.g4"

"zig.g4" owns Zig-specific foreign declarations and boundary syntax.

It must remain an interoperability grammar rather than a second complete Zig compiler.

It must normalize into the canonical:

types
functions
FFI
ABI
calling convention
linkage
ownership
effects
capabilities
resources

architecture.

---

25. "AssemblyLanguage.g4"

Assembly interoperability is target representation interoperability.

It MUST NOT become the universal machine model.

It may represent:

- sections;
- symbols;
- labels;
- directives;
- instructions;
- operands;
- symbolic references;
- relocation intent;
- architecture metadata.

It must not make any one architecture universal.

No fixed:

- register set;
- register count;
- opcode table;
- address width;
- word width;
- instruction width;

belongs in the universal interoperability architecture.

Assembly remains downstream of canonical semantic/IR lowering.

---

26. "api.g4"

Purpose

Defines abstract API interoperability contracts.

An API is a semantic boundary, not necessarily:

- a process;
- a network service;
- a library;
- a device;
- a kernel interface.

It may describe:

- operations;
- inputs;
- outputs;
- errors;
- capabilities;
- requirements;
- effects;
- version;
- compatibility;
- security;
- provenance.

Actual transport is resolved downstream.

---

27. System Interfaces

The repository currently contains both:

SystemInterfaces.g4
system-interfaces.g4

These contain overlapping system-interface concepts and therefore require explicit reconciliation.

The production architecture MUST establish exactly one canonical semantic owner.

The recommended migration is:

system-interfaces.g4
    canonical parser grammar

with:

SystemInterfaces.g4
    compatibility/migration source until all consumers are reconciled

However, this is an architectural migration decision, not a license to silently delete existing functionality.

The migration MUST:

1. compare both grammars rule by rule;
2. identify unique rules;
3. identify duplicate rules;
4. identify conflicting syntax;
5. identify token differences;
6. identify AST consumers;
7. identify parser imports;
8. merge missing unique semantics;
9. select one canonical grammar name;
10. update imports;
11. update tests;
12. verify generated parser output;
13. remove duplicate ownership only after conformance passes.

Neither file may remain permanently as two independent system-interface languages.

---

28. System Interface Semantic Boundary

System interfaces may describe:

- services;
- abstract operations;
- events;
- interrupt-like semantic events;
- handles;
- capabilities;
- resources;
- lifecycle contracts;
- properties;
- bindings.

They MUST NOT encode universal:

- syscall numbers;
- physical addresses;
- MMIO addresses;
- CPU registers;
- interrupt-vector numbers;
- device IDs;
- processor IDs;
- core counts;
- thread counts;
- machine topology.

Those belong to target-specific systems.

---

29. "serialization.g4"

Purpose

Defines source-level serialization intent.

It may express:

- format;
- schema;
- encoding;
- version;
- compatibility;
- ownership;
- conversion;
- validation;
- provenance;
- security requirements.

It MUST NOT execute serialization during parsing.

Supported formats remain extensible.

The grammar must not become a catalog containing every serialization format ever invented.

---

30. "deserialization.g4"

Defines deserialization intent.

The semantic model must distinguish:

decode
validate
convert
construct

A successful parse of an external representation does not automatically mean the resulting value is semantically valid.

Deserialization MUST participate in:

- type checking;
- validation;
- security;
- resource limits;
- provenance;
- compatibility.

No silent unchecked conversion is permitted where the semantic contract requires validation.

---

31. Data Interchange

Interoperability may support:

- JSON;
- XML;
- binary formats;
- scientific formats;
- structured data;
- schema systems;
- graph data;
- tabular data;
- query systems.

These remain external formats or dialects.

They do not automatically become universal Zamani syntax.

---

32. SQL and Query Interoperability

SQL must remain an external/dialect boundary.

The architecture is:

SQL
 │
 ▼
SQL dialect/parser
 │
 ▼
query semantic model
 │
 ▼
Zamani data/query representation
 │
 ▼
canonical semantic model

SQL syntax must not be copied into the universal expression grammar merely to provide interoperability.

---

33. "openqasm.g4"

OpenQASM is an external quantum source format.

Its pipeline is:

OpenQASM source
      │
      ▼
OpenQASM grammar
      │
      ▼
OpenQASM semantic normalization
      │
      ▼
quantum::ir
      │
      ▼
optimization
      │
      ▼
decomposition
      │
      ▼
routing
      │
      ▼
scheduling
      │
      ▼
QEC / resilience
      │
      ▼
ZQN
      │
      ▼
HAL

OpenQASM MUST NOT become Zamani's canonical quantum IR.

---

34. "qasm.g4"

"qasm.g4" MUST have a clearly documented compatibility role relative to "openqasm.g4".

It must not create two competing OpenQASM semantic models.

The repository must establish whether it is:

- legacy QASM compatibility;
- a reduced format;
- a compatibility frontend;
- an alternate version.

Its status must be explicitly recorded in "grammar/grammar.md".

---

35. "qir.g4"

QIR is an interoperability representation.

It must not replace:

quantum::ir

The correct relationship is:

QIR
 │
 ▼
QIR normalization
 │
 ▼
quantum::ir

or:

quantum::ir
 │
 ▼
QIR lowering

depending on direction.

QIR-specific constructs must never leak into Zamani's domain-neutral AST merely because QIR has a particular implementation model.

---

36. "hdl.g4"

HDL interoperability must normalize into Zamani's hardware/HDL semantic architecture.

The pipeline is:

external HDL
    ↓
HDL interoperability grammar
    ↓
external HDL semantic representation
    ↓
Zamani HDL semantic model
    ↓
canonical hardware representation
    ↓
verification
    ↓
synthesis
    ↓
placement/routing
    ↓
target realization

It must not directly encode a universal physical FPGA/ASIC architecture.

---

37. "verilog.g4"

"verilog.g4" owns Verilog source-format syntax.

It does not own:

- Zamani HDL semantics;
- synthesis;
- placement;
- routing;
- hardware discovery;
- target selection;
- physical pin mapping.

Verilog and future SystemVerilog support must remain distinguishable.

SystemVerilog syntax MUST NOT silently become accepted as Verilog syntax.

---

38. "wasm.g4"

WebAssembly interoperability must distinguish:

Wasm source/module representation

from:

Zamani semantic program

The grammar may represent:

- modules;
- imports;
- exports;
- types;
- functions;
- tables;
- memories;
- globals;
- instructions;
- data;
- metadata.

It must not impose a universal machine architecture on Zamani.

Concrete Wasm limitations are format semantics, not Zamani-wide capacity ceilings.

---

39. External Language Identity

Foreign-language identities MUST be extensible.

The architecture must support identities such as:

C
C++
Rust
Python
Zig
vendor::language
organization::language
future::language

The universal grammar must not contain an eternal closed list.

Adding a new external language should normally require:

new interoperability adapter
+
registration/metadata
+
semantic adapter
+
tests

rather than redesigning Zamani's universal grammar.

---

40. External Format Identity

Languages and formats are different concepts.

Examples:

language = C
format   = C-source

language = OpenQASM
format   = OpenQASM

format = JSON
format = XML
format = QIR
format = WebAssembly
format = Verilog

The grammar must preserve that distinction.

---

41. ABI Versus Calling Convention

The following concepts MUST remain separate:

ABI
calling convention
linkage
data representation
foreign type
foreign function
runtime

An ABI may contain or reference calling-convention information, but they are not interchangeable semantic concepts.

---

42. Runtime Boundaries

Runtime identity is separate from:

- language identity;
- format identity;
- ABI identity;
- target identity.

A declaration of a runtime contract does not cause runtime startup.

Parsing remains inert.

---

43. Conversion Model

Interoperability conversions must distinguish:

lossless
lossy
fallible
infallible
validated
contract-authorized

Semantic loss MUST be explicit.

The parser must not silently turn a potentially lossy conversion into an infallible operation.

---

44. Marshaling Model

Marshaling may involve:

- representation conversion;
- ownership transfer;
- borrowing;
- lifetime adaptation;
- serialization;
- deserialization;
- encoding;
- decoding;
- ABI adaptation.

Marshaling execution belongs downstream.

---

45. Callback Model

Callbacks are callable contracts.

A callback declaration must integrate with:

- canonical function types;
- ownership;
- lifetime;
- effects;
- capabilities;
- resources;
- concurrency;
- error semantics;
- provenance.

A callback does not automatically imply:

- thread creation;
- process creation;
- runtime creation;
- network access.

---

46. Ownership and Lifetime

Foreign boundaries MUST use the canonical Zamani ownership/lifetime model.

Allowed semantic states may include:

owned
borrowed
shared
transferred
retained
returned
released
opaque

These are semantic contracts.

They are not physical addresses or allocation instructions.

There must be no second ownership system hidden inside interoperability.

---

47. Effects

Interoperability declarations MUST integrate with the canonical effect system.

Examples include:

effect(io)
effect(network)
effect(native)
effect(foreign)
effect(mutation)
effect(randomness)
effect(distributed)
effect(quantum.measurement)
effect(device)

The actual canonical effect vocabulary belongs to "grammar/effects/".

Interoperability files may reference effects but must not redefine their global meaning.

---

48. Capabilities

Interoperability boundaries may require capabilities such as:

capability("foreign.call")
capability("native.execute")
capability("network")
capability("filesystem")
capability("quantum.execute")
capability("device.access")
capability("accelerator.execute")

Capability semantics belong to the canonical capability/resource architecture.

A grammar declaration does not grant a capability.

It declares a requirement.

---

49. Resources

Interoperability may express resource requirements.

Examples:

requires memory >= required_memory;
requires capability("foreign.call");
requires capability("quantum.execute");
requires topology(required_topology);

These are semantic requirements.

They do not select a physical resource.

The compiler/runtime resolves:

requirement
    ↓
capability
    ↓
resource availability
    ↓
target realization

No finite universal capacity must be encoded.

---

50. Contracts

Interoperability boundaries MUST integrate with canonical contracts:

requires
ensures
invariant
assume
guarantee
property

A foreign declaration may require:

preconditions
postconditions
representation guarantees
ownership guarantees
error guarantees
security guarantees
compatibility guarantees

Contract semantics belong to "grammar/validation/" and the semantic layer.

---

51. Policies

Interoperability must integrate with the canonical policy architecture.

Policies may govern:

- allowed foreign languages;
- allowed runtimes;
- allowed effects;
- allowed capabilities;
- allowed resources;
- allowed network access;
- allowed native calls;
- allowed dynamic loading;
- serialization;
- deserialization;
- adaptation;
- fallback behavior.

Policies constrain realization.

They do not become hidden grammar execution.

---

52. Security

Parsing interoperability syntax MUST be inert.

It must never:

- load a library;
- execute a foreign function;
- execute a syscall;
- access a filesystem;
- access a network;
- enumerate hardware;
- inspect processes;
- inspect environment state;
- allocate native resources;
- resolve physical addresses;
- launch a runtime.

Those operations require downstream authorization and security controls.

---

53. Provenance

Every interoperability transformation that can affect meaning SHOULD preserve provenance.

Relevant provenance may include:

source
source format
source version
adapter
transformation
conversion
ABI
calling convention
foreign symbol
semantic decision
validation
compatibility decision
target realization

This is especially important for:

- foreign data;
- scientific computing;
- quantum programs;
- hardware descriptions;
- AI/data pipelines;
- security-sensitive integrations.

---

54. Compatibility

Interoperability compatibility is multidimensional.

The subsystem must distinguish:

language compatibility
format compatibility
ABI compatibility
calling-convention compatibility
type compatibility
representation compatibility
version compatibility
runtime compatibility
capability compatibility
resource compatibility
security compatibility
semantic compatibility

A successful syntax parse does not prove compatibility.

---

55. Versioning

External versions must remain explicit.

A declaration may identify:

language version
format version
ABI version
API version
runtime version
schema version
compatibility profile

Version identifiers are metadata.

They do not imply that the current target supports the requested version.

Compatibility analysis determines feasibility.

---

56. Dialects

Interoperability adapters may use dialects.

A dialect MUST have:

- identity;
- owner;
- version;
- compatibility contract;
- semantic mapping;
- source-span mapping;
- diagnostics;
- tests.

Vendor extensions must not silently become universal Zamani syntax.

---

57. Domain-Neutral AST Boundary

No interoperability grammar may require the AST to become:

- C AST;
- C++ AST;
- Python AST;
- Rust AST;
- OpenQASM AST as the canonical program AST;
- QIR AST;
- Verilog AST;
- WebAssembly AST.

External source representations may have temporary frontend structures.

They must normalize into the domain-neutral Zamani semantic representation.

The universal AST must remain independent of external vendor topology and target-specific implementation details.

---

58. Quantum Boundary

All quantum interoperability must eventually converge on:

quantum::ir

The canonical path is:

external quantum format
      ↓
format-specific parser
      ↓
format-specific normalization
      ↓
semantic validation
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

No external quantum format may establish a competing canonical quantum IR.

---

59. HDL Boundary

External HDL must converge on the canonical Zamani hardware/HDL semantic model.

The interoperability grammar does not own:

- synthesis;
- placement;
- routing;
- timing closure;
- physical pin assignment;
- device selection.

Those are downstream target realizations.

---

60. Classical Boundary

Foreign classical languages and formats normalize into the same semantic architecture as native Zamani computation.

There must not be:

native type system
+
foreign type system

as permanently competing semantic systems.

Foreign types are boundary types that normalize into canonical semantics.

---

61. Distributed and Network Boundaries

Foreign APIs and system interfaces may ultimately represent:

- local calls;
- RPC;
- messaging;
- services;
- streams;
- distributed operations;
- remote computation.

The grammar does not decide whether a call is local or remote unless the source contract explicitly requires that semantic distinction.

Network realization belongs to networking/runtime layers.

---

62. Interoperability and AI/Data Systems

Interoperability must support external:

- model formats;
- data schemas;
- tensor formats;
- query systems;
- model runtimes;
- inference runtimes;
- scientific data;
- graph formats.

However, application concepts should remain semantic/library abstractions rather than producing an ever-growing list of application-specific keywords.

The same interoperability architecture must support:

model
data
reasoning
learning
quantum computation
classical computation
hardware
distributed execution

through shared:

types
effects
capabilities
resources
contracts
policies
provenance

---

63. Metaprogramming and Reflection

Macros, reflection, compile-time execution, and generated interoperability declarations MUST NOT bypass:

- type validation;
- effect validation;
- capability validation;
- resource validation;
- policy validation;
- security validation;
- provenance.

Generated foreign declarations are still foreign declarations.

---

64. Determinism

Parsing interoperability syntax MUST be deterministic.

For identical:

source
lexer configuration
grammar version
dialect configuration

the parser must produce equivalent syntactic results.

Interoperability parsing must not depend on:

- current hardware;
- filesystem state;
- network state;
- runtime availability;
- library discovery;
- device discovery;
- scheduler state.

---

65. Source Locations

Every public interoperability construct must preserve source locations.

At minimum:

start
end
source file

must remain available to diagnostics and semantic analysis.

Where the frontend supports richer spans, those spans should be preserved through normalization.

---

66. Diagnostics

Every interoperability feature must define diagnostics for:

- unknown foreign identity;
- malformed declaration;
- incompatible version;
- incompatible ABI;
- incompatible calling convention;
- incompatible type;
- invalid conversion;
- ownership mismatch;
- lifetime mismatch;
- nullability mismatch;
- unavailable capability;
- unavailable resource;
- forbidden effect;
- policy violation;
- unsupported format;
- unsupported dialect;
- unsupported runtime;
- semantic mismatch;
- target incompatibility.

Diagnostics must distinguish:

syntax failure

from:

semantic incompatibility

and:

target infeasibility

---

67. Error Handling

Foreign calls must have explicit error semantics where required.

Possible contracts include:

fallible
infallible
exceptional
result-based
status-based
panic/abort
recoverable
retryable

The exact semantic model belongs to the canonical error/type/effect architecture.

Interoperability must not invent an incompatible error system.

---

68. Resource Safety

Foreign declarations must not bypass the resource model.

Examples include:

- memory;
- storage;
- network bandwidth;
- device access;
- accelerator availability;
- quantum resources;
- distributed resources;
- runtime handles.

The compiler may reject an impossible target realization while preserving source validity.

---

69. No Artificial Universal Limits

Interoperability grammar MUST NOT encode artificial universal limits for:

foreign functions
foreign types
parameters
interfaces
APIs
symbols
languages
formats
devices
targets
nodes
threads
memory
storage
qubits
registers
tensor dimensions
network links
messages
services

The following classes of hard-coded language ceilings are prohibited:

MAX_FOREIGN_FUNCTIONS
MAX_FOREIGN_TYPES
MAX_INTERFACES
MAX_TARGETS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_QUBITS
MAX_GPUS
MAX_FPGAS
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE

Real implementation limits may exist because computers are finite.

Such limits must remain implementation/resource policies rather than language semantics.

---

70. Numeric Values

Ordinary numeric literals are program data.

This is valid:

let count = 1024;

It does not establish a universal capacity.

The following distinction must remain absolute:

program value

versus:

language capacity ceiling

The first is allowed.

The second is prohibited when artificial and universal.

---

71. Target Independence

Interoperability declarations MUST NOT silently select a target.

For example:

language "C"

does not mean:

x86

and:

format "OpenQASM"

does not mean:

QPU #n

Likewise:

format "Verilog"

does not mean:

specific FPGA

Target selection is a downstream realization decision.

---

72. Physical Address Independence

Interoperability syntax must not hard-code physical:

- memory addresses;
- MMIO addresses;
- device addresses;
- register addresses;
- interrupt vectors;
- PCI addresses.

If a target-specific physical address is required, it must enter through an explicitly target-specific subsystem and policy.

It must never become a universal Zamani grammar assumption.

---

73. Runtime Inertness

The grammar must be entirely declarative.

ANTLR grammars in this directory MUST NOT contain actions that:

- execute code;
- access files;
- access networks;
- resolve symbols;
- load libraries;
- query hardware;
- query runtimes;
- allocate external resources.

Generated Rust must remain compatible with the repository's safe-Rust requirement.

---

74. Rust Requirements

All Rust implementation surrounding interoperability MUST satisfy:

Rust 2021
Rust 1.97.1 or later
safe Rust only
no unsafe

Grammar files must not rely on embedded Rust actions to provide semantic behavior.

Semantic behavior belongs in the appropriate Rust frontend/compiler subsystem.

Any new Rust implementation must pass the repository's unsafe-code prohibition.

---

75. Lexer Integration

Interoperability grammars consume the canonical Zamani lexer.

They MUST NOT create a second universal lexer.

Shared lexical concepts must come from:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4

Where external formats require format-specific lexical behavior, that behavior must remain isolated to the external-format adapter and must not pollute the universal Zamani token vocabulary unnecessarily.

---

76. Parser Integration

The repository must have one canonical parser composition authority.

Interoperability grammars are leaf/composition modules.

They are not parser roots.

The integration path is:

ZamaniParser.g4
      ↓
interoperability entry
      ↓
feature family
      ↓
specialized grammar

External source formats that genuinely require their own parser may have format-specific parser roots, but those roots must remain external-format frontends and must not replace "ZamaniParser.g4".

---

77. AST Integration

Each interoperability feature must map to an AST contract.

Each AST node must identify:

source span
external identity
semantic category
declaration/reference role
attributes
metadata

The AST must not encode target-specific realization state.

---

78. Semantic Integration

Every interoperability node must pass through:

name resolution
type checking
effect checking
capability checking
resource checking
contract checking
policy checking
security validation
compatibility checking
provenance

The exact subset depends on the feature.

No interoperability declaration may bypass the semantic pipeline.

---

79. IR Integration

Interoperability does not own a universal IR.

Its outputs are:

canonical semantic representation

which then lower into the appropriate IR.

For example:

C boundary
    ↓
canonical semantic model
    ↓
classical IR

or:

OpenQASM
    ↓
quantum semantic normalization
    ↓
quantum::ir

or:

Verilog
    ↓
HDL semantic normalization
    ↓
hardware representation

---

80. Compiler Integration

Compiler stages consume validated interoperability contracts.

The compiler is responsible for:

- ABI realization;
- conversion lowering;
- marshaling lowering;
- target-specific adaptation;
- linking;
- code generation;
- compatibility checking;
- fallback selection where explicitly authorized.

The grammar must never perform these operations.

---

81. Runtime Integration

Runtime infrastructure is responsible for:

- foreign runtime invocation;
- dynamic resource realization;
- device access;
- remote invocation;
- service invocation;
- callback execution;
- runtime error handling;
- runtime capability validation;
- runtime resource validation.

The parser does not perform any of these operations.

---

82. HAL Integration

Hardware abstraction is downstream.

Interoperability may describe a hardware-facing contract, but:

interoperability
    ↓
semantic hardware intent
    ↓
hardware/compiler layers
    ↓
HAL

must remain the architecture.

---

83. POCO-REAF

Interoperability must preserve the central POCO-REAF invariant:

ONE SOURCE PROGRAM
        │
        ▼
ONE STABLE SEMANTIC CONTRACT
        │
        ▼
DOMAIN-NEUTRAL AST
        │
        ▼
CANONICAL SEMANTICS
        │
        ├── classical
        ├── quantum::ir
        └── HDL/hardware
        │
        ▼
TARGET-INDEPENDENT OPTIMIZATION
        │
        ▼
TARGET-SPECIFIC REALIZATION

The same source contract may be realized on:

tiny embedded target
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed system
cloud
future computational substrate

provided the target satisfies the semantic requirements.

---

84. Target Feasibility

The architecture MUST distinguish:

language validity

from:

target feasibility

For example:

requires capability("quantum.measurement");

may be valid source semantics even if a particular target has no such capability.

The correct result is an unsatisfied requirement diagnostic, not source rewriting.

Permitted fallback behavior must be explicitly declared through policy or compilation configuration.

---

85. Simulation

Interoperability may target simulation.

Simulation remains an execution strategy.

Examples:

foreign quantum format
    ↓
quantum::ir
    ↓
simulator

foreign hardware format
    ↓
HDL semantic model
    ↓
hardware simulator

Simulation does not create a second language.

---

86. Adaptive Execution

Interoperability may participate in adaptive execution.

The semantic architecture is:

requirement
    ↓
capability discovery
    ↓
resource analysis
    ↓
policy
    ↓
candidate realization
    ↓
validation
    ↓
execution

Adaptation must not silently change program meaning.

---

87. Security Boundary

The interoperability subsystem must assume foreign boundaries are security-sensitive.

Security analysis must account for:

- native execution;
- foreign calls;
- dynamic loading;
- serialization;
- deserialization;
- network calls;
- filesystem access;
- callbacks;
- reflection;
- external runtimes;
- device access;
- privileged system interfaces.

The grammar declares the boundary.

The security subsystem decides whether the boundary is permitted.

---

88. Provenance Boundary

When external artifacts are imported, the semantic pipeline SHOULD retain:

source artifact
format
version
adapter
transformation
validation
conversion
ABI
target
compiler version
policy

This supports:

- reproducibility;
- debugging;
- audit;
- scientific traceability;
- security;
- compatibility.

---

89. File Contract Standard

Every interoperability ".g4" file MUST contain a feature contract documenting:

Purpose
Owns
Does Not Own
Dependencies
Exports
Consumed By
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
Security Integration
Provenance Integration
IR Destination
Compiler Integration
Runtime Integration
HAL/Target Integration
Diagnostics
Compatibility
Scalability
Determinism
Positive Tests
Negative Tests
Boundary Tests
Cross-Domain Tests
Completion Criteria

This makes a file independently completable.

A later modification of another subsystem must not require reopening the file merely because that subsystem has grown, provided its public contract remains compatible.

---

90. Dependency Contract

Every interoperability file MUST document:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
CAPABILITY_OWNER:
RESOURCE_OWNER:
CONTRACT_OWNER:
POLICY_OWNER:
SECURITY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
COMPILER_CONSUMER:
RUNTIME_CONSUMER:
TEST_OWNER:
SPEC_OWNER:

---

91. Existing File Registry

The current directory includes, at minimum, the following interoperability files:

AssemblyLanguage.g4
SystemInterfaces.g4
abi.g4
api.g4
c.g4
calling-conventions.g4
cpp.g4
data-layout.g4
deserialization.g4
external-foreign-functions.g4
external-functions.g4
external-types.g4
ffi.g4
foreign-functions.g4
foreign-types.g4
foreign.g4
hdl.g4
interoperability.g4        ← canonical target path; current tree has trailing-space issue
linkage.g4
openqasm.g4
python.g4
qasm.g4
qir.g4
rust.g4
serialization.g4
system-interfaces.g4
verilog.g4
wasm.g4
zig.g4
README.md

The directory registry in this README is an architectural registry, not permission to assume every file is already production-ready.

Conformance status must come from the actual grammar, implementation and tests.

---

92. Required New Files

Only create a new file when an existing owner cannot cleanly own the responsibility.

Potential future additions include:

callbacks.g4
conversions.g4
marshaling.g4
symbols.g4

Before creating one, perform an ownership audit.

A new file is justified only if it:

1. has one clear purpose;
2. has distinct ownership;
3. removes duplication;
4. has a stable integration boundary;
5. has tests;
6. has an AST/semantic contract;
7. has a clear downstream consumer.

Do not create files merely to increase modularity.

---

93. No Parallel Hierarchies

Do not create:

interoperability2/
interop/
foreign2/
external2/
ffi2/

as competing architecture.

The existing directory remains the interoperability subsystem.

Subdirectories should only be introduced if the number of files and ownership boundaries make flat organization materially harder to maintain.

If subdirectories are introduced, this README remains the orchestrator.

---

94. Recommended Future Subdirectory Boundary

If growth requires subdirectories, use semantic families rather than technologies:

grammar/interoperability/
├── languages/
├── formats/
├── abi/
├── system/
├── data/
├── quantum/
├── hdl/
└── runtime/

But existing filenames should not be moved solely for aesthetics.

A migration requires:

old path
→ ownership review
→ import update
→ build update
→ test update
→ compatibility record
→ new path
→ old path removal

---

95. Testing Architecture

Interoperability tests must be divided into:

tests/interoperability/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── types/
├── effects/
├── capabilities/
├── resources/
├── contracts/
├── policies/
├── security/
├── provenance/
├── compatibility/
├── ffi/
├── abi/
├── calling-conventions/
├── linkage/
├── foreign-types/
├── foreign-functions/
├── serialization/
├── deserialization/
├── api/
├── system/
├── c/
├── cpp/
├── rust/
├── python/
├── zig/
├── assembly/
├── wasm/
├── openqasm/
├── qir/
├── hdl/
├── verilog/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── determinism/
└── cross-domain/

The actual repository may organize these differently, but every category must be covered.

---

96. Positive Tests

Every feature requires valid examples.

Examples must cover:

- minimal declaration;
- complete declaration;
- optional metadata;
- generic forms;
- ownership;
- lifetime;
- capabilities;
- resources;
- effects;
- compatibility;
- versioning;
- callbacks;
- conversions.

---

97. Negative Tests

Every feature requires invalid examples.

Tests must cover:

- malformed syntax;
- undefined names;
- invalid types;
- incompatible ABI;
- incompatible calling convention;
- invalid conversion;
- invalid ownership;
- invalid lifetime;
- invalid capability;
- policy violations;
- security violations;
- incompatible versions;
- conflicting metadata.

---

98. Boundary Tests

Boundary tests must verify interactions between:

FFI ↔ ABI
FFI ↔ types
FFI ↔ effects
FFI ↔ capabilities
FFI ↔ resources
FFI ↔ contracts
FFI ↔ policies
FFI ↔ provenance

and:

OpenQASM ↔ quantum::ir
QIR ↔ quantum::ir
HDL ↔ hardware semantics
foreign types ↔ canonical types
foreign functions ↔ canonical functions
serialization ↔ data types
system interfaces ↔ resources/capabilities

---

99. Cross-Domain Tests

At least one end-to-end program must combine interoperability with:

- classical computation;
- quantum computation;
- AI/learning;
- data;
- concurrency;
- distributed execution;
- HDL/hardware;
- resource requirements;
- capabilities;
- effects;
- contracts;
- policies;
- provenance.

The expected architecture is:

source
 ↓
AST
 ↓
semantic validation
 ↓
resource/capability/effect analysis
 ↓
canonical semantic model
 ↓
classical representation
 + quantum::ir
 + HDL/hardware representation
 ↓
optimization
 ↓
lowering
 ↓
target realization

---

100. Determinism Tests

Identical input and identical grammar/lexer configuration must produce identical parsing behavior.

No test may depend on:

- host CPU;
- host memory size;
- current hardware;
- filesystem enumeration order;
- network state;
- library installation order;
- device discovery order.

---

101. Scalability Tests

Scalability tests must use generated or symbolic inputs rather than fixed artificial ceilings.

They must verify that the grammar architecture does not contain:

MAX_*

capacity constants.

Test dimensions include:

- many declarations;
- many parameters;
- deep but valid nesting;
- large metadata sets;
- large foreign symbol sets;
- large type graphs;
- large serialization schemas;
- large HDL modules;
- large quantum programs;
- large distributed descriptions.

Practical test-runner limits are permitted as test infrastructure configuration.

They must not become language semantics.

---

102. Hard-Coding Audit

Every production review must search interoperability for artificial universal constraints.

At minimum audit for:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_FOREIGN_FUNCTIONS
MAX_TARGETS
MAX_INTERFACES

Also audit for hard-coded:

- physical addresses;
- register names;
- fixed pointer widths;
- fixed machine word sizes;
- fixed device IDs;
- fixed processor counts;
- fixed network topology;
- fixed quantum topology;
- fixed accelerator counts.

Any occurrence must be justified as:

program data

or:

target-specific configuration

or removed.

---

103. Security Audit

Every interoperability grammar must pass an audit for:

filesystem access
network access
process execution
library loading
runtime invocation
hardware discovery
device access
environment access
native memory access
physical address access
dynamic symbol resolution

None may be performed during parsing.

---

104. Compatibility Audit

Every change must check:

lexer compatibility
parser compatibility
AST compatibility
semantic compatibility
IR compatibility
source compatibility
dialect compatibility
external format compatibility
ABI compatibility
runtime compatibility

Breaking changes require an explicit migration contract.

---

105. Deprecation

A deprecated interoperability grammar MUST document:

deprecated feature
replacement
reason
first deprecated version
migration path
removal policy
tests

Deprecated syntax must not silently acquire new semantics.

---

106. External Format Versioning

External-format grammars must not silently mix incompatible versions.

For example:

Verilog
SystemVerilog
OpenQASM versions
WebAssembly versions
QIR versions

must remain distinguishable.

A parser may support multiple versions, but semantic version selection must remain explicit.

---

107. Vendor Extensions

Vendor-specific interoperability must use explicit extension identity.

Conceptually:

vendor::extension

or the canonical dialect/namespace mechanism.

Vendor behavior must not silently become universal Zamani semantics.

---

108. Future Technology

A future language or format must be integrable without redesigning:

Zamani AST
resource model
capability model
effect model
contract model
policy model
provenance model
canonical IR

The normal extension path is:

new external technology
        ↓
adapter grammar
        ↓
semantic adapter
        ↓
canonical semantic representation
        ↓
existing compiler pipeline

---

109. What Interoperability Must Never Become

This directory must never become:

- a collection of complete foreign compilers;
- a second Zamani parser;
- a second lexer;
- a universal hardware database;
- a runtime;
- a linker;
- a loader;
- a device manager;
- a scheduler;
- a resource allocator;
- a quantum router;
- a QEC engine;
- a HAL;
- a second IR system.

Its responsibility is the language boundary.

---

110. Integration with Resources, Capabilities, Effects and Contracts

Every boundary must be representable through the common semantic model:

FOREIGN BOUNDARY
      │
      ├── TYPE
      ├── EFFECT
      ├── CAPABILITY
      ├── RESOURCE
      ├── CONTRACT
      ├── POLICY
      ├── SECURITY
      └── PROVENANCE
      │
      ▼
CANONICAL SEMANTICS

This is what allows interoperability to participate in POCO-REAF rather than becoming a collection of unrelated adapters.

---

111. Integration with Learning, Reasoning and Adaptive Systems

Interoperability may expose external:

- model runtimes;
- data systems;
- inference systems;
- learning systems;
- reasoning engines;
- simulation engines;
- optimization engines.

These are external capabilities.

They must integrate through:

types
effects
capabilities
resources
contracts
policies
provenance

They must not require an application-specific keyword for every external service.

---

112. Integration with Quantum Systems

A foreign quantum service or format may describe:

- quantum operations;
- measurements;
- classical control;
- dynamic execution;
- error handling;
- calibration metadata;
- device capabilities.

But physical realization remains downstream.

The semantic pipeline remains:

foreign quantum boundary
      ↓
quantum semantics
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

---

113. Integration with Hardware

A foreign hardware interface may describe:

operation
signal
resource
capability
timing intent
data representation
interface

It must not permanently encode a specific hardware topology into the universal grammar.

---

114. Integration with Distributed Systems

Distributed interoperability may describe:

service
endpoint
operation
message
stream
protocol
capability
resource
consistency
failure
retry

Physical node counts remain target/resource information.

---

115. Integration with Networking

Network interoperability must distinguish:

protocol identity
endpoint semantics
message semantics
transport
security
capability
resource

The grammar does not open sockets or discover endpoints.

---

116. Integration with Memory

Foreign memory ownership must map into the canonical memory model.

The interoperability subsystem must never introduce a second memory allocator or ownership authority.

Foreign pointers/handles are boundary representations.

Their semantics are determined by:

type
ownership
lifetime
ABI
target
runtime

---

117. Integration with Concurrency

Foreign calls may declare:

blocking
nonblocking
async
streaming
cancellable
transactional
thread-safe
reentrant

These must map into the canonical concurrency/effect model.

The interoperability grammar must not create a second scheduler.

---

118. Integration with Execution

Execution modes may include:

native
foreign
remote
simulated
emulated
accelerated
quantum
distributed

These are semantic/target realization properties.

Parsing remains inert.

---

119. Integration with Compatibility

Interoperability must consume the canonical compatibility system.

It should allow the compiler to distinguish:

source compatible
binary compatible
ABI compatible
semantic compatible
runtime compatible
target compatible

rather than reducing all compatibility to one boolean.

---

120. Integration with Provenance

Every transformation that changes representation must be traceable when provenance is required:

foreign source
   ↓
adapter
   ↓
normalized representation
   ↓
semantic validation
   ↓
canonical representation
   ↓
lowering

This enables reproducible and auditable interoperability.

---

121. Independent-File Completion Rule

A file is considered complete only when its own contract is complete.

For every file:

Purpose
    ↓
Ownership
    ↓
Dependencies
    ↓
Exports
    ↓
AST mapping
    ↓
Semantic mapping
    ↓
Type mapping
    ↓
Effect mapping
    ↓
Capability mapping
    ↓
Resource mapping
    ↓
Contract mapping
    ↓
Policy mapping
    ↓
Security mapping
    ↓
Provenance mapping
    ↓
IR destination
    ↓
Compiler integration
    ↓
Runtime integration
    ↓
Diagnostics
    ↓
Compatibility
    ↓
Tests
    ↓
Scalability
    ↓
Determinism
    ↓
Hard-coding audit
    ↓
Completion

A file must not depend on undocumented assumptions about future files.

---

122. Implementation Order

The interoperability subsystem should be completed in dependency order.

Phase 1 — Authority

README.md
interoperability.g4

First establish the composition root and ownership map.

Phase 2 — Universal boundary primitives

ffi.g4
foreign.g4
foreign-types.g4
foreign-functions.g4
abi.g4
calling-conventions.g4
linkage.g4
data-layout.g4

Phase 3 — Boundary mechanics

external-types.g4
external-functions.g4
external-foreign-functions.g4
serialization.g4
deserialization.g4
api.g4

Phase 4 — System integration

system-interfaces.g4
SystemInterfaces.g4

Reconcile these before declaring the system-interface family stable.

Phase 5 — Language adapters

c.g4
cpp.g4
rust.g4
python.g4
zig.g4

Phase 6 — Representation adapters

AssemblyLanguage.g4
wasm.g4

Phase 7 — Quantum adapters

openqasm.g4
qasm.g4
qir.g4

Phase 8 — HDL adapters

hdl.g4
verilog.g4

Phase 9 — Conformance

Complete:

positive
negative
boundary
cross-domain
scalability
determinism
security
compatibility

before declaring production readiness.

---

123. Completion Criteria for "interoperability.g4"

Done means:

- canonical filename;
- no trailing-space filename;
- one composition authority;
- no duplicate universal rules;
- all public rules documented;
- all imports resolve;
- all grammar names are unique;
- all token references resolve;
- AST mappings exist;
- semantic mappings exist;
- diagnostics exist;
- tests exist;
- deterministic parsing verified.

---

124. Completion Criteria for "ffi.g4"

Done means:

- one FFI authority;
- no ABI duplication;
- no function-system duplication;
- canonical types consumed;
- ownership/lifetime integrated;
- effects integrated;
- capabilities integrated;
- resources integrated;
- contracts integrated;
- policies integrated;
- security integrated;
- provenance integrated;
- AST mapping verified;
- positive/negative/boundary tests pass.

---

125. Completion Criteria for ABI Files

ABI-related files are complete only when:

ABI identity
+
calling convention
+
linkage
+
representation
+
foreign callable
+
foreign type

have clearly separated ownership.

No physical machine details may be encoded as universal semantics.

---

126. Completion Criteria for Language Adapters

Each language adapter is complete when it has:

language identity
version handling
foreign declarations
foreign types
foreign functions
ABI mapping
calling-convention mapping
linkage mapping
ownership mapping
conversion mapping
error mapping
capability mapping
resource mapping
security mapping
provenance mapping
AST mapping
semantic normalization
tests

It is not necessary to implement the complete external language.

---

127. Completion Criteria for Format Adapters

Each format adapter is complete when:

format version
syntax
validation
normalization
semantic mapping
source locations
diagnostics
compatibility
provenance
tests

are defined.

---

128. Completion Criteria for Quantum Adapters

Each quantum adapter must demonstrate:

external format
    ↓
normalization
    ↓
quantum semantic validation
    ↓
quantum::ir

No alternate canonical quantum IR is permitted.

---

129. Completion Criteria for HDL Adapters

Each HDL adapter must demonstrate:

external HDL
    ↓
HDL normalization
    ↓
Zamani hardware/HDL semantics
    ↓
canonical hardware representation

Physical synthesis remains downstream.

---

130. Production Readiness Gate

"grammar/interoperability/" is production-ready only when every stable interoperability feature has traceability through:

SPECIFICATION
    ↓
LEXER
    ↓
GRAMMAR
    ↓
AST
    ↓
SEMANTICS
    ↓
TYPE CHECKING
    ↓
EFFECT CHECKING
    ↓
CAPABILITY CHECKING
    ↓
RESOURCE CHECKING
    ↓
CONTRACT CHECKING
    ↓
POLICY CHECKING
    ↓
SECURITY
    ↓
PROVENANCE
    ↓
CANONICAL REPRESENTATION
    ↓
IR
    ↓
LOWERING
    ↓
ABI / FORMAT / RUNTIME ADAPTER
    ↓
TARGET
    ↓
TESTS

---

131. Production Readiness Checklist

[ ] README is the interoperability orchestrator
[ ] One interoperability composition authority
[ ] Trailing-space composition filename eliminated
[ ] No competing parser root
[ ] No competing lexer authority
[ ] No duplicated universal grammar
[ ] Every file has one owner
[ ] Every public rule has an AST mapping
[ ] Every public rule has semantic ownership
[ ] Foreign languages are extensible
[ ] Foreign formats are extensible
[ ] Language and format are distinct
[ ] ABI and calling convention are distinct
[ ] Linkage is distinct
[ ] Runtime identity is distinct
[ ] Foreign types use canonical types
[ ] Foreign functions use canonical functions
[ ] Ownership uses canonical memory semantics
[ ] Lifetime uses canonical lifetime semantics
[ ] Effects use canonical effects
[ ] Capabilities use canonical capabilities
[ ] Resources use canonical resources
[ ] Contracts use canonical contracts
[ ] Policies use canonical policies
[ ] Security uses canonical security
[ ] Provenance uses canonical provenance
[ ] AST remains domain-neutral
[ ] No interoperability IR exists
[ ] Quantum interoperability converges on quantum::ir
[ ] OpenQASM does not replace quantum::ir
[ ] QIR does not replace quantum::ir
[ ] HDL interoperability converges on HDL/hardware semantics
[ ] ABI realization is downstream
[ ] Linking is downstream
[ ] Runtime execution is downstream
[ ] Hardware discovery is downstream
[ ] Target selection is downstream
[ ] Routing is downstream
[ ] Scheduling is downstream
[ ] QEC is downstream
[ ] ZQN is downstream
[ ] HAL is downstream
[ ] No artificial universal capacity ceilings
[ ] No fixed universal pointer width
[ ] No fixed universal register width
[ ] No fixed universal word size
[ ] No fixed universal topology
[ ] No fixed device count
[ ] No fixed node count
[ ] No fixed qubit count
[ ] No fixed thread count
[ ] No fixed memory capacity
[ ] No parsing-time external execution
[ ] Parsing is deterministic
[ ] Source spans are preserved
[ ] Diagnostics are defined
[ ] Security behavior is defined
[ ] Compatibility behavior is defined
[ ] Version behavior is defined
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Cross-domain tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Hard-coding audit passes
[ ] Safe Rust only
[ ] Rust 2021
[ ] Rust 1.97.1+ compatibility
[ ] Specification-to-test traceability exists

---

132. Permanent Architectural Invariants

The following are permanent interoperability invariants.

1. "grammar/Zamani.g4" remains the canonical ANTLR composition root.

2. "grammar/antlr/ZamaniLexer.g4" remains the canonical lexer composition authority.

3. "grammar/antlr/ZamaniParser.g4" remains the canonical parser composition authority.

4. "grammar/interoperability/README.md" orchestrates the interoperability directory.

5. "grammar/interoperability/interoperability.g4" is the intended interoperability composition grammar.

6. The trailing-space "interoperability.g4 " path must not remain a production dependency.

7. Interoperability does not create another universal parser root.

8. Interoperability does not create another universal lexer.

9. Interoperability does not duplicate universal identifiers.

10. Interoperability does not duplicate the canonical type system.

11. Interoperability does not duplicate the canonical function system.

12. Interoperability does not duplicate the canonical ownership model.

13. Interoperability does not duplicate the canonical effect model.

14. Interoperability does not duplicate the canonical capability model.

15. Interoperability does not duplicate the canonical resource model.

16. Interoperability does not duplicate the canonical contract model.

17. Interoperability does not duplicate the canonical policy model.

18. Interoperability does not duplicate the canonical security model.

19. Interoperability does not duplicate the canonical provenance model.

20. Foreign languages are extensible identities.

21. Foreign formats are extensible identities.

22. Languages and formats remain distinct concepts.

23. ABIs and calling conventions remain distinct concepts.

24. Linkage remains distinct from ABI identity.

25. Runtime identity remains distinct from language identity.

26. Foreign symbols are semantic identities, not physical addresses.

27. FFI declarations do not execute foreign code.

28. Parsing never loads libraries.

29. Parsing never performs linking.

30. Parsing never performs filesystem access.

31. Parsing never performs network access.

32. Parsing never performs hardware discovery.

33. Parsing never executes system calls.

34. Parsing never starts runtimes.

35. Foreign types use canonical type semantics.

36. Foreign functions use canonical function semantics.

37. Foreign ownership uses canonical memory semantics.

38. Foreign effects use canonical effects.

39. Foreign capabilities use canonical capabilities.

40. Foreign resources use canonical resources.

41. Foreign contracts use canonical contracts.

42. Foreign policies use canonical policies.

43. Foreign security uses canonical security.

44. Foreign provenance uses canonical provenance.

45. The universal AST remains domain-neutral.

46. Interoperability does not create a universal interoperability IR.

47. Quantum interoperability converges on "quantum::ir".

48. OpenQASM does not replace "quantum::ir".

49. QIR does not replace "quantum::ir".

50. HDL interoperability does not replace native Zamani HDL semantics.

51. ABI realization remains downstream.

52. Linker implementation remains downstream.

53. Runtime execution remains downstream.

54. Hardware discovery remains downstream.

55. Target selection remains downstream.

56. Routing remains downstream.

57. Scheduling remains downstream.

58. Resilience remains downstream.

59. QEC remains downstream.

60. ZQN remains downstream.

61. HAL remains downstream.

62. No universal hardware capacity is encoded.

63. No universal CPU ceiling is encoded.

64. No universal GPU ceiling is encoded.

65. No universal FPGA ceiling is encoded.

66. No universal accelerator ceiling is encoded.

67. No universal QPU ceiling is encoded.

68. No universal qubit ceiling is encoded.

69. No universal node ceiling is encoded.

70. No universal thread ceiling is encoded.

71. No universal memory ceiling is encoded.

72. No universal device ceiling is encoded.

73. No universal network-size ceiling is encoded.

74. No universal tensor-rank ceiling is encoded.

75. No universal register-width ceiling is encoded.

76. No universal pointer width is assumed.

77. No universal physical topology is assumed.

78. Program numeric values remain program semantics.

79. Resource requirements are distinct from capabilities.

80. Capabilities are distinct from resources.

81. Constraints are distinct from preferences.

82. Preferences are distinct from implementation decisions.

83. Target infeasibility is distinct from language invalidity.

84. Semantic loss is never silent.

85. External formats do not silently become core syntax.

86. Vendor extensions do not silently become universal semantics.

87. Macros cannot bypass interoperability validation.

88. Metaprogramming cannot bypass interoperability validation.

89. Reflection cannot bypass capability/security validation.

90. Parsing is deterministic.

91. Source spans are preserved.

92. Provenance is preserved where required.

93. Stable features have specification-to-test traceability.

94. Rust implementation remains compatible with Rust 1.97.1 or later.

95. Rust implementation uses safe Rust only.

96. No Rust "unsafe" implementation is permitted.

97. Future languages can be added without redesigning the universal semantic architecture.

98. Future formats can be added without redesigning the universal semantic architecture.

99. Future runtimes can be added without redesigning the universal semantic architecture.

100. Future hardware can be added without redesigning the universal semantic architecture.

101. The same semantic source contract remains usable across different target realizations whenever target requirements are satisfied.

---

133. Final Architecture

The production interoperability architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                       ZamaniLexer
                              │
                              ▼
                      ZamaniParser
                              │
                              ▼
                  interoperability.g4
                              │
        ┌─────────────────────┼──────────────────────┐
        │                     │                      │
        ▼                     ▼                      ▼
       FFI              Foreign Declarations       Formats
        │                     │                      │
        ▼                     ▼                      ▼
      ABI              Foreign Types          OpenQASM / QIR
        │              Foreign Functions       WebAssembly
        ▼                     │               Verilog / HDL
Calling Convention            │               Data formats
        │                     │                      │
        └──────────────┬──────┴──────────────────────┘
                       │
                       ▼
               Domain-Neutral AST
                       │
                       ▼
              Semantic Validation
                       │
       ┌───────────────┼────────────────┐
       │               │                │
       ▼               ▼                ▼
     Types           Effects       Capabilities
       │               │                │
       └───────────────┼────────────────┘
                       │
                       ▼
                   Resources
                       │
                       ▼
                   Contracts
                       │
                       ▼
                    Policies
                       │
                       ▼
                   Security
                       │
                       ▼
                  Provenance
                       │
                       ▼
              Canonical Semantics
                       │
          ┌────────────┼─────────────┐
          │            │             │
          ▼            ▼             ▼
      Classical    quantum::ir   HDL/Hardware
          │            │             │
          └────────────┼─────────────┘
                       │
                       ▼
              Target-Independent
                  Optimization
                       │
                       ▼
                    Lowering
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
         ABI        Routing     Scheduling
          │            │            │
          └────────────┼────────────┘
                       │
                 Resilience / QEC
                       │
                      ZQN
                       │
                      HAL
                       │
                       ▼
                TARGET REALIZATION
                       │
      ┌────────────────┼─────────────────────┐
      │                │                     │
      ▼                ▼                     ▼
     CPU              GPU                   FPGA
      │                │                     │
      ├────────────────┼─────────────────────┤
      │                │                     │
     ASIC          Accelerator               QPU
      │                │                     │
      └────────────────┼─────────────────────┘
                       │
                Embedded / HPC
                       │
                  Distributed
                       │
                     Cloud
                       │
                       ▼
              Future Substrates

---

134. Final Principle

The permanent principle of "grammar/interoperability/" is:

«Zamani interoperability describes the contract between portable Zamani semantics and an external computational ecosystem. It does not turn that ecosystem into the permanent language core, and it does not encode present implementation limitations into future source programs.»

The programmer describes:

WHAT is required
WHAT is guaranteed
WHAT is permitted
WHAT is prohibited
WHAT is interoperable
WHAT compatibility is required

The compiler, linker, runtime, scheduler, resource manager, HAL, and target layers determine:

HOW it is realized
WHERE it is realized
WHEN it is realized
WHICH ABI is used
WHICH runtime is used
WHICH resources are used
WHICH hardware realizes it

Therefore:

                    ONE ZAMANI PROGRAM
                           │
                           ▼
                  ONE SEMANTIC CONTRACT
                           │
                           ▼
                    DOMAIN-NEUTRAL AST
                           │
                           ▼
                 CANONICAL SEMANTICS
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
         Classical     quantum::ir   HDL/Hardware
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                  TARGET-INDEPENDENT
                     OPTIMIZATION
                           │
                           ▼
                       LOWERING
                           │
                 ┌─────────┼─────────┐
                 ▼         ▼         ▼
                ABI      Routing  Scheduling
                 │         │         │
                 └─────────┼─────────┘
                           ▼
                    RESILIENCE / QEC
                           │
                          ZQN
                           │
                          HAL
                           │
                           ▼
                   TARGET REALIZATION
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
            CPU           GPU            QPU
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                   DISTRIBUTED / HPC
                           │
                         CLOUD
                           │
                           ▼
                   FUTURE HARDWARE

"grammar/interoperability/README.md" is therefore the orchestrator of the entire interoperability directory. It defines ownership, prevents duplicate semantic systems, establishes the integration contract for every existing interoperability family, identifies migrations required by the current repository, and provides the production-readiness gate through which every future interoperability feature must pass.
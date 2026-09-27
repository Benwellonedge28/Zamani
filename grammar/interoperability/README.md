Zamani Interoperability Grammar

Path: "grammar/interoperability/"
Primary file: "grammar/interoperability/README.md"
Status: PRODUCTION-READY ARCHITECTURAL CONTRACT
Language: Zamani
Grammar technology: ANTLR grammar composition
Compiler implementation baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Implementation safety: Safe Rust only; "unsafe" Rust is prohibited
Architectural objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

The "grammar/interoperability/" subsystem defines the source-level interoperability boundary of the Zamani programming language.

It provides the syntax and integration contracts required for Zamani programs to interoperate with:

- foreign programming languages;
- foreign functions;
- foreign types;
- foreign values;
- foreign modules;
- foreign interfaces;
- foreign runtimes;
- ABIs;
- calling conventions;
- system interfaces;
- C;
- C++;
- Rust;
- Python;
- Zig;
- assembly;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- hardware interfaces;
- serialization formats;
- future languages;
- future execution environments;
- future computational substrates.

The subsystem exists to make interoperability a language capability, rather than making every external language, compiler, runtime, operating system, processor, device, vendor, or hardware generation part of Zamani's permanent language definition.

The fundamental rule is:

«Zamani syntax describes the semantic contract of an interoperability boundary. Downstream semantic analysis, ABI lowering, linking, compilation, deployment, runtime, hardware abstraction, and execution determine its concrete realization.»

The grammar therefore describes what the program requires and what boundary it declares, not the implementation mechanism used by a particular machine.

---

2. Architectural Authority

Interoperability is governed by the existing Zamani authority model.

The authority order is:

grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/Zamani.g4
        │
        ├── grammar/antlr/ZamaniParser.g4
        │
        └── grammar/antlr/ZamaniLexer.g4
        │
        ▼
canonical lexical/parser implementation
        │
        ▼
src/frontend/ast/
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├── classical semantics / IR
        ├── quantum::ir
        └── HDL / hardware semantics / IR
        │
        ▼
optimization / lowering
        │
        ▼
routing / scheduling / resilience / QEC / ZQN
        │
        ▼
HAL / ABI / backend
        │
        ▼
runtime / deployment / target

The responsibilities of the major existing files remain:

File| Authority
"grammar/DESIGN.md"| Normative grammar architecture
"grammar/README.md"| Grammar navigation and authority map
"grammar/Zamani.g4"| Canonical ANTLR composition root
"grammar/antlr/ZamaniLexer.g4"| Canonical lexer composition
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/extended design reference
"grammar/specification/"| Normative language specification
"grammar/spec/"| Formal subsystem contracts
"grammar/interoperability/"| Interoperability syntax contracts
"src/lexer.rs"| Rust lexer implementation
"src/parser.rs"| Rust parser implementation
"src/frontend/ast/"| Domain-neutral frontend AST
"quantum::ir"| Canonical quantum semantic/IR boundary

"grammar/interoperability/README.md" does not supersede "DESIGN.md", the normative specification, or the canonical parser.

It specializes those contracts for interoperability.

---

3. Ownership

3.1 This subsystem owns

"grammar/interoperability/" owns source-level syntax and contracts for:

- foreign-language declarations;
- foreign-format declarations;
- FFI declarations;
- foreign function bindings;
- foreign type references;
- foreign value references;
- foreign interface declarations;
- adapters;
- conversion declarations;
- callback declarations;
- ABI references;
- calling-convention references;
- linkage intent;
- symbol identity;
- ownership-transfer intent;
- lifetime-boundary intent;
- nullability intent;
- representation intent;
- marshaling intent;
- serialization/deserialization intent;
- compatibility requirements;
- interoperability capabilities;
- interoperability effects;
- interoperability resource requirements;
- foreign runtime requirements;
- source/target format identity;
- language identity;
- interface identity;
- symbol identity;
- foreign module identity;
- asynchronous foreign boundaries;
- streaming boundaries;
- callback contracts;
- system-interface contracts;
- language-specific interoperability extensions;
- external-format import/export intent;
- interoperability provenance;
- interoperability version constraints;
- loss/compatibility declarations;
- boundary-level security requirements.

---

3.2 This subsystem does not own

It does not own:

- the canonical lexer;
- universal identifiers;
- universal qualified names;
- ordinary expressions;
- ordinary types;
- ordinary functions;
- ordinary modules;
- universal declarations;
- ordinary statements;
- semantic type checking;
- ownership checking;
- lifetime checking;
- ABI layout computation;
- ABI implementation;
- symbol lookup;
- linker behavior;
- library discovery;
- filesystem access;
- network access;
- process execution;
- dynamic loading;
- runtime invocation;
- hardware discovery;
- device selection;
- CPU selection;
- GPU selection;
- FPGA selection;
- ASIC selection;
- QPU selection;
- physical-qubit selection;
- topology discovery;
- calibration;
- scheduling;
- routing;
- optimization;
- QEC;
- ZQN;
- HAL implementation;
- canonical quantum IR;
- classical IR;
- HDL/hardware IR;
- deployment;
- resource allocation.

Those systems consume interoperability semantics.

They do not become grammar dependencies.

---

4. The Interoperability Boundary

The complete conceptual model is:

Zamani source
     │
     ▼
interoperability declaration
     │
     ├── language identity
     ├── format identity
     ├── interface identity
     ├── symbol identity
     ├── ABI contract
     ├── calling convention
     ├── type contract
     ├── conversion contract
     ├── ownership contract
     ├── lifetime contract
     ├── effect contract
     ├── capability contract
     ├── resource requirements
     ├── security requirements
     ├── compatibility requirements
     └── portability requirements
     │
     ▼
domain-neutral AST
     │
     ▼
semantic analysis
     │
     ├── type checking
     ├── ownership/lifetime checking
     ├── capability checking
     ├── effect checking
     ├── resource checking
     ├── compatibility checking
     └── portability checking
     │
     ▼
canonical semantic model
     │
     ▼
canonical IR/domain representation
     │
     ▼
target-specific realization

The boundary is deliberately one-directional.

The grammar never reaches downward into target implementation.

---

5. POCO-REAF

Interoperability is part of the POCO-REAF architecture:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

Interoperability must therefore not unnecessarily embed properties of today's target.

For example:

interop language "C"

means:

«The declared boundary follows the C interoperability contract.»

It does not mean:

x86
x86_64
ARM
AArch64
RISC-V
Linux
Windows
macOS
64-bit pointers
specific registers
specific stack layout
specific CPU

Those properties belong to downstream target realization.

Similarly:

interop language "OpenQASM"

does not imply:

- a particular QPU;
- a particular qubit count;
- a particular coupling graph;
- physical-qubit identifiers;
- calibration data;
- gate durations;
- routing decisions;
- error-correction strategy.

---

6. Four Fundamental Interoperability Layers

Interoperability must distinguish four independent layers.

6.1 Language

Examples:

C
C++
Rust
Python
Zig

A language identifies the semantic/source ecosystem.

---

6.2 Format

Examples:

OpenQASM 3
QIR
WebAssembly
Verilog
SystemVerilog
C source
C header

A format describes representation or interchange.

A format is not necessarily a programming language.

---

6.3 ABI

An ABI describes binary-level compatibility expectations.

Examples may include symbolic ABI identities.

ABI semantics include concepts such as:

- calling sequence;
- data layout;
- alignment;
- symbol conventions;
- linkage;
- parameter passing;
- return conventions;
- variadic behavior.

The grammar only declares the ABI contract.

It does not implement ABI lowering.

---

6.4 Runtime

A runtime is the execution environment.

It may include:

- a foreign interpreter;
- a dynamic runtime;
- a managed environment;
- a system runtime;
- a device runtime;
- a distributed runtime.

Runtime identity must not be confused with language identity.

---

7. No Closed Foreign-Language Enumeration

The interoperability architecture must not require a permanently closed enum such as:

C | Cpp | Python | Rust | Zig | ...

That would make adding a new language a grammar architecture change.

Instead, foreign language identity is conceptually symbolic and extensible.

Examples:

C
C++
Rust
Python
Zig
vendor::language
organization::language
future::language

The canonical grammar determines the exact identifier syntax.

The semantic model determines validity.

New foreign languages must not require redesigning the universal interoperability model.

---

8. Source-Format Identity

Language identity and source-format identity are separate.

For example:

language "C"
format "header"

or an equivalent syntax defined by the canonical grammar.

Likewise:

language "OpenQASM"
format "openqasm3"

A source-format identifier is semantic metadata.

It is not automatically:

- a filename;
- a filesystem path;
- a URL;
- an executable;
- a process;
- a network resource.

Parsing must remain inert.

---

9. Canonical Grammar Composition

"grammar/Zamani.g4" remains the single ANTLR composition root.

The interoperability directory must not create another root grammar.

The intended composition is:

grammar/Zamani.g4
        │
        ├── ZamaniParser.g4
        │       │
        │       ├── universal syntax
        │       ├── types
        │       ├── expressions
        │       ├── declarations
        │       ├── statements
        │       ├── functions
        │       ├── modules
        │       ├── effects
        │       ├── memory
        │       ├── concurrency
        │       ├── classical
        │       ├── quantum
        │       ├── hybrid
        │       ├── HDL
        │       ├── hardware
        │       ├── resources
        │       ├── distributed
        │       ├── AI
        │       ├── data
        │       ├── networking
        │       ├── security
        │       ├── compilation
        │       ├── execution
        │       ├── interoperability
        │       ├── dialects
        │       ├── macros
        │       └── metaprogramming
        │
        └── ZamaniLexer.g4

The root must not directly import every interoperability subgrammar.

The interoperability composition grammar owns that internal composition.

---

10. Existing Interoperability Files

The repository already contains specialized interoperability material.

Existing files include:

grammar/interoperability/
├── AssemblyLanguage.g4
├── SystemInterfaces.g4
├── abi.g4
├── c.g4
├── cpp.g4
├── ffi.g4
├── foreign-functions.g4
├── interoperability.g4
├── openqasm.g4
├── python.g4
├── rust.g4
├── system-interfaces.g4
├── verilog.g4
└── zig.g4

Existing files must be expanded and reconciled rather than unnecessarily renamed.

A duplicate or accidentally malformed filename must not become a second authority.

In particular, if the repository contains both:

interoperability.g4

and a filesystem variant whose name differs only by trailing whitespace or another accidental naming artifact, the artifact must be explicitly reconciled and removed/renamed through repository history rather than silently maintained as another grammar.

The canonical logical file remains:

grammar/interoperability/interoperability.g4

---

11. "interoperability.g4"

Owns

The composition-level interoperability syntax.

It should compose:

- FFI;
- foreign functions;
- foreign types;
- ABI;
- calling conventions;
- language identity;
- external formats;
- adapters;
- conversions;
- system interfaces;
- language-specific contracts.

Does not own

It must not redefine:

- identifiers;
- qualified names;
- universal expressions;
- universal types;
- universal parameters;
- universal attributes;
- ordinary functions.

It should delegate those to their canonical owners.

Integration

ZamaniParser.g4
       ↓
interoperability.g4
       ↓
specialized interoperability delegates
       ↓
domain-neutral AST

---

12. "ffi.g4"

"ffi.g4" owns the generic source-level FFI boundary.

The current file already establishes the correct architectural direction: FFI declares a boundary without loading or executing foreign code.

Its complete responsibility is:

FFI declaration
    ↓
foreign identity
    ↓
callable contract
    ↓
type boundary
    ↓
ownership/lifetime/effect/capability/resource metadata

It must support, where specified by the canonical grammar:

- FFI interfaces;
- foreign functions;
- callbacks;
- adapters;
- policies;
- foreign targets;
- contracts;
- marshaling;
- ownership;
- borrowing;
- lifetime;
- nullability;
- representation;
- encoding;
- size expressions;
- alignment expressions;
- direction;
- asynchronous behavior;
- streaming;
- effects;
- requirements;
- compatibility;
- security;
- resources;
- determinism;
- version constraints.

Independent completion contract

"ffi.g4" is complete when:

- every FFI construct has a defined syntactic owner;
- common grammar rules are delegated;
- foreign identity is symbolic;
- ABI is delegated to ABI contracts;
- target implementation is excluded;
- AST mapping is defined;
- semantic mapping is defined;
- IR mapping is defined;
- source spans are preserved;
- diagnostics are defined;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- no hard-coded machine limits exist.

---

13. "foreign-functions.g4"

This file owns declarations for externally implemented callable contracts.

A foreign function declaration means:

«This callable exists outside the Zamani implementation unit and is described by this contract.»

It does not mean:

- load it;
- execute it;
- invoke a compiler;
- invoke a linker;
- search the filesystem;
- inspect hardware;
- inspect the network;
- dynamically load a library.

Example conceptual form:

extern "C" fn external_function(value: SomeType) -> ResultType;

The declaration belongs in the frontend semantic model.

Resolution occurs downstream.

---

14. "foreign-types.g4"

Foreign types must support semantic contracts for:

- named foreign types;
- opaque types;
- handles;
- references;
- ownership;
- borrowing;
- nullability;
- lifetime;
- representation;
- ABI compatibility;
- conversion.

The grammar must never assume:

pointer = 64 bits

or:

word = 32 bits

or:

register = fixed width

Foreign layout is an ABI/backend concern.

An opaque foreign type remains opaque unless semantic information explicitly provides a valid layout contract.

---

15. ABI Integration

"abi.g4" owns ABI syntax.

It may express a symbolic ABI identity.

Examples conceptually include:

abi "C"
abi "system"
abi vendor::abi

The exact syntax remains governed by the canonical grammar.

ABI grammar does not implement:

- alignment calculation;
- structure layout;
- stack layout;
- register allocation;
- calling sequence;
- machine instruction encoding;
- binary relocation;
- linker behavior.

The integration path is:

ABI syntax
   ↓
domain-neutral AST
   ↓
ABI semantic contract
   ↓
target ABI realization

---

16. Calling Conventions

"calling-conventions.g4" owns symbolic calling-convention intent.

A calling convention is not a processor.

A calling convention is not a register set.

A calling convention is not a machine instruction sequence.

The grammar may represent symbolic conventions such as:

c
system
default
vendor::convention

The final set of standard spellings is determined by the authoritative specification.

The grammar must not encode:

- argument registers;
- return registers;
- stack slots;
- stack alignment;
- instruction sequences;
- physical register names.

Those are target-lowering concerns.

---

17. Foreign Symbol Identity

A foreign symbol must be represented as semantic identity.

A symbol may conceptually contain:

language
namespace
module
interface
symbol
version
ABI

It must not inherently contain:

physical address
memory address
device address
register address
process ID
runtime pointer

Those are implementation artifacts.

---

18. Linkage

Interoperability may express linkage intent such as:

- external;
- internal;
- weak;
- imported;
- exported;
- symbolic;
- platform-qualified;
- versioned.

The grammar does not perform linking.

The linker/backend determines whether a declaration can actually be realized.

A source declaration must remain valid independently of the specific linker implementation.

---

19. Ownership

Interoperability crosses memory-management boundaries.

The grammar therefore needs semantic vocabulary for:

borrowed
owned
shared
transferred
retained
returned
released

and equivalent contract forms defined by the language specification.

Ownership is a semantic property.

The grammar only represents the declaration.

Ownership validation belongs to semantic analysis.

Actual allocation/deallocation belongs to the appropriate runtime/backend.

---

20. Lifetime

Foreign boundaries may require lifetime contracts.

Examples include:

- argument lifetime;
- return lifetime;
- callback lifetime;
- retained object lifetime;
- borrowed lifetime;
- resource lifetime.

The grammar must allow lifetime intent to be represented without turning lifetime into a machine address or runtime pointer.

Lifetime checking belongs downstream.

---

21. Nullability

Foreign types may require:

nullable
nonnull
unknown

or extensible semantic forms.

Nullability is a contract.

It must not automatically imply a particular binary representation.

---

22. Callbacks

Callbacks are callable contracts crossing the interoperability boundary in the opposite direction.

A callback contract must be capable of describing:

- callback identity;
- parameters;
- result;
- ABI;
- calling convention;
- effects;
- capabilities;
- ownership;
- lifetime;
- nullability;
- asynchronous behavior;
- reentrancy requirements;
- compatibility.

A callback must not be modeled as an assumed machine pointer width.

---

23. Variadic Functions

Variadic foreign interfaces must not have arbitrary language-level argument limits.

The grammar must represent the concept of variadicity.

It must not encode:

maximum 8 arguments
maximum 16 arguments
maximum 32 arguments

Foreign ABI semantics may require:

- default argument promotion;
- sentinel conventions;
- format contracts;
- validation;
- ABI-specific lowering.

Those are downstream semantic/backend responsibilities.

---

24. Adapters

Adapters describe semantic transformations between interoperability contracts.

Conceptually:

adapter Name from InterfaceA to InterfaceB

An adapter declaration does not specify whether its implementation uses:

- generated code;
- wrapper functions;
- marshaling;
- serialization;
- runtime dispatch;
- compiler lowering;
- hardware bridges;
- quantum/classical bridges.

The adapter is a semantic boundary.

Its implementation belongs downstream.

---

25. Conversions

Interoperability must distinguish:

- identity conversion;
- representation conversion;
- numeric conversion;
- ownership conversion;
- borrowing conversion;
- ABI conversion;
- language conversion;
- serialization;
- deserialization;
- quantum/classical conversion;
- hardware/software conversion.

Two types with similar names must never automatically become interchangeable.

Conversion validity belongs to semantic analysis.

---

26. C Integration — "c.g4"

"c.g4" owns C-specific interoperability syntax.

It may represent:

- C-compatible declarations;
- C functions;
- C types;
- C callbacks;
- C linkage;
- C ABI metadata;
- C attributes;
- nullability;
- ownership contracts;
- compatibility metadata.

It does not own:

- the complete C language;
- C preprocessing;
- C compiler behavior;
- target ABI implementation;
- linker behavior;
- C runtime behavior.

C interoperability is a boundary, not a second C compiler embedded inside Zamani.

---

27. C++ Integration — "cpp.g4"

"cpp.g4" owns C++-specific interoperability contracts.

It must support the semantic requirements needed for:

- C++ symbols;
- C++ types;
- C++ callable interfaces;
- namespaces;
- linkage;
- ABI identity;
- object/lifetime boundaries;
- compatibility.

C++ ABI details remain downstream.

The grammar must not assume one C++ ABI implementation.

---

28. Python Integration — "python.g4"

"python.g4" represents Python interoperability.

It must distinguish:

Python language

from:

Python runtime

and from:

Python installation

The grammar must not hard-code:

- executable paths;
- installation paths;
- one operating system;
- one interpreter implementation;
- one machine;
- one deployment directory.

A version may be expressed as a compatibility requirement where the language specification permits it.

---

29. Rust Integration — "rust.g4"

"rust.g4" represents Rust as a foreign interoperability target.

This must remain distinct from the fact that the Zamani compiler itself is implemented in:

Rust 2021
Rust 1.97 / 1.97.1
safe Rust

The implementation language of the compiler does not restrict the source language's interoperability universe.

The Rust interoperability grammar must therefore remain an external-language contract.

---

30. Zig Integration — "zig.g4"

"zig.g4" represents Zig interoperability.

Zig is treated as:

foreign language

not:

hardware target

Compiler version, platform, ABI, linker, runtime, and deployment remain downstream.

---

31. Assembly Integration — "AssemblyLanguage.g4"

Assembly interoperability is necessarily target-sensitive.

However, the interoperability architecture must distinguish:

assembly language family

from:

instruction-set architecture

and:

processor implementation

and:

specific processor model

The universal Zamani grammar must not accidentally turn one assembly dialect into the definition of all machine execution.

Assembly declarations may explicitly be marked target-specific where source semantics genuinely require that specificity.

That specificity must remain explicit rather than leaking into the universal language.

---

32. System Interfaces

Existing:

SystemInterfaces.g4
system-interfaces.g4

must be reconciled so that only one logical contract owns system-interface syntax.

System interfaces may describe:

- operating-system calls;
- device interfaces;
- service interfaces;
- platform APIs;
- system resources;
- process interfaces.

They must not silently select an operating system or machine.

A system-interface requirement is a compatibility/capability contract.

---

33. WebAssembly Integration

WebAssembly interoperability must distinguish:

WebAssembly format

from:

WebAssembly runtime

and:

host environment

The grammar may describe:

- imports;
- exports;
- modules;
- function signatures;
- compatible value contracts;
- ABI metadata;
- serialization;
- execution requirements.

It must not assume:

- a specific WASM runtime;
- one host;
- one CPU;
- one memory capacity;
- one deployment topology.

---

34. OpenQASM Integration

"openqasm.g4" owns OpenQASM interoperability syntax.

OpenQASM is an external representation.

It is not the canonical Zamani quantum semantic model.

The integration path is:

OpenQASM source
       ↓
OpenQASM interoperability parser
       ↓
domain-neutral Zamani representation
       ↓
semantic quantum analysis
       ↓
quantum::ir
       ↓
optimization
       ↓
routing
       ↓
scheduling
       ↓
QEC / resilience
       ↓
ZQN
       ↓
HAL
       ↓
target

The grammar must not create:

OpenQASM IR

as a competing canonical quantum IR.

---

35. QIR Integration

QIR is an interoperability representation.

QIR must not replace:

quantum::ir

as Zamani's canonical quantum semantic boundary.

The relationship is:

Zamani quantum semantics
        ↕
quantum::ir
        ↕
QIR interoperability adapter

or, where appropriate:

QIR input
   ↓
validated import
   ↓
canonical quantum semantics
   ↓
quantum::ir

The exact direction is determined by the compilation operation.

QIR-specific implementation details must not leak into the universal source grammar.

---

36. Quantum Interoperability Invariants

Interoperability involving quantum computing must never hard-code:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_QPU_COUNT
MAX_GATE_COUNT
MAX_CIRCUIT_DEPTH
MAX_TOPOLOGY_SIZE

The language may express program requirements:

requires qubits >= n

or capabilities:

requires capability("quantum.measurement")

or topology requirements:

requires topology(...)

but the grammar does not establish the available physical resource ceiling.

Physical realization remains downstream.

---

37. HDL Integration

"verilog.g4" and any SystemVerilog-related interoperability material must remain external HDL-format contracts.

The interoperability subsystem may describe:

- HDL source;
- HDL modules;
- HDL interfaces;
- HDL data representation;
- synthesis compatibility;
- simulation compatibility;
- verification interfaces.

It must not replace:

grammar/hdl/

which owns Zamani's native HDL syntax.

The distinction is:

Zamani HDL

versus:

foreign HDL format

---

38. Hardware Interoperability

Foreign hardware interfaces may be described through:

- symbolic interfaces;
- capabilities;
- resources;
- protocols;
- timing requirements;
- communication requirements;
- accelerator interfaces.

They must not encode universal physical assumptions.

Forbidden universal assumptions include:

wire [31:0]
RAM = 64GB
VRAM = 24GB
register = 32-bit
device 0
GPU 0
QPU 0

when used as universal language limits or assumptions.

A concrete value may be valid program data or a deliberately target-specific contract.

It must not silently become a universal language restriction.

---

39. Serialization

"serialization.g4" owns source-level serialization interoperability contracts.

Serialization must distinguish:

semantic value

from:

serialized representation

It may represent:

- encoding identity;
- schema identity;
- version;
- compatibility;
- endian/order requirements where semantically necessary;
- nullability;
- optional fields;
- extensibility;
- loss policy;
- provenance.

The grammar does not perform serialization.

The serializer/deserializer implementation does.

---

40. Lossless Interoperability

Every import/export path must classify semantic fidelity.

At minimum:

lossless
conditionally_lossless
lossy
unsupported

An interoperability adapter must not silently discard semantics.

For example, if a foreign format cannot represent:

- effects;
- ownership;
- resource requirements;
- quantum semantics;
- timing;
- precision;
- security properties;
- annotations;

the adapter must report the mismatch according to the diagnostics/compatibility contract.

Silent semantic loss is not production-ready interoperability.

---

41. Provenance

Interoperability transformations must preserve provenance where the compiler architecture supports it.

A transformed construct should be traceable to:

source format
source location
source version
foreign language
foreign symbol
adapter
conversion
target representation

This integrates with the existing source-span and diagnostic architecture.

Provenance belongs to semantic/compiler infrastructure.

The grammar merely provides the syntax necessary to identify the boundary.

---

42. Versioning

Foreign interfaces must support version constraints without forcing a specific version into the universal language.

Examples conceptually include:

language "C"
version ...

or:

requires compatibility(...)

The exact syntax is governed by the canonical compatibility specification.

Version semantics belong to compatibility analysis.

The grammar must not assume that version numbers are always three-component semantic versions.

Foreign ecosystems may use:

- numeric versions;
- named revisions;
- editions;
- profiles;
- standards revisions;
- vendor revisions.

The representation must therefore remain extensible.

---

43. Compatibility

Interoperability compatibility must distinguish:

language compatibility
format compatibility
ABI compatibility
type compatibility
semantic compatibility
runtime compatibility
platform compatibility
capability compatibility
security compatibility
version compatibility

These are not interchangeable.

A declaration may be syntactically valid while being semantically incompatible with the selected target.

That is a semantic/compiler diagnostic, not a parser failure.

---

44. Capabilities

Interoperability may require capabilities.

Examples:

requires capability("ffi")
requires capability("network")
requires capability("quantum")
requires capability("hardware")

The grammar represents the requirement.

The capability system determines whether the environment satisfies it.

The grammar must not contain a closed universal hardware inventory.

New capabilities must be representable symbolically.

---

45. Resources

Interoperability may depend on resources.

Examples include:

- memory;
- storage;
- communication;
- accelerator access;
- quantum resources;
- external services;
- execution capacity.

The grammar must distinguish:

requirement
constraint
preference
hint
capability
resource

For example:

requires capability("gpu.compute")

is different from:

prefers accelerator(...)

and different from:

requires memory >= required_memory

and different from a downstream physical placement decision.

---

46. Effects

Foreign boundaries can introduce effects such as:

- I/O;
- network access;
- external state;
- system calls;
- hardware access;
- process interaction;
- asynchronous execution;
- quantum execution.

Interoperability syntax may reference these effects.

The canonical effect system remains the authority for effect semantics.

Interoperability must not create a second effect system.

---

47. Security Boundary

Foreign interoperability is a security boundary.

Parsing an interoperability declaration must never:

- load arbitrary libraries;
- execute foreign code;
- invoke processes;
- access credentials;
- read arbitrary files;
- access the network;
- inspect hardware;
- dereference an address;
- perform dynamic linking;
- execute an FFI call.

The parser is inert.

Semantic analysis may validate declarations.

Compilation/linking/runtime may realize them only through explicitly authorized mechanisms.

---

48. No Hidden Execution

A syntax such as:

foreign ...

must never mean:

execute now

Likewise:

import foreign ...

must not imply arbitrary runtime execution.

Declarations describe interfaces.

Execution occurs through the ordinary Zamani semantic and execution pipeline.

---

49. No Hidden Hardware Discovery

Interoperability syntax must not cause the grammar or parser to ask:

Which CPU exists?
Which GPU exists?
Which QPU exists?
How many cores exist?
How much RAM exists?
How many qubits exist?
Which FPGA exists?
Which network exists?

Hardware discovery belongs to:

resource analysis
target selection
HAL
runtime
deployment

depending on the particular operation.

---

50. Domain-Neutral AST Requirement

All interoperability constructs must map into the existing domain-neutral frontend AST.

The interoperability grammar must not introduce a parallel semantic AST architecture.

The conceptual mapping is:

grammar/interoperability/*
             ↓
domain-neutral AST node
             ↓
semantic interoperability model
             ↓
canonical domain representation

The AST should represent concepts such as:

- foreign declaration;
- interface;
- callable boundary;
- type boundary;
- adapter;
- conversion;
- ABI contract;
- language identity;
- format identity;
- compatibility contract.

It should not contain target-specific runtime objects.

---

51. AST Integration Contract

Every production interoperability grammar construct must have a predetermined AST mapping before the grammar construct is considered complete.

For every construct define:

grammar rule
AST node
source span
attributes
semantic identity
semantic validation
IR mapping
diagnostics

No construct should be added with:

«AST mapping will be decided later.»

That violates the independent-file completion requirement.

---

52. Semantic Integration Contract

The semantic layer must validate:

- language identity;
- format identity;
- symbol identity;
- type compatibility;
- ABI compatibility;
- calling convention;
- ownership;
- lifetime;
- nullability;
- conversion;
- effects;
- capabilities;
- resources;
- compatibility;
- security;
- portability;
- determinism where required.

The grammar itself does not perform these checks.

---

53. IR Integration

Interoperability is not itself a universal IR.

After semantic analysis, an interoperability boundary is lowered into the canonical representation required by the computation.

Examples:

foreign classical function
        ↓
classical semantic representation / IR

foreign quantum program
        ↓
quantum semantics
        ↓
quantum::ir

foreign HDL
        ↓
HDL/hardware semantic representation

foreign distributed interface
        ↓
distributed semantic representation

There must not be:

interoperability::ir

that becomes a second universal IR competing with canonical domain representations.

---

54. Canonical Quantum IR Invariant

All quantum interoperability paths converge on:

quantum::ir

This includes interoperability with:

- OpenQASM;
- QIR;
- quantum foreign runtimes;
- vendor quantum formats;
- hybrid interfaces.

No interoperability grammar may create a second canonical quantum IR.

---

55. Classical Interoperability

Classical foreign functions/types should lower through canonical classical semantics.

The interoperability subsystem does not need to know whether the eventual realization uses:

- CPU;
- GPU;
- accelerator;
- vector processor;
- distributed processor;
- future computational substrate.

That decision belongs downstream.

---

56. Hybrid Interoperability

A foreign boundary may cross:

classical ↔ quantum
classical ↔ hardware
quantum ↔ hardware
software ↔ accelerator

The interoperability layer expresses the contract.

The hybrid semantic subsystem determines the meaning.

The canonical IR layers determine representation.

Routing/scheduling/HAL determine realization.

---

57. HDL/Software Co-Design

Interoperability must support the possibility that one Zamani program coordinates:

software computation
+
foreign library
+
accelerator
+
HDL module
+
quantum computation
+
distributed execution

without requiring every component to become part of the core grammar.

The source expresses relationships and contracts.

Downstream systems realize those relationships.

---

58. Calling Convention vs ABI

These concepts must remain separate.

calling convention

describes callable interaction rules.

ABI

describes the broader binary compatibility contract.

One ABI may involve multiple callable conventions.

A calling convention is therefore not an ABI alias.

---

59. Language vs Runtime

Likewise:

Python

is not:

CPython executable

and:

Rust

is not:

rustc binary path

and:

WebAssembly

is not:

specific WASM runtime

The grammar must preserve these distinctions.

---

60. Language vs Platform

The language declaration must not automatically select:

Linux
Windows
macOS
Android
bare metal
cloud

A platform requirement is a compatibility/capability concern.

It is not inherently a language identity.

---

61. Symbolic Targets

Foreign targets may be identified symbolically.

They must not be treated as physical resources merely because the syntax contains an identifier.

For example:

foreign::math::sin

identifies a semantic symbol.

It does not mean:

memory_address(...)

or:

device(...)

---

62. Target-Specific Interoperability

Target-specific declarations are allowed when the programmer genuinely requires them.

They must be explicitly classified.

Conceptually:

portable
target-specific
platform-specific
vendor-specific
experimental

Target-specific constructs must never silently become universal semantics.

---

63. Vendor Extensions

Vendor-specific interoperability may use qualified names.

Conceptually:

vendor::extension

or equivalent syntax.

Vendor extensions must identify:

- vendor namespace;
- extension identity;
- version where applicable;
- compatibility requirements;
- semantic mapping;
- diagnostics;
- fallback behavior where supported.

A vendor extension must not modify the meaning of existing universal Zamani syntax.

---

64. Dialect Integration

Interoperability must integrate with:

grammar/dialects/

without becoming a second language.

A dialect must specify:

identity
version
syntax
AST mapping
semantic mapping
IR mapping
compatibility
feature status

Interoperability-specific syntax must not bypass dialect validation.

---

65. Serialization and Data Exchange

Serialization interoperability must integrate with:

grammar/data/

and:

grammar/interoperability/

without duplicating universal data types.

The interoperability layer describes external representation.

The data subsystem owns the language's data semantics.

---

66. Macro Integration

Macros may generate interoperability declarations.

However:

macro expansion
        ↓
normal syntax validation
        ↓
semantic validation

must remain the order.

Macros must not create an unvalidated FFI escape hatch.

---

67. Metaprogramming Integration

Compile-time reflection or code generation may inspect interoperability contracts where explicitly permitted.

It must not:

- execute arbitrary foreign code during parsing;
- bypass semantic checks;
- bypass security checks;
- invent ABI layouts;
- fabricate target capabilities.

---

68. Determinism

Parsing interoperability syntax must be deterministic.

Its result must depend only on:

- source text;
- language version;
- grammar version;
- explicitly selected dialects;
- explicitly supplied configuration that is part of the language contract.

Parsing must not depend on:

- wall-clock time;
- randomness;
- hardware;
- filesystem state;
- network state;
- environment variables;
- runtime state;
- currently installed libraries;
- currently available devices.

---

69. Reproducibility

Interoperability compilation should preserve enough metadata for reproducible builds where the compiler architecture supports reproducibility.

Foreign dependencies may contribute:

- version;
- interface identity;
- ABI;
- source/format identity;
- compatibility constraints;
- provenance.

A build must not silently resolve a different foreign interface merely because the environment happens to contain one.

---

70. Diagnostics

Interoperability diagnostics must distinguish at least:

lexical error
syntax error
unknown foreign language
unknown format
invalid symbol
invalid ABI
incompatible ABI
invalid calling convention
type mismatch
ownership mismatch
lifetime mismatch
nullability mismatch
conversion failure
effect violation
capability violation
resource violation
compatibility violation
unsupported feature
semantic-loss warning
security violation
target-realization failure

A target-realization failure must not be incorrectly reported as a grammar failure.

---

71. Error Recovery

ANTLR/parser error recovery must remain consistent with the global parser architecture.

The interoperability subsystem must not introduce custom recovery semantics that cause the same source construct to parse differently depending on which domain invokes it.

Recovery must preserve:

- source locations;
- expected-token information;
- construct identity;
- deterministic diagnostics.

---

72. Source Spans

Every interoperability AST construct must preserve source location information sufficient for diagnostics and provenance.

At minimum, source spans should identify:

start
end

and where supported:

source file
source unit
foreign declaration
foreign symbol
contract clause

Source spans belong to the frontend AST infrastructure.

---

73. Scalability

The interoperability grammar must scale according to available resources.

There must be no language-level hard-coded maximum for:

foreign languages
foreign interfaces
foreign functions
foreign types
foreign symbols
callbacks
adapters
conversions
attributes
requirements
capabilities
effects
resources
targets
formats
versions
modules
parameters
arguments

ANTLR repetition and ordinary semantic data structures must represent arbitrary program-defined quantities.

Compiler resource exhaustion is an implementation constraint, not a language semantic maximum.

---

74. Hard-Coding Prohibition

The interoperability grammar must never introduce universal constants such as:

MAX_FFI_FUNCTIONS
MAX_FOREIGN_TYPES
MAX_ABIS
MAX_CALLBACKS
MAX_TARGETS
MAX_LANGUAGES
MAX_SYMBOLS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_QUBITS

Similarly prohibited are hidden assumptions such as:

pointer = 64-bit
register = 32-bit
address = 64-bit
machine = little-endian
CPU = x86

unless explicitly represented as a target-specific interoperability contract.

---

75. Program Values Are Not Language Limits

This distinction is mandatory.

This may be valid:

let n = 1024;

and:

requires memory >= required_memory;

and:

requires qubits >= n;

Those are program semantics.

What is prohibited is:

MAX_QUBITS = 1024

as a universal language restriction.

Likewise:

array<1024>

may be program data.

It must not imply:

«Zamani supports no arrays larger than 1024.»

---

76. Resource Availability

POCO-REAF means that realization may scale with available resources.

Conceptually:

same source semantics
       │
       ├── tiny target
       │
       ├── ordinary target
       │
       ├── accelerator target
       │
       ├── heterogeneous target
       │
       ├── distributed target
       │
       ├── quantum target
       │
       └── future target

Interoperability must not force source-level rewriting merely because the available implementation resources differ.

---

77. What Interoperability Must Not Promise

POCO-REAF does not mean that every foreign interface exists on every possible machine.

Instead, it means:

«The source program expresses its semantic contract independently of unnecessary target details, while compilation and deployment determine whether and how that contract can be realized.»

If a target lacks a required capability, the compiler must report that fact through the resource/capability/compatibility architecture.

The source grammar must not pretend that unavailable resources exist.

---

78. Capability Failure vs Syntax Failure

This distinction is critical.

For example:

requires capability("quantum.measurement")

may parse correctly.

If the selected target lacks that capability, the result is a:

capability/target realization failure

not:

syntax error

Similarly, an ABI incompatibility is not a parser error.

---

79. Foreign Library Identity

A library name is not automatically a filesystem path.

A declaration such as:

library "example"

must remain semantic identity.

It must not cause the parser to open:

/path/to/example

Library discovery belongs to compilation/deployment infrastructure.

---

80. Dynamic Loading

Dynamic loading is a downstream concern.

The grammar may describe intent for dynamic linkage where specified.

The grammar must never itself dynamically load a library.

This applies equally to:

- native libraries;
- plugins;
- language runtimes;
- device drivers;
- quantum runtimes;
- accelerator libraries.

---

81. FFI and Safe Rust

The Zamani compiler implementation must remain:

Rust 2021
Rust 1.97 / 1.97.1
safe Rust

No "unsafe" implementation is required or permitted as part of the grammar/interoperability implementation.

This means:

- no "unsafe" grammar actions;
- no embedded Rust actions in ANTLR;
- no raw-pointer-based grammar implementation;
- no unsafe transmutation;
- no unsafe aliasing;
- no manual unsafe memory management.

A foreign ABI may itself describe an unsafe external system, but the Zamani compiler implementation must isolate that fact behind a safe semantic/backend contract.

---

82. FFI Safety Model

The language must not confuse:

foreign interface exists

with:

foreign interface is safe

Safety is determined through:

- type checking;
- ownership;
- lifetime;
- capability analysis;
- effect analysis;
- security policy;
- ABI validation;
- target validation.

The grammar represents the declaration.

Semantic analysis determines whether it is valid.

---

83. Security Capabilities

Sensitive interoperability operations may require explicit capabilities.

Examples conceptually:

capability("ffi")
capability("system.interface")
capability("process.execution")
capability("native.library")
capability("hardware.access")

The exact canonical capability vocabulary belongs to the resource/security/capability specifications.

The interoperability grammar must not silently grant capabilities merely because an FFI declaration exists.

---

84. Foreign Calls and Effects

A foreign function may have effects.

For example, a function may perform:

network I/O
filesystem I/O
device I/O
process interaction
external state mutation

The foreign declaration must integrate with the canonical effect system.

The interoperability grammar does not define a replacement effect language.

---

85. Foreign Types and the Type System

Foreign types integrate with:

grammar/types/

They do not create an independent foreign type system.

The semantic type system must determine:

compatibility
variance where applicable
ownership
conversion
layout
ABI representation
lifetime
nullability

The grammar only expresses the declaration boundary.

---

86. Foreign Functions and Functions

Foreign functions integrate with:

grammar/functions/

They must not duplicate:

- parameter syntax;
- generic syntax;
- ordinary return syntax;
- ordinary function expressions.

Foreign-specific metadata is layered onto the universal function model.

---

87. Foreign Modules and Modules

Foreign modules integrate with:

grammar/modules/

A foreign module must remain a module boundary rather than creating a parallel module system.

Imports/exports must remain governed by the canonical module architecture.

---

88. Interoperability and Effects

Interoperability metadata may reference:

grammar/effects/

but must not redefine effects.

This ensures that a foreign function and a native function participate in one effect model.

---

89. Interoperability and Memory

Foreign memory contracts integrate with:

grammar/memory/

Ownership and borrowing must therefore use the same semantic model as native Zamani memory.

Interoperability cannot create a second ownership system.

---

90. Interoperability and Resources

Resource requirements integrate with:

grammar/resources/

Examples:

memory requirement
network requirement
accelerator capability
quantum capability
external-service requirement

The interoperability subsystem references those concepts rather than redefining their global semantics.

---

91. Interoperability and Hardware

Hardware-specific interoperability integrates with:

grammar/hardware/

The hardware subsystem owns:

- capabilities;
- topology;
- resource descriptions;
- device semantics;
- hardware intent.

Interoperability identifies the external interface.

It does not choose the device.

---

92. Interoperability and Execution

Execution integration belongs to:

grammar/execution/

Interoperability may declare:

- synchronous boundary;
- asynchronous boundary;
- streaming;
- callback;
- blocking behavior;
- execution requirements.

Execution semantics belong to the execution subsystem.

---

93. Interoperability and Compilation

Compilation integration belongs to:

grammar/compile/

The compiler may determine:

- linking;
- ABI lowering;
- foreign code generation;
- import/export resolution;
- target adaptation;
- cross-compilation.

The grammar only provides the source contract.

---

94. Interoperability and Distributed Computing

Foreign interfaces may cross distributed boundaries.

For example:

service
RPC
remote function
stream
message

must integrate with:

grammar/distributed/
grammar/networking/

The interoperability grammar must not create a separate distributed communication model.

---

95. Interoperability and AI

AI systems may expose foreign models/runtimes.

The boundary may describe:

- model interface;
- tensor interface;
- inference interface;
- training interface;
- runtime compatibility.

AI semantics remain under:

grammar/ai/

The interoperability layer does not become a framework-specific AI grammar.

---

96. Interoperability and Data

External data formats must integrate with:

grammar/data/

Serialization contracts must not redefine native collections/tensors/records.

---

97. Interoperability and Networking

Network-based foreign services integrate with:

grammar/networking/

A network endpoint must not automatically be a physical machine identity.

The source may express a service/interface requirement.

Network discovery and routing remain downstream.

---

98. Interoperability and Security

Security requirements integrate with:

grammar/security/

Interoperability may require:

- authentication;
- authorization;
- confidentiality;
- integrity;
- provenance;
- trust;
- secure execution.

The interoperability subsystem does not implement cryptography.

---

99. Interoperability and Compatibility

All external interfaces must participate in:

grammar/compatibility/

Compatibility must be explicit.

No foreign format should become accepted merely because the parser can recognize its syntax.

---

100. Interoperability and Validation

Validation belongs to:

grammar/validation/

The interoperability validation suite must check:

- grammar ambiguity;
- duplicate rules;
- duplicate tokens;
- unreachable rules;
- AST coverage;
- semantic coverage;
- IR coverage;
- hard-coded limits;
- portability;
- deterministic parsing;
- compatibility;
- security boundaries.

---

101. Test Architecture

Interoperability tests must exist under:

grammar/tests/interoperability/

Tests must be divided into:

lexical/
syntax/
positive/
negative/
boundary/
scalability/
determinism/
compatibility/
portability/
diagnostics/
security/
formats/
languages/
abi/
ffi/
callbacks/
serialization/
quantum/
hdl/

---

102. Positive Tests

Positive tests must include at minimum:

- foreign function declaration;
- foreign type declaration;
- opaque type;
- callback;
- ABI declaration;
- calling convention;
- ownership;
- lifetime;
- nullability;
- adapter;
- conversion;
- language identity;
- format identity;
- compatibility;
- capability;
- resource requirement;
- C;
- C++;
- Rust;
- Python;
- Zig;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- serialization;
- system interface.

---

103. Negative Tests

Negative tests must include:

- malformed foreign identity;
- invalid symbol;
- invalid type boundary;
- invalid ABI declaration;
- invalid calling convention;
- invalid ownership combination;
- invalid lifetime;
- invalid conversion;
- incompatible version;
- unsupported format;
- forbidden execution during parsing;
- invalid capability contract;
- malformed serialization contract;
- ambiguous interoperability construct.

---

104. Boundary Tests

Boundary tests must test:

- zero/empty optional clauses where legal;
- very large identifiers;
- deeply nested interfaces;
- large parameter lists;
- many foreign declarations;
- many attributes;
- large conversion contracts;
- large format metadata;
- arbitrary symbolic names.

The test suite must not use a finite test size as a language maximum.

---

105. Scalability Tests

Scalability tests must demonstrate that interoperability can represent increasingly large programs without a grammar-imposed ceiling.

Test dimensions include:

foreign declarations
interfaces
functions
types
callbacks
adapters
attributes
requirements
effects
capabilities
resources
formats
symbols
modules

Any compiler resource ceiling must be reported as an implementation/resource result rather than encoded as a grammar rule.

---

106. Determinism Tests

Given identical:

source
grammar version
language version
dialect configuration

the parser must produce the same structural result.

The parser must not depend on:

network
filesystem
hardware
runtime
randomness
time
foreign library availability

---

107. Compatibility Tests

Compatibility tests must cover:

language version
format version
ABI version
foreign library/interface version
dialect version
calling convention
type representation
serialization schema
quantum format version
HDL format version

Compatibility failures must be explicit and diagnosable.

---

108. Portability Tests

At least one conceptual interoperability program must be checked across different target classes:

tiny classical target
larger classical target
GPU/accelerator target
FPGA target
distributed target
quantum target
hybrid target
future target

The test verifies that target-specific realization happens downstream.

It must not require source rewriting merely because resource availability changes.

---

109. Quantum Interoperability Tests

Required coverage includes:

- OpenQASM import;
- OpenQASM export;
- QIR import;
- QIR export where supported;
- generic quantum operation;
- custom operation;
- parameterized operation;
- measurement;
- mid-circuit measurement;
- classical feed-forward;
- quantum/classical boundary;
- resource requirements;
- capability requirements;
- logical-qubit semantics;
- physical realization metadata;
- no fixed gate enumeration;
- no fixed qubit limit.

All quantum semantic lowering must converge on:

quantum::ir

---

110. HDL Interoperability Tests

Required coverage includes:

- Verilog;
- SystemVerilog where supported;
- modules;
- ports;
- signals;
- parameters;
- memories;
- timing;
- interfaces;
- assertions;
- synthesis metadata;
- simulation metadata;
- verification metadata.

No universal fixed hardware width may be established by these tests.

---

111. ABI Tests

ABI tests must distinguish:

ABI identity
calling convention
type representation
layout
linkage
symbol naming

and verify that the grammar does not attempt to calculate target-specific layout.

---

112. FFI Tests

FFI tests must verify:

declaration
binding
callback
ownership
lifetime
nullability
marshaling
conversion
effects
capabilities
resources
compatibility
security

---

113. Serialization Tests

Serialization tests must verify:

- schema identity;
- version;
- encoding;
- optional data;
- extensibility;
- compatibility;
- loss classification;
- provenance.

Silent semantic loss is prohibited.

---

114. Diagnostics Tests

Every interoperability diagnostic category must have at least one test.

Diagnostics must identify the smallest meaningful source span.

For example:

foreign function
    ↓
parameter
    ↓
foreign type

should identify the actual incompatible construct rather than merely reporting:

interoperability error

---

115. Feature Status

Every interoperability feature must have an explicit status:

STABLE
EXPERIMENTAL
PROPOSED
DEPRECATED
HISTORICAL
NOT_IMPLEMENTED

A grammar rule existing in a file does not automatically make the feature stable.

---

116. Promotion Process

A feature progresses through:

Zamani-Grammar.md
       ↓
proposal
       ↓
semantic design
       ↓
AST contract
       ↓
canonical grammar
       ↓
semantic implementation
       ↓
IR contract
       ↓
compiler/runtime integration
       ↓
tests
       ↓
STABLE

A feature must not skip this chain.

---

117. "Zamani-Grammar.md" Relationship

"Zamani-Grammar.md" is retained.

It remains an extended/historical design reference.

It may contain concepts that are:

- proposed;
- experimental;
- historical;
- not implemented.

Those concepts do not automatically become legal interoperability syntax.

Only features promoted through the authoritative process become stable.

---

118. "grammar.md" Relationship

"grammar.md" remains the implementation-conformance reference.

It must distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

Interoperability syntax must be reflected there according to actual compiler implementation status.

The existence of an ANTLR production alone does not justify marking it "IMPLEMENTED".

---

119. "DESIGN.md" Relationship

"DESIGN.md" remains the higher-level architecture authority.

This README specializes that architecture for interoperability.

If an interoperability design conflicts with "DESIGN.md", the design conflict must be resolved in the normative architecture rather than silently creating a special interoperability exception.

---

120. "grammar/spec/interoperability.md" Relationship

"grammar/spec/interoperability.md" is the formal interoperability specification.

This README provides:

- repository ownership;
- file responsibilities;
- integration contracts;
- implementation architecture;
- completion criteria.

"spec/interoperability.md" provides the formal normative language-level contract.

The two must not become competing semantic authorities.

---

121. Specialized File Completion Contract

Every specialized interoperability grammar file is considered complete only when all of the following are defined in advance:

Purpose
Status
Owns
Does Not Own
Dependencies
Upstream Contracts
Downstream Consumers
Tokens
Grammar Rules
AST Mapping
Semantic Mapping
IR Mapping
Compiler Integration
Runtime Integration
Security Boundary
Diagnostics
Source Spans
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility Tests
Portability Tests
Hard-Coding Audit
Completion Criteria

This is the required independent-file contract.

---

122. No Re-Editing Cascade

A completed interoperability file must not depend on undocumented future changes in another file.

Before declaring a file complete:

1. All imported grammar symbols must already have a defined owner.
2. All AST nodes must already have a named semantic purpose.
3. All semantic consumers must be identified.
4. All IR consumers must be identified.
5. All diagnostics must have a defined owner.
6. All tests must have a defined location.
7. All compatibility behavior must be defined.
8. All target-specific behavior must have a downstream owner.
9. All hard-coding risks must be audited.

If another file later changes implementation details, the completed interoperability contract should remain valid unless the normative language specification itself changes.

---

123. Cross-Repository Integration Matrix

Interoperability concern| Grammar owner| Semantic owner| Downstream owner
Identifier| "core/"| name resolution| compiler
Type| "types/"| type system| IR/backend
Function| "functions/"| function semantics| compiler
Module| "modules/"| module resolver| compiler
Effect| "effects/"| effect system| runtime
Memory| "memory/"| ownership/memory analysis| backend/runtime
Capability| "resources/" / "security/"| capability analysis| HAL/runtime
Resource| "resources/"| resource analysis| scheduler/runtime
ABI| "interoperability/abi.g4"| ABI analysis| backend/linker
Calling convention| interoperability| ABI analysis| backend
FFI| "ffi.g4"| semantic FFI model| compiler/runtime
Foreign type| "foreign-types.g4"| type system| ABI/backend
Foreign function| "foreign-functions.g4"| function/FFI semantics| linker/runtime
C| "c.g4"| FFI semantics| C backend
C++| "cpp.g4"| FFI semantics| C++ backend
Rust| "rust.g4"| FFI semantics| Rust interoperability
Python| "python.g4"| runtime/FFI semantics| Python adapter
Zig| "zig.g4"| FFI semantics| Zig adapter
WASM| WASM interoperability grammar| format semantics| WASM backend
OpenQASM| "openqasm.g4"| quantum semantics| "quantum::ir"
QIR| QIR interoperability grammar| quantum semantics| "quantum::ir" / QIR adapter
HDL| "verilog.g4" / related| HDL semantics| HDL backend
Serialization| "serialization.g4"| data/serialization semantics| serializer
System API| system-interface grammar| capability/effect analysis| runtime/backend
Security| interoperability + "security/"| security analysis| deployment/runtime
Compatibility| "compatibility/"| compatibility analysis| compiler/deployment

---

124. Dependency Direction

The dependency direction is:

                    UNIVERSAL ZAMANI
                          │
                          ▼
                 interoperability syntax
                          │
                          ▼
                  domain-neutral AST
                          │
                          ▼
                  semantic analysis
                          │
            ┌─────────────┼─────────────┐
            ▼             ▼             ▼
          types         effects      resources
            │             │             │
            └─────────────┼─────────────┘
                          ▼
                canonical semantics
                          │
             ┌────────────┼────────────┐
             ▼            ▼            ▼
         classical    quantum::ir   HDL/hardware
             │            │            │
             └────────────┼────────────┘
                          ▼
                    optimization
                          ▼
                     lowering
                          ▼
                 ABI / backend / HAL
                          ▼
                  runtime / target

The reverse direction is prohibited.

For example:

grammar → hardware discovery

is invalid.

So is:

grammar → runtime execution

or:

grammar → linker invocation

---

125. No Backend Leakage

The interoperability grammar must not contain target backend details merely because they are convenient.

Forbidden examples include universal syntax that assumes:

x86 register
ARM register
CUDA block
GPU 0
FPGA bank 0
QPU 0
physical qubit 7
memory address 0x...

unless explicitly classified as target-specific source semantics and handled by an appropriate target-specific contract.

---

126. No Vendor Lock-In

The generic FFI architecture must remain vendor-neutral.

Vendor APIs may be represented through:

vendor::namespace

but the universal interoperability model must not be redesigned around one vendor.

---

127. Future-Language Extensibility

A future language must be integrable through:

language identity
format identity
ABI contract
calling convention
type contract
function contract
ownership
lifetime
effects
capabilities
resources
compatibility
security
AST mapping
semantic mapping
IR mapping
tests

without modifying universal language fundamentals unnecessarily.

---

128. Future-Format Extensibility

A future format must be able to define:

format identity
version
parser/importer
exporter
semantic mapping
loss model
compatibility
provenance
tests

without becoming the canonical Zamani semantic representation.

---

129. Future-Target Extensibility

A new target must be able to consume an already-validated interoperability semantic contract.

The grammar should not need to know in advance:

- how many CPUs exist;
- how many GPUs exist;
- how many QPUs exist;
- how much memory exists;
- how many nodes exist;
- what future accelerator architectures look like.

This is fundamental to POCO-REAF.

---

130. Interoperability Does Not Guarantee Universal Availability

The language can express an external dependency.

The compiler must still verify that the target environment can realize it.

Therefore:

portable source contract

does not mean:

every machine implements every foreign ecosystem

Instead:

portable semantic source
+
explicit requirements
+
target capability analysis
=
correct realization or explicit failure

---

131. Runtime Adaptation

Runtime systems may adapt interoperability realization according to:

- available resources;
- device health;
- load;
- network state;
- service availability;
- accelerator availability;
- hardware capability;
- resilience state.

Such adaptation must preserve source-level semantics.

Runtime adaptation must not silently change the meaning of the Zamani program.

---

132. Resilience

Interoperability may participate in the existing resilience model.

Relevant states include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These states belong to resilience/runtime semantics, not to the parser.

Interoperability may expose the contract necessary for those systems to operate.

---

133. QEC and ZQN

Interoperability must not implement QEC or ZQN.

For quantum foreign interfaces:

foreign quantum format
        ↓
quantum semantic validation
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
QEC / resilience
        ↓
ZQN
        ↓
HAL

The interoperability grammar stops before these implementation layers.

---

134. Routing

Interoperability must not decide physical routing.

For example, it must not turn:

foreign quantum operation

into:

physical qubit 17
physical qubit 23

during parsing.

Routing belongs downstream.

---

135. Scheduling

Interoperability must not hard-code:

gate duration
clock cycle
device latency

unless those values are explicitly part of source-level semantic intent.

Target-specific scheduling remains downstream.

---

136. HAL

The Hardware Abstraction Layer determines actual target capabilities/state.

Interoperability does not replace HAL.

The relationship is:

source interoperability contract
        ↓
semantic requirements
        ↓
HAL capability query
        ↓
target realization

The grammar itself does not query HAL.

---

137. Interoperability and ZQN

ZQN remains responsible for quantum fault/noise semantics.

An external quantum format may contain noise-related information.

That information must be translated into the canonical quantum semantic representation rather than creating a separate interoperability noise IR.

---

138. Interoperability and Provenance

External source provenance should remain available where required:

foreign language
foreign format
foreign version
foreign source location
foreign symbol
adapter
conversion

This enables diagnostics to say where an imported construct originated.

---

139. Interoperability and Semantic Preservation

The highest-priority rule for import/export is:

«Never silently change the meaning of a program.»

If exact semantic preservation is impossible, the compiler must:

1. detect the mismatch;
2. classify the loss;
3. report it;
4. require explicit acceptance where required by the language specification.

---

140. Interoperability and Deterministic Builds

Foreign interface resolution must be compatible with deterministic/reproducible compilation.

A source declaration must not silently bind to different interfaces merely because:

- a machine changed;
- a library happened to be installed;
- a network service changed;
- a different runtime was discovered.

Resolution metadata must be explicit enough for reproducibility.

---

141. Interoperability Manifest Concept

Where the build system supports machine-readable feature manifests, an interoperability feature may declare:

id
name
status
language
format
version
grammar
tokens
AST mapping
semantic mapping
IR mapping
compiler consumer
runtime consumer
capabilities
resources
effects
security
compatibility
loss policy
tests

This is compatible with the broader Zamani feature-contract architecture.

It does not create a second grammar authority.

---

142. Independent Feature Contract

An interoperability feature is independently complete when the following chain is closed:

source syntax
      ↓
token
      ↓
grammar rule
      ↓
AST node
      ↓
semantic construct
      ↓
canonical IR/domain representation
      ↓
compiler consumer
      ↓
runtime/backend consumer
      ↓
tests

and the reverse trace is also possible:

backend requirement
      ↓
IR requirement
      ↓
semantic requirement
      ↓
AST requirement
      ↓
grammar rule
      ↓
source construct

No stable feature should have an unresolved link.

---

143. Production Completion Checklist

The entire interoperability subsystem is production-ready only when:

Architecture

- [ ] "Zamani.g4" remains the single root.
- [ ] No competing interoperability root exists.
- [ ] Existing filenames are preserved unless removal is justified.
- [ ] Interoperability has one ownership boundary.
- [ ] Dependency direction is correct.

Specification

- [ ] "spec/interoperability.md" defines normative semantics.
- [ ] This README defines repository ownership/integration.
- [ ] "Zamani-Grammar.md" does not silently define syntax.
- [ ] "grammar.md" reports actual implementation status.

Grammar

- [ ] Generic FFI exists.
- [ ] Foreign functions exist.
- [ ] Foreign types exist.
- [ ] ABI syntax exists.
- [ ] Calling conventions are symbolic.
- [ ] Language identities are extensible.
- [ ] Format identities are extensible.
- [ ] Adapters exist.
- [ ] Conversions exist.
- [ ] Callbacks exist.
- [ ] Serialization contracts exist.
- [ ] System-interface contracts exist.

AST

- [ ] Every production construct has an AST mapping.
- [ ] AST remains domain-neutral.
- [ ] Source spans are preserved.
- [ ] No target-specific AST leakage exists.

Semantics

- [ ] Type compatibility is defined.
- [ ] Ownership is defined.
- [ ] Lifetime is defined.
- [ ] Nullability is defined.
- [ ] ABI compatibility is defined.
- [ ] Effects are integrated.
- [ ] Capabilities are integrated.
- [ ] Resources are integrated.
- [ ] Security is integrated.
- [ ] Compatibility is integrated.

IR

- [ ] No universal interoperability IR competes with canonical IR.
- [ ] Classical boundaries reach classical semantics/IR.
- [ ] Quantum boundaries reach "quantum::ir".
- [ ] HDL boundaries reach canonical HDL/hardware semantics.

Safety

- [ ] Parsing is inert.
- [ ] No library loading occurs during parsing.
- [ ] No process execution occurs during parsing.
- [ ] No filesystem access occurs during parsing.
- [ ] No network access occurs during parsing.
- [ ] No hardware discovery occurs during parsing.
- [ ] Rust implementation uses safe Rust only.
- [ ] No "unsafe" Rust is required.

Scalability

- [ ] No hard-coded interoperability maxima exist.
- [ ] No hard-coded machine assumptions exist.
- [ ] No fixed foreign-language list exists.
- [ ] No fixed ABI universe exists.
- [ ] No fixed target universe exists.
- [ ] Program-defined quantities remain program semantics.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Compatibility tests exist.
- [ ] Portability tests exist.
- [ ] Diagnostics tests exist.
- [ ] Security tests exist.
- [ ] Quantum interoperability tests exist.
- [ ] HDL interoperability tests exist.

---

144. Required Repository Integration

The interoperability subsystem integrates with the existing repository as follows:

grammar/
│
├── DESIGN.md
│      └── architecture authority
│
├── README.md
│      └── repository navigation/authority
│
├── Zamani.g4
│      └── canonical ANTLR root
│
├── grammar.md
│      └── implementation conformance
│
├── Zamani-Grammar.md
│      └── historical/extended design
│
├── specification/
│      └── normative language specification
│
├── spec/
│      └── formal subsystem contracts
│
├── core/
│      └── identifiers/names/attributes/etc.
│
├── types/
│      └── canonical type system
│
├── functions/
│      └── callable/function semantics
│
├── modules/
│      └── module/import/export semantics
│
├── effects/
│      └── canonical effect system
│
├── memory/
│      └── ownership/lifetime/memory semantics
│
├── resources/
│      └── resource/capability model
│
├── security/
│      └── security policy
│
├── compile/
│      └── compilation/lowering
│
├── execution/
│      └── runtime execution semantics
│
├── hardware/
│      └── hardware intent/capabilities
│
├── quantum/
│      └── quantum syntax/semantics
│
├── hdl/
│      └── native HDL semantics
│
├── distributed/
│      └── distributed semantics
│
├── networking/
│      └── network semantics
│
├── data/
│      └── data semantics
│
├── dialects/
│      └── controlled extensions
│
├── compatibility/
│      └── version/migration compatibility
│
├── validation/
│      └── structural/conformance validation
│
└── tests/
       └── interoperability conformance

---

145. Repository Implementation Baseline

The implementation consuming this grammar is:

Rust 2021
Rust 1.97
Rust 1.97.1

The production baseline is:

Rust 1.97.1

The implementation must use safe Rust.

"unsafe" Rust is prohibited.

The grammar must contain no embedded target-language execution actions.

---

146. ANTLR Safety Rule

Interoperability grammar files must remain declarative.

They must not contain actions that:

- invoke Rust;
- execute shell commands;
- load libraries;
- inspect hardware;
- access the filesystem;
- access the network;
- inspect environment variables;
- execute foreign code.

Semantic actions belong outside the grammar.

---

147. No Semantic Predicates for Environment Discovery

Semantic predicates must not be used to ask whether:

a library exists
a device exists
a runtime exists
a CPU exists
a GPU exists
a QPU exists
a file exists
a network exists

The parse tree must be independent of target availability.

---

148. Interoperability Is a Contract, Not a Runtime

This subsystem is a language boundary, not an execution engine.

Its job is to say:

WHAT external contract does this source program depend upon?

It is not responsible for:

HOW do we execute it?

---

149. One Language

C, C++, Rust, Python, Zig, WebAssembly, OpenQASM, QIR, Verilog, SystemVerilog, and future external technologies remain interoperability domains.

They do not become separate Zamani languages.

Zamani remains one language with extensible interoperability boundaries.

---

150. One Semantic Architecture

All interoperability must eventually converge on Zamani semantic infrastructure:

foreign syntax
     ↓
Zamani AST
     ↓
Zamani semantics
     ↓
canonical domain representation

No foreign format may become a second semantic authority merely because it is widely used.

---

151. One Quantum Boundary

For every quantum interoperability route:

foreign representation
        ↓
semantic quantum model
        ↓
quantum::ir

"quantum::ir" remains the canonical quantum boundary.

No second quantum IR is permitted.

---

152. One Resource Model

Interoperability uses the same:

requirement
constraint
capability
preference
hint
resource

model as the rest of Zamani.

It must not invent another resource vocabulary.

---

153. One Effect Model

Foreign calls participate in the same effect system as native operations.

There is no separate foreign effect language.

---

154. One Type Model

Foreign types participate in the canonical type system.

There is no independent FFI type universe.

---

155. One Security Model

Foreign interfaces participate in the canonical security/capability system.

There is no hidden privileged mode created merely by writing an FFI declaration.

---

156. One Compatibility Model

Foreign versions and formats participate in the canonical compatibility architecture.

There is no separate ad-hoc version system.

---

157. One Diagnostic Model

Interoperability errors use the common Zamani diagnostic architecture.

They preserve:

- source spans;
- error categories;
- contextual information;
- expected/actual information;
- provenance where available.

---

158. One Portability Model

Interoperability requirements must remain compatible with the overall portability architecture.

A target-specific dependency must be explicit.

A universal Zamani construct must not become target-specific merely because one backend happens to implement it first.

---

159. Production Architecture

The final interoperability pipeline is:

                         ZAMANI SOURCE
                              │
                              ▼
                     grammar/Zamani.g4
                              │
                              ▼
                           Lexer
                              │
                              ▼
                           Parser
                              │
                              ▼
                    Domain-Neutral AST
                              │
                              ▼
                 Interoperability Analysis
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
           Types           Effects        Capabilities
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                    Semantic Validation
                              │
                              ▼
                  Canonical Semantic Model
                              │
          ┌───────────────────┼────────────────────┐
          │                   │                    │
          ▼                   ▼                    ▼
      Classical           quantum::ir         HDL/Hardware
          │                   │                    │
          └───────────────────┼────────────────────┘
                              │
                              ▼
                         Optimization
                              │
                              ▼
                           Lowering
                              │
                    ┌─────────┼─────────┐
                    ▼         ▼         ▼
                   ABI      Routing  Scheduling
                    │         │         │
                    └─────────┼─────────┘
                              │
                              ▼
                           QEC/ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       Target Realization
                              │
           ┌──────────────────┼──────────────────┐
           │                  │                  │
          CPU                GPU                FPGA
           │                  │                  │
           ├──────────────────┼──────────────────┤
           │                  │                  │
          QPU             Distributed       Future Target
           │                  │                  │
           └──────────────────┼──────────────────┘
                              │
                              ▼
                           Runtime

---

160. Final Interoperability Principle

The interoperability subsystem exists to preserve this distinction:

WHAT THE PROGRAM MEANS

versus:

HOW ONE PARTICULAR MACHINE REALIZES IT

The grammar owns the first.

Semantic analysis validates the first.

Canonical IR represents the first.

Compiler lowering transforms the first into an implementation.

ABI infrastructure realizes foreign binary contracts.

Routing realizes physical placement.

Scheduling realizes time/resource ordering.

QEC realizes quantum error-correction strategy.

ZQN models quantum fault/noise behavior.

HAL realizes hardware capability/state.

Runtime executes the resulting realization.

Interoperability must not collapse these layers.

---

161. Final POCO-REAF Contract

The interoperability architecture is successful when the same source-level semantic contract can remain stable while realization changes according to available resources:

                    SAME ZAMANI PROGRAM
                            │
            ┌───────────────┼────────────────┐
            │               │                │
            ▼               ▼                ▼
         tiny target     ordinary target   large target
            │               │                │
            └───────────────┼────────────────┘
                            │
                  heterogeneous target
                            │
                            ▼
                       accelerator
                            │
                            ▼
                      quantum/hybrid
                            │
                            ▼
                       distributed
                            │
                            ▼
                     future substrate

The programmer should not have to rewrite portable computation merely because:

- processor count changed;
- GPU count changed;
- FPGA capacity changed;
- QPU capacity changed;
- memory changed;
- network topology changed;
- accelerator availability changed;
- deployment scale changed.

Those are downstream realization concerns.

---

162. Final Non-Negotiable Rules

1. "grammar/Zamani.g4" remains the canonical ANTLR composition root.

2. "grammar/interoperability/" does not create another root grammar.

3. "grammar/DESIGN.md" remains the normative architecture authority.

4. "grammar/grammar.md" remains the implementation-conformance reference.

5. "grammar/Zamani-Grammar.md" remains historical/extended design material and cannot silently define syntax.

6. Existing interoperability filenames are not unnecessarily renamed.

7. Duplicate or malformed filesystem artifacts must be explicitly reconciled rather than allowed to create competing authorities.

8. Universal grammar rules are not duplicated inside specialized interoperability grammars.

9. Foreign languages are represented through extensible symbolic identities.

10. Foreign formats are distinct from languages.

11. ABIs are distinct from calling conventions.

12. Languages are distinct from runtimes.

13. Libraries are distinct from physical machines.

14. Foreign symbols are symbolic identities, not physical addresses.

15. FFI declarations do not execute foreign code.

16. Parsing never performs library loading.

17. Parsing never performs hardware discovery.

18. Parsing never performs network access.

19. Parsing never performs filesystem access.

20. Parsing never invokes a linker.

21. Parsing never invokes a compiler.

22. Parsing never invokes a runtime.

23. Foreign types integrate with the canonical type system.

24. Foreign functions integrate with the canonical function model.

25. Foreign modules integrate with the canonical module system.

26. Foreign effects integrate with the canonical effect system.

27. Foreign resources integrate with the canonical resource/capability model.

28. Foreign security requirements integrate with the canonical security model.

29. Foreign compatibility integrates with the canonical compatibility system.

30. Interoperability does not create a universal "interoperability::ir".

31. Quantum interoperability always converges on "quantum::ir".

32. OpenQASM is an interoperability format, not a second Zamani semantic model.

33. QIR is an interoperability representation, not a replacement for "quantum::ir".

34. HDL interoperability does not replace native Zamani HDL.

35. No universal hardware capacity may be hard-coded.

36. No universal CPU/GPU/FPGA/QPU/node/thread/memory limit may be encoded.

37. No fixed pointer width may be assumed by the universal grammar.

38. No fixed register width may be assumed by the universal grammar.

39. No fixed topology may be assumed by the universal grammar.

40. Program-defined numeric values remain valid program semantics.

41. Requirements, constraints, capabilities, preferences, hints, and implementation decisions remain distinct.

42. AST representations remain domain-neutral.

43. Source spans are preserved.

44. Semantic validation occurs before target realization.

45. Target realization remains downstream.

46. Routing remains downstream.

47. Scheduling remains downstream.

48. QEC remains downstream.

49. ZQN remains downstream.

50. HAL remains downstream.

51. Macro expansion cannot bypass interoperability validation.

52. Metaprogramming cannot bypass interoperability validation.

53. Dialects cannot silently become separate languages.

54. External-format conversion must classify semantic loss.

55. Silent semantic loss is prohibited.

56. Interoperability parsing is deterministic.

57. Interoperability tests include positive, negative, boundary, scalability, determinism, compatibility, portability, security, and diagnostic coverage.

58. Every stable interoperability feature has a complete syntax → AST → semantic → IR → compiler/runtime → test chain.

59. Rust 1.97.1 is the production implementation baseline.

60. The compiler implementation uses safe Rust only.

61. "unsafe" Rust is prohibited.

62. Compiler/runtime resource exhaustion is not a language-level scalability ceiling.

63. Future languages and formats must be integrable without redesigning universal Zamani semantics.

64. Future hardware must be able to consume existing portable semantics through downstream realization.

65. Interoperability exists to preserve semantic portability, not to freeze today's implementation technology into the language.

---

163. Definition of Done

"grammar/interoperability/" is considered PRODUCTION READY when every stable interoperability feature can be traced:

Source
  ↓
Token
  ↓
Grammar
  ↓
AST
  ↓
Semantic Contract
  ↓
Canonical IR / Domain Representation
  ↓
Compiler Consumer
  ↓
Runtime / Backend Consumer
  ↓
Diagnostics
  ↓
Tests

and backwards:

Backend Requirement
  ↓
IR Requirement
  ↓
Semantic Requirement
  ↓
AST Representation
  ↓
Grammar Rule
  ↓
Source Construct

with no unresolved ownership boundary.

The final invariant is:

                  PORTABLE ZAMANI MEANING
                            │
                            ▼
                    INTEROPERABILITY
                            │
                            ▼
                     DOMAIN-NEUTRAL AST
                            │
                            ▼
                    SEMANTIC VALIDATION
                            │
                            ▼
                  CANONICAL SEMANTIC MODEL
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
         Classical      quantum::ir   HDL/Hardware
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                       CANONICAL IR
                            │
                            ▼
                  OPTIMIZATION / LOWERING
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
          ABI/FFI        Routing       Scheduling
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                        QEC / ZQN
                            │
                            ▼
                           HAL
                            │
                            ▼
                    TARGET REALIZATION
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
            CPU            GPU             QPU
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                      FUTURE TARGETS

Therefore the permanent interoperability rule is:

«Zamani interoperability describes the contract between Zamani semantics and an external computational ecosystem. It does not make that ecosystem part of the permanent language core, and it does not encode today's machine limitations into tomorrow's source language.»

This is the interoperability foundation required for Zamani to scale from the smallest computation to arbitrarily large computation subject to actual resource availability while preserving:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
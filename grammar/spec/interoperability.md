Zamani Interoperability Specification

Path: "grammar/spec/interoperability.md"
Status: Normative
Specification domain: Interoperability
Language: Zamani
Grammar technology: ANTLR4-compatible grammar composition
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: "unsafe" Rust is forbidden
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This specification defines the normative interoperability model for the Zamani programming language.

Interoperability allows Zamani programs to establish explicit, typed, semantically validated boundaries with:

- other programming languages;
- foreign functions;
- foreign types;
- foreign values;
- foreign libraries;
- ABIs;
- calling conventions;
- binary interfaces;
- system interfaces;
- operating-system interfaces;
- runtime services;
- distributed services;
- networking interfaces;
- classical accelerators;
- GPUs;
- FPGAs;
- ASIC-oriented interfaces;
- quantum runtimes;
- OpenQASM;
- QIR;
- HDL languages;
- Verilog/SystemVerilog;
- assembly languages;
- WebAssembly;
- future language ecosystems;
- future computing paradigms.

Interoperability is a boundary mechanism, not a second programming language and not a second intermediate representation.

The central invariant is:

«Zamani source specifies the semantic relationship and interoperability intent. Semantic analysis, capability resolution, compilation, optimization, routing, scheduling, linking, runtime systems, hardware abstraction, and deployment determine the concrete realization.»

Interoperability MUST therefore remain independent of any particular implementation technology.

---

2. Normative Language

The terms below are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHALL — mandatory.
- SHALL NOT — prohibited.
- SHOULD — recommended unless a documented architectural reason prevents it.
- SHOULD NOT — discouraged unless a documented architectural reason requires it.
- MAY — permitted.
- OPTIONAL — permitted but not required.

---

3. Architectural Position

Interoperability belongs between source-language syntax and semantic analysis.

The canonical pipeline is:

Zamani source
      │
      ▼
canonical lexer
      │
      ▼
canonical parser
      │
      ▼
domain-neutral Zamani AST
      │
      ▼
structural validation
      │
      ▼
semantic analysis
      │
      ├───────────────┬────────────────┐
      │               │                │
      ▼               ▼                ▼
 type analysis   effect analysis   capability/resource analysis
      │               │                │
      └───────────────┴────────────────┘
                      │
                      ▼
             canonical semantic model
                      │
          ┌───────────┼───────────────┐
          │           │               │
          ▼           ▼               ▼
     classical    quantum::ir    HDL/hardware
         IR           │          representation
          │           │               │
          └───────────┼───────────────┘
                      │
                      ▼
                 optimization
                      │
          ┌───────────┼──────────────┐
          ▼           ▼              ▼
       routing    scheduling     resilience
          │           │              │
          └───────────┼──────────────┘
                      │
                      ▼
                    ZQN
                      │
                      ▼
                     HAL
                      │
                      ▼
            target-specific lowering
                      │
                      ▼
               ABI/link/runtime
                      │
                      ▼
                  execution

The interoperability grammar MUST NOT reverse this dependency direction.

In particular:

grammar → runtime
grammar → hardware discovery
grammar → linker
grammar → loader
grammar → scheduler
grammar → router
grammar → QEC implementation
grammar → ZQN implementation
grammar → physical device

is prohibited.

---

4. Core Architectural Principle

Interoperability answers:

«What external computational contract does this Zamani program require or provide?»

It does not answer:

«Which physical machine will implement that contract?»

Therefore:

language
≠ ABI

ABI
≠ architecture

architecture
≠ device

device
≠ runtime

runtime
≠ library

library
≠ filesystem path

resource requirement
≠ physical allocation

capability
≠ capability discovery

foreign format
≠ canonical semantic IR

These distinctions are mandatory.

---

5. Repository Integration

The current repository already contains a dedicated interoperability subsystem.

The authoritative interoperability family currently includes:

grammar/interoperability/
├── AssemblyLanguage.g4
├── README.md
├── SystemInterfaces.g4
├── abi.g4
├── c.g4
├── cpp.g4
├── ffi.g4
├── foreign-functions.g4
├── interoperability.g4<existing filename currently contains trailing space>
├── openqasm.g4
├── python.g4
├── rust.g4
├── system-interfaces.g4
├── verilog.g4
└── zig.g4

The specification in this file governs how these files integrate.

It does not replace their specialized syntax.

---

6. Existing Repository Issue: Trailing-Space Filename

The repository currently contains:

grammar/interoperability/interoperability.g4 

with a trailing space in the filename.

This MUST be corrected.

The canonical filename is:

grammar/interoperability/interoperability.g4

The correction MUST be an explicit filesystem rename.

A second correctly named file MUST NOT be created while the malformed filename remains.

After migration:

interoperability.g4<with trailing space>

MUST NOT remain in the authoritative tree.

All references, scripts, imports, documentation, build configuration, and tests MUST point to:

grammar/interoperability/interoperability.g4

---

7. Existing Duplicate System-Interface Files

The repository currently contains both:

grammar/interoperability/SystemInterfaces.g4
grammar/interoperability/system-interfaces.g4

These MUST NOT remain two independent authorities for the same conceptual grammar.

A reconciliation MUST determine:

1. whether the files contain genuinely different concepts;
2. whether one is legacy;
3. whether one is a specialization of the other;
4. which rules are duplicated;
5. which file is consumed by tooling;
6. which file is referenced by "Zamani.g4";
7. which rules are referenced by tests;
8. which AST constructs each rule maps to.

The final architecture MUST have one authoritative owner for each system-interface concept.

If both files contain legitimate distinct domains, their ownership MUST be explicitly differentiated.

If they are duplicates, one MUST become the authoritative source and the other MUST be migrated, deprecated, or removed according to "grammar/compatibility/".

---

8. Authority Hierarchy

Interoperability syntax MUST obey the repository's authority model.

The preferred hierarchy is:

grammar/spec/
        │
        ▼
normative semantic/specification contracts
        │
        ▼
grammar/interoperability/*.g4
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer/parser implementation
        │
        ▼
AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

"grammar/Zamani-Grammar.md" MUST NOT silently introduce normative interoperability syntax.

"grammar/grammar.md" MUST NOT become a second independent grammar authority.

If implementation and specification disagree, the disagreement MUST be recorded and resolved through the compatibility process.

---

9. Relationship to "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the authoritative ANTLR composition root.

Interoperability MUST enter the language through one canonical composition path.

Conceptually:

Zamani.g4
    │
    ▼
interopItem
    │
    ├── declaration
    ├── import
    ├── export
    ├── binding
    └── foreign-call/reference

"Zamani.g4" MUST NOT independently redefine interoperability concepts that belong to:

grammar/interoperability/

In particular, multiple unrelated definitions of:

foreignFunctionCall

MUST NOT remain authoritative.

Legacy syntax MAY remain temporarily as a compatibility alias, but it MUST normalize to the canonical interoperability semantic model.

---

10. Relationship to "grammar/grammar.md"

"grammar/grammar.md" describes implementation acceptance and MUST ultimately be treated as derived/conformance documentation.

It MUST NOT silently define interoperability semantics independently of:

grammar/spec/interoperability.md

and:

grammar/interoperability/*.g4

Any legacy interoperability forms documented there MUST be classified as one of:

- stable;
- compatibility;
- deprecated;
- proposed;
- unsupported;
- historical.

---

11. Relationship to "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" is an extended language-design/reference surface.

Interoperability features appearing there MUST be reconciled with this specification.

A feature described in "Zamani-Grammar.md" is not automatically legal Zamani syntax.

The promotion path is:

design proposal
    ↓
normative specification
    ↓
grammar contract
    ↓
AST contract
    ↓
semantic contract
    ↓
IR contract
    ↓
compiler/runtime integration
    ↓
conformance tests
    ↓
stable feature

---

12. Interoperability Ownership

The interoperability subsystem owns source-level declarations and contracts for:

- foreign languages;
- foreign source formats;
- foreign functions;
- foreign types;
- foreign values;
- FFI boundaries;
- ABI intent;
- calling-convention intent;
- linkage intent;
- interface identity;
- symbol identity;
- ownership transfer;
- borrowing;
- lifetime boundaries;
- nullability;
- adapters;
- conversions;
- callbacks;
- asynchronous foreign boundaries;
- streaming foreign boundaries;
- compatibility requirements;
- portability requirements;
- capability requirements;
- effect requirements;
- resource requirements;
- foreign interface metadata;
- foreign bindings;
- import/export intent.

---

13. Non-Ownership

Interoperability does NOT own:

- lexical tokens;
- identifiers;
- qualified-name syntax;
- paths;
- ordinary expressions;
- ordinary function syntax;
- ordinary type syntax;
- module syntax;
- effect-system implementation;
- resource discovery;
- capability discovery;
- semantic type checking;
- ABI layout calculation;
- register allocation;
- stack layout;
- linker implementation;
- loader implementation;
- filesystem access;
- network access;
- process execution;
- dynamic library loading;
- symbol discovery;
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
- routing;
- scheduling;
- optimization;
- QEC implementation;
- ZQN implementation;
- resilience;
- canonical quantum IR;
- classical IR;
- HDL IR;
- deployment;
- resource allocation.

---

14. Canonical AST Boundary

Interoperability syntax MUST lower into the domain-neutral Zamani AST.

The AST is not an interoperability IR.

The existing frontend architecture already provides an "ExternalDeclaration" abstraction representing source-level external interfaces without embedding concrete backend, hardware, quantum, or foreign-language implementation objects.

The canonical AST contract MUST preserve that separation.

Conceptual interoperability AST constructs include:

InteropDeclaration
InteropImport
InteropExport
InteropBinding
InteropInterface
InteropForeignDeclaration
InteropForeignFunction
InteropForeignType
InteropForeignValue
InteropCallback
InteropAdapter
InteropConversion
InteropLanguage
InteropSourceFormat
InteropAbi
InteropCallingConvention
InteropLinkage
InteropOwnership
InteropLifetime
InteropNullability
InteropCapabilityRequirement
InteropEffectRequirement
InteropResourceRequirement
InteropCompatibility
InteropAttribute

These names describe conceptual responsibilities.

The exact Rust AST node names remain owned by "src/frontend/ast/".

No grammar file may require the AST to introduce a duplicate representation when an existing canonical node can carry the construct.

---

15. Generic AST Operation Integration

Where interoperability refers to a computation rather than a declaration, the semantic model MUST use the repository's generic operation abstraction.

The preferred conceptual model is:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

Interoperability MUST NOT create closed enums such as:

ForeignOperation::CFunction
ForeignOperation::PythonFunction
ForeignOperation::RustFunction
ForeignOperation::QiskitOperation

unless a downstream semantic subsystem has a specific reason to represent a closed semantic category.

The source language must remain extensible.

---

16. Language Identity

Foreign language identity MUST be extensible.

The grammar MUST NOT define the complete universe of languages as a closed enum.

This is prohibited as the universal source-level model:

C
| Cpp
| Python
| Rust
| Zig
| ...

Instead, language identity MUST support symbolic and/or qualified names.

Examples:

C
C++
Rust
Python
Zig
OpenQASM
Verilog
SystemVerilog
Assembly

and future identities such as:

vendor::language
organization::language::dialect
future::language

without redesigning the interoperability architecture.

---

17. Language Identity Is Not Runtime Identity

The following are separate:

language = Python
runtime = CPython
runtime version = ...
environment = ...
target = ...

Likewise:

language = Rust
compiler = ...
ABI = ...
target = ...

A language declaration MUST NOT automatically select:

- a compiler executable;
- an interpreter;
- a runtime;
- a filesystem path;
- a package installation;
- an operating system.

Those are downstream resolution concerns.

---

18. Source-Format Identity

A language and its source representation MUST be distinguishable.

For example:

language "C"
source "header"

or the canonical equivalent.

For quantum interoperability:

language "OpenQASM"
source "openqasm3"

The source-format identifier is semantic metadata.

It MUST NOT automatically mean:

- filesystem path;
- URL;
- executable;
- network location;
- package location.

Parsing MUST remain inert.

---

19. Version Identity

Interoperability MAY specify:

- language version;
- format version;
- ABI version;
- interface version;
- protocol version;
- schema version;
- compatibility range.

Version constraints MUST be represented semantically.

Examples:

language "OpenQASM" version "3"

or the repository's canonical equivalent.

A language identity MUST NOT silently mean:

current version

when exact compatibility matters.

Version resolution belongs to compatibility/semantic analysis.

---

20. ABI Contract

ABI declarations represent a contract.

They do not implement an ABI.

ABI syntax MAY identify:

- ABI family;
- ABI version;
- calling convention;
- data representation contract;
- symbol naming convention;
- linkage class;
- variadic rules;
- compatibility constraints.

ABI grammar MUST NOT calculate:

- structure layout;
- alignment;
- pointer width;
- register allocation;
- stack layout;
- instruction selection;
- binary encoding.

Those belong downstream.

---

21. ABI Must Not Become Architecture

The following implication is prohibited:

ABI → fixed CPU architecture

An ABI may be associated with target families downstream, but the language-level interoperability contract must preserve the distinction.

For example:

abi "c"

does not inherently mean:

x86_64

or:

aarch64

or:

riscv64

The target environment resolves the actual ABI realization.

---

22. Calling Conventions

Calling conventions MUST be symbolic and extensible.

Examples may include:

c
system
default

and qualified future conventions.

Calling-convention syntax MUST NOT contain:

- register names;
- fixed stack slots;
- stack addresses;
- return-register names;
- machine instructions;
- architecture-specific register allocation.

Those belong to ABI lowering.

---

23. Foreign Function Declarations

A foreign function declaration describes an external callable contract.

It does not execute or resolve the function.

Conceptually:

extern "C" fn external_function(
    value: SomeType
) -> ResultType;

means:

«A callable external contract exists with this interface.»

It does not mean:

- load a library;
- search a directory;
- resolve a symbol;
- invoke a linker;
- invoke a compiler;
- execute the function.

---

24. Foreign Symbol Identity

A Zamani symbol and foreign symbol MAY have different names.

The model MUST support:

Zamani name
      │
      ▼
foreign symbol name

For example:

compute

may map to a foreign symbol with a different ABI-level spelling.

Symbol identity MUST therefore be explicit where required.

The grammar MUST NOT force foreign ABI naming conventions into normal Zamani identifiers.

---

25. Foreign Types

Foreign types MUST support at least:

- named foreign types;
- opaque types;
- handles;
- references;
- pointer-like contracts;
- ownership;
- nullability;
- lifetime;
- ABI compatibility;
- representation constraints;
- conversion boundaries.

Foreign type syntax MUST NOT encode an assumed physical representation unless that representation is genuinely part of the semantic interoperability contract.

---

26. Opaque Types

Opaque types are REQUIRED for foreign types whose representation is intentionally hidden.

For example:

foreign type Context : opaque;

means:

«Zamani may hold and use the externally defined abstraction according to its contract.»

It does not expose:

- fields;
- layout;
- pointer width;
- address;
- allocation mechanism.

---

27. Pointer and Address Independence

Generic interoperability MUST NOT assume:

pointer = 64 bits

or:

address = u64

or any other fixed width.

The current toolchain implementation contains a placeholder address-mapping function based on "u64" and an XOR transformation. That implementation MUST NOT be treated as the interoperability semantic model.

Address representation belongs to the target-specific backend/runtime boundary.

The source language should instead represent:

reference
handle
opaque resource
foreign pointer contract
address-space contract

as semantic concepts.

---

28. Ownership

Interoperability MUST explicitly distinguish ownership states.

The semantic model MAY represent:

- borrowed;
- owned;
- transferred;
- retained;
- returned;
- released;
- shared;
- immutable;
- mutable.

The grammar describes intent.

Semantic analysis validates ownership.

Runtime/backend systems implement ownership behavior.

---

29. Lifetime

Foreign interfaces MUST be able to express lifetime constraints where required.

Examples include:

- callback lifetime;
- borrowed argument lifetime;
- returned-resource lifetime;
- context lifetime;
- session lifetime;
- stream lifetime;
- asynchronous-operation lifetime.

Lifetime declarations MUST NOT encode machine addresses or runtime allocation details.

---

30. Nullability

Foreign boundaries MUST support explicit nullability where the external contract requires it.

Semantic analysis MUST distinguish:

nullable

from:

non-null

and from:

unknown

The grammar MUST NOT silently infer safety merely because a foreign language permits null values.

---

31. Variadic Functions

Variadic foreign calls MUST be representable without an arbitrary language-level argument limit.

The grammar MUST NOT contain:

MAX_FOREIGN_ARGUMENTS = 8

or equivalent.

ABI-specific variadic rules belong to ABI semantic analysis.

Where necessary, interoperability contracts MAY express:

- variadicity;
- sentinel requirements;
- argument promotion rules;
- format constraints;
- safety requirements.

---

32. Callbacks

Callbacks are callable contracts crossing the interoperability boundary.

The model MUST support:

- callback identity;
- parameter types;
- result type;
- calling convention;
- ownership;
- lifetime;
- nullability;
- effects;
- capabilities;
- asynchronous behavior where applicable.

The grammar MUST NOT represent a callback merely as a machine pointer.

A callback is a semantic callable contract.

---

33. Asynchronous Foreign Boundaries

Foreign interfaces MAY be asynchronous.

The interoperability model MUST distinguish:

synchronous call

from:

asynchronous operation

and:

streaming operation

without embedding a particular runtime's task/future implementation.

Runtime representation remains downstream.

---

34. Streaming Boundaries

Interop MUST support streams where the foreign interface is inherently streaming.

A stream contract MAY express:

- element type;
- direction;
- lifetime;
- backpressure semantics;
- completion semantics;
- error semantics;
- cancellation;
- ordering.

It MUST NOT hard-code a fixed buffer size or thread count.

---

35. Adapters

Adapters describe transformations between semantic interface contracts.

Conceptually:

adapter Name
    from InterfaceA
    to InterfaceB

An adapter declaration does not determine whether the implementation uses:

- generated code;
- wrapper functions;
- marshaling;
- serialization;
- compiler lowering;
- runtime dispatch;
- hardware bridges;
- protocol translation.

That decision belongs downstream.

---

36. Conversion Categories

Interoperability MUST distinguish at least:

1. identity conversion;
2. type conversion;
3. representation conversion;
4. ownership conversion;
5. borrowing conversion;
6. serialization;
7. deserialization;
8. ABI conversion;
9. language conversion;
10. protocol conversion;
11. classical/quantum conversion;
12. software/hardware conversion.

Conversions MUST be explicit where loss, ownership, safety, or semantic meaning could change.

---

37. No Implicit Foreign Compatibility

Two foreign types MUST NOT become interchangeable merely because:

names match

or:

sizes appear equal

or:

both are opaque

Compatibility MUST be determined by the semantic contract.

---

38. Capability Integration

Interoperability MAY declare capability requirements.

Examples:

capability::ffi
capability::network
capability::quantum
capability::hardware
capability::accelerator

A capability requirement means:

«The program requires this capability.»

It does not mean:

«Discover and select a device now.»

Capability discovery belongs to target/resource resolution.

---

39. Capability Is Not Device Selection

This is prohibited:

requires capability::gpu

being interpreted by the grammar as:

use gpu 0

Likewise:

requires capability::quantum

must not imply:

use qpu 7

or:

use qubits 0..31

---

40. Effect Integration

Foreign calls may have effects such as:

- I/O;
- network;
- hardware;
- process;
- external-state mutation;
- asynchronous execution;
- quantum execution;
- system calls;
- persistent state.

Interoperability MAY reference effect contracts.

The effect system remains owned by:

grammar/effects/

and the corresponding semantic implementation.

Interoperability MUST NOT define a second effect system.

---

41. Resource Integration

Foreign boundaries MAY require resources.

Examples:

resource::memory
resource::network
resource::accelerator
resource::quantum
resource::storage

The resource model MUST distinguish:

requirement
constraint
preference
hint
capability
allocation

These are not interchangeable.

---

42. Resource Requirements Are Not Allocation

For example:

requires quantum

is a semantic requirement.

It is not:

use qpu 0

Likewise:

requires memory >= n

does not mean:

allocate memory bank 3

Physical allocation belongs to runtime/resource management.

---

43. POCO-REAF

Interoperability MUST preserve:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

The same source-level interoperability semantics MUST remain valid as target resources scale.

The source MUST NOT require rewriting merely because the target changes from:

tiny machine

to:

large machine

or:

classical

to:

quantum

or:

single-node

to:

distributed

provided the required semantic capabilities exist.

---

44. Scalability

The grammar MUST NOT impose language-level finite limits on:

- number of languages;
- number of foreign declarations;
- number of interfaces;
- number of functions;
- number of parameters;
- number of callbacks;
- number of adapters;
- number of conversions;
- number of capabilities;
- number of effects;
- number of resources;
- number of devices;
- number of nodes;
- number of processors;
- number of accelerators;
- number of qubits;
- memory capacity;
- topology size;
- source size.

There must be no universal constants such as:

MAX_LANGUAGES
MAX_FOREIGN_FUNCTIONS
MAX_PARAMETERS
MAX_INTERFACES
MAX_DEVICES
MAX_QUBITS
MAX_NODES
MAX_MEMORY

in the language grammar.

---

45. Practical Resource Limits

Implementation limits MAY exist.

For example:

- available RAM;
- available storage;
- parser recursion/resource policy;
- compiler resource policy;
- operating-system limits;
- runtime limits;
- target limits.

These limits MUST be external to language semantics.

A compiler MAY reject a program because the available environment cannot process it.

That does not make the language grammar finite at that scale.

---

46. Foreign Language Specializations

The specialized interoperability grammars have the following ownership.

File| Responsibility
"interoperability.g4"| Composition and generic interoperability boundary
"foreign-functions.g4"| Generic foreign callable contracts
"ffi.g4"| Generic FFI constructs
"abi.g4"| ABI contracts
"c.g4"| C interoperability
"cpp.g4"| C++ interoperability
"python.g4"| Python interoperability
"rust.g4"| Rust interoperability
"zig.g4"| Zig interoperability
"openqasm.g4"| OpenQASM interoperability
"verilog.g4"| Verilog/HDL interoperability
"AssemblyLanguage.g4"| Assembly interoperability
"system-interfaces.g4" / reconciled equivalent| System interfaces

No specialized file may silently become the universal interoperability authority.

---

47. C Interoperability

"c.g4" owns C-specific syntax.

It MAY express:

- C language identity;
- C ABI intent;
- C-compatible declarations;
- foreign C functions;
- C types;
- callbacks;
- linkage;
- ownership;
- nullability;
- attributes;
- compatibility metadata.

It MUST NOT own:

- the complete C language;
- preprocessing;
- C compiler execution;
- linker execution;
- dynamic loading;
- target discovery.

---

48. C++ Interoperability

"cpp.g4" owns C++ interoperability-specific constructs.

It MUST remain a boundary contract.

It MUST NOT make C++ implementation details part of Zamani's permanent semantic core.

C++ ABI details remain downstream.

---

49. Python Interoperability

"python.g4" owns Python-specific interoperability.

Python identity MUST remain separate from:

- CPython;
- PyPy;
- interpreter executable;
- virtual environment;
- filesystem installation;
- host operating system.

A Python interoperability declaration MUST NOT silently invoke an interpreter during parsing or semantic construction.

---

50. Rust Interoperability

"rust.g4" owns Rust-specific interoperability syntax.

The repository implementation baseline is:

Rust 1.97
Rust 1.97.1
Rust 2021
unsafe Rust forbidden

This is an implementation requirement for Zamani itself.

It does NOT make Rust the semantic foundation of all foreign interfaces.

Rust is one interoperability target among many.

---

51. Zig Interoperability

"zig.g4" owns Zig-specific interoperability.

Zig remains a foreign language contract.

It MUST NOT be conflated with:

- CPU target;
- operating system;
- linker;
- ABI implementation;
- runtime;
- device.

---

52. Assembly Interoperability

"AssemblyLanguage.g4" owns assembly interoperability.

Assembly is inherently target-sensitive.

The specification therefore distinguishes:

assembly language family

from:

instruction set architecture

and:

processor implementation

Target-specific assembly MAY be declared explicitly.

Portable Zamani source MUST NOT silently become architecture-specific merely because assembly interoperability exists elsewhere in the program.

---

53. OpenQASM Interoperability

"openqasm.g4" owns OpenQASM-specific source/format syntax.

OpenQASM is an interoperability boundary.

It is NOT the canonical Zamani quantum semantic representation.

The required path is:

OpenQASM source
      ↓
OpenQASM frontend
      ↓
domain-neutral semantic validation
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC/resilience where required
      ↓
ZQN
      ↓
HAL
      ↓
target realization

The interoperability subsystem MUST NOT create:

OpenQASM IR

as a replacement for:

quantum::ir

---

54. Quantum Interoperability

Quantum interoperability MUST NOT own:

- qubit identity;
- physical qubit mapping;
- gate implementation;
- circuit optimization;
- decomposition;
- routing;
- scheduling;
- calibration;
- QEC;
- ZQN;
- resilience;
- device selection.

It only establishes the foreign semantic boundary.

---

55. QEC Integration

A foreign quantum interface MAY expose error-correction information.

Interoperability may describe that external contract.

It MUST NOT implement:

- syndrome extraction;
- decoder algorithms;
- correction;
- code selection;
- code distance;
- recovery;
- QEC scheduling.

Those remain in the quantum resilience/QEC subsystems.

---

56. ZQN Integration

ZQN remains responsible for fault/noise semantics.

Foreign quantum interfaces may provide data that is adapted into ZQN.

Interoperability MUST NOT duplicate:

- fault models;
- noise models;
- channels;
- correlated faults;
- leakage;
- loss;
- erasure semantics.

---

57. HDL Interoperability

"verilog.g4" and other HDL interoperability grammars represent foreign HDL boundaries.

They MAY express:

- module identity;
- interface identity;
- port contracts;
- binding;
- import/export intent;
- compatibility;
- capability requirements.

They MUST NOT perform:

- synthesis;
- placement;
- routing;
- timing closure;
- FPGA discovery;
- ASIC selection;
- physical pin selection;
- clock-tree implementation.

---

58. Hardware Independence

Interoperability MUST NOT encode universal assumptions such as:

32-bit register
64-bit pointer
8 CPU cores
32 GPU units
1024 qubits
24 GB memory
16 FPGA banks

unless such a value is explicitly part of a program's semantic requirement.

Even then, it must remain a requirement, not a hidden compiler limit.

---

59. System Interfaces

System-interface interoperability MAY describe:

- system calls;
- OS services;
- IPC;
- process boundaries;
- device interfaces;
- platform services;
- kernel interfaces.

Parsing MUST NOT:

- execute system calls;
- open files;
- create processes;
- inspect environment variables;
- inspect the host OS;
- access hardware.

---

60. FFI Ownership

"ffi.g4" owns reusable generic FFI constructs.

"foreign-functions.g4" owns generic callable contracts.

"abi.g4" owns ABI contracts.

Language-specific files specialize these concepts.

"interoperability.g4" composes them.

The architecture is:

generic FFI
      │
      ├── ABI
      ├── foreign callable
      └── boundary contract
             │
             ▼
      language specialization
             │
             ▼
      semantic interoperability

No duplicate conceptual foreign-call grammar should exist at every layer.

---

61. Core Grammar Reuse

Interoperability MUST reuse canonical definitions for:

- identifiers;
- qualified names;
- paths;
- attributes;
- metadata;
- expressions;
- types;
- parameters;
- arguments;
- generics.

It MUST NOT redefine them.

Canonical ownership remains with:

grammar/core/
grammar/types/
grammar/expressions/
grammar/functions/
grammar/modules/

as applicable.

---

62. Type Integration

Interoperability consumes the canonical type grammar.

It MUST NOT create a second universal type system.

Foreign-specific type information belongs in semantic metadata surrounding canonical types.

For example:

canonical Zamani type
+
foreign representation contract
+
ABI contract
+
ownership contract

is preferred over an entirely separate foreign type universe.

---

63. Expression Integration

Foreign calls MUST use canonical expression and argument grammar.

The interoperability subsystem MUST NOT create a second argument expression grammar.

This ensures that:

ordinary call
foreign call
callback
adapter
conversion

can participate in a consistent expression model.

---

64. Function Integration

Ordinary Zamani functions remain owned by "grammar/functions/".

Interoperability only establishes foreign callable contracts.

It MUST NOT duplicate ordinary function semantics.

---

65. Module Integration

Interoperability declarations MAY occur inside modules.

Module ownership remains with "grammar/modules/".

The interoperability subsystem only supplies declaration forms that the module system can contain.

---

66. Effects Integration

Effects remain owned by "grammar/effects/".

Interop declarations may reference:

effects

but must not redefine them.

---

67. Resources Integration

Resource requirements remain owned by:

grammar/resources/

Interop MAY reference:

- resource requirements;
- capabilities;
- constraints;
- preferences;
- hints.

It must not create a foreign-specific resource model.

---

68. Security Integration

Interoperability is an explicit trust boundary.

Security requirements MAY be attached to foreign interfaces.

However, interoperability does not own:

- authentication;
- authorization;
- cryptography;
- key management;
- identity management;
- trust stores.

Those remain in the security subsystem.

---

69. Compiler Integration

The compiler consumes semantic interoperability contracts to determine:

- symbol resolution;
- ABI lowering;
- linking;
- marshaling;
- adapter generation;
- compatibility;
- target realization;
- runtime dispatch.

The compiler MUST NOT make the grammar dependent on compiler implementation details.

---

70. Runtime Integration

Runtime integration occurs after semantic lowering.

The intended flow is:

interop AST
      ↓
semantic validation
      ↓
canonical semantic representation
      ↓
compiler lowering
      ↓
runtime boundary
      ↓
foreign execution

The grammar MUST NOT invoke runtime functionality.

---

71. Hardware Integration

Hardware-specific realization is downstream.

Interoperability may express:

requires capability
requires resource
requires interface
requires compatibility

It MUST NOT select:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- physical qubit;
- physical address;
- network node;
- memory bank.

Those are realization decisions.

---

72. Distributed Interoperability

Foreign distributed services MAY expose:

- endpoints;
- protocols;
- service contracts;
- message schemas;
- streams;
- replication semantics;
- consistency requirements.

The grammar MUST NOT hard-code:

N nodes

or:

N replicas

as universal limits.

The actual deployment is resolved downstream.

---

73. Networking Interoperability

Networking interfaces MAY express:

- protocol identity;
- service identity;
- endpoint contracts;
- request/response types;
- streams;
- transport requirements;
- security requirements;
- capability requirements.

Network locations MUST be treated as semantic data where explicitly provided.

Parsing MUST NOT contact the network.

---

74. WebAssembly and Future Binary Targets

The interoperability architecture MUST be extensible to formats such as:

- WebAssembly;
- object formats;
- bytecode formats;
- future portable binary formats.

A future format MUST NOT require redesign of the generic interoperability model.

It should define:

format identity
version
type mapping
calling convention
linkage
capabilities
semantic adapter

as appropriate.

---

75. Future Language Extensions

The architecture MUST support future language files such as:

swift.g4
java.g4
kotlin.g4
fortran.g4
wasm.g4
cuda.g4
opencl.g4
sycl.g4
systemc.g4
vhdl.g4
spice.g4

without requiring a closed language enumeration in the core grammar.

---

76. Dialect Integration

Dialect-specific interoperability MUST remain explicit.

A dialect may extend:

- syntax;
- semantics;
- type mappings;
- capabilities;
- adapters;
- compatibility rules.

It MUST declare:

- dialect identity;
- version;
- namespace;
- syntax extension;
- semantic extension;
- AST mapping;
- IR mapping;
- compatibility;
- feature status.

Dialect syntax MUST NOT silently replace canonical Zamani semantics.

---

77. Canonical IR Boundary

Interoperability is not an IR.

The correct transformation is:

interop syntax
      ↓
AST
      ↓
semantic validation
      ↓
canonical semantic model
      ↓
domain IR

Possible downstream representations include:

classical IR
quantum::ir
HDL/hardware representation
distributed representation
runtime boundary representation

Interoperability MUST NOT create:

ForeignCIR
ForeignRustIR
ForeignPythonIR
ForeignQuantumIR
ForeignHardwareIR

as universal semantic authorities.

---

78. Quantum Canonical Boundary

For all quantum interoperability:

foreign quantum syntax
      ↓
quantum frontend
      ↓
quantum semantic analysis
      ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

No interoperability specification may redefine:

- "QubitId";
- physical-qubit identity;
- gate semantics;
- quantum operation semantics;
- circuit semantics;
- QEC semantics;
- ZQN semantics.

---

79. AST-to-IR Contract

Every interoperability construct MUST have a predetermined path:

grammar rule
      ↓
AST construct
      ↓
structural validation
      ↓
semantic model
      ↓
canonical IR or metadata

No new interoperability grammar feature is complete if its AST and semantic mapping are unspecified.

---

80. Source Spans

Every interoperability AST construct MUST preserve source location information sufficient for diagnostics.

At minimum, diagnostics must be able to identify:

- source file;
- declaration;
- relevant span;
- foreign language;
- interface;
- symbol;
- conflicting contract.

Source locations MUST be deterministic.

---

81. Determinism

Interoperability parsing MUST be deterministic.

The same:

source
+
grammar version
+
lexer configuration

MUST produce the same structural result.

Parsing MUST NOT depend on:

- filesystem state;
- network state;
- installed libraries;
- device availability;
- compiler availability;
- runtime availability;
- environment variables;
- random values;
- wall-clock time.

---

82. Security Boundary

Parsing MUST NEVER:

- execute foreign code;
- load libraries;
- open files;
- access network resources;
- spawn processes;
- invoke compilers;
- invoke linkers;
- invoke interpreters;
- inspect devices;
- inspect hardware;
- dereference addresses;
- mutate runtime state.

ANTLR actions/predicates that violate this boundary MUST NOT be introduced.

---

83. Rust Safety

All Rust implementation supporting interoperability MUST satisfy:

Rust 1.97
Rust 1.97.1
Rust 2021
unsafe forbidden

The preferred repository-level enforcement is:

#![forbid(unsafe_code)]

Interoperability semantics MUST NOT require "unsafe".

If a particular backend eventually requires an unsafe external ABI operation, that operation belongs to an explicitly isolated downstream implementation boundary and MUST NOT leak into:

- grammar;
- AST;
- semantic model;
- portability contract.

The grammar itself is always safe and inert.

---

84. Correction of Current Toolchain Interoperability Model

The current "src/toolchain/interoperability.rs" contains a closed:

enum ForeignLanguage

and placeholder implementation behavior including an address transformation.

These are not acceptable as the canonical interoperability model.

The production architecture MUST instead use extensible semantic identities.

Conceptually:

ForeignLanguageIdentity
    ├── namespace
    ├── name
    ├── optional version
    └── optional dialect

rather than a permanently closed enumeration.

Likewise, address conversion MUST NOT be modeled as arbitrary integer transformation.

A foreign address must be represented as a typed semantic reference/handle/address-space contract and resolved by the appropriate target-specific layer.

The grammar specification does not prescribe the concrete Rust replacement type; that belongs to the implementation contract.

---

85. No Fake Verification

An interoperability binding MUST NOT be marked verified merely because it was syntactically constructed.

For example, the semantic state:

verified = true

must not be assigned simply because a declaration was parsed.

Verification requires the appropriate validation stages.

Possible states include:

unvalidated
structurally_valid
semantically_valid
capability_valid
target_compatible
linkable
runtime_available

These states belong to semantic/toolchain models rather than the grammar.

---

86. Linkage

Linkage MAY express semantic classes such as:

- static;
- dynamic;
- weak;
- strong;
- system;
- runtime;
- framework;
- custom.

The grammar describes linkage intent.

The linker performs linkage.

The loader performs loading.

The runtime performs runtime resolution where applicable.

---

87. Import Semantics

An interoperability import establishes a semantic dependency.

It does not automatically:

- open a file;
- fetch a URL;
- download a package;
- load a library;
- execute a compiler;
- contact a server.

Resolution is performed by downstream tooling according to explicit policy.

---

88. Export Semantics

Exports establish a public interoperability boundary.

An export MAY specify:

- exported name;
- foreign symbol;
- interface;
- ABI;
- calling convention;
- visibility;
- ownership;
- effects;
- capabilities;
- compatibility.

The actual binary/export table is generated downstream.

---

89. Symbol Resolution

Symbol resolution is not parser responsibility.

The semantic/compiler system resolves:

Zamani symbol
        ↓
foreign interface
        ↓
foreign symbol
        ↓
ABI
        ↓
link/runtime mechanism

Failure at each layer must remain distinguishable.

---

90. Error Model

Interoperability errors MUST be separated by phase.

Syntax errors

Examples:

- malformed declaration;
- missing parameter;
- invalid delimiter;
- malformed ABI clause.

Structural AST errors

Examples:

- invalid node relationship;
- duplicate child identity;
- malformed source span;
- invalid AST graph reference.

Semantic errors

Examples:

- incompatible types;
- incompatible ABI;
- invalid ownership;
- invalid lifetime;
- invalid callback contract.

Capability errors

Examples:

- required capability unavailable.

Compatibility errors

Examples:

- incompatible interface version;
- unsupported foreign format.

Linking errors

Examples:

- symbol cannot be resolved.

Runtime errors

Examples:

- external service unavailable;
- runtime contract unavailable.

These error categories MUST NOT be collapsed into parser errors.

---

91. Diagnostics

Interoperability diagnostics SHOULD include:

- stable diagnostic code;
- source span;
- declaration identity;
- language identity;
- source format;
- interface identity;
- symbol identity;
- expected contract;
- actual contract;
- reason;
- severity;
- remediation information where available.

Diagnostics generated during parsing MUST remain independent of external environment state.

---

92. Compatibility With Legacy "foreignFunctionCall"

The repository contains legacy foreign-call concepts.

These MUST be migrated deliberately.

The migration path is:

legacy syntax
      ↓
inventory
      ↓
consumer analysis
      ↓
semantic equivalence analysis
      ↓
canonical interoperability representation
      ↓
compatibility alias
      ↓
deprecation
      ↓
removal when permitted

Legacy syntax MUST NOT be silently deleted.

Equivalent legacy and canonical forms SHOULD normalize to the same semantic representation.

---

93. Compatibility With Existing "ExternalDeclaration"

The existing frontend AST contains a source-level "ExternalDeclaration" abstraction.

That abstraction is compatible with the architectural role of generic interoperability declarations.

The integration rule is:

interop declaration
      ↓
ExternalDeclaration or canonical specialized AST node
      ↓
semantic analysis

The AST MUST NOT be forced to embed:

- ABI implementation;
- hardware handles;
- QPU handles;
- LLVM values;
- MLIR operations;
- QIR values;
- OpenQASM ASTs;
- routing;
- scheduling;
- calibration;
- QEC;
- ZQN.

Those remain downstream.

---

94. Foreign AST Isolation

A foreign format may have its own parser/AST.

For example:

OpenQASM frontend AST
Verilog frontend AST
C frontend representation

must remain owned by their respective frontend/import systems.

The generic Zamani interoperability AST should contain the boundary contract, not embed the complete foreign language AST.

---

95. Serialization

Interoperability AST metadata MUST be serializable where the repository AST serialization model supports serialization.

Serialization MUST preserve:

- language identity;
- source format;
- version;
- ABI identity;
- symbols;
- types;
- ownership;
- lifetime;
- nullability;
- capabilities;
- effects;
- resource requirements;
- compatibility constraints;
- source spans.

Serialization MUST NOT serialize ephemeral runtime handles as if they were portable source semantics.

---

96. Provenance

Interop declarations SHOULD preserve provenance where required.

Provenance may identify:

- source declaration;
- imported format;
- interface version;
- transformation/adaptation;
- compatibility decision;
- semantic lowering source.

Provenance MUST remain deterministic.

It MUST NOT depend on memory addresses or nondeterministic runtime identifiers.

---

97. Reproducibility

Interoperability compilation SHOULD be reproducible given the same:

source
grammar version
compiler version
semantic contracts
foreign interface specifications
declared compatibility constraints
toolchain inputs

Undeclared external state MUST NOT silently alter the semantic meaning of the program.

---

98. Portability Classes

An interoperability dependency MAY be classified as:

Universal

Works across all targets satisfying the semantic contract.

Portable

Works across a defined family of targets.

Conditional

Requires declared capabilities/constraints.

Target-specific

Explicitly depends on a particular target property.

Non-portable

Cannot satisfy POCO-REAF without changing the program.

The classification MUST be explicit where it affects portability.

---

99. Target-Specific Interoperability

Target-specific interoperability is permitted when it is genuinely required by program semantics.

It MUST be explicit.

For example:

requires target capability ...

or the canonical target/resource/constraint syntax.

It MUST NOT be hidden inside a generic declaration.

A program that intentionally targets one architecture is semantically different from a program intended for universal deployment.

---

100. No Hidden Target Binding

The following are prohibited as hidden semantics:

language "C"
    → x86_64

language "Rust"
    → Linux

language "Python"
    → CPython

language "OpenQASM"
    → specific QPU

language "Verilog"
    → specific FPGA

Such relationships may exist in downstream target mappings.

They must not be silently embedded in source-level interoperability meaning.

---

101. Capability Negotiation

When an interface can be implemented by multiple targets, the program SHOULD describe:

required capabilities
required semantics
required guarantees
optional preferences

rather than physical target identity.

Conceptually:

require capability X
prefer capability Y
allow implementation Z

The resource manager and compiler decide the actual realization.

---

102. Requirement vs Preference vs Hint

These concepts MUST remain separate.

Requirement

The program cannot correctly execute without it.

Constraint

A realization must satisfy it.

Preference

A realization is preferred but alternatives are legal.

Hint

A non-binding optimization suggestion.

Capability

A property the target must provide.

Allocation

An actual runtime/compile-time assignment.

Interoperability MUST NOT collapse all of these into one keyword.

---

103. Foreign Memory

Foreign memory MAY be represented as:

- opaque resource;
- borrowed region;
- owned region;
- shared region;
- external buffer;
- mapped resource.

The grammar MUST NOT assume:

- fixed RAM;
- fixed VRAM;
- fixed address width;
- fixed cache size;
- fixed memory-bank count.

---

104. Accelerator Interoperability

Accelerator interfaces MAY describe:

- accelerator capability;
- callable kernels;
- buffer contracts;
- execution semantics;
- synchronization;
- resource requirements;
- effects.

They MUST NOT hard-code:

GPU 0
GPU count = N
fixed warp count
fixed vector width
fixed accelerator memory

unless explicitly declared as target-specific program constraints.

---

105. Quantum-Classical Interoperability

Hybrid programs may cross:

classical
      ↓
quantum
      ↓
measurement/result
      ↓
classical
      ↓
quantum

Interoperability MUST preserve the semantic boundary.

The hybrid subsystem owns hybrid computation semantics.

The quantum subsystem owns quantum semantics.

Interoperability only represents foreign boundaries.

---

106. Hardware/Software Co-Design

Interoperability MUST support hardware/software boundaries without making hardware details part of the universal language core.

A contract may describe:

- hardware interface;
- software interface;
- data contract;
- timing requirement;
- capability;
- resource;
- protocol;
- compatibility.

The hardware subsystem determines implementation.

---

107. HDL and Software Co-Design

A software program MAY interoperate with HDL-generated hardware through a semantic contract.

The source-level relationship should be:

software interface
      +
hardware interface
      +
type/data contract
      +
capability requirement

rather than:

software → physical FPGA pin

unless explicit target-specific syntax is being used.

---

108. Networking and Distributed Interoperability

Foreign network/distributed interfaces MAY describe:

- service contracts;
- message contracts;
- serialization;
- streaming;
- protocol identity;
- security requirements;
- consistency requirements;
- fault semantics.

The grammar MUST NOT perform service discovery.

---

109. AI/ML Interoperability

Interoperability MAY expose foreign AI/ML runtimes or model formats.

Examples include:

- model interfaces;
- tensor interfaces;
- inference interfaces;
- training interfaces;
- accelerator boundaries.

The grammar MUST NOT hard-code a particular AI framework as the universal semantic model.

Framework-specific concepts belong in adapters/dialects/interoperability implementations.

---

110. Data Interoperability

Foreign data formats MAY expose:

- schema;
- serialization;
- deserialization;
- table interfaces;
- stream interfaces;
- tensor formats;
- dataset interfaces.

The data subsystem owns data semantics.

Interoperability owns the boundary.

---

111. Serialization Formats

Serialization format identity MUST be extensible.

The grammar may identify formats such as:

JSON
CBOR
MessagePack
ProtocolBuffers
Arrow

or future formats.

A format identifier does not automatically perform serialization.

The compiler/runtime/library layer performs actual conversion.

---

112. Protocol Interoperability

Protocols MAY be identified semantically.

A protocol declaration MUST NOT cause network activity during parsing.

Protocol negotiation belongs to runtime/toolchain infrastructure.

---

113. Security and Trust

Foreign interfaces MUST be treated as explicit trust boundaries.

The semantic model SHOULD allow declaring security requirements such as:

- trusted boundary;
- untrusted boundary;
- authenticated boundary;
- integrity requirement;
- confidentiality requirement;
- sandbox requirement;
- isolation requirement.

The security subsystem implements these guarantees.

---

114. Sandbox Boundaries

An external interface MAY require sandboxing.

The grammar describes the requirement.

It MUST NOT implement the sandbox.

Runtime/toolchain systems determine:

- isolation mechanism;
- permissions;
- process boundary;
- capability set;
- resource quotas.

---

115. Foreign Exceptions and Errors

Foreign error models MUST be represented explicitly.

The semantic boundary may distinguish:

- return-value errors;
- exception-like errors;
- status codes;
- panic/abort behavior;
- asynchronous errors;
- stream errors.

The grammar MUST NOT assume that all languages share the same error model.

Adapters must normalize semantics where required.

---

116. Foreign Concurrency

Foreign interfaces MAY declare concurrency semantics.

Examples:

- thread-safe;
- serialized;
- reentrant;
- actor-like;
- async;
- blocking;
- non-blocking.

These are semantic contracts.

They MUST NOT be interpreted as a fixed number of threads.

---

117. Deterministic Interoperability

Where a foreign interface is nondeterministic, the semantic model SHOULD be able to record that property.

The compiler MUST NOT incorrectly assume deterministic behavior merely because the syntax is deterministic.

Interoperability MAY therefore carry semantic attributes concerning:

- determinism;
- ordering;
- idempotence;
- purity;
- side effects;
- reproducibility.

---

118. Calling Semantics

A foreign callable contract SHOULD make the following distinguishable where relevant:

pure
impure
blocking
nonblocking
sync
async
reentrant
non-reentrant
deterministic
nondeterministic
idempotent
non-idempotent

These belong to semantic/effect analysis, not ABI implementation.

---

119. Interface Identity

An interface MUST have a stable semantic identity.

Identity MAY include:

namespace
name
version
dialect
source format

Identity MUST NOT be derived from:

- memory address;
- filesystem address;
- process ID;
- device ID;
- runtime pointer.

---

120. Interface Compatibility

Two interfaces are compatible only when their semantic contracts are compatible.

Compatibility may consider:

- names;
- versions;
- types;
- ownership;
- lifetime;
- effects;
- capabilities;
- ABI;
- calling convention;
- error behavior;
- data representation;
- ordering;
- concurrency;
- security.

Compatibility analysis is not parser responsibility.

---

121. Adapter Graphs

Adapters may form arbitrary graphs.

The language MUST NOT limit:

number of adapters

or:

number of conversion steps

The semantic compiler may impose configurable resource limits for hostile or pathological input, but those limits are not language semantics.

---

122. No Cyclic Runtime Assumptions

Interoperability declarations may form dependency graphs.

Semantic analysis MUST detect illegal cycles where cycles violate the relevant contract.

The grammar itself must not attempt runtime graph resolution.

---

123. Cross-Domain Integration

The interoperability subsystem MUST be capable of crossing domains without creating parallel semantic universes.

Examples:

classical + C
classical + C++
classical + Rust
classical + Python
quantum + OpenQASM
quantum + C
quantum + Python
quantum + HDL
classical + quantum + HDL
classical + quantum + distributed
AI + accelerator
AI + networking
HDL + software
quantum + hardware

All such paths must ultimately converge on the repository's canonical semantic architecture.

---

124. Interoperability and Quantum "quantum::ir"

This is a mandatory invariant.

No interoperability feature may introduce a second canonical quantum IR.

The only canonical quantum semantic boundary remains:

quantum::ir

Foreign quantum representations must lower into it.

---

125. Interoperability and ZQN

ZQN remains the authority for quantum fault/noise semantics.

Interop adapters may translate external representations into ZQN-compatible semantics.

They must not duplicate ZQN.

---

126. Interoperability and QEC

QEC remains the authority for error-correction semantics and implementation.

Interop does not implement QEC.

---

127. Interoperability and Routing

Routing remains responsible for physical realization.

Interop does not map logical qubits to physical qubits.

---

128. Interoperability and Scheduling

Scheduling remains responsible for:

- timing;
- ordering;
- resource conflicts;
- concurrency;
- placement timing.

Interop provides constraints where required.

---

129. Interoperability and Optimization

Optimization may transform foreign calls when semantics permit.

Interop itself does not optimize.

Examples of downstream transformations may include:

- inlining;
- specialization;
- elimination;
- vectorization;
- batching;
- fusion;
- lowering.

These transformations must preserve the foreign contract.

---

130. Interoperability and HAL

HAL determines actual hardware capabilities/state.

Interop may require a capability.

HAL determines whether the selected target can provide it.

Interop MUST NOT contain HAL implementation objects.

---

131. Interoperability and Resource Manager

Resource management determines actual resource allocation.

Interop expresses:

requirement
constraint
preference
hint
capability

It does not allocate.

---

132. Interoperability and Runtime

Runtime receives already validated semantic contracts.

Runtime MAY perform:

- dynamic symbol resolution;
- service resolution;
- dispatch;
- foreign calls;
- resource management;
- error propagation.

These operations MUST NOT occur during parsing.

---

133. Tooling Integration

IDE/tooling systems MAY use interoperability metadata for:

- completion;
- navigation;
- symbol inspection;
- foreign interface browsing;
- diagnostics;
- documentation;
- dependency graphs;
- compatibility reports.

Tooling MUST NOT execute foreign code merely to provide ordinary parsing/editor functionality.

---

134. Testing Strategy

The interoperability subsystem MUST have:

positive tests
negative tests
boundary tests
scalability tests
compatibility tests
determinism tests
security tests
cross-domain tests
round-trip tests
AST mapping tests
semantic mapping tests
IR integration tests

---

135. Positive Tests

Positive tests MUST cover:

- generic foreign declaration;
- import;
- export;
- binding;
- foreign function;
- foreign type;
- opaque type;
- callback;
- adapter;
- conversion;
- ABI;
- calling convention;
- linkage;
- ownership;
- lifetime;
- nullability;
- capability;
- effect;
- resource requirement;
- C;
- C++;
- Python;
- Rust;
- Zig;
- OpenQASM;
- Verilog/HDL;
- assembly;
- system interfaces.

---

136. Negative Tests

Negative tests MUST cover:

- malformed foreign declaration;
- invalid language identity;
- invalid version;
- invalid ABI;
- incompatible type;
- invalid ownership;
- invalid lifetime;
- invalid callback;
- invalid adapter;
- invalid conversion;
- malformed linkage;
- invalid capability;
- invalid effect;
- invalid resource contract;
- duplicate identity;
- illegal target binding;
- illegal hardware dependency.

---

137. Boundary Tests

Boundary tests MUST include:

- zero interoperability declarations;
- one declaration;
- many declarations;
- deeply nested interfaces;
- long qualified names;
- large parameter lists;
- large adapter graphs;
- many foreign languages;
- many capabilities;
- many effects;
- many resource requirements.

No arbitrary maximum may be used as a language semantic boundary.

---

138. Scalability Tests

Scalability tests MUST demonstrate that the same interoperability semantics can be used with:

tiny target
small target
medium target
large target
distributed target
accelerator target
quantum target
hybrid target
future target

The program should not require syntax changes merely because available resources scale.

---

139. Quantum Scalability Tests

Quantum interoperability tests MUST cover:

- one qubit;
- multiple qubits;
- symbolic qubit counts;
- parameterized registers;
- dynamic resources;
- custom operations;
- OpenQASM;
- quantum/classical boundaries;
- foreign quantum services.

No test may establish an artificial maximum qubit count.

---

140. Hardware Scalability Tests

Hardware interoperability tests MUST vary:

- CPU resources;
- GPU resources;
- FPGA resources;
- QPU resources;
- memory;
- accelerators;
- network resources.

The source-level interoperability contract must remain stable when the resource context changes.

---

141. Distributed Scalability Tests

Distributed interoperability tests MUST vary:

one node
many nodes
large cluster
dynamic cluster

without encoding a universal node-count limit.

---

142. Determinism Tests

Repeated parsing of the same source MUST yield equivalent parse structures and AST semantics.

The tests MUST prove that results do not depend on:

- current machine;
- installed libraries;
- network state;
- filesystem contents;
- device availability;
- environment variables;
- wall-clock time.

---

143. Security Tests

Security tests MUST prove that parsing and AST construction do not:

- open files;
- access network;
- execute programs;
- load libraries;
- inspect devices;
- invoke foreign code;
- access environment variables;
- dereference native addresses.

---

144. Round-Trip Tests

Where a canonical printer exists:

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

must preserve semantic interoperability meaning.

Formatting differences are permitted.

Semantic changes are not.

---

145. Hard-Coding Audit

Every interoperability grammar and implementation file MUST be checked for accidental hard-coding.

Audit targets include:

MAX_FOREIGN_FUNCTIONS
MAX_LANGUAGES
MAX_PARAMETERS
MAX_INTERFACES
MAX_DEVICES
MAX_NODES
MAX_QUBITS
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_MEMORY

Also audit for:

- fixed device IDs;
- fixed physical addresses;
- fixed pointer widths;
- fixed register widths;
- fixed topology;
- fixed library paths;
- fixed executable paths;
- fixed compiler paths;
- fixed operating systems;
- fixed accelerator counts.

Each occurrence must be classified as:

1. language semantic;
2. interoperability semantic;
3. explicit target requirement;
4. resource constraint;
5. implementation limit;
6. test limit;
7. documentation example;
8. accidental hard-coding.

Accidental hard-coding MUST be removed.

---

146. No Hidden Machine Constants

The following are prohibited as universal interoperability semantics:

QPU_0
GPU_0
CPU_0
FPGA_0
NODE_0
PHYSICAL_QUBIT_0
ADDRESS_0
REGISTER_0

A target-specific declaration MAY explicitly name a resource when that is genuinely required by program semantics.

That dependency must then be visible and classified as target-specific.

---

147. Compatibility Matrix

Interoperability compatibility MUST be tracked across:

specification
     ↕
interoperability grammar
     ↕
Zamani.g4
     ↕
lexer
     ↕
parser
     ↕
AST
     ↕
semantic analysis
     ↕
canonical semantic model
     ↕
IR
     ↕
compiler
     ↕
runtime
     ↕
foreign interface

A feature is not complete merely because its grammar parses.

---

148. Feature Completion Contract

Each interoperability feature MUST be independently completable.

The feature contract MUST identify:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Security Tests
Hard-Coding Audit
Diagnostics
Performance Considerations
Completion Criteria

No later feature should require reopening a completed feature merely to invent its integration contract.

---

149. Dependency-First Completion Order

The preferred implementation order is:

1. grammar/spec/interoperability.md
        ↓
2. canonical core names/paths/metadata
        ↓
3. canonical types
        ↓
4. canonical expressions
        ↓
5. canonical functions
        ↓
6. modules
        ↓
7. effects
        ↓
8. resources
        ↓
9. abi.g4
        ↓
10. ffi.g4
        ↓
11. foreign-functions.g4
        ↓
12. interoperability.g4
        ↓
13. c.g4
        ↓
14. cpp.g4
        ↓
15. python.g4
        ↓
16. rust.g4
        ↓
17. zig.g4
        ↓
18. openqasm.g4
        ↓
19. verilog.g4
        ↓
20. AssemblyLanguage.g4
        ↓
21. reconciled system-interface grammar
        ↓
22. Zamani.g4
        ↓
23. AST
        ↓
24. structural validation
        ↓
25. semantic analysis
        ↓
26. canonical semantic model
        ↓
27. compiler
        ↓
28. runtime
        ↓
29. conformance

The exact order may be adjusted only when repository inspection demonstrates a different canonical dependency.

---

150. Specialized Grammar Completion

Each specialized grammar is complete only when:

- syntax is defined;
- ownership is explicit;
- duplicate concepts are removed;
- canonical core grammar is reused;
- AST mapping exists;
- semantic mapping exists;
- compatibility is defined;
- diagnostics are defined;
- tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- hard-coding audit passes;
- security audit passes;
- compiler integration is defined;
- runtime integration is defined.

---

151. Interoperability Composition Grammar Completion

"grammar/interoperability/interoperability.g4" is complete only when:

- the filename has no trailing whitespace;
- it is the single composition boundary;
- it does not duplicate core grammar;
- it does not duplicate foreign-language grammar;
- it uses canonical names;
- it uses canonical paths;
- it uses canonical types;
- it uses canonical expressions;
- it uses canonical parameters/arguments;
- it is deterministic;
- it performs no external I/O;
- it performs no target discovery;
- it performs no runtime actions;
- it contains no unsafe dependency;
- it has no arbitrary machine-scale limit;
- it has an AST contract;
- it has a semantic contract;
- it integrates with "Zamani.g4";
- it has compatibility coverage;
- it has positive tests;
- it has negative tests;
- it has boundary tests;
- it has scalability tests;
- it has cross-domain tests.

---

152. "grammar/spec/interoperability.md" Completion Contract

This specification is complete when it establishes all of the following before dependent grammar implementation begins:

- interoperability ownership;
- non-ownership;
- authority;
- composition;
- language identity;
- format identity;
- version identity;
- ABI identity;
- calling conventions;
- foreign functions;
- foreign types;
- foreign values;
- callbacks;
- ownership;
- lifetimes;
- nullability;
- adapters;
- conversions;
- capabilities;
- effects;
- resources;
- linkage;
- symbols;
- imports;
- exports;
- asynchronous boundaries;
- streaming;
- C;
- C++;
- Python;
- Rust;
- Zig;
- OpenQASM;
- HDL;
- assembly;
- system interfaces;
- quantum integration;
- "quantum::ir";
- QEC boundaries;
- ZQN boundaries;
- hardware boundaries;
- distributed boundaries;
- security boundaries;
- AST mapping;
- semantic mapping;
- IR mapping;
- compiler mapping;
- runtime mapping;
- tooling mapping;
- scalability;
- POCO-REAF;
- compatibility;
- diagnostics;
- determinism;
- testing;
- hard-coding audit;
- security audit;
- migration rules.

No later interoperability grammar should need to invent these architectural boundaries.

---

153. Fundamental Invariants

Invariant 1 — Syntax is not execution

Parsing never executes foreign behavior.

Invariant 2 — Language is not hardware

A foreign-language declaration never inherently selects hardware.

Invariant 3 — ABI is not architecture

An ABI declaration does not inherently select one processor.

Invariant 4 — Resource is not allocation

A resource requirement does not allocate a physical resource.

Invariant 5 — Capability is not discovery

A capability declaration does not inspect the target.

Invariant 6 — Foreign syntax is not foreign IR

Foreign syntax does not become a competing canonical IR.

Invariant 7 — OpenQASM is not quantum IR

OpenQASM ultimately lowers through "quantum::ir".

Invariant 8 — HDL syntax is not hardware placement

HDL interoperability does not perform physical implementation.

Invariant 9 — Portability is explicit

Target-specific dependencies must be visible.

Invariant 10 — Scale is not grammar state

Machine scale must never become a grammar constant.

Invariant 11 — External declarations are inert

Creating an AST node never loads or executes an external implementation.

Invariant 12 — Foreign identity is extensible

Adding a new language must not require redesigning the generic interoperability architecture.

Invariant 13 — Canonical AST remains domain-neutral

The AST must not become LLVM/QIR/MLIR/OpenQASM/vendor IR.

Invariant 14 — Canonical quantum semantics remain "quantum::ir"

No interoperability subsystem may replace it.

Invariant 15 — Safe Rust

Interoperability implementation requires no "unsafe".

---

154. Production-Readiness Checklist

The interoperability specification and implementation are production-ready only when all applicable items are satisfied.

Authority

- [ ] "grammar/spec/interoperability.md" is normative.
- [ ] "grammar/interoperability/" is the source-level interoperability grammar family.
- [ ] "grammar/Zamani.g4" is the composition root.
- [ ] "grammar/grammar.md" does not act as an independent authority.
- [ ] "grammar/Zamani-Grammar.md" does not silently introduce syntax.

Structure

- [ ] "interoperability.g4 " with trailing whitespace is removed.
- [ ] "interoperability.g4" is the canonical composition grammar.
- [ ] "SystemInterfaces.g4" and "system-interfaces.g4" are reconciled.
- [ ] Duplicate foreign-call rules are reconciled.
- [ ] Duplicate ABI/FFI concepts are reconciled.

Language model

- [ ] Language identity is extensible.
- [ ] Source format is distinct from language.
- [ ] Version is explicit where required.
- [ ] ABI is distinct from architecture.
- [ ] Calling convention is symbolic.
- [ ] Foreign symbols are representable.
- [ ] Foreign functions are representable.
- [ ] Foreign types are representable.
- [ ] Opaque types are representable.
- [ ] Callbacks are representable.
- [ ] Adapters are representable.
- [ ] Conversions are representable.
- [ ] Ownership is representable.
- [ ] Lifetime is representable.
- [ ] Nullability is representable.
- [ ] Async/streaming boundaries are representable.

Integration

- [ ] Core names are reused.
- [ ] Core paths are reused.
- [ ] Types are reused.
- [ ] Expressions are reused.
- [ ] Function parameter grammar is reused.
- [ ] Module grammar is reused.
- [ ] Effects are reused.
- [ ] Resources are reused.
- [ ] Security is reused.
- [ ] AST integration exists.
- [ ] Semantic integration exists.
- [ ] IR integration exists.
- [ ] Compiler integration exists.
- [ ] Runtime integration exists.
- [ ] Tooling integration exists.

Quantum

- [ ] OpenQASM is treated as interoperability.
- [ ] OpenQASM lowers into "quantum::ir".
- [ ] No second quantum IR exists.
- [ ] No fixed qubit count exists.
- [ ] No physical-qubit mapping is encoded in generic syntax.
- [ ] QEC remains separate.
- [ ] ZQN remains separate.
- [ ] Routing remains separate.
- [ ] Scheduling remains separate.
- [ ] HAL remains separate.

HDL/hardware

- [ ] HDL is a boundary contract.
- [ ] Synthesis is downstream.
- [ ] placement is downstream.
- [ ] routing is downstream.
- [ ] target discovery is downstream.
- [ ] hardware selection is downstream.
- [ ] fixed device IDs are absent from portable syntax.

POCO-REAF

- [ ] Program semantics remain target-independent.
- [ ] Resource requirements are separate from allocation.
- [ ] Capabilities are separate from discovery.
- [ ] Preferences are separate from requirements.
- [ ] Target-specific dependencies are explicit.
- [ ] Source does not require rewriting merely because resources scale.

Safety

- [ ] Rust 1.97 supported.
- [ ] Rust 1.97.1 supported.
- [ ] Rust 2021 supported.
- [ ] "unsafe" forbidden.
- [ ] Parsing performs no filesystem access.
- [ ] Parsing performs no network access.
- [ ] Parsing performs no process execution.
- [ ] Parsing performs no dynamic loading.
- [ ] Parsing performs no hardware discovery.
- [ ] Parsing performs no runtime execution.

Scalability

- [ ] No fixed language count.
- [ ] No fixed interface count.
- [ ] No fixed parameter count.
- [ ] No fixed callback count.
- [ ] No fixed adapter count.
- [ ] No fixed device count.
- [ ] No fixed node count.
- [ ] No fixed qubit count.
- [ ] No fixed memory capacity.
- [ ] No fixed topology size.
- [ ] Practical limits are configurable implementation/resource policies.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Security tests.
- [ ] Compatibility tests.
- [ ] AST tests.
- [ ] Semantic tests.
- [ ] IR tests.
- [ ] Cross-domain tests.
- [ ] Round-trip tests where applicable.
- [ ] Hard-coding audit.
- [ ] Duplicate-rule audit.

---

155. Final Architectural Model

The completed interoperability system is:

                         Zamani Source
                              │
                              ▼
                       Zamani Grammar
                              │
                              ▼
                         Interop Syntax
                              │
                              ▼
                         Zamani AST
                              │
                              ▼
                   Structural Validation
                              │
                              ▼
                    Semantic Interoperability
                              │
          ┌───────────────────┼────────────────────┐
          │                   │                    │
          ▼                   ▼                    ▼
       Language             ABI               Interface
       contract           contract             contract
          │                   │                    │
          └───────────────────┼────────────────────┘
                              │
                              ▼
                    Type / Effect / Capability
                         / Resource Analysis
                              │
                              ▼
                    Canonical Semantic Model
                              │
          ┌───────────────────┼─────────────────────┐
          │                   │                     │
          ▼                   ▼                     ▼
     Classical IR       quantum::ir          HDL/Hardware
          │                   │                     │
          └───────────────────┼─────────────────────┘
                              │
                              ▼
                         Optimization
                              │
              ┌───────────────┼────────────────┐
              │               │                │
              ▼               ▼                ▼
           Routing        Scheduling       Resilience
              │               │                │
              └───────────────┼────────────────┘
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                      Target Realization
                              │
       ┌──────────┬──────────┼───────────┬───────────┐
       ▼          ▼          ▼           ▼           ▼
      CPU        GPU        FPGA         QPU       Future
       │          │          │           │         targets
       └──────────┴──────────┴───────────┴───────────┘

The interoperability subsystem therefore provides a universal boundary without becoming a universal collection of special cases.

The governing rule is:

«Zamani source declares the semantic relationship required between computational domains. It does not permanently encode the machine, device, ABI realization, runtime, topology, or physical resource that happens to implement that relationship today.»

This preserves:

Program_Once
      ↓
Compile_Once
      ↓
Run_Everywhere
      ↓
Run_Anywhere
      ↓
Run_Forever

while allowing the same language to interoperate with classical, quantum, hybrid, HDL, hardware, AI, data, networking, distributed, embedded, accelerator, scientific, security, and future computational systems.

Definition of Done: this specification is the completed normative contract for interoperability. Implementation files are complete only when they conform to this contract and their own file-level integration contracts, rather than when syntax alone parses successfully.
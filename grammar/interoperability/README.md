Zamani Interoperability Grammar

Production Interoperability Architecture

Path

"grammar/interoperability/"

Language

Zamani

Grammar technology

ANTLR-compatible grammar composition

Compiler implementation baseline

- Rust 2021
- Rust 1.97
- Rust 1.97.1
- "unsafe" Rust forbidden

Architectural objective

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

The "grammar/interoperability/" subsystem defines the source-language interoperability boundary of Zamani.

It allows a Zamani program to explicitly describe relationships with other programming languages, foreign functions, foreign types, ABIs, binary interfaces, system interfaces, hardware interfaces, quantum formats, HDL formats, foreign runtimes, and future computational ecosystems.

The subsystem exists so that Zamani can integrate with existing and future computational technologies without making those technologies part of Zamani's permanent semantic core.

The central architectural rule is:

«Zamani source describes the semantic boundary and interoperability intent. Downstream compilation, semantic analysis, linking, deployment, runtime, hardware abstraction, and execution systems determine the concrete realization.»

Interoperability syntax MUST therefore distinguish:

- language identity;
- source-format identity;
- ABI identity;
- interface identity;
- type compatibility;
- calling conventions;
- ownership;
- lifetime;
- conversion;
- capability requirements;
- effect requirements;
- resource requirements;
- portability constraints;
- compatibility requirements;
- linkage intent;
- execution intent;

from implementation-specific facts such as:

- CPU model;
- GPU model;
- QPU model;
- FPGA model;
- ASIC model;
- operating system;
- pointer width;
- register count;
- memory capacity;
- physical address;
- device ID;
- machine topology;
- node count;
- qubit count;
- compiler executable;
- linker executable;
- library path;
- runtime process;
- network location.

---

2. Architectural Position

The interoperability subsystem is upstream of semantic lowering and downstream execution.

The intended architecture is:

Zamani source
     |
     v
canonical lexer
     |
     v
authoritative parser
     |
     v
Zamani AST
     |
     v
interoperability syntax
     |
     v
semantic analysis
     |
     +--------------------+
     |                    |
     v                    v
type analysis       capability/effect analysis
     |                    |
     +---------+----------+
               |
               v
       canonical semantic representation
               |
       +-------+--------+------------------+
       |       |        |                  |
       v       v        v                  v
 classical  quantum   hardware/HDL    distributed/etc.
 IR         ::ir      semantic IR     semantic IR
       |       |        |                  |
       +-------+--------+------------------+
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
                    lowering
                       |
                       v
                 ABI/link/runtime
                       |
                       v
                    execution

The grammar MUST NOT reverse this direction.

Forbidden dependencies include:

grammar
  -> runtime
  -> grammar

grammar
  -> hardware discovery
  -> grammar

grammar
  -> quantum::ir
  -> grammar

grammar
  -> linker
  -> grammar

The grammar establishes syntax.

Semantic infrastructure interprets the syntax.

IR infrastructure represents canonical semantics.

Backends and runtimes realize those semantics.

---

3. Repository Reality and Existing Integration

The repository already contains a specialized interoperability grammar family.

Current files include:

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

The repository currently also contains a filename:

interoperability.g4 

with a trailing space.

That filename is structurally invalid for the intended architecture and MUST be renamed to:

interoperability.g4

The rename MUST be performed explicitly rather than silently creating another file.

The current specialized grammars already express substantial interoperability concepts. For example, the existing C grammar describes C ABI declarations, foreign functions, C-compatible types, callbacks, linkage, ownership, nullability, and ABI attributes.

The current top-level grammar also already contains a "foreignFunctionCall" production.

The older grammar documentation contains another "foreignFunctionCall" form:

foreignFunctionCall:
    'foreign' IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';'

Therefore interoperability work MUST include reconciliation between:

1. "grammar/Zamani.g4";
2. "grammar/Zamani-Grammar.md";
3. "grammar/grammar.md";
4. "grammar/interoperability/interoperability.g4";
5. specialized interoperability grammars;
6. the frontend AST;
7. the parser implementation;
8. semantic analysis.

No interoperability syntax may be considered production-ready merely because ANTLR accepts it.

---

4. Ownership

4.1 This directory owns

"grammar/interoperability/" owns source syntax and composition contracts for:

- interoperability declarations;
- interoperability imports;
- interoperability exports;
- foreign declarations;
- foreign callable references;
- foreign type references;
- foreign value references;
- interface declarations;
- adapters;
- conversions;
- callbacks;
- ABI intent;
- calling-convention intent;
- linkage intent;
- ownership-transfer intent;
- borrowing/lifetime-boundary intent;
- nullability intent;
- compatibility declarations;
- portability requirements;
- capability requirements;
- effect requirements;
- foreign-resource requirements;
- source-format identity;
- language identity;
- foreign interface identity;
- foreign symbol identity;
- interoperability metadata;
- interoperability attributes;
- asynchronous foreign boundaries;
- streaming foreign boundaries;
- cross-language contracts;
- system-interface contracts;
- language-specific interoperability composition.

---

5. Non-Ownership

This directory does NOT own:

- the canonical lexer;
- token definitions;
- ordinary identifiers;
- ordinary qualified names;
- ordinary expressions;
- ordinary types;
- ordinary functions;
- ordinary modules;
- canonical AST implementation;
- semantic type checking;
- ABI layout computation;
- calling-convention implementation;
- symbol resolution;
- library discovery;
- filesystem access;
- network access;
- process execution;
- dynamic loading;
- compiler invocation;
- linker invocation;
- runtime invocation;
- hardware discovery;
- hardware selection;
- QPU selection;
- physical-qubit selection;
- CPU selection;
- GPU selection;
- FPGA selection;
- ASIC selection;
- topology discovery;
- calibration;
- scheduling;
- routing;
- optimization;
- QEC;
- ZQN;
- resilience;
- simulation;
- canonical quantum IR;
- classical IR;
- hardware IR;
- deployment;
- resource allocation.

Those systems consume interoperability semantics.

They do not become dependencies of the grammar.

---

6. POCO-REAF Contract

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

A foreign integration MUST NOT force portable Zamani source to contain temporary characteristics of a particular machine.

For example:

interop language "C"

means:

«This boundary follows a C interoperability contract.»

It does NOT mean:

x86

or:

x86_64

or:

ARM

or:

AArch64

or:

RISC-V

or:

Linux

or:

Windows

or:

macOS

or:

32-bit pointer

or:

64-bit pointer

The target ABI determines those properties later.

Likewise:

interop language "OpenQASM"

MUST NOT imply:

- a fixed QPU;
- a fixed qubit count;
- a fixed topology;
- a fixed gate duration;
- a fixed calibration;
- a fixed physical-qubit mapping.

OpenQASM interoperability MUST eventually lower through the canonical quantum semantic boundary.

---

7. Universal Interoperability Model

Interoperability MUST be modeled as:

Source Program
    |
    v
Boundary Declaration
    |
    +-- language identity
    +-- format identity
    +-- ABI identity
    +-- interface identity
    +-- symbol identity
    +-- type contract
    +-- conversion contract
    +-- ownership contract
    +-- lifetime contract
    +-- capability requirements
    +-- effect requirements
    +-- compatibility requirements
    +-- portability requirements
    |
    v
Semantic Analysis
    |
    v
Canonical Representation
    |
    v
Target-specific realization

These concepts MUST remain distinct.

A language is not an ABI.

An ABI is not a library.

A library is not a device.

A device is not a target.

A target is not a runtime.

A runtime is not a physical machine.

---

8. Language Identity

Language identity MUST be extensible.

Do NOT implement the language universe as a closed enum such as:

C | Cpp | Python | Rust | ...

because that creates a scalability bottleneck.

Instead, language identities MUST support symbolic or qualified names.

Examples include:

C
C++
Rust
Python
Zig
OpenQASM
Verilog
SystemVerilog
Assembly

Future languages MUST be expressible without modifying the fundamental interoperability architecture.

Vendor-specific language extensions SHOULD use qualified namespaces.

Example:

language vendor::language

or an equivalent canonical syntax established by the authoritative grammar.

---

9. Source Format Identity

A foreign source format MUST be distinguishable from the programming language itself.

Examples:

language "C"
source "header"

language "OpenQASM"
source "openqasm3"

language "SystemVerilog"
source "sv"

The source identifier is semantic metadata.

It MUST NOT automatically be interpreted as:

- a filename;
- a filesystem path;
- a URL;
- an executable;
- a network resource.

The grammar is inert.

---

10. ABI Contract

ABI declarations describe interoperability requirements.

They MUST NOT calculate:

- structure layout;
- alignment;
- pointer width;
- register allocation;
- stack layout;
- calling sequence;
- binary encoding.

Those belong to ABI analysis and target lowering.

The grammar may express:

abi c

or another symbolic ABI contract.

It MUST allow future ABI families without requiring grammar redesign.

---

11. Foreign Functions

Foreign functions are declarations of externally implemented callable contracts.

A foreign declaration MUST distinguish:

declaration

from:

implementation

For example:

extern "C" fn external_function(value: SomeType) -> ResultType;

declares an interface.

It does not:

- load the symbol;
- execute the function;
- invoke a C compiler;
- invoke a linker;
- search the filesystem;
- access a library;
- inspect hardware.

The semantic layer MUST verify that the declaration can be safely and correctly realized.

---

12. Foreign Types

Foreign types MUST support:

- named foreign types;
- opaque types;
- handles;
- references;
- pointer-like contracts;
- ownership metadata;
- nullability;
- lifetime intent;
- ABI compatibility;
- conversion boundaries.

The grammar MUST NOT hard-code foreign layout.

For example, an opaque foreign object MUST remain opaque unless downstream semantic information explicitly provides a layout contract.

The grammar MUST NOT assume that:

pointer = 64 bits

or any other finite representation.

---

13. Ownership and Lifetime

Interoperability crosses trust and ownership boundaries.

The grammar must therefore allow semantic intent for:

- borrowed;
- owned;
- transferred;
- returned;
- retained;
- released;
- immutable;
- mutable;
- nullable;
- non-null;
- lifetime-bound;
- callback lifetime;
- resource lifetime.

These are contracts.

They are not implementations.

Actual ownership checking belongs to semantic analysis.

Actual allocation/deallocation belongs to the appropriate runtime or backend.

---

14. Calling Conventions

Calling conventions MUST be symbolic and extensible.

The grammar MUST NOT assume that a calling convention corresponds to one architecture.

Examples may include:

c
system
default

and extensible qualified identifiers.

Target-specific interpretation belongs to ABI lowering.

The grammar MUST NOT encode:

- register names;
- stack slots;
- stack alignment;
- machine instruction sequences;
- argument registers;
- return registers.

---

15. Variadic Interoperability

Variadic foreign calls MUST be representable without an arbitrary maximum argument count.

The grammar MUST support the semantic concept of variadicity.

It MUST NOT impose rules such as:

maximum 8 arguments

or:

maximum 16 arguments

because such values are ABI/runtime properties rather than language-scale limits.

Where the foreign ABI requires:

- sentinel information;
- promotion rules;
- format contracts;
- validation requirements;

those MUST be representable as metadata or semantic contracts.

---

16. Callbacks

Callbacks are foreign callable contracts crossing the interoperability boundary.

The grammar MUST support:

- callback identity;
- callback parameter contracts;
- callback result contracts;
- calling convention;
- ownership;
- lifetime;
- nullability;
- capability requirements;
- effect requirements.

The grammar MUST NOT expose a concrete machine function-pointer representation.

For example, a callback is not inherently:

usize

or:

pointer

or:

64-bit address

The representation is target-dependent.

---

17. Adapters

Adapters describe semantic transformations between interoperability contracts.

An adapter may conceptually express:

adapter Name from InterfaceA to InterfaceB

An adapter declaration is not an implementation.

The grammar does not decide whether the transformation is implemented through:

- generated code;
- wrapper functions;
- marshaling;
- serialization;
- runtime dispatch;
- compiler lowering;
- hardware bridges;
- quantum-classical bridges.

Those are downstream decisions.

---

18. Conversions

Conversions MUST be explicit enough for semantic analysis to distinguish:

- identity conversion;
- representation conversion;
- ownership conversion;
- borrowing conversion;
- numeric conversion;
- serialization;
- deserialization;
- ABI conversion;
- language conversion;
- quantum/classical conversion;
- hardware/software conversion.

The grammar MUST NOT silently assume that two foreign types are interchangeable merely because they have similar names.

---

19. Capability Integration

Interoperability MAY require capabilities.

Examples:

requires capability::ffi

requires capability::network

requires capability::quantum

requires capability::hardware

The grammar describes the requirement.

The capability checker determines whether the target environment satisfies it.

The grammar MUST NOT discover capabilities.

The grammar MUST NOT select devices.

The grammar MUST NOT embed a fixed capability inventory.

---

20. Effect Integration

Foreign calls may have effects.

Examples include:

- I/O;
- network access;
- hardware access;
- process interaction;
- external state mutation;
- asynchronous execution;
- quantum execution;
- system calls.

Interoperability syntax may reference effects.

It MUST NOT implement the effect system.

The canonical effect grammar owns effect declarations.

Interoperability only establishes its boundary requirements.

---

21. Resource Integration

Foreign interfaces MAY require resources.

Examples:

resource::memory
resource::network
resource::accelerator
resource::quantum

The grammar MUST distinguish:

requirement
constraint
preference
hint
capability
resource

These concepts MUST NOT collapse into one mechanism.

For example:

requires quantum

does not mean:

requires device "specific-device"

and:

prefers accelerator

does not mean:

use accelerator 0

---

22. C Integration

"c.g4" owns C-specific interoperability syntax.

The composition grammar MUST consume its declarations through a stable integration boundary.

C grammar MUST NOT own:

- the complete C language;
- C preprocessing;
- target ABI implementation;
- linking;
- loading;
- runtime execution.

The current C grammar already establishes this separation and defines C interoperability concepts such as C ABI declarations, foreign functions, callbacks, types, ownership, nullability, linkage, and attributes.

Any C syntax duplicated in "interoperability.g4" MUST be removed or deliberately restricted to composition-level syntax.

---

23. C++ Integration

"cpp.g4" owns C++ interoperability-specific syntax.

It MUST integrate with the common interoperability contract rather than duplicate:

- ordinary type syntax;
- ordinary function syntax;
- ABI implementation;
- linker behavior;
- runtime behavior.

C++ ABI details remain downstream.

C++ interoperability MUST remain target-independent at the source grammar level.

---

24. Python Integration

"python.g4" owns Python-specific interoperability syntax.

It MUST distinguish:

Python language identity

from:

Python interpreter implementation

and:

specific Python runtime

The grammar MUST NOT encode:

- a Python installation path;
- a fixed interpreter;
- a fixed operating system;
- a fixed CPython version unless explicitly expressed as a compatibility requirement;
- a fixed Python executable.

---

25. Rust Integration

"rust.g4" owns Rust-specific interoperability syntax.

Zamani's compiler implementation itself remains:

Rust 1.97 / Rust 1.97.1
Rust 2021
unsafe forbidden

This does not mean the Zamani source language is restricted to Rust interoperability.

Rust interoperability MUST remain one foreign-language contract among many.

No grammar rule may assume that Rust is the implementation language of every foreign interface.

---

26. Zig Integration

"zig.g4" provides Zig-specific interoperability syntax.

Zig must be treated as an external language contract, not as a machine target.

Compiler version, ABI, platform, linker, and runtime decisions remain downstream.

---

27. Assembly Integration

"AssemblyLanguage.g4" owns assembly-language interoperability.

Assembly integration is inherently target-sensitive, but the Zamani interoperability boundary MUST distinguish:

assembly language family

from:

specific instruction set

and:

specific processor

Target-specific assembly constraints must not leak into the universal interoperability root.

Assembly source may therefore be explicitly marked as target-specific when required, but target specificity must be represented as semantic metadata rather than hidden in grammar assumptions.

---

28. OpenQASM Integration

"openqasm.g4" owns OpenQASM-specific syntax.

Interoperability MUST treat OpenQASM as a quantum-language/format boundary.

The integration path is:

Zamani
   |
   v
OpenQASM interoperability boundary
   |
   v
OpenQASM frontend
   |
   v
canonical quantum::ir

The interoperability grammar MUST NOT define another quantum IR.

It MUST NOT own:

- quantum gates;
- quantum circuit semantics;
- physical qubit mapping;
- routing;
- scheduling;
- calibration;
- QEC;
- ZQN.

Those belong to the quantum subsystem and downstream compilation layers.

---

29. Verilog / HDL Integration

"verilog.g4" owns Verilog-specific interoperability.

HDL interoperability MUST distinguish:

HDL interface contract

from:

hardware implementation

The grammar may express:

- HDL module identity;
- interface identity;
- port binding;
- compatibility;
- import/export intent;
- target requirements.

It MUST NOT perform:

- synthesis;
- place-and-route;
- timing analysis;
- FPGA discovery;
- ASIC selection;
- clock-tree implementation;
- physical placement.

Those belong to HDL/hardware compilation infrastructure.

---

30. System Interfaces

"system-interfaces.g4" and "SystemInterfaces.g4" MUST be reconciled.

There must ultimately be one authoritative ownership model.

System-interface syntax may describe:

- operating-system interfaces;
- system calls;
- process boundaries;
- device interfaces;
- IPC boundaries;
- platform services.

It MUST NOT itself execute system calls.

It MUST NOT open files or sockets during parsing.

It MUST NOT discover operating-system capabilities during parsing.

---

31. ABI Grammar

"abi.g4" owns reusable ABI syntax.

"interoperability.g4" owns the composition relationship.

The architecture is:

interoperability.g4
       |
       v
ABI contract
       |
       v
abi.g4
       |
       v
semantic ABI analysis
       |
       v
target-specific ABI lowering

There must be no reverse dependency from ABI implementation into the grammar composition root.

---

32. FFI Grammar

"ffi.g4" owns reusable FFI syntax.

The interoperability root MUST compose FFI declarations rather than duplicate every FFI construct.

The FFI subsystem MUST remain language-neutral.

Language-specific extensions belong in:

c.g4
cpp.g4
python.g4
rust.g4
zig.g4
...

---

33. Foreign Functions Grammar

"foreign-functions.g4" owns generic foreign-function constructs.

The architecture MUST avoid having:

foreign-functions.g4
ffi.g4
interoperability.g4
c.g4

all define the same foreign-call production independently.

There must be one conceptual ownership path.

Recommended ownership:

foreign-functions.g4
    -> generic foreign callable contracts

ffi.g4
    -> generic FFI boundary

abi.g4
    -> ABI contracts

c.g4
    -> C specialization

cpp.g4
    -> C++ specialization

...

"interoperability.g4" composes them.

---

34. No Duplicate Core Grammar

Interoperability grammars MUST reuse canonical definitions for:

- identifiers;
- qualified names;
- paths;
- expressions;
- types;
- parameters;
- arguments;
- attributes;
- annotations;
- generics.

They MUST NOT redefine these concepts.

Canonical ownership remains with the corresponding core grammar files.

This prevents grammar drift and circular dependencies.

---

35. AST Contract

The interoperability parser layer MUST lower into AST concepts equivalent to:

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

These are AST concepts.

They are NOT IR types.

The AST MUST retain source spans and enough structured information for diagnostics.

---

36. Semantic Contract

Semantic analysis MUST validate at least:

- language identity;
- source-format compatibility;
- ABI compatibility;
- function signature compatibility;
- parameter compatibility;
- result compatibility;
- type compatibility;
- conversion legality;
- ownership;
- lifetime;
- nullability;
- callback compatibility;
- linkage;
- visibility;
- capability requirements;
- effect requirements;
- resource requirements;
- portability;
- version compatibility;
- foreign-boundary safety;
- target availability.

A syntactically valid interoperability declaration MUST NOT automatically be considered executable.

---

37. Canonical IR Contract

Interoperability MUST NOT become an IR.

The correct flow is:

Interoperability AST
        |
        v
semantic validation
        |
        v
canonical semantic representation
        |
        +--> classical IR
        |
        +--> quantum::ir
        |
        +--> hardware/HDL representation
        |
        +--> distributed representation
        |
        +--> runtime boundary representation

A foreign declaration MUST therefore not create a private:

ForeignQuantumIR
ForeignHardwareIR
ForeignCIR
ForeignPythonIR

inside the grammar.

---

38. Quantum Boundary

For quantum interoperability:

grammar/interoperability/openqasm.g4

may parse OpenQASM-specific source constructs.

Semantic lowering MUST ultimately use:

quantum::ir

as the canonical quantum semantic boundary.

Interoperability MUST NOT duplicate:

- "QubitId";
- gate representations;
- circuit representations;
- quantum operation representations;
- QEC representations;
- ZQN representations.

---

39. QEC Integration

QEC is not owned by interoperability.

If a foreign quantum API exposes QEC operations, interoperability may describe the external interface.

Semantic lowering then maps the resulting computation into the appropriate quantum/QEC subsystem.

Interoperability MUST NOT implement:

- syndrome extraction;
- decoding;
- correction;
- code selection;
- distance;
- recovery;
- QEC scheduling.

---

40. ZQN Integration

ZQN describes quantum noise/fault semantics.

Interoperability may describe a foreign interface that exposes noise or fault information.

It MUST NOT duplicate ZQN's:

- fault model;
- channel model;
- correlated fault model;
- leakage semantics;
- loss semantics;
- erasure semantics.

A foreign quantum interface may provide data that is adapted into ZQN downstream.

---

41. Optimization Integration

Interoperability does not optimize foreign calls.

The optimizer may later:

- inline;
- specialize;
- eliminate;
- fuse;
- reorder where semantically legal;
- lower;
- replace;
- vectorize;
- transform.

The grammar must remain independent of those implementation decisions.

---

42. Scheduling Integration

Interoperability does not schedule.

A foreign call may impose semantic or resource constraints.

Those constraints are passed to scheduling.

The scheduler determines:

- ordering;
- timing;
- resource usage;
- concurrency;
- synchronization;
- execution placement.

No fixed timing or resource count may be encoded in interoperability grammar merely because one foreign implementation currently uses it.

---

43. Hardware Integration

Interoperability may express a hardware interface contract.

It must not select:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- physical device.

Hardware abstraction determines available targets.

Compilation determines realization.

Runtime determines deployment.

---

44. Security Boundary

Foreign interoperability is an explicit trust boundary.

Parsing MUST be completely inert.

Parsing MUST NOT:

- execute foreign code;
- execute compiler commands;
- load shared libraries;
- inspect the filesystem;
- access a network;
- invoke a process;
- access environment variables;
- inspect hardware;
- dereference addresses;
- create runtime handles.

The grammar MUST contain no semantic actions that perform external I/O.

---

45. Rust Safety

The Rust implementation surrounding this grammar MUST compile under:

Rust 1.97

and:

Rust 1.97.1

with:

Rust 2021

and:

#![forbid(unsafe_code)]

or an equivalent repository-level enforcement mechanism.

No interoperability feature may require Rust "unsafe".

External unsafe operations, where unavoidable in a backend, must remain behind an explicitly owned and audited downstream boundary.

The grammar itself must never require unsafe Rust.

---

46. Scalability Contract

The grammar MUST NOT impose arbitrary limits on:

- number of interoperability declarations;
- number of foreign functions;
- number of parameters;
- number of callbacks;
- number of interfaces;
- number of adapters;
- number of conversions;
- number of languages;
- number of source formats;
- number of capabilities;
- number of effects;
- number of resources;
- number of devices;
- number of nodes;
- number of cores;
- number of threads;
- number of qubits;
- amount of memory;
- number of foreign modules.

There must be no:

MAX_FOREIGN_FUNCTIONS
MAX_INTERFACES
MAX_DEVICES
MAX_LANGUAGES
MAX_PARAMETERS

inside the grammar.

Practical limits are allowed to arise from:

- available memory;
- parser implementation;
- compiler resources;
- operating-system limits;
- runtime limits;
- target capabilities.

Those are implementation/environment constraints, not language semantics.

---

47. Extensibility

New interoperability domains MUST be addable without redesigning the root.

For example, future files may include:

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
...

The root MUST NOT require an exhaustive enumeration of all possible languages.

New domains should register through the interoperability composition boundary.

---

48. Versioning

Interoperability declarations MAY specify compatibility/version constraints.

Version constraints MUST be symbolic and semantic.

The grammar MUST NOT silently interpret:

language "X"

as:

language X version current

when the semantic meaning requires explicit compatibility.

Version compatibility belongs to semantic analysis and compatibility infrastructure.

---

49. Target Independence

A portable interoperability declaration should remain portable whenever the external contract itself is portable.

The grammar MUST NOT automatically bind:

language

to:

target

or:

ABI

to:

device

or:

library

to:

path

or:

resource

to:

fixed quantity

---

50. Target-Specific Interoperability

Target-specific interoperability is permitted when it is genuinely part of program semantics.

It MUST be represented explicitly.

For example:

requires target capability ...

or an equivalent resource/constraint mechanism.

It must never be hidden in a supposedly portable declaration.

A source program that intentionally depends on a particular target is semantically different from a POCO-REAF portable program.

That distinction must remain visible.

---

51. Import and Export Semantics

Interoperability imports and exports describe semantic boundaries.

They do not themselves:

- open files;
- fetch network resources;
- load libraries;
- resolve symbols.

Actual resolution is downstream.

Import source identifiers should therefore be treated as identifiers/references unless an explicit compiler/tooling operation later resolves them.

---

52. Linkage

Linkage declarations may express:

- static;
- dynamic;
- system;
- runtime;
- weak;
- strong;
- framework;
- custom linkage classes.

The grammar MUST NOT implement linking.

Linker selection and binary resolution remain compiler/toolchain responsibilities.

---

53. Symbol Identity

External symbol names may differ from Zamani names.

The grammar should support an explicit semantic relationship between:

Zamani name

and:

foreign symbol name

This allows:

Zamani identifier
    !=
foreign ABI symbol

without requiring illegal identifier spellings in the Zamani language.

---

54. Error Model

Interoperability syntax errors MUST be distinguishable from semantic interoperability errors.

Examples:

Syntax

missing function parameter

Semantic

foreign parameter type incompatible with ABI

Capability

required capability unavailable

Compatibility

foreign interface version incompatible

Security

foreign boundary violates required trust policy

Runtime

symbol unavailable at execution time

The grammar owns syntax diagnostics.

Semantic and runtime layers own their respective diagnostics.

---

55. Diagnostics

Interoperability diagnostics MUST preserve:

- source span;
- declaration identity;
- foreign language;
- interface identity;
- symbol identity;
- expected contract;
- actual contract;
- reason;
- severity;
- stable diagnostic code.

Diagnostics MUST NOT depend on machine discovery during parsing.

---

56. Determinism

Parsing must be deterministic.

The interoperability grammar MUST contain no:

- random decisions;
- filesystem-dependent parsing;
- network-dependent parsing;
- runtime-dependent parsing;
- hardware-dependent parsing;
- semantic actions that inspect external state.

The same source and grammar version MUST produce the same parse structure.

---

57. Compatibility With Existing Zamani Syntax

Existing valid interoperability syntax MUST be preserved or migrated explicitly.

The current repository contains legacy forms including:

extern ...

and:

foreign ...

The top-level grammar currently references "foreignFunctionCall".

The older grammar documentation separately specifies:

foreignFunctionCall:
    'foreign' IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';'

Therefore migration MUST proceed through:

inventory
    ↓
consumer analysis
    ↓
semantic comparison
    ↓
canonical form
    ↓
compatibility aliases
    ↓
deprecation
    ↓
eventual removal

No syntax should be silently removed.

---

58. Canonical Composition Contract

"interoperability.g4" is the composition boundary.

It should expose reusable rules conceptually equivalent to:

interopItem
interopDeclaration
interopImportDeclaration
interopExportDeclaration
interopBindingDeclaration
interopCallStatement
interopInterfaceDeclaration
interopForeignDeclaration
interopTypeDeclaration
interopValueDeclaration
interopFunctionDeclaration
interopCallbackDeclaration
interopAdapterDeclaration
interopConversionDeclaration
interopCapabilityDeclaration
interopEffectDeclaration
interopRequirementDeclaration
interopCompatibilityDeclaration
interopLinkageDeclaration
interopResourceDeclaration

Specialized grammars contribute their language-specific contracts.

The root composes them.

---

59. Specialized Grammar Ownership Matrix

File| Ownership
"interoperability.g4"| Composition/root interoperability boundary
"foreign-functions.g4"| Generic foreign callable declarations
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
"system-interfaces.g4"| Generic system interfaces
"SystemInterfaces.g4"| Must be reconciled with "system-interfaces.g4"

There MUST ultimately be one authoritative owner for each semantic grammar concept.

---

60. Required File Corrections

The interoperability directory requires the following structural cleanup.

60.1 Rename

Rename:

interoperability.g4 

to:

interoperability.g4

The trailing-space filename must not remain.

60.2 Reconcile duplicate system-interface grammars

Reconcile:

SystemInterfaces.g4
system-interfaces.g4

into one authoritative implementation plus compatibility documentation where required.

60.3 Reconcile duplicate interoperability concepts

Audit:

ffi.g4
foreign-functions.g4
abi.g4
c.g4
cpp.g4
interoperability.g4

for duplicated rules.

60.4 Reconcile top-level grammar

Audit "grammar/Zamani.g4" so interoperability declarations are exposed through the canonical composition path rather than through disconnected duplicate rules.

---

61. Integration With "Zamani.g4"

The top-level grammar should consume interoperability through a single authoritative entry point.

Conceptually:

declaration
    |
    +--> interopItem

or an equivalent composition structure.

It MUST NOT independently reproduce:

foreignFunctionCall

in several unrelated forms.

The canonical interoperability root must own interoperability syntax.

---

62. Integration With Core Grammar

Interoperability depends on:

core/names.g4
core/paths.g4
core/qualified-names.g4
core/attributes.g4
core/metadata.g4
core/versioning.g4
core/capabilities.g4
core/requirements.g4
core/constraints.g4
core/hints.g4

where those files exist as canonical owners.

Interoperability MUST consume them.

It must not recreate their definitions.

---

63. Integration With Types

Interoperability consumes canonical:

typeExpr

Foreign types are boundary declarations around canonical type references.

It MUST NOT create a competing universal type system.

Foreign-specific type semantics are handled by semantic analysis.

---

64. Integration With Expressions

Foreign calls consume the canonical expression and argument grammar.

Interoperability MUST NOT create a second argument-expression grammar.

This ensures:

ordinary call
foreign call
callback
adapter
conversion

can all share compatible expression semantics.

---

65. Integration With Modules

Foreign interfaces may belong to modules.

Module ownership remains with:

grammar/modules/

Interoperability only supplies declarations that can appear within the module system.

---

66. Integration With Effects

Foreign calls may declare effects.

Effects remain owned by:

grammar/effects/

Interoperability references effect contracts.

It does not redefine the effect model.

---

67. Integration With Resources

Resource requirements remain owned by:

grammar/resources/

Interoperability references them.

It must not create a foreign-specific resource model.

---

68. Integration With Security

Security contracts remain owned by:

grammar/security/

Interoperability may declare that an external boundary requires a security capability or trust property.

It does not implement:

- authentication;
- authorization;
- cryptography;
- key management;
- identity management.

---

69. Integration With Runtime

Runtime integration occurs only after semantic lowering.

The grammar MUST NOT directly depend on runtime code.

The intended flow is:

interop AST
    ↓
semantic validation
    ↓
canonical representation
    ↓
compiler lowering
    ↓
runtime boundary

---

70. Integration With Compiler

The compiler consumes interoperability semantics to determine:

- lowering;
- symbol resolution;
- ABI realization;
- linking;
- marshaling;
- adapter generation;
- target compatibility;
- runtime dispatch.

The compiler MUST NOT modify grammar ownership.

---

71. Integration With Tooling

Tooling should be able to use interoperability metadata for:

- completion;
- navigation;
- diagnostics;
- documentation;
- symbol inspection;
- dependency visualization;
- compatibility analysis;
- foreign interface inspection.

Tooling MUST NOT execute foreign code simply because a source file is parsed.

---

72. Testing Contract

Interoperability tests MUST include:

Positive

- generic foreign declarations;
- C;
- C++;
- Python;
- Rust;
- Zig;
- OpenQASM;
- Verilog;
- assembly;
- system interfaces;
- callbacks;
- adapters;
- conversions;
- ABI declarations;
- linkage;
- ownership;
- nullability;
- capabilities;
- effects;
- requirements.

Negative

- malformed ABI declarations;
- invalid function contracts;
- invalid parameter types;
- malformed callbacks;
- malformed adapters;
- invalid conversions;
- invalid ownership combinations;
- invalid lifetime combinations;
- malformed linkage;
- malformed imports;
- invalid compatibility expressions.

Boundary

Test:

- zero foreign declarations;
- one declaration;
- many declarations;
- deeply nested interfaces;
- long qualified names;
- large parameter lists;
- large adapter graphs;
- large cross-language programs.

Tests MUST NOT use artificial limits as proof of scalability.

---

73. Cross-Domain Tests

Mandatory integration examples include:

classical + C

classical + C++

classical + Python

classical + Rust

quantum + OpenQASM

quantum + C

quantum + Python

quantum + hardware

quantum + HDL

classical + quantum + foreign runtime

classical + quantum + HDL + hardware + distributed

The final cross-domain tests must verify that interoperability does not create independent semantic models.

---

74. Quantum Semantic Preservation Tests

A Zamani program using OpenQASM interoperability must preserve its quantum meaning through:

source
 ↓
parser
 ↓
AST
 ↓
semantic analysis
 ↓
quantum::ir

The test must verify that interoperability has not introduced:

- duplicate gate semantics;
- duplicate qubit IDs;
- duplicate circuit representations;
- fixed qubit limits;
- target-specific mappings.

---

75. HDL Semantic Preservation Tests

HDL interoperability must preserve:

module
ports
signals
timing semantics
hardware interfaces

without accidentally embedding:

specific FPGA
specific ASIC
specific device
specific physical pin

unless explicitly requested by a target-specific semantic contract.

---

76. POCO-REAF Scalability Tests

The same semantic source must be testable against different hypothetical resource contexts.

For example:

small classical target

large multicore target

accelerator target

quantum target

hybrid target

distributed target

The source interoperability declaration must not need to change merely because the available resource scale changes.

---

77. Determinism Tests

Parsing the same interoperability source repeatedly MUST produce equivalent parse trees/ASTs.

Tests must prove that parser output does not depend on:

- filesystem state;
- network state;
- environment variables;
- device availability;
- runtime availability;
- library availability;
- compiler availability.

---

78. Round-Trip Tests

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

must preserve interoperability semantics.

Formatting may differ.

Semantic meaning must not.

---

79. Hard-Coding Audit

Every interoperability grammar file MUST be audited for:

- fixed device IDs;
- fixed ABI widths;
- fixed pointer widths;
- fixed architecture names;
- fixed machine counts;
- fixed accelerator counts;
- fixed qubit counts;
- fixed node counts;
- fixed memory sizes;
- fixed topology;
- fixed register counts;
- fixed library paths;
- fixed executable paths;
- fixed operating systems;
- fixed compiler versions where not semantically required.

Each occurrence must be classified as:

1. language semantic requirement;
2. interoperability requirement;
3. target-specific requirement;
4. resource constraint;
5. implementation limitation;
6. test-only limitation;
7. documentation-only limitation;
8. accidental hard-coding.

Accidental hard-coding MUST be removed.

---

80. Security Audit

The interoperability grammar MUST pass an audit proving that parsing cannot:

- execute arbitrary code;
- invoke foreign functions;
- access the network;
- access the filesystem;
- execute a compiler;
- execute a linker;
- load a library;
- inspect environment variables;
- inspect hardware;
- mutate runtime state.

ANTLR semantic actions that violate this boundary MUST NOT be used.

---

81. Dependency Audit

The interoperability grammar MUST depend only on upstream syntax contracts.

Allowed conceptual dependencies:

lexer
core names
core paths
core metadata
core versioning
types
expressions
functions
modules
effects
resources
security metadata

Forbidden conceptual dependencies:

runtime implementation
hardware implementation
compiler implementation
linker implementation
scheduler
router
optimizer
QEC implementation
ZQN implementation
resilience implementation

---

82. Completion Contract for "interoperability/README.md"

This README is complete when it establishes:

- ownership;
- non-ownership;
- composition;
- specialized grammar responsibilities;
- AST expectations;
- semantic boundaries;
- IR boundaries;
- compiler integration;
- runtime integration;
- quantum integration;
- HDL integration;
- resource integration;
- capability integration;
- effect integration;
- security boundaries;
- scalability;
- POCO-REAF;
- deterministic parsing;
- testing;
- compatibility;
- hard-coding rules;
- migration requirements;
- duplicate-file reconciliation.

No subsequent grammar file should need to redefine these architectural boundaries.

---

83. Completion Contract for "interoperability.g4"

"interoperability.g4" itself is complete only when:

1. It has no trailing-space filename.
2. It composes the canonical interoperability declarations.
3. It does not duplicate core grammar.
4. It does not duplicate specialized language grammar.
5. It uses canonical identifier/path/type/expression rules.
6. It is deterministic.
7. It contains no external I/O.
8. It contains no target discovery.
9. It contains no runtime actions.
10. It contains no unsafe implementation dependency.
11. It imposes no arbitrary machine-scale limits.
12. It has parser tests.
13. It has negative tests.
14. It has boundary tests.
15. It has cross-domain tests.
16. It has round-trip tests where applicable.
17. Its AST contract is implemented.
18. Its semantic contract is implemented.
19. Its integration with "Zamani.g4" is complete.
20. Its compatibility path for existing foreign syntax is documented.

---

84. Completion Contract for Specialized Files

Each specialized grammar is independently complete only when it declares:

Purpose
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
Tests
Negative Tests
Boundary Tests
Compatibility Requirements
Scalability Requirements
Hard-Coding Audit
Completion Criteria

This prevents a later grammar from forcing fundamental redesign of an earlier completed grammar.

---

85. Implementation Order

The interoperability subsystem MUST be implemented dependency-first.

Recommended order:

1. grammar/interoperability/README.md
       |
       v
2. core/names.g4
       |
       v
3. core/paths.g4
       |
       v
4. core/qualified-names.g4
       |
       v
5. core/metadata.g4
       |
       v
6. core/versioning.g4
       |
       v
7. core/capabilities.g4
       |
       v
8. core/requirements.g4
       |
       v
9. types/
       |
       v
10. expressions/
       |
       v
11. functions/
       |
       v
12. modules/
       |
       v
13. effects/
       |
       v
14. resources/
       |
       v
15. interoperability/abi.g4
       |
       v
16. interoperability/ffi.g4
       |
       v
17. interoperability/foreign-functions.g4
       |
       v
18. interoperability/interoperability.g4
       |
       +-------------------+
       |        |          |
       v        v          v
      c.g4   cpp.g4    python.g4
       |
       +---- rust.g4
       |
       +---- zig.g4
       |
       +---- openqasm.g4
       |
       +---- verilog.g4
       |
       +---- AssemblyLanguage.g4
       |
       +---- system-interfaces.g4
       |
       v
19. Zamani.g4 integration
       |
       v
20. AST integration
       |
       v
21. semantic integration
       |
       v
22. compiler integration
       |
       v
23. runtime integration
       |
       v
24. complete cross-domain validation

The exact dependency order must be adjusted if repository inspection reveals an earlier canonical owner.

---

86. Required Migration of Existing "foreignFunctionCall"

The existing top-level "foreignFunctionCall" must not remain as an isolated duplicate indefinitely.

The migration target is:

Zamani.g4
     |
     v
interopItem
     |
     v
interop foreign-function contract

The AST must normalize legacy and canonical forms into the same semantic representation where their semantics are equivalent.

This is essential because the repository currently contains multiple historical definitions of foreign-call syntax.

---

87. Required Integration With "grammar/grammar.md"

"grammar/grammar.md" currently describes the grammar accepted by the parser implementation and includes a legacy foreign-function form.

After interoperability integration:

"grammar/grammar.md" MUST either:

1. become a generated/derived reference; or
2. explicitly state that it is a compatibility/reference document.

It MUST NOT silently remain a second authoritative grammar.

---

88. Required Integration With "Zamani-Grammar.md"

"Zamani-Grammar.md" must be reconciled with the authoritative grammar source.

Any foreign interoperability syntax described there must be:

- implemented;
- explicitly planned;
- deprecated;
- or removed from the normative specification.

There must be no documentation-only language feature presented as production syntax.

---

89. Future-Proofing

The interoperability root MUST be able to accommodate future computational domains including:

- new programming languages;
- new ABIs;
- new binary formats;
- new accelerator interfaces;
- new quantum formats;
- new HDL formats;
- new distributed runtimes;
- new AI runtimes;
- new operating systems;
- new execution models;
- new memory models;
- new hardware models;
- future computing paradigms not yet known.

The root must therefore prefer:

symbolic extension
qualified namespace
versioned contract
capability requirement
semantic adapter

over:

closed enum
fixed list
fixed machine model
fixed device list

---

90. Fundamental Invariants

The interoperability subsystem MUST preserve these invariants.

Invariant 1 — Syntax is not execution

Parsing never executes foreign behavior.

Invariant 2 — Language is not hardware

A foreign language declaration does not select a machine.

Invariant 3 — ABI is not architecture

An ABI contract does not inherently select one processor architecture.

Invariant 4 — Resource is not allocation

A resource requirement does not allocate a physical resource.

Invariant 5 — Capability is not discovery

A capability declaration does not inspect the target.

Invariant 6 — Foreign syntax is not foreign IR

Interoperability AST nodes do not replace canonical IR.

Invariant 7 — OpenQASM is not quantum IR

OpenQASM must eventually lower into "quantum::ir".

Invariant 8 — HDL syntax is not hardware placement

HDL interoperability does not perform synthesis or physical implementation.

Invariant 9 — Portability is explicit

Target-specific dependencies must be visible.

Invariant 10 — Scale is not grammar state

Machine scale must never become a grammar constant.

---

91. Production-Readiness Checklist

The interoperability subsystem is production-ready only when all of the following are true:

- [ ] One authoritative interoperability composition grammar exists.
- [ ] The trailing-space "interoperability.g4 " filename is removed.
- [ ] Duplicate system-interface grammars are reconciled.
- [ ] "Zamani.g4" uses the canonical interoperability boundary.
- [ ] Legacy foreign syntax is migrated deliberately.
- [ ] "grammar.md" is reconciled.
- [ ] "Zamani-Grammar.md" is reconciled.
- [ ] "README.md" documents ownership.
- [ ] Specialized grammars have explicit ownership.
- [ ] Core grammar is not duplicated.
- [ ] Types are not duplicated.
- [ ] Expressions are not duplicated.
- [ ] AST contracts exist.
- [ ] Semantic contracts exist.
- [ ] ABI contracts exist.
- [ ] FFI contracts exist.
- [ ] Calling conventions are extensible.
- [ ] Foreign symbols are representable.
- [ ] Foreign types are representable.
- [ ] Opaque types are representable.
- [ ] Callbacks are representable.
- [ ] Adapters are representable.
- [ ] Conversions are representable.
- [ ] Ownership is representable.
- [ ] Lifetime intent is representable.
- [ ] Nullability is representable.
- [ ] Capability requirements are representable.
- [ ] Effect requirements are representable.
- [ ] Resource requirements are representable.
- [ ] Compatibility requirements are representable.
- [ ] OpenQASM integration preserves "quantum::ir".
- [ ] HDL integration remains separate from hardware realization.
- [ ] QEC is not duplicated.
- [ ] ZQN is not duplicated.
- [ ] Scheduling is not duplicated.
- [ ] Routing is not duplicated.
- [ ] Optimization is not duplicated.
- [ ] Hardware discovery is not performed by parsing.
- [ ] Runtime execution is not performed by parsing.
- [ ] Filesystem access is absent from grammar actions.
- [ ] Network access is absent from grammar actions.
- [ ] Rust "unsafe" is not required.
- [ ] Rust 1.97 is supported.
- [ ] Rust 1.97.1 is supported.
- [ ] Rust 2021 is supported.
- [ ] No arbitrary machine limits exist.
- [ ] No fixed qubit limits exist.
- [ ] No fixed device limits exist.
- [ ] No fixed CPU/GPU/FPGA limits exist.
- [ ] No fixed topology exists in portable syntax.
- [ ] No fixed pointer width exists in generic syntax.
- [ ] No fixed memory size exists in generic syntax.
- [ ] Deterministic parsing is verified.
- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Cross-domain tests exist.
- [ ] Round-trip tests exist where applicable.
- [ ] Hard-coding audit passes.
- [ ] Security audit passes.
- [ ] Documentation and grammar agree.
- [ ] AST and grammar agree.
- [ ] Semantic analysis and AST agree.
- [ ] Compiler and semantic contracts agree.
- [ ] Runtime integration occurs only downstream.
- [ ] Future interoperability extensions do not require redesign of the root.

---

92. Final Architectural Principle

The interoperability subsystem exists to make Zamani more interoperable without making Zamani less universal.

The governing principle is:

«Zamani describes what computation and interoperability relationship is required; the compiler, semantic system, resource system, hardware abstraction, runtime, and deployment system determine how that relationship is realized.»

Therefore:

One Zamani program
        |
        v
One stable semantic meaning
        |
        +-------------------+
        |                   |
        v                   v
 classical             quantum
        |                   |
        +---------+---------+
                  |
                  v
             hybrid
                  |
        +---------+----------+
        |         |          |
        v         v          v
       HDL     hardware   distributed
        |         |          |
        +---------+----------+
                  |
                  v
             many targets
                  |
                  v
             many scales
                  |
                  v
          many environments
                  |
                  v
          future architectures

Interoperability is therefore a boundary, not a collection of special cases.

The ultimate invariant is:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

while preserving:

semantic stability
hardware independence
language interoperability
quantum/classical interoperability
HDL interoperability
resource independence
deterministic parsing
strong diagnostics
security
compatibility
extensibility
scalability
clear ownership
canonical IR boundaries

This is the interoperability foundation required for:

Zamani — From Atom to Everywhere.
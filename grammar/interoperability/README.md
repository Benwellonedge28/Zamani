Zamani Interoperability Grammar

Path: "grammar/interoperability/"
Primary document: "grammar/interoperability/README.md"
Role: Interoperability architecture and integration contract
Language: Zamani
Grammar technology: ANTLR 4 composition
Implementation baseline: Rust 2021, Rust 1.97+
Implementation requirement: Safe Rust only
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

The "grammar/interoperability/" subsystem defines how Zamani programs declare, consume, expose, exchange, translate, and execute across external computational ecosystems.

It provides the source-level contracts required for interoperability with:

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
- serialization formats;
- data interchange formats;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- C;
- C++;
- Rust;
- Python;
- Zig;
- assembly ecosystems;
- operating-system interfaces;
- accelerator interfaces;
- quantum systems;
- hardware/software interfaces;
- future languages;
- future formats;
- future runtimes;
- future computational substrates.

The subsystem exists so that external technologies can be integrated without making each external technology a permanent part of Zamani's universal semantic core.

The fundamental rule is:

«Interoperability declares an external contract. It does not determine how that contract is physically realized.»

Therefore:

Zamani source
    ↓
interoperability declaration
    ↓
domain-neutral AST
    ↓
semantic validation
    ↓
canonical semantic representation
    ↓
canonical/domain IR
    ↓
lowering
    ↓
ABI / runtime / backend realization

Interoperability must preserve program meaning while allowing implementation technology to evolve.

---

2. Architectural Objective

The interoperability subsystem exists to support:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

POCO-REAF does not mean that every target can execute every program.

It means that a valid Zamani program should not require source rewriting merely because its realization changes from:

tiny system
    ↓
embedded system
    ↓
single processor
    ↓
multicore
    ↓
GPU
    ↓
FPGA
    ↓
ASIC
    ↓
accelerator
    ↓
QPU
    ↓
simulator
    ↓
HPC
    ↓
cluster
    ↓
distributed system
    ↓
cloud
    ↓
future computational substrate

provided the target satisfies the program's semantic requirements and available resources.

The distinction is:

portable source meaning
        ≠
target feasibility

A target may lack the required capability or resources without making the source program invalid.

---

3. Authority Model

"grammar/interoperability/" is subordinate to the repository's global language architecture.

The authority chain is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
grammar/antlr/ZamaniLexer.g4
grammar/antlr/ZamaniParser.g4
        ↓
domain grammar composition
        ↓
Rust lexer/parser
        ↓
domain-neutral AST
        ↓
structural validation
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR / domain IR
        ↓
optimization
        ↓
lowering
        ↓
routing / scheduling / resilience
        ↓
ZQN / QEC where applicable
        ↓
HAL
        ↓
target realization

Authority responsibilities

Location| Responsibility
"grammar/DESIGN.md"| Global grammar architecture
"grammar/README.md"| Grammar navigation and authority map
"grammar/Zamani.g4"| Canonical ANTLR composition root
"grammar/antlr/ZamaniLexer.g4"| Canonical lexer composition
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/grammar.md"| Implementation/conformance status
"grammar/Zamani-Grammar.md"| Historical/extended/proposed grammar material
"grammar/specification/"| Normative language specification
"grammar/spec/"| Formal subsystem contracts
"grammar/interoperability/"| Interoperability grammar contracts
"src/lexer.rs"| Rust lexical implementation
"src/parser.rs"| Rust parsing implementation
"src/ast/" / frontend AST implementation| Domain-neutral AST
semantic subsystem| Meaning and validation
classical IR| Classical canonical representation
"quantum::ir"| Canonical quantum representation
HDL/hardware semantic/IR subsystem| Hardware representation
compiler/lowering| Target transformation
ABI infrastructure| ABI realization
runtime| Runtime execution
HAL| Hardware abstraction and target state

This README does not supersede any higher-level authority.

---

4. Core Ownership Rule

The interoperability subsystem owns boundary declarations, not the complete semantics of the systems it connects.

It owns:

- foreign-language identity;
- foreign-format identity;
- foreign-interface declarations;
- FFI declarations;
- foreign-function declarations;
- foreign-type declarations;
- ABI declarations;
- calling-convention intent;
- linkage intent;
- symbol identity;
- conversion intent;
- marshaling intent;
- ownership-boundary declarations;
- lifetime-boundary declarations;
- nullability declarations;
- representation declarations;
- callback declarations;
- foreign runtime requirements;
- serialization declarations;
- external-format declarations;
- interoperability capabilities;
- interoperability effects;
- interoperability resource requirements;
- interoperability policies;
- interoperability security requirements;
- interoperability compatibility requirements;
- interoperability provenance.

It does not own:

- universal identifiers;
- universal names;
- ordinary functions;
- ordinary types;
- ordinary expressions;
- ordinary statements;
- ordinary modules;
- type checking implementation;
- ownership checking implementation;
- lifetime checking implementation;
- ABI lowering;
- register allocation;
- machine instruction selection;
- linking implementation;
- library discovery;
- runtime loading;
- filesystem operations;
- network operations;
- hardware discovery;
- target selection;
- physical device allocation;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- deployment;
- runtime execution.

---

5. One Language, Extensible Boundaries

Zamani remains one programming language.

External ecosystems do not become separate Zamani languages.

The model is:

Zamani
  │
  ├── native computation
  │
  ├── foreign-language boundary
  │
  ├── foreign-format boundary
  │
  ├── ABI boundary
  │
  ├── runtime boundary
  │
  ├── data boundary
  │
  ├── quantum boundary
  │
  └── hardware boundary

External technologies are adapters and contracts around Zamani semantics.

They must not create competing semantic universes.

---

6. Four Distinct Interoperability Dimensions

Interoperability must distinguish at least four independent dimensions.

6.1 Language

A language identifies an external programming ecosystem.

Examples:

C
C++
Rust
Python
Zig

Language identity is symbolic and extensible.

The architecture must remain open to future languages.

---

6.2 Format

A format describes an interchange or representation ecosystem.

Examples:

OpenQASM
QIR
WebAssembly
Verilog
SystemVerilog
C source
C header
JSON
XML

A format is not automatically a programming language.

---

6.3 ABI

An ABI describes binary compatibility expectations.

It may describe:

- calling sequence;
- data representation;
- alignment;
- parameter passing;
- return conventions;
- linkage;
- symbol naming;
- variadic behavior;
- foreign object representation.

The grammar declares ABI intent.

The backend realizes the ABI.

---

6.4 Runtime

A runtime is an execution environment.

Examples include:

- foreign language runtime;
- system runtime;
- managed runtime;
- accelerator runtime;
- quantum runtime;
- distributed runtime;
- device runtime.

Runtime identity must not be confused with language identity or ABI identity.

---

7. Extensible Foreign Identity

Interoperability must not use a permanently closed enumeration such as:

C | Cpp | Rust | Python | Zig | ...

Instead, external identities must be representable through the canonical Zamani naming system.

Conceptually:

C
C++
Rust
Python
Zig
vendor::language
organization::language
future::language

The precise lexical form is owned by the canonical identifier/name grammar.

Adding a future external language must not require redesigning the universal interoperability architecture.

---

8. No Target Encoding in Foreign Identity

A foreign declaration must not silently encode a machine.

For example:

language "C"

identifies the C contract.

It does not inherently mean:

x86
x86_64
AArch64
RISC-V
Linux
Windows
macOS
specific compiler
specific CPU
specific register set
specific pointer width

Those are separate realization properties.

Likewise:

format "OpenQASM"

does not inherently select:

- a QPU;
- a physical qubit;
- a coupling graph;
- calibration;
- gate timing;
- routing;
- scheduling;
- QEC strategy.

---

9. Canonical Grammar Composition

"grammar/Zamani.g4" remains the only root grammar.

It must not directly import every interoperability leaf grammar.

The intended hierarchy is:

grammar/Zamani.g4
        │
        └── grammar/antlr/ZamaniParser.g4
                  │
                  ├── universal grammar
                  ├── classical
                  ├── quantum
                  ├── HDL
                  ├── hardware
                  ├── AI
                  ├── distributed
                  ├── networking
                  ├── resources
                  ├── effects
                  ├── security
                  ├── execution
                  ├── interoperability
                  ├── dialects
                  ├── macros
                  └── metaprogramming

Within interoperability:

ZamaniParser
      ↓
interoperability composition
      ↓
generic interoperability
      ↓
specialized boundary grammar
      ↓
domain-neutral AST

No interoperability file becomes another parser root.

---

10. Existing Interoperability Files

The current repository contains interoperability material including:

grammar/interoperability/
├── README.md
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

These files are retained unless a deliberate architectural migration establishes a better owner.

Existing filenames must not be renamed merely for stylistic consistency.

Where required functionality is missing, new files may be added.

Recommended additional files include:

foreign-types.g4
calling-conventions.g4
linkage.g4
symbols.g4
conversions.g4
marshaling.g4
callbacks.g4
serialization.g4
wasm.g4
qir.g4
hdl.g4

These are extensions of the existing architecture, not replacements for the existing files.

---

11. "interoperability.g4"

Purpose

"interoperability.g4" is the interoperability composition grammar.

Owns

It owns dispatch/composition among:

- FFI;
- foreign functions;
- foreign types;
- ABI;
- calling conventions;
- linkage;
- symbols;
- conversions;
- callbacks;
- serialization;
- external formats;
- system interfaces;
- language-specific interoperability grammars.

Does not own

It must not redefine:

- identifiers;
- names;
- types;
- expressions;
- functions;
- modules;
- attributes;
- effects;
- resources;
- capabilities;
- contracts.

Integration

ZamaniParser.g4
        ↓
interoperability.g4
        ↓
specialized interoperability grammars
        ↓
AST interoperability nodes

Completion

Done means:

- all imported rules have one owner;
- no universal grammar is duplicated;
- every public rule has an AST mapping;
- every public rule has semantic ownership;
- external identities are extensible;
- target realization is downstream;
- tests exist;
- source spans are preserved;
- deterministic parsing is demonstrated.

---

12. "ffi.g4"

Purpose

Defines the generic source-level foreign-function interface boundary.

Owns

- FFI declarations;
- foreign interfaces;
- foreign callable references;
- callback intent;
- conversion intent;
- marshaling intent;
- ownership-boundary declarations;
- lifetime-boundary declarations;
- nullability;
- effects;
- capabilities;
- resources;
- security requirements;
- compatibility requirements.

Does not own

It does not:

- load libraries;
- execute foreign functions;
- inspect hardware;
- inspect the filesystem;
- inspect the network;
- resolve physical addresses;
- perform linking;
- select a backend.

Integration

FFI syntax
    ↓
AST
    ↓
foreign callable semantic model
    ↓
type/effect/capability/resource validation
    ↓
compiler lowering
    ↓
ABI realization
    ↓
link/runtime integration

---

13. "foreign-functions.g4"

Purpose

Declares externally implemented callable contracts.

Conceptual syntax:

extern "C" fn external_function(value: SomeType) -> ResultType;

The exact syntax is determined by the canonical grammar.

The declaration means:

«A callable contract exists outside the current Zamani implementation unit.»

It does not itself execute anything.

Integration

foreign function
      ↓
canonical function model
      ↓
foreign-function semantic contract
      ↓
ABI/calling convention
      ↓
backend/runtime

---

14. "foreign-types.g4"

Purpose

Defines source-level declarations for types whose representation or implementation originates outside Zamani.

It must support semantic concepts such as:

- named foreign types;
- opaque types;
- handles;
- references;
- ownership;
- borrowing;
- lifetime;
- nullability;
- representation;
- conversion;
- ABI compatibility.

Critical scalability rule

The grammar must never assume:

pointer = fixed width
register = fixed width
word = fixed width
address = fixed width

Representation belongs to the relevant ABI and target semantic layers.

An opaque foreign type remains opaque unless a valid representation contract is explicitly available.

---

15. ABI Architecture

"abi.g4" owns ABI declarations.

It may represent symbolic ABI identity:

abi "C"
abi "system"
abi vendor::abi

The exact syntax is governed by the canonical grammar.

ABI grammar does not calculate:

- structure layout;
- alignment;
- stack layout;
- register allocation;
- machine instructions;
- relocations;
- linking.

The pipeline is:

ABI declaration
      ↓
AST
      ↓
ABI semantic contract
      ↓
target ABI realization

---

16. Calling Conventions

"calling-conventions.g4" owns calling-convention intent.

A calling convention is not:

- a processor;
- a register file;
- an instruction sequence;
- a physical stack layout.

It may identify a symbolic convention:

c
system
default
vendor::convention

The actual realization is backend-specific.

No universal grammar rule may encode:

- physical argument registers;
- physical return registers;
- stack slots;
- register names;
- instruction sequences.

---

17. Linkage

"linkage.g4" owns linkage intent.

Possible semantic categories include:

- external;
- imported;
- exported;
- internal;
- weak;
- symbolic;
- versioned;
- platform-qualified.

Linkage realization belongs to the compiler/linker infrastructure.

A linkage declaration must remain a semantic declaration rather than an instruction to perform linking during parsing.

---

18. Foreign Symbols

"s‍ymbols.g4" should own foreign symbol identity where a dedicated file is required.

A symbol may contain:

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
process identifier
runtime pointer
register address

Those are runtime or target artifacts.

---

19. Ownership Across Foreign Boundaries

Interoperability must integrate with the canonical memory/ownership architecture.

Supported semantic intent may include:

borrowed
owned
shared
transferred
retained
returned
released

The grammar declares intent.

Semantic analysis determines validity.

Runtime/backend infrastructure realizes the contract.

There must be no second ownership system inside interoperability.

---

20. Lifetime

Foreign boundaries may carry lifetime requirements for:

- arguments;
- return values;
- callbacks;
- borrowed objects;
- retained objects;
- foreign resources;
- asynchronous operations;
- streams.

Lifetime declarations must remain symbolic semantic contracts.

They must never require physical addresses.

---

21. Nullability

Foreign boundaries may declare:

nullable
nonnull
unknown

or extensible equivalent semantic forms.

Nullability is part of the type/foreign-boundary contract.

It must integrate with the canonical type system.

It must not create an independent foreign type universe.

---

22. Representation and Layout

Foreign interoperability may need representation intent.

Examples of semantic concepts include:

- opaque;
- transparent;
- compatible;
- encoded;
- packed;
- aligned;
- externally represented.

However, the grammar must distinguish:

representation intent

from:

target layout realization

The compiler/backend computes concrete layout.

No universal layout constants belong in the grammar.

---

23. Conversions

"conversions.g4" should own explicit interoperability conversion declarations when the existing grammar does not already provide an appropriate owner.

Conversions must distinguish:

lossless
lossy
fallible
infallible
validated
unchecked-by-contract

Semantic conversion validation belongs outside the parser.

Every lossy conversion must be explicit.

Silent semantic loss is prohibited.

---

24. Marshaling

"marshaling.g4" should represent source-level marshaling intent.

It may describe:

- input conversion;
- output conversion;
- encoding;
- decoding;
- ownership transfer;
- lifetime transfer;
- serialization;
- deserialization;
- representation adaptation.

It must not execute serialization during parsing.

The runtime/compiler realizes the marshaling operation.

---

25. Callbacks

"callbacks.g4" should define callback contracts where the generic function grammar does not already provide the required abstraction.

A callback contract must identify:

- callable type;
- parameter types;
- result type;
- ownership;
- lifetime;
- effects;
- capabilities;
- resource requirements;
- concurrency requirements;
- error behavior.

Callback execution belongs to runtime/backend infrastructure.

---

26. Asynchronous Foreign Boundaries

Foreign calls may be:

- synchronous;
- asynchronous;
- streaming;
- callback-based;
- event-driven.

The interoperability grammar declares the boundary.

Concurrency semantics remain owned by:

grammar/concurrency/
grammar/execution/
grammar/effects/

Interoperability must not create a second asynchronous execution model.

---

27. Effects

Foreign operations participate in the canonical effect system.

A foreign function may carry effects such as:

io
network
native
foreign
mutation
randomness
distributed
measurement
simulation

The exact effect vocabulary is owned by "grammar/effects/".

Interoperability references that model.

It must not create a competing effect hierarchy.

Pipeline:

foreign declaration
      ↓
effect metadata
      ↓
effect validation
      ↓
policy/capability analysis
      ↓
execution

---

28. Capabilities

Foreign interfaces participate in the canonical capability model.

Examples of semantic capability requirements include:

capability("foreign.call")
capability("native.interop")
capability("system.interface")
capability("network")
capability("filesystem")

The precise capability registry is owned outside this directory.

The interoperability grammar merely declares required capabilities.

Capability availability is determined downstream.

---

29. Resources

Foreign operations may require resources.

Examples:

requires capability("tensor.compute");
requires memory >= required_memory;
requires topology(required_topology);

The interoperability layer must consume the same resource model as the rest of Zamani:

requirement
constraint
capability
budget
preference
hint
negotiation

It must not define another resource system.

---

30. Resource Scalability

Interoperability must impose no artificial universal limits on:

- memory;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- ASIC resources;
- accelerators;
- QPUs;
- qubits;
- nodes;
- devices;
- channels;
- connections;
- tensor dimensions;
- tensor rank;
- data volume;
- program size;
- number of foreign interfaces.

A program-defined numeric value remains program semantics.

A numeric value must not become a language capacity ceiling merely because an implementation currently has a finite resource.

---

31. Security

Foreign boundaries are security-sensitive.

The interoperability subsystem must integrate with:

grammar/security/
grammar/policies/
grammar/capabilities/
grammar/effects/

Foreign declarations may require:

- authorization;
- trust;
- sandbox restrictions;
- prohibited effects;
- permitted capabilities;
- resource constraints;
- provenance;
- auditability.

A foreign declaration must never implicitly grant elevated privileges.

---

32. Sandboxing

A foreign operation may be restricted by policy.

Conceptual intent:

sandbox {
    forbid effect("network");
    forbid capability("native.execute");
}

The exact syntax belongs to the canonical policy/security grammar.

Interoperability consumes the policy.

It does not implement the sandbox.

---

33. Provenance

Every interoperability boundary must be capable of carrying provenance where required.

Relevant provenance information may include:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
timestamp

Provenance integrates with the repository-wide provenance model.

It must not create an interoperability-specific provenance universe.

The purpose is traceability of:

Zamani declaration
      ↓
foreign contract
      ↓
conversion
      ↓
lowering
      ↓
artifact
      ↓
runtime realization

---

34. Determinism

Parsing interoperability declarations must be deterministic.

The result must depend only on:

- source text;
- grammar version;
- selected language version;
- explicitly configured dialects;
- explicitly supplied compilation context where the architecture permits it.

Parsing must not depend on:

- wall-clock time;
- randomness;
- current hardware;
- filesystem state;
- network state;
- environment variables;
- runtime state;
- target availability.

Target discovery belongs downstream.

---

35. No Runtime Actions in Grammar

ANTLR grammar files under this directory must remain declarative.

They must not:

- execute Rust;
- invoke shell commands;
- load libraries;
- inspect hardware;
- access files;
- access networks;
- inspect environment variables;
- execute foreign code;
- invoke a linker;
- invoke a compiler;
- invoke a runtime.

Semantic actions belong outside the grammar.

---

36. No Environment-Dependent Parsing

Parser predicates must not determine syntax by asking whether:

- a library exists;
- a compiler exists;
- a runtime exists;
- a CPU exists;
- a GPU exists;
- a QPU exists;
- a device exists;
- a file exists;
- a network service exists.

The same source must parse consistently independent of target availability.

---

37. C Interoperability

"c.g4" owns C-specific interoperability syntax.

It must build on generic interoperability contracts.

C-specific syntax must map into:

C declaration
    ↓
generic foreign declaration
    ↓
canonical Zamani type/function model
    ↓
ABI contract
    ↓
target realization

C-specific grammar must not redefine:

- universal identifiers;
- universal types;
- universal expressions;
- universal functions;
- ABI semantics.

---

38. C++ Interoperability

"cpp.g4" owns C++-specific boundary syntax.

It must integrate with:

- foreign types;
- foreign functions;
- ABI;
- ownership;
- lifetime;
- conversion;
- exceptions/error contracts where supported;
- linkage;
- namespaces;
- templates where the interoperability specification supports them.

C++ implementation semantics must not become part of the Zamani core.

---

39. Rust Interoperability

"rust.g4" owns Rust-specific interoperability declarations.

It must integrate with:

- foreign functions;
- foreign types;
- ownership;
- borrowing;
- lifetime;
- ABI;
- calling convention;
- conversion;
- effects;
- capabilities.

The Zamani type system remains authoritative for Zamani semantics.

---

40. Python Interoperability

"python.g4" owns Python-specific boundary declarations.

The grammar must represent contracts such as:

- callable identity;
- module identity;
- type identity;
- value conversion;
- runtime requirement;
- ownership;
- lifetime;
- error behavior;
- capability;
- effects.

It must not require the parser to start a Python runtime.

---

41. Zig Interoperability

"zig.g4" owns Zig-specific interoperability declarations.

It must use the generic:

foreign type
foreign function
ABI
calling convention
linkage
conversion
ownership
lifetime
effect
capability
resource

architecture.

---

42. Assembly Interoperability

"AssemblyLanguage.g4" represents assembly-oriented interoperability contracts.

Assembly integration must remain explicitly target-specific.

Assembly is not a universal representation of Zamani computation.

The architecture is:

Zamani semantics
      ↓
explicit assembly boundary
      ↓
ABI / target contract
      ↓
target backend

Assembly-specific physical details are allowed only within explicitly target-specific interoperability contracts.

They must not leak into universal Zamani grammar.

---

43. System Interfaces

"SystemInterfaces.g4" and "system-interfaces.g4" must be reconciled as one logical ownership area.

They must not silently become two competing semantic authorities.

The system-interface subsystem may represent:

- operating-system calls;
- system services;
- device interfaces;
- process interfaces;
- file interfaces;
- IPC;
- system resources.

The grammar declares contracts.

Actual system access belongs to runtime/backend infrastructure.

If both files remain for compatibility, one must be explicitly designated the canonical composition owner and the other must delegate to it.

---

44. WebAssembly

Create "wasm.g4" when WebAssembly interoperability requires dedicated syntax.

WebAssembly must be treated as an external representation/execution boundary.

It must not replace canonical Zamani semantics.

Pipeline:

Zamani semantics
      ↓
WebAssembly interoperability model
      ↓
Wasm lowering
      ↓
Wasm artifact/runtime

Wasm-specific details must remain downstream from portable semantics.

---

45. OpenQASM

"openqasm.g4" represents OpenQASM interoperability.

OpenQASM must not become a second Zamani quantum semantic model.

The required path is:

OpenQASM
    ↓
interoperability AST
    ↓
quantum semantic model
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
resilience/QEC
    ↓
ZQN
    ↓
HAL
    ↓
target

No physical qubit allocation belongs in the grammar.

No universal gate inventory belongs in the grammar.

---

46. QIR

Create "qir.g4" when QIR interoperability requires dedicated grammar support.

QIR is an interoperability representation.

It must not replace:

quantum::ir

The canonical direction is:

QIR
    ↓
quantum semantic interpretation
    ↓
quantum::ir

or, for output:

quantum::ir
    ↓
QIR lowering

The architecture must never establish a second permanent quantum IR merely to support QIR.

---

47. Quantum Interoperability Invariant

All quantum interoperability must converge through:

external quantum representation
        ↓
Zamani semantic quantum model
        ↓
quantum::ir

"quantum::ir" remains the canonical quantum boundary.

Interoperability must not own:

- physical qubit mapping;
- coupling topology;
- calibration;
- gate scheduling;
- routing;
- error correction;
- physical noise modeling;
- QPU selection.

Those belong downstream.

---

48. Generic Quantum Operations

External quantum formats may contain named operations.

Interoperability must not require the universal Zamani grammar to enumerate every operation.

The semantic architecture should support:

operation namespace
operation name
operands
parameters
results
attributes
modifiers
effects
capabilities
source

This permits:

- standard operations;
- vendor operations;
- user-defined operations;
- parameterized operations;
- future operations.

New quantum operations therefore do not require redesigning the universal grammar.

---

49. HDL Interoperability

Create "hdl.g4" when dedicated HDL-format interoperability syntax is required.

HDL interoperability includes formats such as:

Verilog
SystemVerilog
other supported HDL ecosystems
future HDL formats

The architecture is:

HDL format
    ↓
interoperability representation
    ↓
Zamani hardware semantic model
    ↓
HDL/hardware IR
    ↓
synthesis/lowering
    ↓
target realization

Native Zamani HDL remains distinct from external HDL interchange.

---

50. "verilog.g4"

"verilog.g4" owns Verilog-specific interoperability.

It must not redefine the complete Zamani HDL architecture.

It maps Verilog constructs into the canonical hardware/HDL semantic model where supported.

Target-specific widths may be represented when they are explicitly part of the source contract, but they must not become universal language limits.

---

51. Serialization

Create "serialization.g4" when serialization syntax requires a dedicated grammar owner.

Serialization must support semantic declarations for:

- encoding;
- decoding;
- schema identity;
- version;
- compatibility;
- loss policy;
- nullability;
- representation;
- provenance.

Supported formats may include:

JSON
XML
binary formats
schema-based formats
future formats

Format-specific parsers belong under appropriate dialect/format boundaries.

---

52. JSON and XML

JSON/XML are data-interchange formats.

They must not become universal Zamani programming syntax.

Recommended architecture:

grammar/dialects/json/
grammar/dialects/xml/

or the repository's established interoperability/dialect ownership if another canonical location is chosen.

The pipeline is:

external data format
      ↓
format parser
      ↓
canonical Zamani data model
      ↓
semantic validation
      ↓
canonical IR/data representation

---

53. SQL and Query Languages

SQL must not become universal Zamani syntax.

Use an external dialect boundary such as:

grammar/dialects/sql/

or the repository's established interoperability location.

The architecture is:

SQL
 ↓
SQL dialect parser
 ↓
canonical data/query semantic model
 ↓
Zamani semantic infrastructure
 ↓
canonical IR

SQL must not become a second language authority.

---

54. Data Interoperability

Interoperability with data systems must integrate with:

grammar/data/
grammar/types/
grammar/expressions/
grammar/interoperability/

Data interoperability may cover:

- schemas;
- records;
- graphs;
- datasets;
- streams;
- queries;
- serialization;
- deserialization;
- provenance;
- uncertainty;
- external schemas.

The data semantic model remains canonical.

---

55. Graph and Knowledge Interoperability

Graph and knowledge structures may be imported from external ecosystems.

Interoperability should map them into the common Zamani semantic model.

Conceptual information may include:

subject
relation
object
metadata
provenance
confidence
source

The interoperability subsystem does not create a separate knowledge semantic engine.

---

56. AI and Learned-Model Interoperability

External AI models may be integrated through:

- model interfaces;
- foreign functions;
- data contracts;
- tensor representations;
- model serialization;
- runtime bindings;
- accelerator interfaces.

AI-specific interoperability must use the same:

types
effects
capabilities
resources
policies
contracts
provenance

as the rest of Zamani.

An external model is not automatically part of the language grammar.

---

57. Neural-Symbolic Interoperability

A learned model may interoperate with symbolic computation through canonical semantic operations.

Conceptual pipeline:

symbolic computation
       ↕
semantic boundary
       ↕
learned model
       ↕
tensor/data representation
       ↕
accelerator/runtime

The interoperability subsystem declares the boundary.

AI semantics remain owned by the AI semantic architecture.

Tensor semantics remain owned by the canonical type/data/classical architecture.

---

58. FFI and Contracts

Every foreign callable may participate in:

requires
ensures
invariant
assume
guarantee
property

where supported by the canonical contract system.

Example conceptual meaning:

foreign callable
    ↓
requires capability(...)
requires resource(...)
requires condition
    ↓
call
    ↓
ensures condition

The contract grammar is not duplicated here.

Interoperability references the canonical contract model.

---

59. Policies

Foreign boundaries may be constrained by policies.

Policies may govern:

- allowed languages;
- allowed ABIs;
- allowed runtimes;
- allowed effects;
- permitted capabilities;
- resource budgets;
- network access;
- filesystem access;
- native execution;
- data movement;
- adaptation;
- deployment;
- provenance.

Policy ownership belongs to the canonical policy/security architecture.

---

60. Compatibility

Interoperability compatibility is multidimensional.

At minimum, distinguish:

language compatibility
format compatibility
source compatibility
syntax compatibility
AST compatibility
semantic compatibility
type compatibility
effect compatibility
capability compatibility
resource compatibility
ABI compatibility
artifact compatibility
runtime compatibility
target compatibility
dialect compatibility

A target inability is not automatically a language incompatibility.

For example:

valid source
+
valid semantics
+
target lacks required capability

means:

target-infeasible

not necessarily:

invalid Zamani

---

61. Versioning

External versions and Zamani versions are independent dimensions.

For example:

Zamani language version
+
foreign language version
+
ABI version
+
format version
+
runtime version

must not be collapsed into one version number.

The compatibility architecture must propagate version metadata through:

lexer
parser
AST
semantic analysis
IR
artifact
runtime
diagnostics
tooling

where required.

---

62. Migration

Legacy interoperability syntax must have an explicit migration state.

A migration may be required because of:

- ambiguity;
- semantic contradiction;
- security issues;
- parser conflicts;
- deliberately removed syntax;
- incompatible semantic contracts.

Migration must not be introduced merely because:

- a new processor exists;
- a new accelerator exists;
- a vendor changed hardware;
- an implementation has temporary limitations.

---

63. Semantic Loss

Interoperability conversions must classify semantic loss.

Possible categories include:

lossless
lossy
conditionally-lossless
unsupported
implementation-defined
target-dependent

The exact taxonomy is owned by the formal interoperability specification.

Silent semantic loss is prohibited.

If a conversion can alter program meaning, that fact must be visible to semantic analysis and, where required, to the programmer.

---

64. Error and Diagnostic Model

Interoperability diagnostics must use the common Zamani diagnostic architecture.

Diagnostics must preserve, where available:

- source span;
- file identity;
- construct identity;
- expected information;
- actual information;
- external language/format identity;
- version;
- ABI;
- calling convention;
- semantic context;
- provenance.

Examples of diagnostic categories include:

unknown foreign language
unknown format
unsupported version
invalid ABI
invalid calling convention
foreign type mismatch
ownership mismatch
lifetime violation
conversion failure
missing capability
insufficient resources
policy violation
unsupported target realization
semantic loss

The parser must report structural errors.

Semantic analysis reports semantic errors.

Target realization reports target feasibility errors.

---

65. AST Contract

Interoperability syntax must map into a domain-neutral AST.

AST nodes should represent concepts such as:

ForeignDeclaration
ForeignFunction
ForeignType
ForeignModule
ForeignInterface
ForeignSymbol
AbiContract
CallingConvention
Linkage
Conversion
Marshaling
Callback
SerializationBoundary
InteroperabilityRequirement

The AST must preserve:

- source spans;
- names;
- external identities;
- attributes;
- modifiers;
- parameters;
- type references;
- contract references;
- effect references;
- capability references;
- resource requirements;
- policy references;
- provenance metadata where applicable.

The AST must not embed:

- physical CPU identity;
- physical GPU identity;
- physical QPU identity;
- physical qubit mapping;
- register allocation;
- scheduler decisions;
- routing decisions;
- calibration;
- machine addresses.

---

66. Semantic Contract

After parsing:

AST
 ↓
interoperability semantic analysis

must validate:

- language identity;
- format identity;
- ABI;
- calling convention;
- symbol identity;
- type compatibility;
- ownership;
- lifetime;
- nullability;
- conversion;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- compatibility;
- provenance;
- portability.

The semantic layer determines meaning.

The grammar only recognizes structure.

---

67. Canonical IR Boundary

Interoperability must not create a universal:

interoperability::ir

Foreign boundaries lower into the appropriate canonical representation.

Examples:

foreign classical function
        ↓
classical semantic model / IR

foreign quantum representation
        ↓
quantum semantic model
        ↓
quantum::ir

foreign HDL
        ↓
hardware/HDL semantic model

Interoperability therefore remains a boundary, not a permanent IR layer.

---

68. Quantum IR Boundary

The single canonical quantum path is:

foreign quantum syntax/format
        ↓
domain-neutral AST
        ↓
quantum semantic analysis
        ↓
quantum::ir

No second quantum IR may be introduced solely for interoperability.

---

69. Classical IR Boundary

Classical foreign functions and representations must lower into the canonical classical semantic/IR architecture.

The interoperability subsystem must not create:

CIR
CppIR
PythonIR
RustIR
ZigIR

as permanent semantic layers.

Language-specific information is preserved as metadata and contracts where necessary.

---

70. HDL/HW Boundary

HDL interoperability must converge into the repository's canonical HDL/hardware semantic representation.

The pipeline is:

foreign HDL
      ↓
AST
      ↓
hardware semantics
      ↓
HDL/hardware IR
      ↓
synthesis/lowering
      ↓
target realization

Interoperability must not become a replacement for the native hardware architecture.

---

71. ABI Lowering

ABI lowering is downstream.

The complete conceptual chain is:

Zamani callable
      ↓
foreign callable contract
      ↓
ABI semantic model
      ↓
target ABI
      ↓
calling convention
      ↓
layout
      ↓
lowering
      ↓
link/runtime

The grammar does not perform these transformations.

---

72. Runtime Boundary

The interoperability grammar does not execute foreign code.

Runtime responsibilities include:

- loading;
- invocation;
- scheduling;
- asynchronous execution;
- callback execution;
- resource management;
- error propagation;
- data conversion;
- foreign runtime interaction.

The runtime must consume already validated semantic contracts.

---

73. Hardware Boundary

Hardware discovery belongs outside the grammar.

The interoperability grammar must never decide:

which CPU?
which GPU?
which FPGA?
which accelerator?
which QPU?
which node?
which device?
which physical qubit?

Those decisions are made through:

resource analysis
capability negotiation
target selection
routing
scheduling
HAL
runtime

---

74. POCO-REAF Resource Model

A portable program describes intent.

It may express:

requires capability("gpu.compute");
requires capability("quantum.measurement");
requires capability("tensor.compute");
requires memory >= required_memory;
requires topology(required_topology);

The compiler/runtime determines whether a realization is possible.

The grammar must never replace symbolic requirements with machine constants.

---

75. Target Feasibility

The correct distinction is:

Program semantics
      ↓
requirements
      ↓
target capabilities
      ↓
available resources
      ↓
feasibility

A program can therefore be:

syntactically valid
semantically valid
type valid
effect valid
policy valid
resource-valid as an abstract requirement

while a particular target is unable to execute it.

That is a target-feasibility result.

---

76. Infinite-Scale Principle

"Infinity" in POCO-REAF means that the language does not impose an arbitrary finite ceiling on computational scale.

It does not claim that physical resources are infinite.

The correct model is:

source semantics
      ↓
required resources
      ↓
available resources
      ↓
target capabilities
      ↓
realization feasibility

Any finite physical limitation belongs to the realization environment.

The language must remain open-ended.

---

77. No Artificial Capacity Constants

The interoperability subsystem must not introduce universal constants equivalent in meaning to:

maximum qubits
maximum CPUs
maximum GPUs
maximum FPGAs
maximum nodes
maximum memory
maximum threads
maximum tensor rank
maximum register width
maximum network size
maximum device count

The same prohibition applies to indirect equivalents hidden inside:

- grammar alternatives;
- parser predicates;
- enums;
- validation rules;
- default configurations;
- documentation;
- test fixtures;
- generated code;
- backend assumptions.

A target-specific contract may contain a real target property when that property is explicitly target-specific.

It must not be promoted into a universal Zamani limit.

---

78. Numeric Values

Numeric literals remain program semantics.

For example:

let n = 1024;

is a program value.

It does not establish a universal limit of "1024".

Interoperability grammar must distinguish:

program value

from:

language capacity limit

---

79. Dialects

Interoperability extensions may use the dialect architecture.

A dialect must have:

name
version
owner
syntax
AST mapping
semantic mapping
IR mapping
compatibility contract
capability requirements
effect requirements
resource requirements
security requirements
tests

A dialect must not silently become a separate language.

External formats should normally be represented as dialects or adapters when appropriate.

---

80. Future Languages

Adding a future language should require:

new interoperability adapter
+
semantic mapping
+
ABI/runtime contract where applicable
+
tests

It should not require:

new universal AST
new universal IR
new parser root
new resource model
new effect model
new security model

unless the external technology genuinely exposes a new universal semantic concept that must first be designed at the language level.

---

81. Future Formats

The same principle applies to formats.

A new format should integrate through:

format identity
      ↓
format adapter
      ↓
canonical semantic model

rather than becoming a permanent universal syntax branch.

---

82. Future Hardware

Interoperability must remain independent of today's hardware.

A future device may introduce:

- new processor architecture;
- new accelerator model;
- new quantum technology;
- new memory architecture;
- new communication topology;
- new execution model.

The source language must not require redesign merely because the target changed.

The integration point is:

portable semantics
      ↓
capability/resource model
      ↓
target lowering

---

83. Metaprogramming Integration

Macros and metaprogramming may generate interoperability declarations.

However:

generated source
      ↓
normal parsing
      ↓
normal semantic validation
      ↓
normal interoperability validation

must remain mandatory.

Metaprogramming cannot bypass:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- policy checking;
- security checking;
- compatibility checking.

---

84. Reflection Integration

Reflection may inspect interoperability metadata where explicitly permitted.

Reflection must not automatically gain:

- library loading;
- process execution;
- filesystem access;
- network access;
- hardware discovery;
- privilege escalation.

Those operations require the canonical capability/effect/policy mechanisms.

---

85. Simulation

Interoperability may declare simulation-oriented external boundaries.

Simulation may represent:

- foreign runtime behavior;
- hardware behavior;
- quantum execution;
- distributed execution;
- accelerator behavior;
- performance characteristics.

Simulation is an execution strategy.

It is not another source language.

---

86. Deterministic Reproduction

Interoperability artifacts should preserve sufficient metadata for reproducibility where required.

Potential metadata includes:

language version
format version
ABI identity
calling convention
dialect versions
conversion rules
source provenance
artifact identity
compiler information
semantic configuration

Reproducibility metadata must be deterministic and auditable.

---

87. Security and Provenance Together

For sensitive foreign boundaries:

foreign declaration
      ↓
identity
      ↓
capability
      ↓
effect
      ↓
policy
      ↓
authorization
      ↓
provenance
      ↓
execution

A foreign interface must not bypass the security architecture simply because it is declared externally.

---

88. Error Propagation

Foreign boundaries must provide a semantic mechanism for errors.

Depending on the external contract, errors may be represented through:

- "Result";
- status values;
- exceptions;
- error objects;
- callbacks;
- asynchronous completion;
- explicit failure contracts.

The exact Zamani representation must use the canonical type/effect/function architecture.

The interoperability grammar declares the boundary.

It does not create a second error system.

---

89. Data Conversion Safety

Conversions crossing interoperability boundaries must be validated.

The semantic layer must consider:

- type compatibility;
- representation;
- encoding;
- ownership;
- lifetime;
- nullability;
- alignment;
- ABI requirements;
- semantic loss;
- failure behavior.

No conversion may silently produce a semantically different value while claiming equivalence.

---

90. Streaming

Interoperability may expose streams.

A stream contract may carry:

element type
direction
lifetime
ownership
backpressure semantics
effects
capabilities
resources
error behavior
termination behavior

Streaming integrates with:

grammar/concurrency/
grammar/execution/
grammar/networking/
grammar/data/

rather than creating a second stream model.

---

91. Networking

Network-based foreign interfaces integrate with:

grammar/networking/
grammar/security/
grammar/resources/
grammar/effects/

A network boundary may require:

effect("network")
capability("network.*")

plus resource and policy constraints.

The interoperability grammar does not implement network discovery or connection establishment.

---

92. Distributed Interoperability

Distributed foreign boundaries may integrate with:

grammar/distributed/
grammar/networking/
grammar/concurrency/

They may describe:

- service contracts;
- remote functions;
- messages;
- streams;
- endpoints;
- serialization;
- consistency;
- fault behavior.

Node counts and topology sizes remain resource/target properties.

---

93. Actor Integration

Foreign actors must use the canonical actor/concurrency model.

The architecture is:

foreign actor boundary
      ↓
canonical actor semantic model
      ↓
message
      ↓
channel
      ↓
task
      ↓
scheduler

Interoperability must not create a second actor runtime.

---

94. AI Agent Integration

Foreign AI agents may interoperate through the canonical agent/concurrency architecture.

The relationship is:

AI agent
      ↓
canonical agent semantics
      ↓
actor/task/message model
      ↓
runtime

AI-specific behavior remains in the AI semantic layer.

Concurrency remains in the concurrency layer.

---

95. Knowledge and Reasoning Integration

External knowledge/reasoning systems may expose:

facts
queries
evidence
claims
confidence
provenance
decisions

These must map into canonical Zamani semantic structures.

Interoperability does not create a second knowledge model.

---

96. Learning and Adaptation Integration

External learning systems may expose model-training or adaptation boundaries.

Such operations must participate in:

effects
capabilities
resources
policies
contracts
provenance

Adaptation must remain controlled and authorized.

An external learning runtime must not obtain unrestricted program-modification authority merely through an interoperability declaration.

---

97. Contract With "grammar/effects/"

Interoperability consumes the effect system.

Required integration:

interoperability/*.g4
        ↓
effect references
        ↓
grammar/effects/
        ↓
effect analysis

The interoperability subsystem must not duplicate effect definitions.

---

98. Contract With "grammar/resources/"

Interoperability consumes the universal resource model.

Required integration:

foreign declaration
      ↓
requirements
      ↓
capabilities
      ↓
constraints
      ↓
resource analysis
      ↓
target feasibility

No foreign-language-specific resource universe is permitted.

---

99. Contract With "grammar/security/"

Required integration:

foreign declaration
      ↓
security requirements
      ↓
capability validation
      ↓
policy validation
      ↓
authorization

Security must be enforced downstream.

---

100. Contract With "grammar/validation/"

Validation must verify:

specification
 ↔ grammar
 ↔ lexer
 ↔ parser
 ↔ AST
 ↔ semantic model
 ↔ IR
 ↔ compiler
 ↔ runtime
 ↔ tests

Interoperability validation must include:

- ownership validation;
- dependency validation;
- compatibility validation;
- semantic-loss validation;
- hard-coding audit;
- determinism validation;
- scalability validation.

---

101. Contract With "grammar/compatibility/"

Compatibility owns cross-version compatibility.

Interoperability supplies:

foreign language version
foreign format version
ABI version
runtime version
dialect version

where applicable.

The compatibility subsystem determines compatibility relationships.

---

102. Contract With "grammar/dialects/"

External formats may be implemented as controlled dialects.

Dialect loading must be explicit and deterministic.

A dialect must declare:

identity
version
owner
syntax
AST mapping
semantic mapping
IR mapping
compatibility
capabilities
effects
resources
security
tests

---

103. Contract With "grammar/modules/"

Foreign modules must integrate with the canonical module system.

Interoperability must not create another import/export mechanism.

The semantic model must distinguish:

Zamani module
foreign module
foreign symbol
foreign runtime

while preserving one module architecture.

---

104. Contract With "grammar/functions/"

Foreign functions must map to the canonical function model.

There must not be separate permanent function universes for each external language.

---

105. Contract With "grammar/types/"

Foreign types must integrate with the canonical type system.

The architecture must distinguish:

Zamani type
foreign type
opaque foreign type
representation contract
conversion
ABI realization

but retain one semantic type framework.

---

106. Contract With "grammar/memory/"

Ownership and lifetime declarations must integrate with the canonical memory model.

The interoperability layer declares boundaries.

The memory subsystem validates them.

---

107. Contract With "grammar/quantum/"

Quantum external formats must map into the canonical quantum semantic model and then:

quantum::ir

No external quantum representation may establish an independent permanent quantum pipeline.

---

108. Contract With "grammar/hdl/"

HDL formats must map into the canonical HDL/hardware semantic architecture.

External HDL does not replace native Zamani hardware intent.

---

109. Contract With "grammar/hardware/"

Hardware-specific interoperability must use:

capability
resource
topology
placement
performance
power
thermal
reliability

as semantic concepts where appropriate.

Physical realization remains downstream.

---

110. Contract With "grammar/execution/"

Runtime behavior belongs to the execution architecture.

Interoperability supplies execution requirements and boundary contracts.

Execution determines realization.

---

111. Contract With "grammar/compile/"

Compilation owns:

- specialization;
- lowering;
- optimization;
- artifact production;
- target realization.

Interoperability supplies the external contracts required for those operations.

---

112. Contract With "grammar/data/"

Data interchange uses canonical data types, schemas, queries, graphs, streams, and provenance.

Interoperability must not duplicate those semantic concepts.

---

113. Required Feature Contract

Every interoperability ".g4" file must document:

Purpose
Owns
Does Not Own
Public Rules
Private Rules
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
Provenance Integration
IR Destination
Compiler Integration
Runtime Integration
Diagnostics
Security
Compatibility
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Completion Criteria

This makes each file independently completable.

---

114. Required Dependency Contract

Every stable interoperability grammar file must identify:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
CAPABILITY_OWNER:
RESOURCE_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
COMPILER_CONSUMER:
RUNTIME_CONSUMER:
SPEC_OWNER:
TEST_OWNER:
COMPATIBILITY_OWNER:

The declarations are architectural metadata.

They prevent accidental ownership duplication.

---

115. Example Dependency Contract

For:

grammar/interoperability/ffi.g4

the contract is conceptually:

DEPENDS_ON:
    core names/identifiers
    types
    functions
    effects
    resources
    security
    validation

EXPORTS:
    foreignInterface
    foreignFunctionBinding
    foreignBoundary

CONSUMED_BY:
    interoperability.g4
    ZamaniParser.g4

AST_OWNER:
    domain-neutral frontend AST

SEMANTIC_OWNER:
    interoperability semantic analysis

TYPE_OWNER:
    canonical type system

EFFECT_OWNER:
    grammar/effects/

CAPABILITY_OWNER:
    grammar/resources/ and security/

RESOURCE_OWNER:
    grammar/resources/

POLICY_OWNER:
    grammar/policies/ and security/

PROVENANCE_OWNER:
    canonical provenance subsystem

IR_OWNER:
    canonical semantic/domain IR

COMPILER_CONSUMER:
    compile/lowering/backend

RUNTIME_CONSUMER:
    execution/runtime/FFI infrastructure

SPEC_OWNER:
    grammar/spec/ and grammar/specification/

TEST_OWNER:
    grammar/tests/ interoperability suites

COMPATIBILITY_OWNER:
    grammar/compatibility/

---

116. File Independence Rule

A file is not complete merely because its parser rules work.

A file is complete when its ownership and downstream contracts are fixed sufficiently that ordinary changes elsewhere do not require reopening the file merely to discover what the file was supposed to mean.

A later semantic implementation may expose a genuine design defect.

That is a new architectural issue, not an excuse for leaving file ownership undefined.

---

117. No Circular Grammar Ownership

The grammar dependency direction must remain acyclic.

Required direction:

universal grammar
      ↓
domain grammar
      ↓
specialized interoperability grammar

Never:

FFI → foreign language → FFI

or:

quantum format → quantum grammar → format grammar → quantum grammar

Composition must always have a clear owner.

---

118. No Duplicate Token Authorities

Interoperability files must not independently invent duplicate lexer tokens.

Token ownership belongs to the canonical lexical architecture.

If a foreign technology requires a keyword, it must be evaluated against the lexical design before becoming reserved syntax.

Where ordinary identifiers are sufficient, they should remain identifiers.

---

119. External Names

External names must support symbolic identity without imposing universal limits.

A name may contain:

namespace
module
interface
symbol
version
vendor
organization
format
language

The canonical name grammar determines exact syntax.

---

120. Vendor Extensions

Vendor-specific interoperability must be namespaced.

Conceptually:

vendor::language
vendor::abi
vendor::format
vendor::runtime
vendor::extension

Vendor extensions must not silently become universal Zamani semantics.

---

121. Open-World Architecture

The interoperability architecture is explicitly open-world.

It must support future:

- languages;
- ABIs;
- calling conventions;
- runtimes;
- formats;
- data systems;
- quantum technologies;
- hardware;
- accelerators;
- execution substrates.

The architecture must evolve by adding adapters and semantic mappings rather than by continually redesigning the universal core.

---

122. Security Boundary for Foreign Code

Foreign code is untrusted unless the canonical security model establishes otherwise.

A foreign declaration must not itself grant:

native execution
filesystem access
network access
device access
privileged system access

Each capability must be explicit.

---

123. Capability Negotiation

Capability negotiation occurs after parsing.

The architecture is:

source requirement
      ↓
semantic requirement
      ↓
capability request
      ↓
available target capabilities
      ↓
negotiation
      ↓
feasible realization

The parser must not perform negotiation.

---

124. Resource Negotiation

Likewise:

foreign operation
      ↓
resource requirement
      ↓
resource analysis
      ↓
available resources
      ↓
feasibility

No resource discovery belongs inside the grammar.

---

125. Target Adaptation

A valid interoperability declaration may be realized differently on different targets.

For example:

same foreign semantic contract
        ↓
different ABI realization
        ↓
different runtime
        ↓
different target

This is essential to POCO-REAF.

---

126. No Silent Semantic Substitution

The compiler must not silently replace one foreign contract with another merely because the replacement is available.

For example:

C ABI

must not silently become:

different ABI

unless the compatibility architecture explicitly proves equivalence.

Likewise:

OpenQASM semantics

must not silently become a different quantum semantic contract.

---

127. Adaptation and Fallback

Fallback may be used when explicitly allowed by:

- policy;
- contract;
- capability model;
- compatibility model.

Possible execution outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

The interoperability grammar declares relevant intent.

Execution/resilience infrastructure determines actual behavior.

---

128. Simulation Boundary

A foreign boundary may be simulated.

The architecture remains:

foreign contract
      ↓
semantic model
      ↓
simulation execution strategy

Simulation must not create a different language meaning.

---

129. Reproducibility

Interoperability builds must preserve enough information to reproduce the same semantic result where deterministic reproduction is promised.

Relevant metadata includes:

- language version;
- format version;
- ABI;
- calling convention;
- dialect versions;
- conversion policy;
- compiler version;
- semantic configuration;
- provenance;
- artifact identity.

---

130. Artifact Boundaries

Interoperability may produce or consume:

- source files;
- object files;
- libraries;
- modules;
- data artifacts;
- Wasm artifacts;
- quantum representations;
- HDL artifacts;
- serialized data.

Artifacts are downstream products.

The grammar does not define physical artifact storage behavior.

---

131. ABI and Artifact Separation

The architecture must distinguish:

source contract
ABI contract
artifact format
runtime contract
target realization

These may have independent versions.

One must not be inferred from another without an explicit semantic rule.

---

132. Language/Runtime Separation

The following are distinct:

language
format
ABI
runtime
compiler
linker
hardware

For example:

Python

is not the same thing as:

Python runtime

and:

C ABI

is not the same thing as:

C compiler

This distinction must remain throughout the architecture.

---

133. Parser/Compiler Separation

Parsing recognizes declarations.

Compilation determines realization.

Therefore:

parse
 ≠
link
 ≠
compile
 ≠
execute

The interoperability grammar must preserve these boundaries.

---

134. Parser/Hardware Separation

Parsing must not depend on hardware.

The same source must produce the same parse result whether the target is:

absent
tiny
large
quantum
heterogeneous
distributed
future

provided the selected language/dialect configuration is the same.

---

135. AST Domain Neutrality

Interoperability AST nodes must remain domain-neutral.

They may identify that a declaration refers to:

foreign language
foreign format
ABI
runtime

but must not directly contain backend implementation structures such as:

LLVM internals
vendor-specific physical topology
physical qubit maps
register allocation
routing schedules
calibration tables
machine-specific instruction selection

Those belong downstream.

---

136. Provenance Preservation

Source spans must survive:

source
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
lowering
 ↓
artifact

where the relevant infrastructure supports provenance.

This is essential for diagnostics and reproducibility.

---

137. Explainability

Interoperability decisions may require explanations.

Examples:

why this ABI was selected
why a conversion was inserted
why a target was rejected
why a capability was required
why semantic loss was reported
why a fallback was selected

Explanation is a semantic/tooling concern.

The interoperability grammar provides metadata needed to explain boundary decisions.

---

138. Evidence

Interoperability decisions may carry evidence.

Evidence may identify:

claim
source
verification
confidence
provenance
decision

Evidence integrates with the common provenance/validation architecture.

---

139. Knowledge Integration

Foreign schema and metadata systems may contribute knowledge about:

- interfaces;
- symbols;
- versions;
- capabilities;
- formats;
- compatibility.

Such knowledge is consumed by semantic/tooling systems.

It must not alter parsing nondeterministically.

---

140. Controlled Reflection

Reflection may inspect interoperability declarations only under the canonical reflection/capability model.

Reflection must not become an implicit mechanism for:

- arbitrary foreign code execution;
- arbitrary library loading;
- arbitrary target discovery;
- arbitrary privilege acquisition.

---

141. Testing Architecture

Interoperability tests must exist across:

grammar/tests/

and the repository's established test structure.

Required categories include:

lexical
parser
AST
semantic
type
effect
capability
resource
contract
policy
provenance
security
compatibility
quantum
HDL
classical
distributed
networking
data
dialect
runtime boundary
negative
boundary
scalability
determinism

---

142. Positive Tests

Positive tests must verify valid declarations for:

- foreign functions;
- foreign types;
- ABIs;
- calling conventions;
- symbols;
- conversions;
- callbacks;
- serialization;
- C;
- C++;
- Rust;
- Python;
- Zig;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- system interfaces;
- future symbolic identities.

---

143. Negative Tests

Negative tests must verify rejection of:

- malformed foreign identities;
- malformed ABI declarations;
- invalid calling conventions;
- invalid type boundaries;
- invalid ownership;
- invalid lifetimes;
- incompatible conversions;
- unsupported versions;
- conflicting policies;
- missing capabilities;
- invalid contracts;
- semantic-loss violations;
- malformed external formats.

---

144. Boundary Tests

Boundary tests must cover combinations such as:

foreign function + generic type
foreign function + async
foreign function + ownership
foreign function + capability
foreign function + resource
foreign function + policy
foreign function + provenance
foreign quantum format + quantum::ir
foreign HDL + hardware semantics
foreign AI model + tensor
foreign service + networking
foreign data + serialization

---

145. Scalability Tests

Scalability tests must verify that the grammar architecture does not impose artificial limits.

Tests should exercise symbolically scalable constructs involving:

- many foreign declarations;
- many parameters;
- large type structures;
- large data structures;
- large graphs;
- large tensor descriptions;
- many modules;
- many interfaces;
- large distributed descriptions;
- large quantum programs;
- large HDL descriptions.

Tests must not confuse test fixture sizes with language capacity limits.

---

146. Determinism Tests

The same source plus the same explicit grammar/dialect configuration must produce the same parsing result.

Repeated parsing must not depend on:

- time;
- randomness;
- machine;
- filesystem;
- network;
- runtime;
- target availability.

---

147. Compatibility Tests

Compatibility tests must cover independent dimensions:

language version
format version
ABI version
runtime version
dialect version
semantic compatibility
artifact compatibility

Target feasibility must be tested separately from language compatibility.

---

148. Quantum Interoperability Tests

Required quantum tests include:

OpenQASM → AST
OpenQASM → semantic quantum model
OpenQASM → quantum::ir
QIR → quantum semantic model
QIR → quantum::ir
quantum operation preservation
parameter preservation
measurement preservation
classical control preservation
resource requirement preservation
provenance preservation

No test may establish a second permanent quantum IR.

---

149. HDL Interoperability Tests

Required HDL tests include:

Verilog → AST
HDL → hardware semantics
HDL → HDL/hardware representation
simulation boundary
verification boundary
synthesis boundary
target realization

Hardware intent must remain semantically distinct from physical target selection.

---

150. Cross-Domain Test

A mandatory integration test must combine:

foreign function
+
foreign type
+
ABI
+
calling convention
+
resource requirement
+
capability
+
effect
+
contract
+
policy
+
provenance
+
classical computation
+
tensor/data
+
quantum operation
+
measurement
+
hybrid control
+
concurrency
+
networking or distributed execution
+
simulation
+
target realization

The expected pipeline is:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
structural validation
 ↓
type analysis
 ↓
effect analysis
 ↓
capability analysis
 ↓
resource analysis
 ↓
contract analysis
 ↓
policy analysis
 ↓
provenance
 ↓
semantic model
 ↓
classical IR / quantum::ir / HDL-hardware representation
 ↓
optimization
 ↓
lowering
 ↓
routing
 ↓
scheduling
 ↓
resilience/QEC where applicable
 ↓
ZQN where applicable
 ↓
HAL
 ↓
target realization

---

151. Hard-Coding Audit

Every interoperability change must be checked for accidental hard-coding.

Audit:

grammar files
parser files
lexer files
AST structures
semantic models
IR structures
validation
tests
generated artifacts
documentation
backend interfaces

Search for:

- fixed resource ceilings;
- fixed device counts;
- fixed topology assumptions;
- fixed register widths;
- fixed pointer widths;
- fixed tensor rank limits;
- fixed qubit counts;
- fixed node counts;
- fixed memory capacities.

A hard-coded implementation detail is not automatically invalid.

The question is whether it has been incorrectly promoted into universal language semantics.

---

152. Safe Rust Requirement

The Rust implementation consuming this grammar must use:

Rust 2021
Rust 1.97 or later
safe Rust

The interoperability architecture must not require memory-unsafe implementation mechanisms.

Grammar files themselves must remain declarative.

No target-language execution actions are permitted in the grammar.

---

153. Implementation Isolation

The grammar must not depend on:

- Rust implementation types;
- Rust memory layout;
- Rust pointer size;
- Rust register assumptions;
- Rust compiler internals.

Rust is an implementation technology.

Zamani semantics remain language-level contracts.

---

154. Build Integration

ANTLR generation must consume the canonical grammar hierarchy.

The build must ensure:

Zamani.g4
 ↓
ZamaniParser.g4
 ↓
interoperability composition
 ↓
interoperability leaf grammars

and:

Zamani.g4
 ↓
ZamaniLexer.g4
 ↓
canonical lexical hierarchy

remain consistent.

Generated artifacts must be reproducible.

---

155. Rust Frontend Integration

The generated/parser architecture must remain compatible with:

src/lexer.rs
src/parser.rs
src/ast/

The Rust frontend must not create a second interoperability syntax authority.

If the Rust frontend contains additional interoperability parsing logic, that logic must be treated as an implementation/conformance layer and must remain consistent with the canonical grammar specification.

---

156. Specification Integration

Each stable interoperability construct must be traceable to:

grammar/specification/

and, where appropriate:

grammar/spec/

The traceability chain is:

specification
 ↓
grammar
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
compiler
 ↓
runtime
 ↓
tests

No undocumented stable feature should be accepted as part of the production language.

---

157. Documentation Status

Documentation must distinguish:

NORMATIVE
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
EXPERIMENTAL
DEPRECATED
HISTORICAL
UNSUPPORTED

A design proposal must not silently become stable syntax.

"grammar/grammar.md" remains the implementation-conformance reference.

"grammar/Zamani-Grammar.md" remains historical/extended design material.

---

158. File Completion Standard

An interoperability file is DONE only when all of the following are established:

[ ] Purpose
[ ] Ownership
[ ] Non-ownership
[ ] Dependencies
[ ] Public rules
[ ] Lexer integration
[ ] Parser integration
[ ] AST mapping
[ ] Semantic mapping
[ ] Type mapping
[ ] Effect mapping
[ ] Capability mapping
[ ] Resource mapping
[ ] Contract mapping
[ ] Policy mapping
[ ] Provenance mapping
[ ] IR destination
[ ] Compiler consumer
[ ] Runtime consumer
[ ] Security model
[ ] Diagnostics
[ ] Compatibility
[ ] Positive tests
[ ] Negative tests
[ ] Boundary tests
[ ] Scalability tests
[ ] Determinism tests
[ ] Hard-coding audit
[ ] Specification traceability

---

159. Repository Integration Matrix

Interoperability concern| Primary owner| Consumes| Produces
Foreign language| "interoperability/"| names/core| language identity
Foreign format| "interoperability/" / dialects| names/core| format identity
FFI| "ffi.g4"| functions/types| foreign callable contract
Foreign functions| "foreign-functions.g4"| functions/types| callable boundary
Foreign types| "foreign-types.g4"| types/memory| type boundary
ABI| "abi.g4"| core metadata| ABI contract
Calling convention| "calling-conventions.g4"| ABI/core| convention intent
Linkage| "linkage.g4"| symbols/modules| linkage intent
Symbols| "symbols.g4"| names/modules| symbol identity
Conversion| "conversions.g4"| types| conversion contract
Marshaling| "marshaling.g4"| data/types| boundary conversion
Callbacks| "callbacks.g4"| functions/concurrency| callback contract
Serialization| "serialization.g4"| data/types| serialization boundary
C| "c.g4"| generic interoperability| C boundary
C++| "cpp.g4"| generic interoperability| C++ boundary
Rust| "rust.g4"| generic interoperability| Rust boundary
Python| "python.g4"| generic interoperability| Python boundary
Zig| "zig.g4"| generic interoperability| Zig boundary
Assembly| "AssemblyLanguage.g4"| generic interoperability| assembly boundary
System APIs| system-interface grammar| FFI/security| system boundary
OpenQASM| "openqasm.g4"| quantum semantics| quantum boundary
QIR| "qir.g4"| quantum semantics| QIR boundary
WebAssembly| "wasm.g4"| canonical semantics| Wasm boundary
HDL| "hdl.g4" / "verilog.g4"| HDL semantics| HDL boundary
Effects| "grammar/effects/"| interoperability metadata| effect analysis
Resources| "grammar/resources/"| requirements| resource analysis
Security| "grammar/security/"| capabilities/effects| authorization
Policies| "grammar/policies/"| requirements| policy analysis
Compatibility| "grammar/compatibility/"| version metadata| compatibility result
Provenance| canonical provenance subsystem| source metadata| provenance
Classical IR| classical compiler| semantic model| classical IR
Quantum IR| "quantum::ir"| quantum semantic model| quantum IR
HDL/hardware IR| hardware compiler| hardware semantics| hardware IR
ABI realization| backend/compiler| ABI contract| target ABI
Runtime| execution subsystem| validated boundary| execution
HAL| hardware subsystem| target contract| hardware realization

---

160. Required Integration Direction

The interoperability architecture must always follow:

SOURCE
  ↓
LEXER
  ↓
PARSER
  ↓
DOMAIN-NEUTRAL AST
  ↓
STRUCTURAL VALIDATION
  ↓
TYPE ANALYSIS
  ↓
EFFECT ANALYSIS
  ↓
CAPABILITY ANALYSIS
  ↓
RESOURCE ANALYSIS
  ↓
CONTRACT ANALYSIS
  ↓
POLICY ANALYSIS
  ↓
PROVENANCE
  ↓
SEMANTIC MODEL
  ↓
CANONICAL IR / DOMAIN IR
  ↓
OPTIMIZATION
  ↓
LOWERING
  ↓
ABI / ROUTING / SCHEDULING
  ↓
RESILIENCE / QEC / ZQN WHERE APPLICABLE
  ↓
HAL
  ↓
TARGET REALIZATION
  ↓
RUNTIME

Interoperability must never skip directly from:

source

to:

runtime

or:

source

to:

hardware

---

161. Reverse Traceability

Every backend interoperability requirement must be traceable backward:

target requirement
      ↓
backend requirement
      ↓
IR requirement
      ↓
semantic requirement
      ↓
AST representation
      ↓
grammar rule
      ↓
source construct

If no source representation is required, the requirement must remain downstream.

This prevents backend implementation details from leaking into the source grammar.

---

162. What Must Never Become Core Syntax

Application-specific concepts must remain outside the universal interoperability grammar unless they represent a genuinely universal language primitive.

Examples that normally belong in libraries, dialects, APIs, services, or applications include:

specific computer-vision operations
specific sentiment operations
specific robotics commands
specific payment operations
specific administrative operations
specific legal workflows
specific blockchain operations
specific VR/AR operations
specific enterprise workflows
specific vendor products
specific cloud products
specific model names

The universal language should provide the primitives needed to build these systems rather than encoding every application into the grammar.

---

163. Interoperability and Universal Computation

The ultimate model is:

                    ZAMANI PROGRAM
                          │
                          ▼
                 PORTABLE SEMANTICS
                          │
                          ▼
                DOMAIN-NEUTRAL AST
                          │
                          ▼
              SEMANTIC VALIDATION
                          │
        ┌─────────────────┼──────────────────┐
        │                 │                  │
        ▼                 ▼                  ▼
      Types            Effects           Resources
        │                 │                  │
        └─────────────────┼──────────────────┘
                          │
                          ▼
                 Canonical Semantics
                          │
          ┌───────────────┼────────────────┐
          │               │                │
          ▼               ▼                ▼
      Classical       quantum::ir     HDL/Hardware
          │               │                │
          └───────────────┼────────────────┘
                          │
                          ▼
                    Optimization
                          │
                          ▼
                      Lowering
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
         ABI           Routing         Scheduling
          │               │                │
          └───────────────┼────────────────┘
                          │
                     Resilience
                          │
                    QEC / ZQN
                          │
                          ▼
                         HAL
                          │
                          ▼
                  TARGET REALIZATION

Interoperability is the boundary between portable Zamani meaning and external computational ecosystems.

---

164. POCO-REAF Interoperability Guarantee

The desired invariant is:

                    SAME ZAMANI SOURCE
                            │
              ┌─────────────┼─────────────┐
              │             │             │
              ▼             ▼             ▼
          small target   large target   heterogeneous
              │             │             │
              └─────────────┼─────────────┘
                            │
                       accelerator
                            │
                          QPU
                            │
                       distributed
                            │
                           HPC
                            │
                          cloud
                            │
                    future substrate

The source program remains the semantic contract.

The realization may change.

---

165. Production Readiness Gate

"grammar/interoperability/" must not be considered production-ready merely because its ".g4" files parse.

Production readiness requires:

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
PROVENANCE
     ↓
CANONICAL IR
     ↓
COMPILER
     ↓
ABI / LOWERING
     ↓
RUNTIME
     ↓
TARGET
     ↓
TESTS

Every stable feature must have this traceability.

---

166. Production Readiness Checklist

[ ] One interoperability composition authority
[ ] No competing parser root
[ ] No competing lexer authority
[ ] No duplicated universal grammar
[ ] Foreign language identity is extensible
[ ] Foreign format identity is extensible
[ ] Language and format are distinct
[ ] ABI and calling convention are distinct
[ ] Language and runtime are distinct
[ ] Foreign types use the canonical type system
[ ] Foreign functions use the canonical function system
[ ] Ownership uses the canonical memory model
[ ] Lifetime uses the canonical lifetime model
[ ] Effects use the canonical effect model
[ ] Capabilities use the canonical capability model
[ ] Resources use the canonical resource model
[ ] Contracts use the canonical contract model
[ ] Policies use the canonical policy model
[ ] Provenance uses the canonical provenance model
[ ] Security uses the canonical security model
[ ] Compatibility uses the canonical compatibility model
[ ] AST remains domain-neutral
[ ] No interoperability IR is introduced
[ ] Quantum interoperability converges on quantum::ir
[ ] HDL interoperability converges on hardware/HDL semantics
[ ] ABI realization remains downstream
[ ] Runtime execution remains downstream
[ ] Hardware discovery remains downstream
[ ] Target selection remains downstream
[ ] Routing remains downstream
[ ] Scheduling remains downstream
[ ] QEC remains downstream
[ ] ZQN remains downstream
[ ] HAL remains downstream
[ ] No universal capacity ceilings
[ ] No fixed universal pointer width
[ ] No fixed universal register width
[ ] No fixed universal topology
[ ] No target-dependent parsing
[ ] No grammar execution actions
[ ] Deterministic parsing
[ ] Safe Rust implementation
[ ] Rust 1.97+ compatibility
[ ] Source spans preserved
[ ] Diagnostics defined
[ ] Security behavior defined
[ ] Compatibility behavior defined
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Cross-domain tests exist
[ ] Hard-coding audit passes
[ ] Specification traceability exists

---

167. Definition of Done for Each Interoperability File

A file is complete only when:

one owner
    ↓
one purpose
    ↓
explicit dependencies
    ↓
explicit exports
    ↓
explicit AST mapping
    ↓
explicit semantic mapping
    ↓
explicit type/effect/capability/resource mapping
    ↓
explicit contract/policy/provenance mapping
    ↓
explicit IR destination
    ↓
explicit compiler consumer
    ↓
explicit runtime consumer
    ↓
explicit diagnostics
    ↓
explicit security
    ↓
explicit compatibility
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
determinism tests
    ↓
hard-coding audit

Only then is the file considered complete.

---

168. Final Architectural Invariants

The following invariants are permanent:

1. "grammar/Zamani.g4" is the canonical ANTLR composition root.

2. "grammar/antlr/ZamaniParser.g4" is the parser composition authority.

3. "grammar/antlr/ZamaniLexer.g4" is the lexer composition authority.

4. "grammar/interoperability/" does not create another root grammar.

5. Universal grammar constructs are not duplicated inside interoperability grammars.

6. Foreign languages are symbolic, extensible identities.

7. Foreign formats are distinct from languages.

8. ABIs are distinct from calling conventions.

9. Runtimes are distinct from languages.

10. Foreign symbols are semantic identities, not physical addresses.

11. FFI declarations do not execute foreign code.

12. Parsing never loads foreign libraries.

13. Parsing never performs filesystem access.

14. Parsing never performs network access.

15. Parsing never performs hardware discovery.

16. Parsing never invokes a linker.

17. Parsing never invokes a compiler.

18. Parsing never invokes a runtime.

19. Foreign types use the canonical type system.

20. Foreign functions use the canonical function model.

21. Foreign modules use the canonical module model.

22. Foreign ownership uses the canonical memory model.

23. Foreign effects use the canonical effect model.

24. Foreign capabilities use the canonical capability model.

25. Foreign resources use the canonical resource model.

26. Foreign security uses the canonical security model.

27. Foreign policies use the canonical policy model.

28. Foreign compatibility uses the canonical compatibility model.

29. Foreign provenance uses the canonical provenance model.

30. AST representations remain domain-neutral.

31. Interoperability does not create a universal interoperability IR.

32. Quantum interoperability converges on "quantum::ir".

33. OpenQASM does not replace "quantum::ir".

34. QIR does not replace "quantum::ir".

35. HDL interoperability does not replace native Zamani HDL semantics.

36. ABI implementation remains downstream.

37. Linker behavior remains downstream.

38. Runtime execution remains downstream.

39. Hardware discovery remains downstream.

40. Target selection remains downstream.

41. Routing remains downstream.

42. Scheduling remains downstream.

43. Resilience remains downstream.

44. QEC remains downstream.

45. ZQN remains downstream.

46. HAL remains downstream.

47. No universal hardware capacity is hard-coded.

48. No universal CPU, GPU, FPGA, accelerator, QPU, node, thread, memory, device, tensor, network, or qubit ceiling is encoded.

49. No universal pointer width is assumed.

50. No universal register width is assumed.

51. No universal physical topology is assumed.

52. Numeric literals remain program semantics.

53. Requirements are distinct from capabilities.

54. Capabilities are distinct from resources.

55. Constraints are distinct from preferences.

56. Preferences are distinct from implementation decisions.

57. Target infeasibility is distinct from language invalidity.

58. Semantic loss must not be silent.

59. Compatibility dimensions must remain explicit.

60. Dialects must remain controlled extensions.

61. Macros cannot bypass interoperability validation.

62. Metaprogramming cannot bypass interoperability validation.

63. Reflection cannot bypass capability or security validation.

64. External formats cannot silently become core syntax.

65. Vendor extensions cannot silently become universal semantics.

66. Parsing is deterministic.

67. Source spans are preserved.

68. Provenance is preserved where required.

69. Stable features have specification-to-test traceability.

70. The Rust implementation remains compatible with Rust 1.97+.

71. The Rust implementation uses safe Rust only.

72. Interoperability must remain open to future languages, formats, runtimes, hardware, and computational substrates.

73. The same semantic source contract must remain usable across different target realizations whenever the target satisfies its requirements.

---

169. Final Architecture

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
             ┌────────────────┼─────────────────┐
             │                │                 │
             ▼                ▼                 ▼
           FFI          Foreign Types      Foreign Functions
             │                │                 │
             ├──────────┬─────┴──────┬──────────┤
             │          │            │          │
             ▼          ▼            ▼          ▼
            ABI     Calling       Symbols   Conversions
                    Convention
             │          │            │          │
             └──────────┴────────────┴──────────┘
                              │
                              ▼
                     Domain-Neutral AST
                              │
                              ▼
                    Semantic Validation
                              │
       ┌──────────────────────┼──────────────────────┐
       │                      │                      │
       ▼                      ▼                      ▼
     Types                  Effects              Capabilities
       │                      │                      │
       └──────────────────────┼──────────────────────┘
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
                         Provenance
                              │
                              ▼
                    Canonical Semantics
                              │
            ┌─────────────────┼─────────────────┐
            │                 │                 │
            ▼                 ▼                 ▼
        Classical        quantum::ir       HDL/Hardware
            │                 │                 │
            └─────────────────┼─────────────────┘
                              │
                              ▼
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
          ┌───────────────────┼────────────────────┐
          │                   │                    │
         CPU                 GPU                  FPGA
          │                   │                    │
          ├───────────────────┼────────────────────┤
          │                   │                    │
         ASIC               QPU              Accelerator
          │                   │                    │
          └───────────────────┼────────────────────┘
                              │
                    Distributed / HPC
                              │
                            Cloud
                              │
                              ▼
                     Future Substrates

---

170. Final Principle

The permanent interoperability principle is:

«Zamani interoperability describes the contract between portable Zamani semantics and an external computational ecosystem. It does not turn that ecosystem into the permanent language core, and it does not encode present-day implementation limitations into future source programs.»

The programmer describes:

WHAT is required
WHAT is guaranteed
WHAT is permitted
WHAT is prohibited
WHAT is interoperable

The compiler and runtime determine:

HOW it is realized
WHERE it is realized
WHEN it is realized
WHICH ABI is used
WHICH runtime is used
WHICH resources are used
WHICH hardware realizes it

Therefore the final model remains:

                  ONE ZAMANI PROGRAM
                         │
                         ▼
                ONE SEMANTIC CONTRACT
                         │
                         ▼
                   ONE AST MODEL
                         │
                         ▼
                ONE SEMANTIC ARCHITECTURE
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      Classical      quantum::ir   HDL/Hardware
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  TARGET-INDEPENDENT
                    OPTIMIZATION
                         │
                         ▼
                      LOWERING
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
            ABI       Routing    Scheduling
             │           │           │
             └───────────┼───────────┘
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
             ┌───────────┼───────────┐
             ▼           ▼           ▼
            CPU         GPU          QPU
             │           │           │
             └───────────┼───────────┘
                         ▼
                DISTRIBUTED / HPC
                         │
                       CLOUD
                         │
                         ▼
                 FUTURE HARDWARE

This is the required interoperability foundation for Zamani to remain a single, extensible programming language while supporting foreign languages, ABIs, runtimes, data formats, quantum ecosystems, HDL ecosystems, AI systems, distributed systems, and future computational substrates under the POCO-REAF architecture.
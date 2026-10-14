Worked for 28s

# Zamani Universal Intermediate Representation (ZUIR) Specification

**Canonical path:** `src/zuir/ZUIR-SPEC.md`
**Repository:** `github.com/Benwellonedge28/Zamani`
**Status:** Normative architectural and implementation contract
**Language:** Zamani
**Rust edition:** 2021
**Minimum Rust version:** Rust 1.97.1 or later
**Safety policy:** Safe Rust only; `unsafe` is prohibited
**Primary objective:** Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
**Scope:** Universal computational representation, semantic preservation, extensibility, verification, serialization, transformation, lowering, interoperability, scalability, and repository integration.

---

## 1. Purpose

This document defines the normative architecture and implementation requirements for Zamani's Universal Intermediate Representation (ZUIR).

ZUIR is the canonical, target-independent intermediate representation for computational meaning established by Zamani's frontend and semantic analysis.

It MUST support the representation and integration of:

- Classical, systems, embedded, scientific, and numerical computation.
- Quantum computation, quantum simulation, logical quantum computation, and hybrid quantum-classical execution.
- Hardware description, digital and other supported hardware models, and hardware/software co-design.
- AI, machine learning, neural-symbolic computation, and model execution.
- Tensor, array, stream, data, and symbolic computation.
- Parallel, concurrent, distributed, and high-performance computation.
- Networking, communication, cryptography, and secure computation.
- Temporal, reactive, event-driven, and time-constrained computation.
- Metaprogramming, reflection, controlled adaptation, and future computational domains.
- New computational models that have not yet been implemented or standardized.

ZUIR MUST enable one source program to retain its meaning as it is compiled, optimized, partitioned, distributed, mapped, and executed across compatible environments.

The same program MUST NOT require its source semantics to be rewritten merely because a target has a different number of processors, a different memory capacity, a different quantum architecture, a different network topology, or a new accelerator technology.

This specification defines the required architecture. It does not claim that every specified feature, module, adapter, or backend already exists.

## 2. Normative language

The terms MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY express normative requirements.

- **MUST / MUST NOT:** Mandatory for conformance.
- **SHOULD / SHOULD NOT:** Expected unless a documented, justified exception exists.
- **MAY:** Optional, subject to the stated contracts.

A proposed feature is not implemented merely because it appears in this document. Implementation status MUST be established by source code, tests, and integration evidence.

## 3. Authority and repository integration

### 3.1 Specification authority

ZUIR MUST conform to the repository's established language and architecture contracts.

The governing documents and components are:

| Authority | Responsibility |
|---|---|
| `grammar/DESIGN.md` | Grammar architecture, repository-wide language boundaries, safety, and ownership rules. |
| `grammar/specification/README.md` | Normative language-specification structure and authority. |
| `grammar/specification/semantic-model.md` | Source-level semantic meaning and semantic boundaries. |
| `grammar/specification/compatibility.md` | Language and compatibility policy. |
| `grammar/specification/compilation-model.md` | Compilation stages and their relationships. |
| `grammar/specification/execution-model.md` | Execution semantics and execution boundaries. |
| `grammar/specification/domains.md` | Computational-domain responsibilities. |
| `grammar/specification/extensibility.md`, if present | Extension and dialect architecture. |
| `grammar/Zamani.g4` | Root grammar composition contract. |
| `grammar/antlr/ZamaniLexer.g4` | Public ANTLR lexer boundary. |
| `grammar/antlr/ZamaniParser.g4` | Public ANTLR parser composition boundary. |
| `grammar/grammar.md` | Grammar implementation-conformance reference. |
| `grammar/Zamani-Grammar.md` | Extended language-design reference; proposals do not automatically become legal syntax. |
| `src/frontend/ast/` | Canonical frontend AST ownership, subject to the repository's documented migration state. |
| `src/frontend/semantic/` | Native semantic-analysis API and semantic model. |
| `src/zuir/` | Canonical ZUIR implementation and public API. |
| `src/quantum/ir/` | Existing canonical quantum IR and its compatibility contracts. |
| Existing classical and HDL IR implementations | Their respective domain-specific representations and lowering contracts. |

This document owns ZUIR-specific representation semantics. It MUST NOT redefine the source language, replace the semantic model, or silently override an established domain IR contract.

If an existing specification conflicts with this document, implementation MUST stop at the affected boundary until the conflict is resolved through an explicit, reviewed specification change. The conflict MUST NOT be hidden by choosing whichever document is easiest to implement.

### 3.2 Canonical pipeline

The intended pipeline is:

```text
Zamani source / supported imported formats
                  |
                  v
            Lexer and parser
                  |
                  v
          Canonical frontend AST
                  |
                  v
          Semantic analysis
                  |
                  v
          Validated semantic model
                  |
                  v
                 ZUIR
                  |
          +-------+--------+
          |       |        |
          v       v        v
       Classical Quantum  HDL/
          IR      IR      hardware IR
          |       |        |
          +-------+--------+
                  |
                  v
        Domain-aware optimization
                  |
                  v
       Legalization and lowering
                  |
                  v
       Mapping / routing / scheduling
                  |
                  v
     Resilience / QEC / ZQN / HAL
                  |
                  v
      Compatible target and runtime
```

This is a logical architecture, not a requirement that every compilation use every stage or every domain IR.

A compiler MAY omit irrelevant stages, introduce additional intermediate representations, or perform valid optimizations across stage boundaries. Every such change MUST preserve the defined semantics and must not create competing sources of truth.

### 3.3 AST, semantic model, and ZUIR ownership

The following distinctions are mandatory.

**AST**

Owns source structure, syntactic relationships, and source-oriented information. ZUIR MUST NOT become a second AST.

**Semantic model**

Owns resolved names, type meaning, effects, capabilities, resource requirements, contracts, ownership rules, and other established semantic facts.

**ZUIR**

Owns the canonical representation of validated computational meaning, its structural invariants, operation and value relationships, explicit control and data flow, representation metadata, and transformation contracts.

**Domain IRs**

Own specialized computational representations and their domain-specific invariants. They MUST receive sufficient semantic information to preserve program meaning.

**Target and runtime layers**

Own physical realization, target-specific layout, allocation, routing, scheduling, code generation, hardware interaction, deployment, and runtime services.

No layer may silently assume ownership of another layer's semantics.

---

## 4. Fundamental design principles

### 4.1 One canonical universal core

ZUIR MUST have one canonical core model for program identity, modules, symbols, types, values, operations, regions, control flow, effects, resources, provenance, and representation versions.

Computational domains MUST extend this model through explicit dialects, registered operation definitions, type definitions, attributes, and interfaces.

A new computational domain MUST NOT require a second universal IR or an unrelated parallel type system.

### 4.2 Open-ended computational extensibility

The universal core MUST NOT require every operation to be added to a permanent closed enumeration.

Operations MUST be identified by qualified names, for example:

```text
core.add
classical.call
quantum.operation
quantum.measure
hdl.signal_assign
tensor.contract
distributed.send
```

These are illustrative names, not a complete or permanently fixed instruction list.

A dialect MUST define the semantics, operands, results, attributes, regions, effects, verification rules, and version compatibility of its operations.

Unknown operations MUST NOT be assigned invented semantics. An implementation that lacks the required operation definition MUST preserve the operation as opaque data where permitted or reject the relevant processing stage with a structured diagnostic.

An implementation MUST NOT execute, optimize, or lower an opaque operation as if its meaning were known.

### 4.3 No universal hardware ceilings

ZUIR MUST NOT encode arbitrary universal limits on:

- Qubits or logical quantum resources.
- CPUs, cores, threads, GPUs, FPGAs, ASICs, QPUs, or accelerators.
- Processes, tasks, actors, modules, functions, blocks, regions, or operations.
- Memory, storage, bandwidth, or network size.
- Register width, vector width, tensor rank, tensor dimensions, or signal width.
- Devices, nodes, links, topology size, or execution timelines.

The implementation MUST use appropriately scalable collections, symbolic quantities, checked arithmetic, and explicit resource policies.

This does not prohibit finite target capabilities, finite integer types, configurable compiler budgets, or genuine protocol limits. Such constraints MUST be explicitly owned, scoped, and enforced by the relevant component.

### 4.4 Logical meaning is independent of physical realization

A logical resource requirement is not a physical allocation.

A logical qubit is not a physical qubit assignment. A logical tensor shape is not a fixed accelerator-memory layout. A logical signal width is not a particular board's register capacity. A logical parallel computation is not a fixed thread count.

ZUIR MUST retain these distinctions until the appropriate realization stage.

### 4.5 Semantic preservation

Every transformation MUST either:

1. Preserve the required observable behavior and all applicable semantic guarantees; or
2. Produce an explicit, structured failure stating why the transformation cannot legally be performed.

Silent semantic loss is forbidden.

### 4.6 Safe Rust

All ZUIR implementation code MUST use safe Rust and MUST declare the appropriate crate-level or module-level prohibition:

```rust
#![forbid(unsafe_code)]
```

Production ZUIR MUST NOT use unsafe blocks, unsafe functions, unsafe traits, or other unsafe Rust constructs.

The implementation MUST compile on the declared stable Rust baseline without relying on nightly-only features.

---

## 5. Scalability and POCO-REAF

### 5.1 Meaning of scalability

ZUIR MUST be capable of representing arbitrarily large finite computations within the limits of the representation and the resources available to the implementation.

It MUST NOT impose an arbitrary language-wide ceiling on computational size.

Literal infinity is not a feasible execution or storage guarantee. Every concrete compiler process, artifact, data structure, and physical target is finite. Therefore, scalability means that growth is governed by actual representational capabilities, available resources, declared constraints, and explicit policies—not by arbitrary universal hardware assumptions.

### 5.2 Program portability contract

A program's source semantics MUST remain stable across compatible targets.

Compilation for different targets MAY produce different physical implementations, but those implementations MUST preserve the program's defined observable behavior.

Differences in floating-point precision, concurrency ordering, timing, measurement outcomes, memory models, or numerical accuracy MUST be handled by explicit language and target contracts. They MUST NOT be concealed under an unconditional portability claim.

### 5.3 Resource model

ZUIR MUST distinguish:

- **Requirement:** A condition that must be satisfied.
- **Constraint:** A restriction on legal realizations.
- **Preference:** A non-mandatory selection criterion.
- **Hint:** Optimization guidance that cannot alter program semantics.
- **Budget:** A limit on a particular compilation, verification, transformation, or execution invocation.
- **Capability:** A property or service supported by an execution environment.
- **Allocation:** A concrete assignment of resources.
- **Observation:** A fact established by analysis or runtime measurement.

For example, source-level resource intent may include:

```text
requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(required_topology)
```

These expressions are illustrative representations of requirements, not claims about the currently implemented Zamani grammar.

The semantic model determines their meaning. ZUIR preserves the established requirement contract. Resource planning and target realization determine whether and how those requirements can be satisfied.

A preference MUST NOT be silently converted into a mandatory requirement. A hint MUST NOT alter program meaning. An unmet mandatory requirement MUST cause an explicit feasibility failure or a documented, semantically valid alternative.

### 5.4 Operational budgets

Implementations MAY enforce configurable budgets for:

- Input size and nesting.
- IR node counts and graph traversal.
- Memory consumption.
- Compilation time and cancellation.
- Constant evaluation.
- Verification and optimization.
- Serialization and deserialization.
- Extension processing.
- Incremental-cache size.

Budgets MUST be scoped to the relevant invocation or policy.

Exhaustion MUST produce a structured result such as `BudgetExceeded`, `Cancelled`, or `Incomplete`. An incomplete operation MUST NOT be reported as successfully verified.

The implementation MUST NOT translate a compiler budget into a universal limit on the Zamani language or the hardware that a program may target.

---

## 6. Canonical representation model

### 6.1 Required conceptual entities

The ZUIR schema MUST define the following conceptual entities. The final Rust type names MAY differ if the implementation documents an equivalent, unambiguous mapping.

| Entity | Required responsibility |
|---|---|
| Program | Representation identity, version, modules, requirements, and global metadata. |
| Module | Symbols, declarations, imports, exports, functions, and type definitions. |
| Symbol | Stable identity, linkage, visibility, and declaration association. |
| Function | Signature, parameters, results, regions, effects, and calling semantics. |
| Region | Nested computation, region arguments, capture rules, and control semantics. |
| Block | Ordered operations, block arguments, and terminator contract. |
| Operation | Qualified operation identity, operands, results, attributes, regions, effects, and provenance. |
| Value | A typed result, block argument, constant, or other explicitly defined value source. |
| Operand | A reference to a value with any required role or association. |
| Result | A value produced by an operation, with a defined type and identity. |
| Successor | A control-flow edge with target block and any required edge arguments. |
| Type | A primitive, aggregate, nominal, generic, symbolic, opaque, or dialect-defined type. |
| Attribute | Typed, validated operation or entity metadata. |
| Effect | A declaration of observable or semantically relevant behavior. |
| Resource requirement | A symbolic or concrete condition needed for legal execution. |
| Provenance | Source origins, imports, transformation history, and verification evidence. |
| Dialect manifest | A versioned definition of operations, types, attributes, interfaces, and dependencies. |

The schema MUST define ownership and reference integrity for every entity.

### 6.2 Identity and references

Entity identifiers MUST be type-safe. Program, module, function, block, region, operation, value, type, dialect, and symbol identities MUST NOT be freely interchangeable.

Identifiers MUST have a documented uniqueness scope.

The implementation MUST distinguish:

- Identity within an in-memory construction.
- Identity within a serialized artifact.
- Stable identity across transformations, where guaranteed.
- Canonical content identity or fingerprint.

A process-local identifier MUST NOT be presented as a stable cross-process identifier unless its stability is guaranteed by the contract.

Identity allocation MUST detect exhaustion or collision and return a structured error. Wrapping arithmetic MUST NOT silently produce duplicate identities.

### 6.3 Program and module structure

A program MUST identify the ZUIR schema version and declare the dialects and extension versions required to interpret it.

A module MUST define its symbol ownership and import/export rules.

A verified program MUST NOT contain unresolved required references, conflicting definitions, or invalid cross-module relationships.

Import resolution MUST be explicit. Loading external modules, plugins, files, or network resources MUST NOT occur as an implicit side effect of basic IR verification.

### 6.4 Functions and callable entities

The model MUST support ordinary functions and domain-specific callable entities through explicit contracts.

Function signatures MUST define parameter types, result types, calling semantics, effects, ownership requirements, and applicable attributes.

The core MUST NOT assume every callable is a conventional CPU function.

A quantum procedure, hardware process, distributed task, tensor computation, or extension-defined callable MUST be represented through the same foundational entity model while retaining its domain-specific semantics.

### 6.5 Regions and blocks

Regions MUST explicitly define their argument, capture, nesting, and control semantics.

Blocks MUST define their argument model and operation ordering.

Every block MUST satisfy its declared terminator contract. A block MUST NOT have ambiguous control-flow completion.

Structured control flow MAY be represented through regions and structured operations, explicit basic blocks, or a documented combination. The chosen representation MUST have one normative definition.

### 6.6 SSA and value semantics

The canonical core MUST use a formally specified value model.

For the initial canonical data-flow model, **SSA is the recommended default** because it makes definitions, data dependencies, and many transformations easier to verify. Adoption of SSA MUST be confirmed by the implementation contract rather than inferred from the legacy IR.

If SSA is adopted:

1. Each SSA value has exactly one definition.
2. Function parameters and block arguments are explicit definitions.
3. Control-flow merges use block arguments or a formally specified equivalent.
4. Every operand refers to a valid definition.
5. Uses satisfy the applicable dominance and region-scope rules.
6. Terminators and successor arguments satisfy the control-flow contract.
7. Effect ordering is represented explicitly where required.
8. Quantum ownership and linearity rules are enforced independently of ordinary SSA uniqueness.

The implementation MUST NOT assume that every value is copyable, unrestricted, or freely movable.

If the existing repository requires a different representation during migration, it MUST provide a documented adapter and conformance tests. It MUST NOT silently mix incompatible value models.

### 6.7 Def-use information

Def-use information MUST have one authoritative derivation rule.

An implementation MAY store def-use indexes as caches, but the canonical source of truth MUST remain unambiguous.

If indexes are cached, the verifier MUST be able to detect inconsistencies. Transformations MUST update or invalidate affected indexes according to a documented contract.

Stale cached information MUST NOT be used to justify an otherwise invalid transformation.

---

## 7. Types and values

### 7.1 Type-system boundary

ZUIR MUST consume established semantic types rather than reimplement source-level name resolution or type inference.

The IR type model MUST represent the types needed by supported computational domains and allow registered dialects to define additional types.

The core MUST support appropriate forms of:

- Primitive and arbitrary-precision or explicitly bounded numeric types.
- Boolean, character, string, and byte-oriented values.
- Product, record, struct, tuple, and aggregate types.
- Sum, enum, union, and variant types.
- Function and callable types.
- Reference, pointer, ownership, linear, and affine types where defined by the language.
- Generic, parameterized, and symbolic types.
- Opaque and nominal types.
- Arrays, vectors, tensors, streams, and symbolic shapes.
- Quantum, classical-quantum, and domain-specific resource types.
- HDL signals, clocked values, hardware aggregates, and other registered hardware types.
- Extension-defined types.

The presence of a type category in this list does not make it legal source syntax. Its source-level legality is governed by the language specification.

### 7.2 Type identity and compatibility

The schema MUST specify nominal versus structural identity and the rules for recursive types.

Type equality, assignability, conversion legality, and target representation compatibility MUST be separate queries.

Two types MUST NOT be considered semantically equivalent merely because a particular backend happens to represent them using the same machine type.

Target-specific physical layout, ABI layout, alignment, register placement, and memory representation MUST be determined downstream.

### 7.3 Symbolic types and constraints

The type model MUST allow symbolic parameters and constraints where required by generics, tensor shapes, hardware widths, and other scalable abstractions.

It MUST NOT require a universal maximum tensor rank, signal width, vector width, or number of generic parameters.

Constraint evaluation MUST distinguish satisfied, unsatisfied, unknown, unsupported, and incomplete outcomes where these distinctions are relevant.

An unknown constraint MUST NOT be silently treated as satisfied.

### 7.4 Numeric values

Numeric representation MUST avoid accidental dependence on the compiler host's native integer width.

The schema MUST distinguish exact values from approximations and preserve signedness, precision, rounding, overflow, and conversion semantics.

Arbitrary-precision integers MAY be used where appropriate. Bounded numeric types MUST explicitly specify their bounds and behavior.

Floating-point operations MUST follow a documented numerical contract, including applicable rounding, exceptional values, and reproducibility requirements.

An optimizer MUST NOT change numerical behavior beyond what the language and operation contracts permit.

### 7.5 Constants and symbolic expressions

Constants MUST have defined types and valid values.

Compile-time evaluation MUST be bounded by explicit policies. An expression that cannot be evaluated within its permitted budget MUST produce an explicit unresolved or incomplete result, not an invented constant.

Symbolic expressions MAY remain unresolved until a later legal stage.

The implementation MUST distinguish an unknown value from a known invalid value and from an unavailable computation result.

---

## 8. Operation and attribute contracts

### 8.1 Operation identity

Each operation MUST have a qualified identity consisting of a registered dialect namespace and an operation name.

Operation identity MUST NOT depend on a permanent global integer assignment or a closed enum containing every domain's operations.

The operation definition MUST specify:

- Operand count and constraints.
- Result count and types.
- Operand and result relationships.
- Attribute schema.
- Region structure and semantics.
- Terminator and control-flow behavior.
- Effects and ordering requirements.
- Ownership and resource constraints.
- Verification rules.
- Constant-folding and transformation contracts, where applicable.
- Lowering requirements and supported destinations.
- Versioning and compatibility behavior.

### 8.2 Attributes

Attributes MUST be typed and validated.

The schema MUST distinguish semantic attributes from descriptive metadata.

Semantic attributes MUST participate in verification, compatibility, and any transformation or fingerprint calculation required by their contract.

Descriptive metadata MUST NOT change program meaning unless its schema explicitly declares semantic significance.

Unknown extension attributes MAY be preserved if the enclosing format allows it. Unknown reserved attributes MUST be rejected when their semantics are required for correct interpretation.

### 8.3 Operation execution semantics

An operation MUST define the behavior needed to reason about its valid inputs, outputs, effects, ordering, and failure conditions.

An operation MUST NOT acquire meaning solely from a backend's implementation.

A backend may implement an operation differently on different targets, but the implementation MUST satisfy the operation's semantic contract.

---

## 9. Effects, ownership, and concurrency

### 9.1 Effects

ZUIR MUST represent the effects established by semantic analysis and operation definitions.

The effect system MUST be extensible. It MUST be capable of expressing applicable effects such as:

- Memory reads and writes.
- Allocation and deallocation.
- Input and output.
- Exceptions and error propagation.
- Synchronization and atomic operations.
- Concurrency and communication.
- Nondeterminism.
- Quantum measurement and other quantum observations.
- Hardware interaction.
- I/O and external services.
- Reflection, code generation, and controlled adaptation.
- Security-sensitive or authorization-dependent operations.

This is an illustrative set, not a closed universal effect enumeration.

Effects MUST remain distinct from capabilities, resource requirements, and authorization policies.

A declared effect does not automatically grant permission to perform it.

### 9.2 Ownership and linearity

The IR MUST preserve applicable ownership, borrowing, aliasing, lifetime, move, linear, and affine rules established by the language's semantic model.

Quantum resources MUST NOT be treated as ordinary copyable classical values unless their semantic definitions explicitly permit that operation.

Transformations MUST preserve ownership and resource-use rules.

A transformation that duplicates, removes, reorders, or merges operations MUST prove that it remains legal under the applicable ownership and effect contracts.

### 9.3 Concurrency and memory ordering

The IR MUST make semantically relevant ordering and synchronization requirements explicit.

Independent operations MAY be reordered only when their data, effect, ownership, timing, and domain-specific contracts permit it.

The core MUST NOT assume that all concurrent execution is sequentially consistent, that distributed communication is instantaneous, or that hardware signals have no timing constraints.

The memory model and concurrency guarantees MUST be defined by the relevant language and operation contracts.

---

## 10. Quantum computation

### 10.1 Domain boundary

ZUIR MUST support quantum computational meaning without replacing the repository's canonical quantum IR.

`src/quantum/ir/` remains the authoritative quantum IR boundary unless a reviewed repository-wide architecture change explicitly establishes another owner.

ZUIR-to-quantum-IR integration MUST use a versioned adapter or an explicitly documented shared representation.

It MUST NOT create a second incompatible `QubitId`, duplicate quantum type system, or competing quantum program authority.

### 10.2 Quantum semantics

The architecture MUST accommodate the supported quantum models, including applicable forms of:

- Circuit and dynamic quantum computation.
- Quantum operations with targets, controls, parameters, results, and attributes.
- Measurement and classical feedback.
- Hybrid quantum-classical control flow.
- Logical quantum resources.
- Analog and Hamiltonian-based computation.
- Annealing and other registered quantum models.
- Quantum simulation and symbolic quantum operations.
- Quantum error-correction and resilience annotations.
- Pulse-oriented semantic intent where supported.
- Distributed quantum computation and communication where supported.
- Future quantum operations introduced through registered dialects.

These capabilities MUST be represented by defined operation contracts and domain models rather than a permanent universal gate list.

The core MUST NOT require every quantum program to be represented as a flat vector of gates.

### 10.3 Measurement and nondeterminism

Measurement MUST preserve its defined semantic behavior, including classical results, dependencies, relevant effects, and ordering.

A compiler MUST NOT replace a measurement with a constant merely because one simulation or target configuration produced a particular outcome.

Probabilistic behavior MUST remain probabilistic unless a transformation is justified by the formal operation semantics and applicable assumptions.

### 10.4 Logical and physical resources

Logical qubit identities and requirements MUST remain distinct from physical qubit identities, calibration data, physical topology, and device-specific routing.

Physical allocation, routing, scheduling, pulse synthesis, and QEC realization MUST be performed by their designated downstream components.

The ZUIR core MUST NOT define a universal maximum qubit count or embed physical-device topology as an invariant of every program.

### 10.5 Quantum integration requirements

The adapter between ZUIR and `quantum::ir` MUST:

1. Define which operations and types it supports.
2. Preserve quantum semantics and effects.
3. Preserve measurement and control-flow dependencies.
4. Preserve logical resource identities and constraints.
5. Report unsupported operations explicitly.
6. Avoid duplicate canonical identities and registries.
7. Be versioned and tested against both representations.
8. Avoid silently substituting approximate or classical behavior for quantum behavior.

---

## 11. Classical computation

ZUIR MUST support classical operations through a registered classical dialect and the repository's canonical classical IR.

The classical dialect MUST define operation contracts for applicable arithmetic, comparison, Boolean logic, memory operations, calls, returns, branching, iteration, and other supported constructs.

It MUST specify numeric overflow, division-by-zero behavior, conversion rules, pointer and memory semantics, exceptions, and other behavior as required by the language contract.

The legacy `ZAMANI_IR_SPEC.md` describes a narrower representation containing operations such as `Add`, `Sub`, `Mul`, `Div`, `Call`, `Return`, and `Phi`, with primitive values such as `ConstInt(i64)` and `ConstFloat(f64)`.

That legacy representation MUST NOT constrain the universal ZUIR schema to a fixed set of operations or host-width values.

Any migration MUST preserve existing supported behavior through an explicit adapter or versioned migration. The legacy IR MUST NOT be silently treated as equivalent to the complete ZUIR architecture.

---

## 12. HDL and hardware description

ZUIR MUST preserve the semantic information required to represent supported HDL and hardware computations.

The architecture MUST accommodate, where defined by the relevant dialect:

- Signals and signal assignments.
- Symbolic widths and parameterized hardware types.
- Combinational and sequential logic.
- Clock and reset semantics.
- Timing constraints and temporal relationships.
- Hardware hierarchy and component interfaces.
- State machines and event-driven behavior.
- Pipeline and parallel hardware structures.
- Hardware/software boundaries.
- Resource and implementation constraints.
- Domain-specific verification and lowering contracts.

Hardware timing MUST NOT be reduced to ordinary software instruction ordering when the distinction affects meaning.

Signal width, register count, pipeline depth, memory capacity, clock count, and topology MUST NOT have universal fixed maxima in the core.

Board-specific and device-specific details belong to downstream hardware realization.

---

## 13. AI, tensors, data, distributed computing, and future domains

### 13.1 Extensible domain support

The same universal model MUST accommodate domain-specific representations without forcing all operations into classical instructions or quantum gates.

Standard dialects SHOULD be organized into independently maintained modules, including:

- `core`
- `classical`
- `quantum`
- `hdl`
- `tensor`
- `ai`
- `distributed`
- `networking`
- `crypto`

The exact directory layout MUST be reconciled with existing repository files before implementation. These names are architectural responsibilities, not permission to create duplicate existing modules.

### 13.2 Tensors and data

Tensor and data dialects MUST preserve symbolic shapes, element types, precision requirements, layout constraints, and any applicable data dependencies.

The core MUST NOT impose a fixed tensor rank, dimension count, dataset size, or accelerator-memory capacity.

Shape inference and compatibility MUST distinguish proven constraints from unknown or incomplete information.

### 13.3 AI and machine learning

AI/ML operations MUST declare their inputs, outputs, relevant effects, numerical behavior, resource requirements, and transformation contracts.

Model execution, differentiation, training, inference, and related operations MUST be defined through explicit dialect semantics where supported.

An optimizer MUST NOT assume that an operation is pure, deterministic, or freely reorderable merely because it belongs to an AI/ML dialect.

### 13.4 Distributed computation and networking

Distributed and networking dialects MUST preserve applicable communication, ordering, delivery, synchronization, failure, and consistency semantics.

Node identities, partitioning, topology, and routing MUST remain distinct from universal source-level identity unless explicitly defined as part of the program's semantics.

The core MUST NOT require a fixed node count or network size.

### 13.5 Cryptography and security

Security-sensitive operations MUST preserve their declared effects, authorization requirements, data-handling restrictions, and relevant timing or confidentiality contracts.

The presence of a cryptographic operation MUST NOT be treated as proof that the operation is secure.

Optimization and lowering MUST respect the operation's defined security requirements and permitted transformation rules.

### 13.6 Adding future domains

A new domain MUST be addable through the dialect and extension mechanisms without modifying the universal core's operation model for every new operation.

A domain may require a new adapter or an explicitly approved core capability if the existing contracts cannot express its semantics. Such a change MUST be justified, versioned, and tested rather than hidden in an undocumented special case.

---

## 14. Dialects and extension registration

### 14.1 Dialect manifest

Every registered dialect MUST declare:

- A globally unambiguous qualified name.
- Its schema and semantic version.
- Its operation and type definitions.
- Attribute definitions and namespaces.
- Required interfaces and verification hooks.
- Dependencies on other dialects.
- Compatibility and migration rules.
- Supported serialization forms.
- Lowering capabilities and unsupported cases.
- Any executable or trusted extension components.
- Required safety, isolation, and authorization properties.

### 14.2 Registry behavior

The dialect registry MUST provide deterministic lookup and namespace conflict detection.

Registration MUST reject incompatible duplicate definitions and invalid dependency relationships.

The registry MUST define ownership, lifetimes, and thread-safety requirements for registered definitions.

A compilation MUST use a consistent registry snapshot so that concurrent registry changes cannot silently change the meaning of an in-progress compilation.

### 14.3 Trust boundary

Serialized metadata MUST NOT be executed merely because it appears in an IR artifact.

Extensions that supply executable verifier hooks, transformation code, or lowering code MUST be treated as trusted compiler components or executed within an explicitly designed isolation boundary.

A manifest MUST NOT grant capabilities or authorization merely by declaring them.

### 14.4 Version evolution

An extension MUST NOT silently redefine an existing operation's meaning.

Breaking semantic changes require an appropriate new version or qualified identity and an explicit migration policy.

Unknown versions MUST produce an unsupported-version result when correct interpretation cannot be established.

---

## 15. Builder and construction contracts

Builders are construction tools, not the ultimate authority for IR validity.

The builder API MUST provide safe, documented ways to construct programs, modules, types, functions, regions, blocks, operations, values, and control-flow edges.

Builders SHOULD validate local invariants early, including:

- Identifier ownership.
- Operand and result relationships.
- Attribute validity.
- Region structure.
- Terminator placement.
- Required dialect registration.
- Type constraints that can be checked locally.

Builders MUST NOT duplicate the verifier's authoritative semantic rules in a way that allows the two implementations to diverge.

An explicitly internal unchecked construction path MAY exist when necessary for decoding or transformations, but it MUST be inaccessible as an ordinary public construction shortcut and MUST NOT bypass mandatory verification before a program is accepted for downstream compilation.

Builder failures MUST be structured and deterministic.

---

## 16. Verification

### 16.1 Verification is mandatory

Every externally constructed, deserialized, imported, or transformed ZUIR program MUST pass the required verification gates before it is accepted by a downstream stage.

Verification MUST be independent of builder discipline.

The verifier MUST NOT assume that a program is valid merely because it was created by an internal compiler component.

### 16.2 Verification layers

The verifier MUST support logically distinct checks for:

1. Schema and representation version validity.
2. Structural integrity.
3. Identifier uniqueness and reference integrity.
4. Symbol and module integrity.
5. Type correctness and constraints.
6. Control-flow integrity.
7. SSA and dominance rules, if SSA is adopted.
8. Def-use consistency.
9. Effect correctness and ordering.
10. Ownership, linearity, and resource-use rules.
11. Capability and resource requirement validity.
12. Dialect and operation legality.
13. Attribute and extension validity.
14. Provenance consistency where required.
15. Domain-specific invariants.
16. Whole-program consistency.

The implementation MAY split these checks across files, passes, or services, but their ownership and execution requirements MUST be explicit.

### 16.3 Verification results

Verification MUST distinguish at least:

- Valid.
- Invalid.
- Incomplete.
- Unsupported.
- Cancelled or budget-exhausted, where applicable.

A verifier MUST NOT report `Valid` if a mandatory check was skipped, failed, or could not be completed.

Optional verification modes MUST clearly declare which guarantees they provide.

### 16.4 Diagnostics

Verification failures MUST produce structured diagnostics with stable codes, severity, relevant entity identities, source locations where available, and contextual information.

Diagnostic ordering MUST be deterministic wherever the public contract promises deterministic output.

Diagnostics MUST not require terminal output or a particular user interface.

---

## 17. Analysis and transformation

### 17.1 Analysis contracts

Read-only analyses MAY derive control-flow graphs, dominance relationships, liveness, def-use information, resource estimates, effect summaries, and provenance information.

Derived information MUST be distinguishable from canonical IR state.

An analysis MUST document its prerequisites, output guarantees, invalidation requirements, and behavior when its inputs are incomplete.

### 17.2 Pass contracts

Every transformation pass MUST declare:

- A stable identity and version.
- Its input and output requirements.
- Its required dialects and capabilities.
- Its preconditions and postconditions.
- The semantic properties it preserves.
- Its possible diagnostics and failure outcomes.
- Its analysis dependencies and invalidation behavior.
- Its determinism guarantees.
- Its resource-budget behavior.
- Its test and conformance requirements.

A pass MUST NOT silently leave the IR invalid.

### 17.3 Pass manager

The pass manager MUST execute only compatible passes in a legal order.

It MUST make failure, cancellation, invalidation, and incomplete results explicit.

Pass registration MUST be extensible, but extension registration MUST NOT bypass required verification.

### 17.4 Canonicalization and optimization

Canonicalization MAY normalize equivalent forms when equivalence is established by the operation contracts.

Optimization MAY eliminate, combine, reorder, or replace operations only when the transformation preserves all applicable semantics.

The optimizer MUST account for effects, ownership, numerical behavior, concurrency, quantum measurement, hardware timing, and domain-specific constraints.

A target-specific optimization MUST declare its target assumptions and MUST NOT leak those assumptions into the universal representation as unconditional semantics.

### 17.5 Transactional transformation

A transformation that fails MUST NOT leave a publicly accepted program in a partially mutated, invalid state.

The implementation SHOULD use validated replacement artifacts, transactional mutation, or another documented mechanism that guarantees either a valid committed result or an explicit failure.

---

## 18. Lowering and target feasibility

### 18.1 Lowering contract

Lowering maps established ZUIR semantics into another representation.

Each lowering adapter MUST declare:

- Its source and destination representation versions.
- Supported operations, types, effects, and attributes.
- Required capabilities.
- Preservation guarantees.
- Known semantic restrictions.
- Numerical and concurrency assumptions.
- Resource and target constraints.
- Unsupported conversion behavior.
- Diagnostics and provenance requirements.

### 18.2 Legality before realization

A lowering pipeline MUST distinguish:

- A representation that is structurally valid.
- A transformation that is semantically legal.
- A target that satisfies the program's requirements.
- A target mapping that can be completed.
- An executable artifact that meets its deployment contract.

These are separate success conditions.

A valid universal representation does not imply that every target can execute it.

### 18.3 No silent fallback

If a target lacks a required capability, the compiler MUST report the limitation unless a permitted alternative exists.

Fallback execution MUST be explicitly authorized and MUST preserve the required semantics.

For example, a quantum operation MUST NOT silently become a classical approximation, and an HDL timing requirement MUST NOT be discarded merely because the chosen backend cannot represent it.

### 18.4 Resource planning

Target-specific feasibility analysis MAY consider available processors, memory, accelerators, qubits, signal widths, topology, timing, power, bandwidth, and other target properties.

These are downstream target facts, not universal ZUIR constants.

An infeasible realization MUST produce an explicit failure or an authorized alternative plan.

---

## 19. Serialization, interchange, and migration

### 19.1 Canonical serialization

ZUIR MUST define a versioned, documented serialization schema.

The format MUST preserve all information required to reconstruct the declared semantics, including:

- Representation and schema versions.
- Required dialect identities and versions.
- Program, module, symbol, type, and operation relationships.
- Operand, result, and control-flow references.
- Semantic attributes and effects.
- Ownership and resource contracts.
- Required provenance and compatibility metadata.
- Extension information needed for correct interpretation.

Descriptive metadata MAY be optional when the schema explicitly permits its omission.

### 19.2 Safe decoding

Decoding MUST validate schema structure, version compatibility, reference integrity, and applicable resource budgets.

Malformed lengths, invalid identifiers, recursive structures, unsupported encodings, and resource exhaustion MUST be handled explicitly.

Decoding MUST NOT execute arbitrary serialized content.

The decoder MUST NOT accept malformed or incomplete artifacts as verified programs.

### 19.3 Round-trip guarantees

For every supported schema version, the specification MUST define which properties are preserved by decode/encode round trips.

Canonical round trips MUST preserve the semantics and required metadata of supported artifacts.

If byte-for-byte equality is not guaranteed, the canonicalization contract MUST define which normalized properties are stable.

### 19.4 Interchange

Adapters for external IRs and formats MUST declare their supported subset and semantic preservation guarantees.

Importing an external format MUST NOT silently turn unsupported constructs into no-ops, constants, or unrelated operations.

Lossy import or export MUST require an explicit policy and a report of the information that cannot be preserved.

### 19.5 Migration

Schema migrations MUST be versioned, deterministic where required, and testable.

A migration MUST either produce an artifact that satisfies the destination schema or return a structured failure.

The original artifact MUST remain available to the caller if migration fails.

---

## 20. Determinism, fingerprints, and incremental compilation

### 20.1 Determinism

Equivalent inputs, schema versions, dialect snapshots, configuration, and pass versions MUST produce equivalent results wherever deterministic behavior is promised.

The implementation MUST define deterministic ordering for canonical traversal, serialization, diagnostics, registry resolution, and stable fingerprints.

The use of hash maps, parallel processing, or nondeterministic runtime scheduling MUST NOT accidentally change canonical output.

### 20.2 Canonical fingerprints

The fingerprint system MUST distinguish semantic identity from incidental process-local identity.

A canonical fingerprint MUST include all semantic fields required by the fingerprint contract, including relevant dialect versions and attributes.

Descriptive metadata MAY be excluded only when its exclusion is explicitly defined.

Hash algorithm and encoding changes MUST be versioned. A hash MUST NOT be treated as a mathematical proof of semantic equivalence.

### 20.3 Incremental compilation

Incremental compilation MAY reuse verified artifacts and analysis results when their dependencies remain valid.

Cache keys MUST account for every input that can affect the result, including representation versions, relevant dialect definitions, semantic configuration, and pass versions.

When a dependency changes, affected analyses and cached results MUST be invalidated.

An incomplete or incompatible cache entry MUST NOT be accepted as a valid result.

---

## 21. Diagnostics, source maps, and provenance

### 21.1 Source locations

ZUIR SHOULD preserve source provenance through optional source-map associations.

An IR entity MAY originate from multiple source spans, imported formats, generated code, or transformation steps.

The source-map model MUST support absent source locations, synthetic entities, and multiple origins.

### 21.2 Provenance

Provenance MUST distinguish source origin, transformation history, verification status, and target-specific realization information.

Provenance records MUST NOT be confused with proof that a program is correct.

A transformation MUST preserve or update provenance according to the documented contract.

### 21.3 Error ownership

The ZUIR subsystem owns representation-specific errors and diagnostics.

The lexer and parser own syntax errors. Semantic analysis owns semantic validation errors. Domain IRs own their domain-specific diagnostics. Target and runtime components own their realization and execution failures.

ZUIR MAY carry upstream diagnostics or source associations but MUST NOT duplicate another component's authority.

---

## 22. Public API and Rust implementation

### 22.1 Public API boundary

`src/zuir/mod.rs` MUST eventually become the ZUIR module-composition and public re-export boundary.

It MUST NOT contain the complete implementation or introduce competing definitions of types owned by child modules.

Public APIs MUST be deliberately selected and documented.

Glob re-exports SHOULD be avoided when they obscure ownership or create unstable name collisions.

### 22.2 Rust baseline

The implementation MUST compile on stable Rust 1.97.1 or later, edition 2021, and MUST avoid unnecessary nightly dependencies.

The actual repository toolchain configuration MUST be reconciled with this requirement before the implementation is frozen.

### 22.3 Error handling

Public APIs MUST return structured errors for expected failure conditions.

Invalid user input, malformed IR, unsupported operations, resource exhaustion, incompatible versions, and lowering failures MUST NOT be represented by unexplained panics.

Panics MUST NOT be used as ordinary control flow for expected compilation failures.

### 22.4 Dependency policy

Dependencies MUST be reviewed for:

- Compatibility with the supported Rust baseline.
- Safe-Rust requirements.
- Maintenance and licensing.
- Determinism and reproducibility.
- Serialization stability, if applicable.
- Resource behavior and denial-of-service exposure.
- Compatibility with the repository's existing dependency architecture.

ZUIR MUST NOT introduce an unnecessary dependency on a particular hardware vendor, runtime, simulator, or backend.

### 22.5 Thread safety and reentrancy

The public API MUST document thread-safety and concurrency guarantees.

Independent compilation sessions MUST NOT accidentally share mutable state that changes semantics or makes results nondeterministic.

Global registries, caches, and configuration MUST have explicit synchronization and lifecycle rules.

---

## 23. Required repository structure and file ownership

The following is the target architecture for `src/zuir/`. It MUST be reconciled with the actual repository before files are created. Existing files and canonical implementations take precedence over speculative duplicate modules.

```text
src/zuir/
├── mod.rs
├── api.rs
├── config.rs
├── ids.rs
├── version.rs
├── errors.rs
├── diagnostics.rs
├── source_map.rs
├── spec/
│   ├── mod.rs
│   ├── invariants.rs
│   ├── compatibility.rs
│   └── conformance.rs
├── schema/
│   ├── mod.rs
│   ├── types.rs
│   ├── attributes.rs
│   ├── metadata.rs
│   └── extensions.rs
├── types/
│   ├── mod.rs
│   ├── type_id.rs
│   ├── type_def.rs
│   ├── layout.rs
│   ├── constraints.rs
│   └── compatibility.rs
├── values/
│   ├── mod.rs
│   ├── symbols.rs
│   ├── literals.rs
│   ├── constants.rs
│   └── expressions.rs
├── ir/
│   ├── mod.rs
│   ├── program.rs
│   ├── module.rs
│   ├── function.rs
│   ├── region.rs
│   ├── block.rs
│   ├── operation.rs
│   ├── operand.rs
│   ├── result.rs
│   ├── successor.rs
│   ├── use_def.rs
│   ├── effects.rs
│   ├── ownership.rs
│   └── resources.rs
├── dialect/
│   ├── mod.rs
│   ├── definition.rs
│   ├── operation.rs
│   ├── interfaces.rs
│   ├── traits.rs
│   ├── registry.rs
│   └── versioning.rs
├── builder/
│   ├── mod.rs
│   ├── program.rs
│   ├── operation.rs
│   ├── regions.rs
│   └── validation.rs
├── verify/
│   ├── mod.rs
│   ├── structural.rs
│   ├── types.rs
│   ├── dominance.rs
│   ├── control_flow.rs
│   ├── effects.rs
│   ├── ownership.rs
│   ├── resources.rs
│   ├── dialects.rs
│   └── full.rs
├── analysis/
│   ├── mod.rs
│   ├── cfg.rs
│   ├── dominance.rs
│   ├── def_use.rs
│   ├── effects.rs
│   ├── liveness.rs
│   ├── resources.rs
│   └── provenance.rs
├── pass/
│   ├── mod.rs
│   ├── manager.rs
│   ├── registry.rs
│   ├── contracts.rs
│   ├── invalidation.rs
│   └── determinism.rs
├── transform/
│   ├── mod.rs
│   ├── canonicalize.rs
│   ├── simplify.rs
│   ├── inline.rs
│   ├── control_flow.rs
│   └── legalize.rs
├── lowering/
│   ├── mod.rs
│   ├── api.rs
│   ├── context.rs
│   ├── legality.rs
│   ├── diagnostics.rs
│   └── provenance.rs
├── serialization/
│   ├── mod.rs
│   ├── schema.rs
│   ├── encode.rs
│   ├── decode.rs
│   ├── validation.rs
│   └── canonical.rs
├── interchange/
│   ├── mod.rs
│   ├── format.rs
│   ├── import.rs
│   ├── export.rs
│   └── migration.rs
├── fingerprint/
│   ├── mod.rs
│   └── stable_hash.rs
├── incremental/
│   ├── mod.rs
│   ├── dependencies.rs
│   ├── invalidation.rs
│   └── cache_keys.rs
├── dialects/
│   ├── mod.rs
│   ├── core/
│   ├── classical/
│   ├── quantum/
│   ├── hdl/
│   ├── tensor/
│   ├── ai/
│   ├── distributed/
│   ├── networking/
│   └── crypto/
└── tests/
    ├── api.rs
    ├── schema.rs
    ├── builders.rs
    ├── verifier.rs
    ├── types.rs
    ├── control_flow.rs
    ├── effects.rs
    ├── ownership.rs
    ├── resources.rs
    ├── dialects.rs
    ├── lowering.rs
    ├── serialization.rs
    ├── compatibility.rs
    ├── determinism.rs
    ├── incremental.rs
    ├── property.rs
    ├── fuzz.rs
    ├── stress.rs
    └── conformance/
```

The directories above express responsibilities. They MUST NOT be created as empty placeholder trees simply to make the repository appear complete.

A directory or file SHOULD be introduced when its contract is stable and it has a meaningful implementation or test responsibility. If an existing module already owns a responsibility, that module MUST be reused or migrated deliberately.

### 23.1 Normative specification file

`src/zuir/ZUIR-SPEC.md` owns:

- Universal representation semantics.
- The required entities and invariants.
- SSA or alternative value-model requirements.
- Type, effect, ownership, and resource contracts.
- Dialect and extension semantics.
- Verification and transformation guarantees.
- Serialization, compatibility, and determinism.
- Domain and target integration.
- Required conformance and acceptance gates.

It MUST NOT duplicate the full source grammar, define target-specific code generation, or claim implementation features without evidence.

### 23.2 Independent foundational files

These files MUST be specified before their dependents are implemented.

| File | Sole responsibility and freeze criteria |
|---|---|
| `ids.rs` | Typed identities, uniqueness scope, allocation rules, collision and exhaustion handling. No dependency on concrete IR nodes. |
| `version.rs` | Representation, schema, and compatibility version types; comparison and supported-version negotiation. |
| `errors.rs` | Stable structured error categories, error context, and propagation rules. No dependency on verifier internals. |
| `config.rs` | Explicit verification policies, operational budgets, compatibility options, cancellation, and determinism settings. |
| `diagnostics.rs` | Structured diagnostics, codes, severity, related entities, and deterministic ordering contract. |
| `source_map.rs` | Source spans, generated origins, imported origins, and optional multi-origin provenance. |
| `spec/invariants.rs` | Machine-enforceable invariant identifiers and their authoritative verifier owners. |
| `spec/compatibility.rs` | API/schema compatibility policy, breaking-change definitions, and migration requirements. |
| `spec/conformance.rs` | Reusable acceptance requirements for implementations, dialects, builders, and adapters. |
| `schema/types.rs` | Schema-level type descriptors independent of concrete IR nodes. |
| `schema/attributes.rs` | Typed attribute schemas, reserved names, extension rules, and validation behavior. |
| `schema/metadata.rs` | Semantic versus descriptive metadata and its serialization/fingerprint behavior. |
| `schema/extensions.rs` | Extension manifests, namespaces, dependency declarations, and compatibility rules. |

Each file MUST declare its purpose, authority, ownership, non-ownership, inputs, outputs, dependencies, exports, consumers, error behavior, safety requirements, compatibility, scalability, tests, and freeze criteria.

### 23.3 Type and value files

| File | Sole responsibility |
|---|---|
| `types/type_id.rs` | Type identity, reference representation, canonical identity, and recursive-type policy. |
| `types/layout.rs` | Logical layout and symbolic layout constraints; no target-specific physical layout. |
| `types/constraints.rs` | Generic constraints, symbolic shapes, widths, bounds, and constraint-solving interfaces. |
| `types/compatibility.rs` | Semantic equivalence, assignability, legal conversions, and representation compatibility queries. |
| `types/type_def.rs` | Canonical type definitions and extensible domain-specific type references. |
| `values/symbols.rs` | Symbol identities, linkage, visibility, and resolved symbol references. |
| `values/literals.rs` | Portable, typed literal values and exact numeric representation rules. |
| `values/constants.rs` | Typed constants and bounded constant-evaluation outcomes. |
| `values/expressions.rs` | Symbolic and IR-level value expressions and their evaluation boundaries. |

These files MUST not implement competing source-level name resolution or type inference.

### 23.4 Core IR files

| File | Sole responsibility |
|---|---|
| `ir/program.rs` | Program identity, representation version, modules, required dialects, and global invariants. |
| `ir/module.rs` | Module ownership, symbols, declarations, imports, exports, and type references. |
| `ir/function.rs` | Callable signatures, parameters, results, effects, linkage, and body regions. |
| `ir/region.rs` | Nested computation, captures, region arguments, and control semantics. |
| `ir/block.rs` | Block arguments, ordered operations, and terminator rules. |
| `ir/operation.rs` | Qualified operation identity, operands, results, attributes, regions, and effects. |
| `ir/operand.rs` | Typed operand references and applicable operand roles. |
| `ir/result.rs` | Operation-produced values and definition ownership. |
| `ir/successor.rs` | Control-flow edges and successor arguments. |
| `ir/use_def.rs` | Def-use relationships and derived-index consistency. |
| `ir/effects.rs` | Structured effect declarations and ordering contracts. |
| `ir/ownership.rs` | Ownership, borrowing, aliasing, linearity, and affine constraints. |
| `ir/resources.rs` | Symbolic resource requirements, constraints, and associated metadata. |

These files MUST agree with the frozen identity, type, and schema contracts before implementation begins.

### 23.5 Dialect and builder files

| File | Sole responsibility |
|---|---|
| `dialect/definition.rs` | Dialect manifest and declared schema ownership. |
| `dialect/operation.rs` | Operation schemas and validation contracts. |
| `dialect/interfaces.rs` | Reusable operation interfaces and their required guarantees. |
| `dialect/traits.rs` | Reusable semantic traits and contracts. |
| `dialect/registry.rs` | Registration, namespace validation, lookup, and immutable registry snapshots. |
| `dialect/versioning.rs` | Dialect evolution, compatibility, deprecation, and migration. |
| `builder/validation.rs` | Builder preconditions and reuse of authoritative validation rules. |
| `builder/operation.rs` | Safe operation construction against registered schemas. |
| `builder/regions.rs` | Region, block, terminator, and control-flow construction. |
| `builder/program.rs` | Program assembly and cross-entity ownership checks. |
| `builder/mod.rs` | Deliberately selected builder API exports. |
| `api.rs` | Stable public entry points for construction, inspection, verification, serialization, transformation, and lowering. |
| `mod.rs` | Module composition and selected public re-exports only. |

### 23.6 Verification and analysis files

| File | Sole responsibility |
|---|---|
| `verify/structural.rs` | Structural invariants and reference integrity. |
| `verify/types.rs` | Type consistency and required type constraints. |
| `verify/dominance.rs` | Dominance and SSA use legality, if applicable. |
| `verify/control_flow.rs` | Terminators, successors, region control, and edge arguments. |
| `verify/effects.rs` | Effect declarations, ordering, and legality. |
| `verify/ownership.rs` | Ownership, aliasing, linearity, and affine invariants. |
| `verify/resources.rs` | Resource requirement structure and constraint validity. |
| `verify/dialects.rs` | Registered operation and dialect conformance. |
| `verify/full.rs` | Orchestration of mandatory verification stages. |
| `analysis/cfg.rs` | Control-flow graph derivation. |
| `analysis/dominance.rs` | Dominance analysis. |
| `analysis/def_use.rs` | Def-use analysis and index derivation. |
| `analysis/effects.rs` | Effect summaries and dependency analysis. |
| `analysis/liveness.rs` | Liveness analysis under the declared value model. |
| `analysis/resources.rs` | Resource estimates and constraint analysis. |
| `analysis/provenance.rs` | Provenance analysis and origin tracing. |

### 23.7 Passes, lowering, and persistence files

| File | Sole responsibility |
|---|---|
| `pass/contracts.rs` | Pass preconditions, postconditions, and preservation guarantees. |
| `pass/registry.rs` | Pass registration and versioned lookup. |
| `pass/manager.rs` | Legal pass scheduling and result orchestration. |
| `pass/invalidation.rs` | Analysis invalidation and dependency updates. |
| `pass/determinism.rs` | Pass-level reproducibility contracts. |
| `transform/canonicalize.rs` | Semantics-preserving canonicalization. |
| `transform/simplify.rs` | Semantics-preserving simplification. |
| `transform/inline.rs` | Contract-checked inlining where supported. |
| `transform/control_flow.rs` | Legal control-flow transformations. |
| `transform/legalize.rs` | Conversion to explicitly supported operation subsets. |
| `lowering/api.rs` | Public lowering entry points. |
| `lowering/context.rs` | Immutable lowering configuration and registry snapshots. |
| `lowering/legality.rs` | Source/destination legality and preservation checks. |
| `lowering/diagnostics.rs` | Lowering-specific diagnostic construction. |
| `lowering/provenance.rs` | Source-to-destination origin tracking. |
| `serialization/schema.rs` | Serialization schema and version contract. |
| `serialization/encode.rs` | Encoding of valid artifacts. |
| `serialization/decode.rs` | Safe decoding under explicit budgets. |
| `serialization/validation.rs` | Artifact validation before acceptance. |
| `serialization/canonical.rs` | Canonical ordering and stable encoding rules. |
| `interchange/format.rs` | External-format adapter contracts. |
| `interchange/import.rs` | Validated import. |
| `interchange/export.rs` | Explicitly supported export. |
| `interchange/migration.rs` | Versioned artifact migration. |
| `fingerprint/stable_hash.rs` | Versioned canonical content fingerprints. |
| `incremental/dependencies.rs` | Dependency graph for incremental reuse. |
| `incremental/invalidation.rs` | Invalidation of stale incremental results. |
| `incremental/cache_keys.rs` | Stable, complete cache-key construction. |

### 23.8 Standard dialect files

Each standard dialect MUST have its own documented contract, tests, and versioning policy.

The `core` dialect owns operations required for shared IR structure and universal control/data-flow semantics. It MUST remain small and domain-neutral.

The `classical` dialect owns classical computational operations and their numeric, memory, and control semantics.

The `quantum` dialect owns ZUIR-level quantum operation contracts and the adapter to `quantum::ir`. It MUST NOT create a competing quantum IR.

The `hdl` dialect owns hardware-description operations and the adapter to the canonical hardware representation.

The `tensor` dialect owns tensor operations, symbolic shapes, and shape constraints.

The `ai` dialect owns registered AI/ML operation contracts and numerical requirements.

The `distributed` dialect owns distributed computation and communication semantics.

The `networking` dialect owns network operations and applicable communication contracts.

The `crypto` dialect owns cryptographic operation contracts and security-related constraints.

A dialect MUST NOT be marked production-ready until its required operations, verification, serialization, lowering status, compatibility, and tests are implemented.

---

## 24. Integration contracts with existing repository files

### 24.1 Frontend AST

The frontend MUST produce the canonical AST defined by the repository's current architecture.

ZUIR generation MUST consume validated semantic information rather than re-parse source text or reconstruct semantics from AST spelling.

The adapter MUST preserve source locations and relevant source provenance.

### 24.2 Semantic analysis

The integration point is `src/frontend/semantic/`, including its public API and canonical semantic model.

The semantic model MUST remain the authority for resolved names, types, effects, capabilities, resource requirements, contracts, and other semantic facts.

ZUIR generation MUST reject or explicitly represent incomplete semantic information according to the agreed contract. It MUST NOT silently convert unknown types, unresolved symbols, or failed validation into valid IR entities.

### 24.3 Existing `src/ir_gen.rs`

The existing IR-generation implementation MUST be inspected and mapped to the new ZUIR generation boundary before migration.

The migration MUST document:

- Existing consumers and public types.
- Current type-conversion behavior.
- Unsupported or incomplete source constructs.
- Numeric-width and conversion assumptions.
- The mapping from legacy IR entities to canonical ZUIR entities.
- Compatibility requirements and removal criteria.

The existing code contains legacy mappings involving fixed-width numeric types and an unknown-type fallback. Such behavior MUST be reviewed before it is reused in a universal representation.

An unknown semantic type MUST NOT silently become a valid opaque type unless the schema explicitly establishes that behavior and preserves the uncertainty correctly.

### 24.4 Existing `ZAMANI_IR_SPEC.md`

The legacy specification MUST be retained until its consumers and compatibility requirements are understood.

Its arithmetic, call, return, and SSA-like operations MUST be mapped explicitly to the appropriate ZUIR operations.

Any legacy behavior that cannot be represented without semantic loss MUST be documented and handled through a compatibility adapter or explicit rejection.

The legacy specification MUST NOT become a competing universal IR authority.

### 24.5 Existing `src/quantum/ir/`

Quantum integration MUST use the canonical quantum identities and types already owned by that subsystem.

ZUIR MUST NOT redefine `QubitId`, physical-qubit identity, canonical quantum serialization, or canonical quantum hashing.

Quantum routing, scheduling, QEC, resilience, ZQN, HAL, simulation, and backend code MUST remain downstream responsibilities.

### 24.6 Classical and HDL consumers

The actual classical and HDL IR paths MUST be located and audited before adapter files are frozen.

Each adapter MUST document its supported subset, semantic preservation guarantees, unsupported constructs, and version compatibility.

No integration MUST depend on an assumed directory or type name without verifying that the repository actually contains it.

### 24.7 Grammar integration

ZUIR MUST NOT own or modify source grammar merely to make IR construction easier.

`grammar/Zamani.g4` remains the grammar composition root. `grammar/antlr/ZamaniLexer.g4` and `grammar/antlr/ZamaniParser.g4` remain the respective public grammar boundaries.

The grammar and specification directories govern source syntax. The semantic model governs meaning. ZUIR governs canonical IR representation.

A new IR operation does not automatically require a new source keyword. Domain operations MAY be expressed through existing generic syntax and dialect metadata where the language specification permits it.

Grammar changes MUST be made only when a genuine source-language feature requires them, and MUST pass the repository's grammar and frontend conformance tests.

---

## 25. Compatibility and API stability

The implementation MUST distinguish language, AST, semantic-model, ZUIR schema, dialect, serialized artifact, and public Rust API versions.

These versions MUST NOT be conflated.

Additive, backward-compatible changes SHOULD preserve existing behavior.

Breaking changes MUST require an explicit versioning decision, migration strategy, and conformance tests.

The compatibility policy MUST define:

- Which public Rust types and module paths are stable.
- Which serialized versions are accepted.
- How deprecated operations and attributes are handled.
- How unknown dialects and future versions are treated.
- Which migrations preserve semantics.
- Which transformations or exports are potentially lossy.
- How legacy adapters are retired.

Compatibility MUST NOT be achieved by silently interpreting a new operation as an old operation with different semantics.

---

## 26. Security and robustness

ZUIR is a compiler data structure and MUST treat externally supplied IR as untrusted input.

The implementation MUST defend against malformed references, invalid recursion, excessive nesting, pathological graph structures, invalid metadata, resource exhaustion, and hostile extension definitions.

The following requirements are mandatory:

1. No unsafe Rust.
2. No implicit execution of serialized content.
3. No implicit filesystem or network access during basic IR verification.
4. No unchecked arithmetic where overflow could invalidate an invariant.
5. No silent acceptance of malformed or unsupported operations.
6. No panic-based handling of ordinary validation failures.
7. No incomplete verification reported as successful.
8. No unbounded compile-time evaluation without an explicit budget.
9. No trust in caller-supplied fingerprints as proof of validity.
10. No authorization granted solely by a declaration in IR metadata.

Resource limits MUST be configurable and explicit where appropriate. A limit imposed by a security policy MUST be distinguishable from a semantic or target constraint.

---

## 27. Testing and conformance requirements

### 27.1 Unit tests

Every implementation file MUST have tests appropriate to its responsibility.

Tests MUST cover valid inputs, invalid inputs, boundary cases, error propagation, and the relevant invariants.

### 27.2 Integration tests

Integration tests MUST cover:

- Semantic model to ZUIR generation.
- ZUIR verification.
- Classical IR adaptation.
- Quantum IR adaptation.
- HDL and hardware adaptation.
- Dialect registration and extension handling.
- Serialization and deserialization.
- Compatibility and migration.
- Lowering legality.
- Diagnostics and source provenance.

A test MUST exercise the actual integration boundary rather than merely constructing an isolated substitute.

### 27.3 Property-based tests

Property tests SHOULD generate valid and invalid IR graphs to test:

- Identifier uniqueness.
- Reference integrity.
- Builder/verifier agreement.
- Type consistency.
- Control-flow invariants.
- Serialization round trips.
- Transformation preservation.
- Deterministic canonicalization.
- Cache invalidation.
- Error handling under budget exhaustion.

Generated inputs MUST be bounded by explicit test budgets without introducing universal limits into the implementation.

### 27.4 Fuzzing

Fuzz targets SHOULD cover decoding, verification, dialect metadata, operation attributes, graph references, migration, and any complex transformation boundary.

Fuzzing MUST run without unsafe code and MUST test graceful rejection of malformed inputs.

### 27.5 Stress tests

Stress tests MUST exercise large generated modules, deep region nesting, large control-flow graphs, many symbols, many registered operations, large symbolic constraints, and substantial resource metadata.

The tests MUST demonstrate that implementation limits are explicit and failures are controlled.

They MUST NOT imply that a finite test proves scalability to literal infinity.

### 27.6 Determinism tests

Repeated processing with equivalent inputs, configurations, registry snapshots, and pass versions MUST produce equivalent canonical results wherever determinism is promised.

Tests MUST cover serialization order, diagnostic order, fingerprints, and reproducible transformations.

### 27.7 Conformance corpus

The repository SHOULD use a shared corpus containing at least:

- `minimal.zm`
- `classical.zm`
- `quantum.zm`
- `hybrid.zm`
- `hdl.zm`
- `poco-reaf.zm`

These examples MUST be valid under the actual language specification and grammar. They MUST NOT be treated as evidence that an unsupported feature has already been implemented.

Each supported example SHOULD have tests for semantic analysis, ZUIR generation, verification, and the applicable downstream adapter.

---

## 28. Production acceptance gates

ZUIR MUST NOT be declared production-ready merely because the crate compiles.

The following gates are mandatory.

### Gate A — Specification consistency

- This document agrees with `grammar/DESIGN.md`.
- Semantic ownership agrees with `grammar/specification/semantic-model.md`.
- Grammar ownership agrees with `grammar/Zamani.g4` and its canonical lexer/parser boundaries.
- Legacy IR behavior and migration are documented.
- The canonical quantum IR remains authoritative.
- No unresolved ownership conflicts remain.

### Gate B — Foundational contracts

- Identifier and version contracts are stable.
- Errors and diagnostics are structured.
- Resource budgets are explicit.
- Type and value semantics are defined.
- Extension namespaces and compatibility are defined.
- Public API and serialization versioning are documented.

### Gate C — Core IR

- The complete required entity model is implemented.
- Structural invariants are verified.
- Control flow and def-use relationships are valid.
- Type, effect, ownership, and resource contracts are enforced.
- Unsupported or incomplete states cannot silently pass verification.

### Gate D — Extensibility

- A test-only third-party dialect can register an operation without modifying the universal operation model.
- Conflicting registrations are rejected.
- Unknown operations are handled explicitly.
- Dialect versions and dependencies are validated.
- New domains can reuse the published contracts.

### Gate E — Serialization and compatibility

- Valid artifacts round-trip according to the defined schema contract.
- Invalid artifacts are rejected.
- Unsupported versions fail explicitly.
- Migration is deterministic where required and preserves documented semantics.
- Canonical fingerprints obey their versioned contract.

### Gate F — Semantic preservation

- Every production transformation declares and tests its preservation guarantees.
- Quantum measurement semantics are preserved.
- Numerical, ownership, concurrency, effect, and hardware-timing contracts are preserved.
- Unsupported lowerings fail explicitly.
- No unauthorized fallback changes program meaning.

### Gate G — Repository integration

- The canonical frontend AST and semantic model generate ZUIR through a documented interface.
- Existing classical, quantum, and HDL paths use explicit adapters.
- Legacy APIs remain compatible until their migration gates pass.
- Diagnostics and provenance cross integration boundaries correctly.
- The real repository pipeline is tested end to end.

### Gate H — Production hardening

- Formatting and linting pass.
- The declared Rust baseline builds.
- No unsafe Rust is present.
- Unit, integration, property, stress, determinism, and conformance tests pass as required.
- Failure and resource-exhaustion behavior is explicit.
- Public APIs and serialized formats have documented compatibility commitments.
- Implementation status is supported by actual test and review evidence.

---

## 29. File-freeze governance

A file is frozen only when its contract and implementation are complete enough that downstream implementation does not need to invent missing behavior.

Every file MUST document:

- **PURPOSE:** Its sole responsibility.
- **STATUS:** Proposed, in progress, frozen, deprecated, or replaced.
- **AUTHORITY:** The specification and contracts governing it.
- **OWNS:** The types, behavior, and invariants it exclusively owns.
- **DOES_NOT_OWN:** The responsibilities delegated elsewhere.
- **INPUTS:** Accepted values and required preconditions.
- **OUTPUTS:** Results, guarantees, and failure outcomes.
- **DEPENDS_ON:** Approved dependency direction.
- **IMPORTS:** Required upstream types and traits.
- **EXPORTS:** Public API and stability commitments.
- **CONSUMED_BY:** Known downstream consumers.
- **AST_OWNER:** Its relationship to source AST information.
- **SEMANTIC_OWNER:** Which semantic facts it accepts or validates.
- **IR_OWNER:** Its exclusive IR responsibilities.
- **RUNTIME_OWNER:** Runtime behavior within or outside its scope.
- **TOOLING_OWNER:** Diagnostics, debugging, serialization, or tooling responsibilities.
- **DIAGNOSTIC_OWNER:** Error categories and reporting boundaries.
- **TEST_OWNER:** Required unit, integration, property, and conformance tests.
- **COMPATIBILITY:** API, schema, migration, and versioning guarantees.
- **SCALABILITY:** Dynamic collections, symbolic constraints, checked arithmetic, and budget behavior.
- **POCO-REAF:** How target independence and semantic portability are preserved.
- **HARD-CODING:** Prohibited assumptions and permitted explicit limits.
- **AMBIGUITY:** Handling of unknown, invalid, unsupported, and incomplete states.
- **ERRORS:** Failure propagation and diagnostic behavior.
- **FREEZE CRITERIA:** Exact acceptance conditions.

A frozen file MUST NOT require an architectural rewrite merely because a planned dependent file is implemented.

A genuine defect, compatibility issue, or specification change MAY require reopening a frozen file. Such a change MUST be explicit, reviewed, tested, and reflected in the relevant contracts. Freeze status is not permission to ignore defects.

---

## 30. Required implementation order

Implementation MUST follow dependency order.

### Stage 1 — Audit and specification freeze

Inspect the complete existing ZUIR directory, legacy IR, grammar and semantic specifications, frontend, classical IR, quantum IR, HDL implementation, tests, and consumers.

Produce a verified file-ownership and dependency matrix.

Resolve the value-model decision, identifier policy, schema-version policy, extension mechanism, and compatibility boundaries.

**Exit condition:** No unresolved ownership conflict or undocumented core semantic decision.

### Stage 2 — Independent contracts

Implement and freeze identity, versioning, errors, configuration, diagnostics, source maps, schema primitives, metadata, and extension contracts.

**Exit condition:** Public types, dependencies, error behavior, and compatibility rules are stable.

### Stage 3 — Types and values

Implement type identity, type definitions, constraints, compatibility, symbols, literals, constants, and symbolic expressions.

**Exit condition:** The required classical, quantum, HDL, and symbolic types fit the shared schema without fixed machine-capacity assumptions.

### Stage 4 — Core IR

Implement the program, module, function, region, block, operation, operand, result, successor, def-use, effect, ownership, and resource models.

**Exit condition:** The complete representation can express valid programs without depending on builder discipline to hide missing invariants.

### Stage 5 — Dialect system and builders

Implement dialect manifests, operation schemas, interfaces, registration, versioning, and validated construction.

**Exit condition:** A test-only extension can add a new operation without changing the universal core.

### Stage 6 — Verification and analysis

Implement mandatory structural, type, control-flow, effect, ownership, resource, and dialect checks, followed by the required analyses.

**Exit condition:** Invalid or incompletely verified IR cannot be accepted as valid.

### Stage 7 — Serialization and compatibility

Implement encoding, decoding, schema validation, canonicalization, fingerprints, migration, and version checks.

**Exit condition:** Supported artifacts round-trip correctly, and incompatible artifacts fail explicitly.

### Stage 8 — Standard dialects

Implement core and classical dialects, then quantum and HDL adapters, followed by tensor, AI, distributed, networking, and cryptographic dialects according to verified repository requirements.

**Exit condition:** Each implemented dialect passes its conformance suite and declares its lowering status.

### Stage 9 — Pass manager and transformations

Implement pass contracts, registration, invalidation, canonicalization, simplification, control-flow transformations, and legalization.

**Exit condition:** Every production transformation preserves its required semantics and leaves a verified result.

### Stage 10 — Repository integration

Connect the actual semantic model and IR-generation entry points to ZUIR and connect the canonical domain IR adapters.

**Exit condition:** Representative programs pass through the real repository pipeline without bypassing verification or silently losing meaning.

### Stage 11 — Production hardening

Complete compatibility, fuzzing, property testing, stress testing, determinism testing, and CI enforcement.

**Exit condition:** All mandatory production acceptance gates pass with recorded evidence.

---

## 31. Final architectural invariants

The following invariants are non-negotiable:

1. ZUIR is a canonical representation of validated computational meaning, not a second AST or a replacement semantic model.
2. The universal core has one authoritative definition for each shared representation concept.
3. Domain extensions are versioned, registered, validated, and explicitly integrated.
4. No universal machine-capacity ceiling is imposed by the language or ZUIR.
5. Symbolic resource requirements remain distinct from physical allocations.
6. Unknown, invalid, unsupported, and incomplete states are never silently treated as valid.
7. Semantic preservation is mandatory for transformations and lowering.
8. Quantum semantics continue through the canonical `quantum::ir` boundary.
9. Classical and HDL integrations use explicit adapters and preserve their domain contracts.
10. Verification is mandatory before an artifact is accepted by downstream compilation stages.
11. Serialization, migration, and fingerprints have explicit versioned contracts.
12. Determinism is guaranteed wherever the public contract requires it.
13. Rust implementation uses safe Rust and Rust 1.97.1 or later.
14. Existing repository implementations are audited and migrated rather than overwritten to match a proposed tree.
15. Production readiness is established by passing implementation and integration evidence, not by specification completeness alone.

The architectural objective is straightforward: **ZUIR defines a stable, extensible, target-independent representation of computational meaning.** It allows Zamani to expand across classical, quantum, HDL, and future computing domains while keeping source semantics independent of today's hardware and preserving the contracts required for correct realization on compatible targets.

The specification is complete as the intended normative design contract. The implementation MUST still satisfy the repository audit, dependency, migration, and acceptance requirements before ZUIR can be declared production-ready.

Important integration note: This is the specification content for `src/zuir/ZUIR-SPEC.md`; I have not committed it to GitHub. The repository currently has only a minimal `src/zuir/mod.rs`, so the implementation and compatibility gates in the document remain work to be completed, not verified achievements.



### Zamani ZUIR-SPEC.md — Production specification

Target file: [`src/zuir/ZUIR-SPEC.md`](https://github.com/Benwellonedge28/Zamani/blob/main/src/zuir/ZUIR-SPEC.md)

The specification below is designed to establish ZUIR as Zamani's canonical, extensible, target-independent intermediate representation for classical computing, quantum computing, HDL, hardware/software co-design, AI/ML, tensors, distributed computing, networking, cryptography, and future computational domains.

It preserves the central POCO-REAF principle: write a program once, compile its portable meaning once, and reuse that meaning across compatible execution environments without rewriting the source for each machine.

There are three important engineering distinctions:

* Unbounded by design does not mean physically infinite. ZUIR must impose no artificial universal ceiling on program size, qubits, processors, tensor rank, device count, memory, or topology. Actual execution remains subject to available resources, representable data, implementation limits, and target capabilities.

* Universal representation does not imply universal executability. A target that cannot preserve a program's required semantics must reject the lowering or report a precisely defined limitation. An explicitly authorized alternative execution strategy may be used only when it preserves the program's specified behavior.

* A specification is not proof of implementation completeness. The document defines the contracts, file ownership, integration boundaries, and acceptance tests required for production readiness. Existing implementation behavior must be audited and tested before any feature is labelled production-ready.

### Repository findings that shape this specification

The repository's existing grammar and quantum IR documentation establish several important architectural boundaries:

* `grammar/Zamani.g4` is the canonical grammar composition root, with lexer and parser composition beneath it.

* `grammar/grammar.md` defines a conformance-oriented view of the language and its compilation pipeline.

* `src/quantum/ir/mod.rs` describes the existing quantum semantic IR as hardware-independent and separates it from routing, scheduling, error correction, hardware realization, and backend execution.

* `src/ir_gen.rs` contains an existing instruction-oriented representation, including classical operations, control flow, calls, and quantum-related instruction variants. Its exact relationship to ZUIR must be settled through an explicit migration/adaptation contract.

* `Cargo.toml` declares Rust edition 2021 and `rust-version = "1.97"`.

* `src/zuir/mod.rs` currently appears to contain only a ZUIR label in the inspected default-branch version, rather than a completed module API.

The requested `src/zuir/ZUIR-SPEC.md` was not present on the inspected default branch, so the following is a new specification draft rather than a verified replacement of an existing file. I have not established that every source file and grammar module in the repository has been inspected; the integration inventory below therefore defines the required audit rather than claiming it has already been completed.

# Zamani Universal Intermediate Representation (ZUIR)
## Normative Specification and Production Integration Contract

**File:** `src/zuir/ZUIR-SPEC.md`
**Status:** Normative design specification; implementation conformance must be demonstrated separately
**Language:** Zamani
**Implementation language:** Rust 2021
**Minimum Rust version:** Rust 1.97
**Safety:** Safe Rust only; `unsafe` is prohibited
**Portability objective:** Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

## 1. Purpose

ZUIR defines the canonical, target-independent representation of computational meaning produced by the Zamani frontend and semantic analysis pipeline.

It MUST support the representation and composition of:

- Classical computation, control flow, functions, memory, and concurrency.
- Quantum computation, measurement, classical feedback, hybrid execution, and quantum resource requirements.
- Hardware description language (HDL), digital and mixed computational structures, signals, timing, and hardware/software co-design.
- Tensor, numerical, scientific, and AI/ML computation.
- Distributed computation, parallel execution, communication, networking, and data movement.
- Cryptographic computation and operations requiring explicit security or execution policies.
- New computational domains introduced after this specification is implemented.

ZUIR MUST provide a stable semantic foundation that allows domain-specific intermediate representations and target-specific implementations to evolve without requiring every new operation, processor, accelerator, quantum device, or execution model to modify the universal core.

ZUIR defines computational meaning and the information required to preserve it. It does not promise that every target can execute every program, nor does it prescribe one physical implementation for every operation.

### 1.1 Normative terminology

The words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** express specification requirements.

A requirement expressed with MUST or MUST NOT is mandatory for conformance unless a later, explicitly versioned amendment supersedes it.

### 1.2 Production-readiness distinction

This document defines the required architecture, interfaces, invariants, implementation sequence, and acceptance gates. A documented feature MUST NOT be reported as implemented or production-ready merely because this specification describes it.

Each feature MUST have an implementation status, a responsible file, a test owner, and a verifiable acceptance criterion.

---

## 2. Fundamental architectural principles

### 2.1 Separate syntax, meaning, representation, and execution

The canonical pipeline is:

```text
Zamani source / supported external formats
                  |
                  v
       Canonical lexical and parser layer
                  |
                  v
          Domain-neutral source AST
                  |
                  v
       Structural validation and semantics
                  |
                  v
       Canonical semantic model
                  |
                  v
                 ZUIR
                  |
          Verification and analysis
                  |
                  v
       Domain-specific intermediate forms
        /            |             \
       v             v              v
 Classical IR     quantum::ir     HDL/Hardware IR
        \            |             /
         \           |            /
          v          v           v
        Optimization, legalization and lowering
                  |
                  v
     Mapping, allocation, routing and scheduling
                  |
                  v
       Target-specific code or execution plan
                  |
                  v
        Runtime / simulator / physical target
```

The diagram specifies ownership and intended integration. It is not evidence that every stage is currently implemented.

### 2.2 Representation ownership

Each layer MUST have a single authoritative responsibility.

| Layer | Owns | MUST NOT own |
|---|---|---|
| Lexer and parser | Tokenization and source syntax | Hardware discovery, execution, or physical allocation |
| Source AST | Source structure, source identity, and relevant source metadata | Target instructions or runtime decisions |
| Semantic model | Name resolution, typing, effects, capabilities, contracts, and semantic facts | Target-specific code generation |
| ZUIR | Canonical computational meaning, generic data/control flow, effects, resource intent, and extension contracts | Source parsing or physical-device implementation |
| Domain IR | Specialized computation models and domain-specific invariants | A competing universal representation |
| Target/backend | Target legalization, representation, allocation, scheduling, and realization | Silent alteration of source-level meaning |
| Runtime | Execution, monitoring, failure handling, and deployment | Redefining the language's semantics |

### 2.3 One canonical universal core

ZUIR MUST have one canonical core representation.

A domain MAY provide its own dialect, type definitions, operation definitions, analyses, transformations, and lowering adapters. A domain MUST NOT introduce a second universal IR that duplicates ZUIR's ownership.

The existing `quantum::ir` MUST remain the canonical quantum semantic IR unless a separately reviewed architecture amendment establishes a migration. ZUIR MUST integrate with it through explicit adapters rather than silently replacing or duplicating its quantum-specific semantic model.

The same principle applies to existing classical and HDL representations: reuse, adapt, or deliberately migrate them under documented contracts.

---

## 3. POCO-REAF and scalability

### 3.1 Portability contract

A program SHOULD describe its intended computation, correctness conditions, resource requirements, constraints, capabilities, and optional preferences rather than requiring one specific physical machine.

A compatible compiler and execution environment MUST preserve the program's specified observable behavior, subject to the language's defined nondeterminism, concurrency, numerical rules, and execution model.

POCO-REAF means that the program's portable meaning can be reused across compatible targets and scales. It does not mean that one target's machine code is universally executable on all other targets.

### 3.2 No artificial universal machine limits

The universal ZUIR model MUST NOT encode architectural maxima for:

- Qubits, quantum registers, gates, operation arity, or circuit depth.
- CPUs, cores, threads, processes, GPUs, FPGAs, QPUs, or accelerators.
- Memory, storage, registers, vector widths, or address-space capacity.
- Tensor rank, tensor dimensions, network size, topology size, or device count.
- Functions, modules, blocks, regions, operations, declarations, or program size.
- Clock domains, pipeline stages, signal widths, channels, actors, or distributed nodes.

The forbidden design pattern is a language-wide constant that restricts a computational domain to an arbitrary fixed capacity.

Numeric literals and finite-width types MAY exist when they are actual program semantics. A source-level integer value is not an implementation limit merely because it happens to be numerically large or small.

### 3.3 The meaning of unbounded scalability

“From atom to everywhere” means that ZUIR introduces no arbitrary, universal machine-size ceiling and supports composition, partitioning, specialization, and lowering to the extent permitted by the available implementation and resources.

It does NOT promise literal infinity, unlimited memory, infinite compilation time, or execution beyond a target's physical capabilities.

Every concrete program, process, serialized artifact, and physical execution remains finite.

### 3.4 Explicit limits and resource exhaustion

Implementations MAY enforce resource budgets to protect against malformed input, denial of service, pathological graphs, excessive compile-time evaluation, or exhausted memory.

Each budget MUST:

1. Be explicitly represented in configuration or the applicable security policy.
2. Have a documented unit, scope, and defaulting rule.
3. Be distinguishable from a semantic restriction and a physical target capability.
4. Produce a structured diagnostic or error on exhaustion.
5. Avoid silently truncating operations, values, dimensions, references, or required metadata.
6. Avoid silently changing program meaning.

A configurable safety budget MUST NOT be presented as a universal language limit.

### 3.5 No assumptions about infinite resources

The implementation MUST use dynamically sized collections where appropriate, checked arithmetic, explicit allocation-failure handling where supported, and algorithms that can report resource exhaustion.

It MUST NOT claim that a collection can exceed the host address space or that a computation can complete when required resources do not exist.

---

## 4. Authority and repository integration

### 4.1 Authority hierarchy

The repository MUST maintain one consistent authority chain.

1. `DESIGN.md` and the repository's governing architecture contracts.
2. The authoritative Zamani language specification under `grammar/specification/`, together with applicable normative contracts under `grammar/spec/`.
3. This file, `src/zuir/ZUIR-SPEC.md`, for ZUIR-specific semantics and interfaces.
4. ZUIR schema and API contracts.
5. Conforming Rust implementation.
6. Conformance tests, integration tests, and reference examples.
7. Generated implementation-conformance documentation and explanatory guides.

If an existing repository document conflicts with this specification, implementation MUST NOT arbitrarily select whichever interpretation is convenient. The conflict MUST be recorded, resolved by the owners of the relevant authority documents, and reflected in a versioned change.

### 4.2 Grammar integration

The following existing paths are relevant and MUST be audited:

- `grammar/Zamani.g4` — canonical ANTLR grammar composition root.
- `grammar/antlr/ZamaniLexer.g4` — canonical lexer composition boundary.
- `grammar/antlr/ZamaniParser.g4` — canonical parser composition boundary.
- `grammar/specification/` — normative language specification.
- `grammar/spec/` — supporting specifications and contracts.
- `grammar/grammar.md` — implementation-conformance reference.
- `grammar/Zamani-Grammar.md` — historical and extended design reference, subject to its declared status.
- `grammar/contracts/` and `grammar/validation/` — existing mapping and validation contracts where applicable.

ZUIR MUST NOT independently redefine source syntax or require grammar changes merely to introduce a new registered IR operation.

A new source-language feature MUST be integrated through the appropriate language specification, grammar, AST, and semantic-analysis contracts before its lowering to ZUIR is considered complete.

External formats MUST use their respective import frontends and explicit mapping contracts. Imported syntax MUST NOT bypass semantic validation simply because it originates from an external representation.

### 4.3 Existing implementation integration

The following paths MUST be examined and integrated through explicit contracts:

- `src/zuir/mod.rs` — ZUIR module root and public API.
- `src/frontend/ast/` — source AST.
- `src/frontend/semantic/` — semantic analysis and semantic facts.
- `src/semantic.rs` — existing semantic-analysis implementation, where still used.
- `src/ir_gen.rs` — existing IR generation and instruction representation.
- `src/ir_verify.rs` — existing IR verification.
- `src/quantum/ir/` — canonical quantum semantic IR.
- Existing classical IR, HDL/hardware IR, optimization, compiler, runtime, and backend modules discovered during the audit.
- `Cargo.toml` — Rust version, dependencies, features, and build constraints.

These paths are integration candidates, not an assertion that all are already wired into one pipeline.

The repository audit MUST determine which implementations are authoritative, which are compatibility surfaces, and which are candidates for migration or retirement.

### 4.4 No duplicate sources of truth

No file may independently define a competing version of:

- ZUIR entity identity.
- Canonical type identity.
- Operation semantics.
- Dialect compatibility.
- Resource requirement semantics.
- Error-code ownership.
- Serialization version rules.
- Canonical fingerprint rules.
- Quantum identity or quantum semantic ownership.

Compatibility exports MAY expose an authoritative type under a historical path. They MUST NOT create duplicate type definitions or separate behavioral implementations.

---

## 5. Core representation contract

### 5.1 Required concepts

The core model MUST be capable of representing, directly or through validated extensible descriptors:

- Programs and modules.
- Symbols, declarations, functions, and signatures.
- Types, type references, type constraints, and type parameters.
- Values, constants, expressions, operands, and results.
- Regions, blocks, control-flow edges, and terminators.
- Operations, qualified operation names, attributes, and extension data.
- Effects, ordering, ownership, and relevant lifetime constraints.
- Resource requirements, capabilities, constraints, preferences, and hints.
- Source provenance and transformation provenance.
- Dialect dependencies and version information.
- Verification state and diagnostic context, without confusing diagnostics with program semantics.

An implementation MAY organize these concepts differently from the proposed file tree, but each concept MUST have a unique owner and a testable contract.

### 5.2 Program and module identity

Every program and module MUST have an identity within its documented identity scope.

The implementation MUST distinguish:

- In-memory identity.
- Stable serialized identity, where required.
- Human-readable names.
- Linkage and symbol identity.
- Content fingerprints.

A memory-local identifier MUST NOT be treated as a stable cross-process identity unless its contract explicitly guarantees that property.

### 5.3 Operations

Each operation MUST have a qualified name consisting of an extension or dialect namespace and an operation name.

The universal core MUST NOT enumerate every domain operation in one closed universal operation enum.

An operation definition MUST declare, as applicable:

- Operand and result schemas.
- Type and shape constraints.
- Attribute schemas.
- Region and control-flow requirements.
- Terminator behavior.
- Declared effects and ordering requirements.
- Required capabilities and resource constraints.
- Verification rules.
- Compatibility and versioning rules.
- Lowering interfaces or an explicit unsupported-lowering status.

Unknown operations MUST NOT acquire executable meaning through name matching, implicit guessing, or permissive deserialization.

An implementation MAY preserve an unknown operation as opaque, non-executable data when its schema and preservation policy permit it. Such preservation MUST NOT be confused with successful semantic verification or permission to execute.

### 5.4 Values and identity

Value references MUST be unambiguous and MUST resolve to a valid definition or explicitly specified external/imported value.

The core MUST distinguish a value's semantic identity from its serialized position or incidental collection index.

Every result MUST have a declared type or an explicitly specified representation for unresolved or symbolic typing during a permitted intermediate phase.

Malformed references MUST be rejected by verification.

### 5.5 SSA policy

The canonical core SHOULD use a formally specified SSA-style value model where appropriate to the computation being represented.

If SSA is used, the specification and implementation MUST define:

- Single-definition rules.
- Block arguments and merge semantics.
- Dominance and legal use positions.
- Def-use consistency.
- Control-flow terminators and edge arguments.
- Effect ordering and observable side effects.
- Rules for regions that introduce distinct control or execution models.

A specialized domain MAY use a model that is not ordinary SSA when its semantics require it. Such a domain MUST declare its model and validation rules explicitly; it MUST NOT silently violate core SSA invariants.

The chosen representation MUST support classical control flow and domain-specific control models without assuming every computation is a simple linear sequence of instructions.

---

## 6. Type system and values

### 6.1 Type extensibility

ZUIR MUST support type definitions for primitive, aggregate, product, sum, nominal, opaque, generic, parameterized, and extension-defined types as required by the language's type system.

The universal core MUST distinguish semantic type identity from physical representation.

It MUST support symbolic type constraints and parameterized shapes without imposing a universal maximum bit width, tensor rank, array length, or domain-specific type count.

### 6.2 Layout and representation

Logical type meaning MUST be separate from target-specific representation.

Physical size, alignment, ABI conventions, register assignment, memory placement, signal mapping, and device-specific layout MUST be supplied by the relevant target or domain layer.

A logical type MUST NOT silently change meaning because one target represents it differently.

Any conversion between representations MUST be explicit and validated.

### 6.3 Literals and constants

Literal representations MUST avoid accidental host-width truncation.

The implementation MUST define the semantics of integer overflow, floating-point values, NaNs, signed zero, rounding, and conversions according to the relevant language contracts.

Exact or arbitrary-precision numeric forms SHOULD be used where required by source semantics or domain correctness.

Constant evaluation MUST be deterministic where its operations are specified as deterministic, and MUST obey explicit resource budgets.

An unsupported or exhausted constant evaluation MUST return a defined result or diagnostic rather than silently substituting an incorrect value.

### 6.4 Symbolic expressions

ZUIR MUST be able to preserve expressions whose values depend on compile-time parameters, runtime values, symbolic dimensions, constraints, or target-specific facts.

The IR MUST NOT force every resource requirement, shape, width, topology, or timing parameter to be resolved before the relevant compilation phase can proceed.

Unresolved expressions MUST carry sufficient information to be checked, constrained, specialized, or rejected later.

---

## 7. Effects, ownership, ordering, and concurrency

### 7.1 Effects

Effects MUST be represented independently from operation names and from target capability declarations.

The model MUST be extensible to include effects such as:

- Memory reads and writes.
- Allocation and deallocation.
- I/O and external calls.
- Synchronization and communication.
- Nondeterminism and observable state changes.
- Quantum measurement and measurement-dependent control.
- Timing-sensitive or signal-related behavior.
- Reflection, dynamic loading, code generation, or other declared extension effects.

The standard effect vocabulary MAY grow. Adding a new effect MUST NOT require a redesign of the universal operation representation.

### 7.2 Effect preservation

An optimization or lowering MUST preserve all observable effects and required ordering.

An operation MUST NOT be reordered, removed, duplicated, speculated, or fused when that change violates its effect, ownership, control, timing, or domain-specific contract.

Unknown effects MUST be treated conservatively. They MUST NOT be assumed pure.

### 7.3 Ownership and linear resources

Where ownership, borrowing, aliasing, lifetimes, linearity, or affine use are relevant, ZUIR MUST preserve the semantic facts needed for verification and lowering.

Quantum resources MUST NOT be assumed to have the same copying, aliasing, or disposal behavior as ordinary classical values.

The core MUST provide extensible validation interfaces for domain-specific ownership rules rather than hard-coding all future resource semantics into a universal enum.

### 7.4 Concurrency and distributed execution

The representation MUST distinguish data dependencies, control dependencies, effect ordering, synchronization, and communication dependencies.

Parallel execution MUST NOT be inferred solely from the absence of a direct data dependency when effects or domain rules prevent concurrent execution.

Distributed execution MUST preserve the specified ordering, consistency, failure, and communication semantics.

The IR MUST NOT require a fixed number of workers, nodes, actors, channels, or execution devices.

---

## 8. Resource and capability semantics

### 8.1 Five distinct concepts

ZUIR MUST keep the following concepts distinct:

1. **Requirement:** necessary for a valid realization of the program.
2. **Capability:** an ability offered by a target or execution environment.
3. **Constraint:** a condition that a valid realization must satisfy.
4. **Preference:** an optimization preference that may be unmet without changing program meaning, where the language contract allows it.
5. **Hint:** non-authoritative implementation guidance.

A hint MUST NOT silently become a requirement. A preference MUST NOT override a requirement. A missing required capability MUST NOT be concealed by a successful-looking lowering result.

### 8.2 Resource requirements

The resource model MUST support symbolic and extensible dimensions, including processing, memory, quantum resources, bandwidth, precision, latency, energy, topology, storage, and domain-defined resources.

The universal core MUST NOT define a fixed catalogue that prevents future resource dimensions from being represented.

Resource requirements MUST be separable from concrete allocations. A requirement describes what is needed; allocation identifies what a particular realization uses.

### 8.3 Capability negotiation

Target feasibility MUST be evaluated using explicit target capability descriptions and compilation policies.

The process MUST distinguish:

- Satisfied requirements.
- Unsatisfied requirements.
- Unknown or unverified capabilities.
- Constraints that cannot yet be solved.
- Preferences that cannot be fulfilled.
- Authorized alternative execution strategies.

Unknown capability status MUST NOT be treated as success.

### 8.4 Failure and fallback

When no valid realization is available, the compiler MUST return a structured, actionable result explaining the incompatibility.

Fallback is permitted only if authorized and semantically valid. Examples include simulation or a different supported execution mechanism, provided the specified program semantics remain preserved.

The implementation MUST NOT silently drop a quantum measurement, weaken a hardware timing guarantee, remove a required security property, change numerical precision beyond its contract, or discard required effects to make a target appear compatible.

---

## 9. Quantum integration

### 9.1 Canonical ownership

`src/quantum/ir/` remains the authoritative quantum semantic representation.

ZUIR MUST represent the universal program and its cross-domain composition. `quantum::ir` MUST retain ownership of the specialized quantum semantics defined by its own contracts.

Adapters MUST map between the representations explicitly and preserve the relevant semantics.

### 9.2 Quantum concepts

The combined architecture MUST support, directly or through the canonical quantum adapter:

- Logical quantum resources and quantum values.
- Quantum operations with extensible names, operands, controls, parameters, results, and attributes.
- Measurement, classical results, and feedback.
- Hybrid classical/quantum control flow.
- Quantum ownership and permitted resource use.
- Circuit and non-circuit computational models.
- Symbolic resource requirements and topology constraints.
- Analog, pulse, timing, calibration-related intent, and other specialized models where supported by the quantum IR.
- Explicit lowerability and target-feasibility status.

The universal core MUST NOT maintain a permanent closed gate list. Gate and operation semantics MUST be owned by the relevant dialect or quantum model.

### 9.3 Physical implementation boundary

Physical qubit selection, device calibration, routing, native instruction selection, pulse synthesis, quantum error correction, and device execution belong to the specialized quantum pipeline and target layers.

ZUIR MUST NOT embed a particular vendor, device, calibration table, physical topology, or physical qubit assignment into the portable core semantics.

### 9.4 Quantum correctness

Lowering MUST preserve the semantics of measurement, state evolution, control dependencies, classical feedback, and any relevant timing or resource contracts.

A simulator MAY be an authorized realization, but it MUST NOT be assumed equivalent to a physical execution in all observable respects. Any distinction specified by the program's contract MUST remain explicit.

---

## 10. Classical, HDL, and future-domain integration

### 10.1 Classical computation

The core MUST support typed values, functions, control flow, calls, memory effects, and domain-appropriate operations.

Target-specific instruction selection, physical registers, ABI details, and machine scheduling belong downstream.

The existing `src/ir_gen.rs` representation MUST be audited for semantic coverage and migrated through a deliberate adapter or replacement plan. The current instruction list MUST NOT become the universal catalogue of future ZUIR operations.

### 10.2 HDL and hardware design

HDL integration MUST preserve the distinctions among hardware structure, logical signals, timing, clock domains, reset semantics, sequential and combinational behavior, and physical realization.

Symbolic widths, generic parameters, and constraint-based hardware descriptions MUST be supported where required.

Synthesis, placement, routing, board-specific constraints, and physical timing closure MUST belong to appropriate downstream stages.

ZUIR MUST NOT pretend that ordinary software control flow and clocked hardware behavior are interchangeable without a defined semantic mapping.

### 10.3 AI, tensors, and scientific computation

The representation MUST allow domain extensions for symbolic shapes, tensor operations, numerical precision, differentiation, model computation, and data movement.

It MUST NOT impose a universal maximum tensor rank or model dimension.

Floating-point transformations MUST preserve the specified numerical contract. Reassociation, fusion, approximation, reduced precision, and nondeterministic reductions MUST be governed by explicit semantic permissions.

### 10.4 Distributed systems and networking

Extensions MUST be able to represent communication, data movement, synchronization, topology requirements, security constraints, and distributed failure semantics.

Network addressing, device placement, transport selection, deployment configuration, and routing decisions belong to appropriate downstream layers unless explicitly part of the program's meaning.

### 10.5 Future domains

A new domain MUST be addable through versioned extension contracts, registered schemas, operation definitions, types, interfaces, verifiers, and lowering adapters.

Adding a new domain MUST NOT require editing the universal core operation model for each new operation.

A domain MAY require a versioned extension to the core when a genuinely new universal semantic primitive is necessary. Such changes require an architecture review, compatibility analysis, and conformance tests.

---

## 11. Dialects and extension system

### 11.1 Dialect identity

Every dialect MUST have a qualified, stable namespace, declared version, manifest, and compatibility policy.

A dialect manifest MUST declare its dependencies and the schemas and interfaces it contributes.

Namespace collisions and incompatible dependencies MUST be rejected deterministically.

### 11.2 Operation schemas

An operation schema MUST declare the properties necessary to validate an operation without requiring the universal core to understand every domain's internal semantics.

Schemas MUST support extension-defined attributes and types while preserving the distinction between recognized semantics and opaque data.

Reserved attributes MUST be validated according to their declared schemas.

### 11.3 Executable extension behavior

Executable extension hooks MUST have explicit trust and registration boundaries.

Untrusted serialized metadata MUST NOT be executed as code.

An extension MUST NOT gain filesystem, network, process, or hardware access merely by being registered as descriptive metadata.

Verification and lowering hooks MUST report errors through the documented structured interface.

### 11.4 Versioning

Changes to operation semantics, type identity, required attributes, effect behavior, or serialized meaning MUST follow the declared compatibility policy.

An operation MUST NOT change meaning silently under an unchanged identity and version contract.

Migration hooks MUST be explicit, testable, and deterministic wherever their operations are defined as deterministic.

---

## 12. Verification

### 12.1 Mandatory verification categories

The verifier MUST support the applicable checks for:

- Structural integrity and identifier uniqueness.
- Valid ownership and reference resolution.
- Type and constraint consistency.
- Region, block, and terminator correctness.
- Control-flow edge integrity and legal value use.
- SSA, dominance, and def-use consistency where applicable.
- Effect declarations and effect-ordering requirements.
- Ownership and linear-resource rules.
- Resource and capability constraints.
- Dialect registration and operation schemas.
- Extension and version compatibility.
- Serialization validity and required metadata integrity.
- Domain-specific invariants through registered verifier interfaces.

### 12.2 Verification modes

Verification MAY provide documented levels, such as structural validation, type validation, full semantic validation, and domain-specific validation.

A weaker verification mode MUST NOT be reported as equivalent to full verification.

The API MUST identify which checks were performed and whether unresolved obligations remain.

### 12.3 Invalid IR

Malformed IR MUST NOT proceed to a stage that assumes it is valid unless that stage explicitly accepts and safely handles the relevant invalid state.

Verification failures MUST be deterministic in their classification and stable in diagnostic codes.

The verifier MUST report actionable context without depending on terminal formatting or a particular user interface.

### 12.4 Complexity and robustness

Verification algorithms SHOULD avoid unnecessary repeated traversal and MUST document their expected complexity where practical.

Cycle handling, recursive types, deeply nested regions, very large graphs, and malformed references MUST be handled without unbounded recursion or uncontrolled resource consumption.

Any implementation budget MUST be configurable and failures MUST be explicit.

---

## 13. Analysis and transformation contracts

### 13.1 Read-only analysis

Analyses MUST clearly state whether they are read-only and which facts they derive.

Derived indexes and caches MUST NOT become competing semantic authorities.

Any cached analysis MUST be associated with the program revision or dependency state to which it applies.

### 13.2 Pass contracts

Each transformation pass MUST declare:

- Purpose and owned transformation.
- Input invariants and required dialects.
- Output invariants.
- Semantic-preservation obligations.
- Effects on source and transformation provenance.
- Analyses consumed and invalidated.
- Determinism guarantees.
- Resource-budget behavior.
- Diagnostics and failure behavior.
- Tests and freeze criteria.

### 13.3 Mandatory postconditions

A pass that claims to produce verified ZUIR MUST ensure its output satisfies the required verification level.

A failed pass MUST NOT expose a partially mutated program as a successful result.

Where atomic transformation cannot be guaranteed, partial-state ownership and recovery behavior MUST be explicit.

### 13.4 Legalization

Legalization MUST make target or dialect restrictions explicit.

An operation that cannot be lowered under the requested constraints MUST result in a precise failure or an authorized, semantics-preserving alternative.

Legalization MUST NOT erase unsupported operations or convert them to meaningless placeholders and report success.

---

## 14. Lowering and POCO-REAF integration

### 14.1 Lowering boundary

Lowering converts ZUIR into a specified downstream representation using an explicit lowering context.

The context MUST identify the source and destination representations, relevant dialect versions, target capabilities, constraints, policy, and resource budgets.

### 14.2 Required preservation

Every lowering adapter MUST document how it preserves or accounts for:

- Types and value identity.
- Control and data dependencies.
- Observable effects and ordering.
- Ownership and lifetime constraints.
- Numerical and concurrency semantics.
- Quantum measurement and feedback.
- HDL timing and signal semantics.
- Resource requirements and capabilities.
- Provenance and diagnostics.
- Unsupported operations and partial conversion.

### 14.3 Lowering result

A successful lowering MUST provide a valid destination representation and its relevant verification status.

An unsuccessful lowering MUST provide structured diagnostics and MUST NOT masquerade as a complete artifact.

Partial lowering MAY be supported for tooling or staged compilation, but its incomplete status MUST be explicit and it MUST NOT be treated as executable without the required subsequent stages.

### 14.4 Target independence

Target-specific facts MUST enter through explicit target descriptions and lowering contexts, not through hidden global state or hard-coded machine assumptions in the core.

Target-independent ZUIR MUST remain reusable even when a particular target or backend is unavailable.

---

## 15. Serialization, interchange, and canonical fingerprints

### 15.1 Versioned serialization

Serialized ZUIR MUST identify the schema or format version and the required dialect versions.

Decoding MUST validate the envelope and applicable schemas before treating the decoded object as valid IR.

Unknown fields MAY be preserved when the format's forward-compatibility rules permit it. Unknown executable semantics MUST NOT be inferred.

### 15.2 Round-trip correctness

For supported schema versions, encode/decode round trips MUST preserve the semantics and the fields guaranteed by the format contract.

Loss of semantic information MUST cause a defined error or explicit conversion result.

Source metadata and non-semantic annotations MAY follow separate preservation guarantees, but these guarantees MUST be documented.

### 15.3 Canonical representation

Canonical serialization MUST define ordering, number representation, string encoding, and treatment of semantically irrelevant metadata.

Canonicalization MUST NOT reorder operations or remove metadata when doing so changes semantics or violates an explicit preservation guarantee.

### 15.4 Fingerprinting

A canonical fingerprint MUST be computed from the fields defined by the fingerprint contract, using a documented stable algorithm and version.

The contract MUST distinguish semantic fingerprints from exact serialized-byte hashes and build-cache keys.

A memory address, process-randomized hash, unspecified map iteration order, or incidental identifier allocation order MUST NOT silently determine a portable fingerprint.

Cryptographic hashes SHOULD be used where collision resistance is required. Cache correctness MUST NOT depend solely on an unverified assumption that hash collisions are impossible.

### 15.5 Import and export

External formats MUST be imported through explicit adapters.

Every import adapter MUST declare which source constructs are represented exactly, approximated, rejected, or retained as opaque metadata.

Every export adapter MUST report unsupported or lossy conversions. It MUST NOT silently discard required meaning.

---

## 16. Determinism and reproducibility

### 16.1 Deterministic core behavior

For equivalent canonical input, the same schema and dialect versions, and the same relevant configuration and pass versions, operations defined as deterministic MUST produce equivalent canonical results.

Determinism MUST NOT depend on wall-clock time, host memory addresses, hash-map iteration order, uncontrolled randomness, or the incidental order in which unrelated tasks finish.

### 16.2 Nondeterministic semantics

Nondeterministic computation is permitted when defined by the language and domain contracts.

Its presence MUST be represented or otherwise preserved explicitly where it affects observable behavior.

The compiler MUST NOT claim deterministic execution merely because the serialized IR is deterministic.

### 16.3 Reproducible builds

Build and cache contracts MUST identify all semantic inputs that affect results, including relevant compiler versions, dialect versions, target descriptions, configuration, and pass pipelines.

Any intentionally omitted input MUST be demonstrated to be irrelevant to the result under the applicable contract.

---

## 17. Incremental compilation and caching

Incremental compilation MAY reuse prior analysis or transformation results only when their dependencies remain valid.

The dependency model MUST cover relevant changes to:

- IR entities and referenced definitions.
- Type and constraint information.
- Operation and dialect schemas.
- Effect and capability declarations.
- Compiler and pass versions.
- Target descriptions and lowering policies.
- Configuration that changes semantic results.

Invalidation MUST be conservative when dependencies cannot be established.

A stale cached result MUST NOT be treated as valid merely because the top-level program identifier remains unchanged.

Cache keys MUST be versioned and derived from the declared semantic inputs.

---

## 18. Diagnostics, provenance, and observability

### 18.1 Structured diagnostics

Diagnostics MUST have stable machine-readable codes, severity, contextual information, and a documented relationship to recoverability.

Source spans MAY be absent for generated or imported IR, but the absence MUST be represented explicitly.

Diagnostics MUST be usable by command-line tools, editors, automated tests, and APIs without depending on terminal-specific formatting.

### 18.2 Provenance

The IR MUST support tracing entities to source constructs, imported formats, generated constructs, and transformation origins where provenance is available.

Transformations MUST update or preserve provenance according to their declared contracts.

Provenance MUST NOT be mistaken for semantic identity.

### 18.3 Observability

Debug dumps, tracing, metrics, and diagnostic rendering MAY be provided by separate tooling layers.

Observability features MUST NOT alter the meaning of the program or inject hidden target decisions.

Sensitive values and secrets MUST not be exposed inadvertently through diagnostics, logs, serialized metadata, or fingerprints.

---

## 19. Rust implementation and safety

### 19.1 Language requirements

The implementation MUST compile on the repository's declared Rust baseline, Rust 1.97, using the Rust 2021 edition and stable language features.

The implementation MUST NOT depend on nightly-only features.

### 19.2 Prohibition of unsafe Rust

ZUIR MUST use safe Rust only.

The ZUIR crate/module MUST enforce this with:

```rust
#![forbid(unsafe_code)]
```

The prohibition includes direct unsafe blocks, unsafe functions, and unsafe implementations in ZUIR-owned code.

A dependency's internal implementation is subject to its own supply-chain and safety review; using a dependency does not permit ZUIR to introduce unsafe code.

### 19.3 Error handling

Expected invalid input, unsupported dialects, incompatible versions, verification failures, resource exhaustion, and lowering failures MUST use structured error results.

Panics MUST NOT be used as ordinary error handling for user-controlled input.

The implementation MUST use checked arithmetic where overflow could affect correctness, allocation sizes, indices, counts, identifiers, or serialization lengths.

### 19.4 Allocation and synchronization

Allocation and synchronization strategies MUST be selected according to documented ownership and concurrency requirements.

Global mutable state MUST NOT be required for correctness or deterministic behavior.

The implementation SHOULD favor explicit contexts, owned data, and well-defined shared-state contracts.

### 19.5 Dependencies

Dependencies MUST be reviewed for maintenance, license compatibility, security, Rust-version compatibility, and suitability for the required guarantees.

ZUIR MUST NOT require a particular hardware vendor SDK merely to represent portable IR.

---

## 20. Public API and module ownership

The public API MUST expose supported operations for constructing, inspecting, validating, serializing, transforming, and lowering ZUIR, as those capabilities become implemented.

The API MUST distinguish verified from unverified IR and complete from partial lowering.

Public exports MUST be deliberate. Glob re-exports SHOULD be avoided when they create ambiguous ownership or accidental compatibility commitments.

Adding an internal module MUST NOT force unrelated public API changes unless its new behavior genuinely changes the public contract.

The module root MUST remain a composition and export boundary rather than a second implementation of IR semantics.

Compatibility APIs MAY remain available during migration, but their ownership and deprecation status MUST be documented.

---

## 21. Proposed file structure and responsibilities

The following structure is the target architecture. It MUST be reconciled with the actual repository before files are created. Existing authoritative files SHOULD be retained when they already satisfy a responsibility; duplicate implementations MUST NOT be introduced just to match a directory plan.

```text
src/zuir/
├── mod.rs
├── api.rs
├── config.rs
├── ids.rs
├── version.rs
├── errors.rs
├── diagnostics.rs
├── source_map.rs
├── spec/
│   ├── mod.rs
│   ├── invariants.rs
│   ├── compatibility.rs
│   └── conformance.rs
├── schema/
│   ├── mod.rs
│   ├── types.rs
│   ├── attributes.rs
│   ├── metadata.rs
│   └── extensions.rs
├── types/
│   ├── mod.rs
│   ├── type_id.rs
│   ├── type_def.rs
│   ├── layout.rs
│   ├── constraints.rs
│   └── compatibility.rs
├── values/
│   ├── mod.rs
│   ├── symbols.rs
│   ├── literals.rs
│   ├── constants.rs
│   └── expressions.rs
├── ir/
│   ├── mod.rs
│   ├── program.rs
│   ├── module.rs
│   ├── function.rs
│   ├── region.rs
│   ├── block.rs
│   ├── operation.rs
│   ├── operand.rs
│   ├── result.rs
│   ├── successor.rs
│   ├── use_def.rs
│   ├── effects.rs
│   ├── ownership.rs
│   └── resources.rs
├── dialect/
│   ├── mod.rs
│   ├── definition.rs
│   ├── operation.rs
│   ├── interfaces.rs
│   ├── traits.rs
│   ├── versioning.rs
│   └── registry.rs
├── builder/
│   ├── mod.rs
│   ├── program.rs
│   ├── operation.rs
│   ├── regions.rs
│   └── validation.rs
├── verify/
│   ├── mod.rs
│   ├── structural.rs
│   ├── types.rs
│   ├── dominance.rs
│   ├── control_flow.rs
│   ├── effects.rs
│   ├── ownership.rs
│   ├── resources.rs
│   ├── dialects.rs
│   └── full.rs
├── analysis/
│   ├── mod.rs
│   ├── cfg.rs
│   ├── dominance.rs
│   ├── def_use.rs
│   ├── effects.rs
│   ├── liveness.rs
│   ├── resources.rs
│   └── provenance.rs
├── pass/
│   ├── mod.rs
│   ├── manager.rs
│   ├── registry.rs
│   ├── contracts.rs
│   ├── invalidation.rs
│   └── determinism.rs
├── transform/
│   ├── mod.rs
│   ├── canonicalize.rs
│   ├── simplify.rs
│   ├── inline.rs
│   ├── control_flow.rs
│   └── legalize.rs
├── lowering/
│   ├── mod.rs
│   ├── api.rs
│   ├── context.rs
│   ├── legality.rs
│   ├── diagnostics.rs
│   └── provenance.rs
├── interchange/
│   ├── mod.rs
│   ├── format.rs
│   ├── import.rs
│   ├── export.rs
│   └── migration.rs
├── serialization/
│   ├── mod.rs
│   ├── schema.rs
│   ├── encode.rs
│   ├── decode.rs
│   ├── validation.rs
│   └── canonical.rs
├── fingerprint/
│   ├── mod.rs
│   └── stable_hash.rs
├── incremental/
│   ├── mod.rs
│   ├── dependencies.rs
│   ├── invalidation.rs
│   └── cache_keys.rs
├── dialects/
│   ├── mod.rs
│   ├── core/
│   ├── classical/
│   ├── quantum/
│   ├── hdl/
│   ├── tensor/
│   ├── ai/
│   ├── distributed/
│   ├── networking/
│   └── crypto/
└── tests/
    ├── api.rs
    ├── schema.rs
    ├── builders.rs
    ├── verifier.rs
    ├── types.rs
    ├── control_flow.rs
    ├── effects.rs
    ├── ownership.rs
    ├── resources.rs
    ├── dialects.rs
    ├── lowering.rs
    ├── serialization.rs
    ├── compatibility.rs
    ├── determinism.rs
    ├── incremental.rs
    ├── property.rs
    ├── fuzz.rs
    ├── stress.rs
    └── conformance/
```

A directory MUST be added only when it has an approved responsibility and implementation or a concrete, immediately scheduled set of modules. Empty modules MUST NOT be treated as production functionality.

---

## 22. File-by-file ownership and integration contracts

Every file MUST have one primary responsibility, a stable interface, an owner, and tests. The table below defines the planned contract before implementation begins.

| File or group | Owns | Dependencies and integration contract |
|---|---|---|
| `ZUIR-SPEC.md` | Normative ZUIR semantics and invariants | Align with repository architecture and language specifications; no implementation dependency |
| `mod.rs` | Module declarations and selected public re-exports | Declares only existing modules; no duplicated algorithms |
| `api.rs` | Stable public entry points and facade types | Uses the frozen IDs, version, errors, builder, verifier, and lowering contracts |
| `ids.rs` | Typed identities and identity-scope rules | Independent of IR nodes; consumed by schema and IR modules |
| `version.rs` | Format, schema, and compatibility version values | Independent of IR implementation; consumed by serialization, dialects, and migration |
| `errors.rs` | Structured error kinds and context | Independent of verifier internals; consumed by all fallible modules |
| `config.rs` | Per-invocation configuration and explicit budgets | Uses stable policy types; consumed by builder, verifier, passes, serialization, and lowering |
| `diagnostics.rs` | Diagnostic codes, severity, notes, and reporting data | Integrates source maps and error categories; does not own parser diagnostics |
| `source_map.rs` | Source and transformation provenance | Uses source-span adapters and IR identities; does not resolve names |
| `spec/invariants.rs` | Machine-checkable invariant identifiers and descriptions | Mirrors this specification; each invariant maps to a verifier or test |
| `spec/compatibility.rs` | Compatibility and migration policy | Governs public APIs and serialized formats |
| `spec/conformance.rs` | Required conformance behavior | Supplies reusable test obligations to all dialects and adapters |
| `schema/types.rs` | Schema-level descriptor primitives | Independent of builders and concrete operation nodes |
| `schema/attributes.rs` | Attribute value schemas and validation contracts | Uses schema primitives; consumed by dialect definitions and verifier |
| `schema/metadata.rs` | Semantic versus descriptive metadata classification | Governs serialization, fingerprinting, and transformation preservation |
| `schema/extensions.rs` | Extension manifests and namespaces | Uses version contracts; consumed by the dialect registry |
| `types/type_id.rs` | Type identity and type references | Independent of type definitions; defines nominal/structural identity policy |
| `types/layout.rs` | Logical layout constraints | Independent of physical target layouts; target adapters resolve physical representation |
| `types/constraints.rs` | Symbolic type and shape constraints | Consumed by type definitions, verifier, and target feasibility analysis |
| `types/compatibility.rs` | Type equivalence and conversion legality | Uses type identity and definitions; distinguishes semantic from physical compatibility |
| `types/type_def.rs` | Type definitions and extension points | Uses frozen type IDs and constraints; consumed by the verifier and dialect registry |
| `values/symbols.rs` | Symbol identity, linkage, and references | Consumes resolved semantic identity; does not implement name resolution |
| `values/literals.rs` | Portable literal representation | Independent of target layout; consumed by constants and serialization |
| `values/constants.rs` | Typed constant values and evaluation outcomes | Uses literal/type contracts and explicit budgets |
| `values/expressions.rs` | Symbolic and typed value expressions | Uses type/value contracts; evaluator and simplifier remain separate |
| `ir/program.rs` | Program identity, modules, required dialects, global metadata | Uses foundational types; exposes deterministic traversal |
| `ir/module.rs` | Declarations, symbols, imports, and exports | Uses program/type contracts; consumed by symbol validation |
| `ir/function.rs` | Signatures, parameters, results, effects, linkage, and body regions | Uses type, symbol, and region contracts |
| `ir/region.rs` | Nested computations, captures, and region semantics | Uses block and value contracts; domain extensions declare specialized rules |
| `ir/block.rs` | Block arguments, operation order, and terminator structure | Uses operation, result, and successor contracts |
| `ir/operation.rs` | Generic operation representation | Uses qualified operation identity and schemas; contains no closed list of every domain operation |
| `ir/operand.rs` | Operand references and roles | Uses value identity and schema contracts |
| `ir/result.rs` | Result identity, type, and definition ownership | Uses value and type contracts |
| `ir/successor.rs` | Control-flow edges and edge arguments | Uses block and value contracts; consumed by control-flow verification |
| `ir/use_def.rs` | Def-use index contract and consistency | Derived indexes MUST be checked against authoritative definitions |
| `ir/effects.rs` | Effect descriptors and ordering contracts | Extensible and independent of target capability discovery |
| `ir/ownership.rs` | Ownership and linear/affine constraints | Uses value identity; integrates semantic facts and domain verification |
| `ir/resources.rs` | Symbolic requirements and resource constraints | Independent of physical allocation; consumed by feasibility analysis |
| `dialect/definition.rs` | Dialect manifests and declared dependencies | Uses schema/version contracts |
| `dialect/operation.rs` | Operation schemas and verifier declarations | Uses schema types and effect contracts |
| `dialect/interfaces.rs` | Stable reusable behavioral interfaces | Does not encode individual vendor instructions |
| `dialect/traits.rs` | Reusable operation properties and semantic contracts | Must distinguish declarative traits from trusted executable hooks |
| `dialect/versioning.rs` | Dialect evolution and migration | Uses version and compatibility contracts |
| `dialect/registry.rs` | Registration, namespace uniqueness, and deterministic lookup | Consumes validated dialect definitions |
| `builder/validation.rs` | Builder preconditions and validation policy | Reuses verifier rules; MUST NOT create a competing verifier |
| `builder/operation.rs` | Schema-driven operation construction | Uses the registry and core operation model |
| `builder/regions.rs` | Region, block, terminator, and edge construction | Uses frozen structural invariants |
| `builder/program.rs` | Program assembly | Uses module, symbol, type, and dialect contracts |
| `builder/mod.rs` | Supported builder facade | Exposes validated construction; keeps unchecked internals private |
| `verify/structural.rs` | Structural and reference integrity | Uses IDs and core nodes; emits structured diagnostics |
| `verify/types.rs` | Type and constraint validation | Uses type compatibility and constraint contracts |
| `verify/dominance.rs` | Dominance and SSA-use validation where applicable | Uses CFG analysis and value definitions |
| `verify/control_flow.rs` | Terminators, edges, and control-flow legality | Uses CFG and region contracts |
| `verify/effects.rs` | Effect declarations and ordering validity | Uses operation schemas and effect contracts |
| `verify/ownership.rs` | Ownership and linear-resource validation | Uses ownership rules and domain interfaces |
| `verify/resources.rs` | Resource/capability constraint validity | Does not perform physical allocation |
| `verify/dialects.rs` | Dialect/schema conformance | Uses the frozen registry and operation definitions |
| `verify/full.rs` | Orchestration of the required verification stages | Composes existing verifiers; contains no duplicated domain algorithms |
| `analysis/cfg.rs` | Control-flow graph view | Read-only projection from canonical IR |
| `analysis/dominance.rs` | Dominance analysis | Depends on CFG; invalidated after relevant control-flow changes |
| `analysis/def_use.rs` | Def-use analysis | Depends on canonical definitions and uses |
| `analysis/effects.rs` | Derived effect analysis | Depends on effect declarations and call summaries |
| `analysis/liveness.rs` | Liveness and related data-flow analysis | Depends on CFG and value-use information |
| `analysis/resources.rs` | Resource estimates and constraints analysis | Reports estimates distinctly from proven bounds |
| `analysis/provenance.rs` | Provenance queries | Uses source and transformation metadata |
| `pass/contracts.rs` | Pass preconditions, postconditions, and preservation rules | Governs all registered passes |
| `pass/registry.rs` | Pass identity and registration | Uses stable pass descriptors |
| `pass/manager.rs` | Ordered pass execution and failure propagation | Uses pass contracts and explicit configuration |
| `pass/invalidation.rs` | Analysis invalidation rules | Uses declared mutation/dependency information |
| `pass/determinism.rs` | Determinism declarations and enforcement support | Uses pass identity, version, and configuration |
| `transform/*` | Individual transformations | Each pass owns its algorithm and tests; all obey pass contracts |
| `lowering/context.rs` | Source/destination, capabilities, constraints, and policies | Receives explicit target descriptions; no hidden discovery |
| `lowering/legality.rs` | Legalization and support checks | Uses dialect schemas and destination capabilities |
| `lowering/provenance.rs` | Provenance mapping across lowering | Uses source-map contracts |
| `lowering/diagnostics.rs` | Lowering-specific diagnostic mapping | Uses common errors and diagnostic codes |
| `lowering/api.rs` | Stable lowering entry points and result types | Composes the context, legality, adapters, and verification |
| `serialization/schema.rs` | Serialized schema contract | Uses version and extension contracts |
| `serialization/encode.rs` | Encoding | Does not decide semantic validity |
| `serialization/decode.rs` | Decoding untrusted input | Produces a validated parsing result, not implicitly trusted executable IR |
| `serialization/validation.rs` | Schema and decoded-reference validation | Reuses verifier contracts where applicable |
| `serialization/canonical.rs` | Canonical encoding rules | Uses metadata classification and deterministic ordering |
| `interchange/*` | External format adapters and migrations | Declares supported mappings and all lossy or unsupported cases |
| `fingerprint/stable_hash.rs` | Versioned canonical fingerprinting | Uses canonical semantic inputs, not host-local identity |
| `incremental/dependencies.rs` | Dependency tracking | Records every semantic input that affects derived results |
| `incremental/invalidation.rs` | Change invalidation | Invalidates conservatively where dependencies are uncertain |
| `incremental/cache_keys.rs` | Stable, versioned cache-key construction | Uses declared semantic inputs and canonical fingerprints |
| `dialects/core/` | Generic core operation definitions | Uses the dialect API; does not contain every future operation |
| `dialects/classical/` | Standard classical operations | Integrates with classical IR through explicit adapters |
| `dialects/quantum/` | ZUIR-to-quantum semantic mappings | Uses `src/quantum/ir/` as the canonical quantum owner |
| `dialects/hdl/` | Hardware structure and timing mappings | Integrates with the actual HDL implementation |
| `dialects/tensor/`, `ai/` | Numerical, tensor, and model operation definitions | Uses symbolic shapes and numerical contracts |
| `dialects/distributed/`, `networking/` | Communication and distributed semantics | Uses effect, resource, and failure contracts |
| `dialects/crypto/` | Cryptographic operation definitions | Preserves security requirements and approved implementation boundaries |
| `tests/*` | Test suites for the corresponding contracts | Must use public APIs where appropriate and include integration tests |

### 22.1 File contract header

Every implemented Rust module SHOULD begin with a concise ownership contract containing:

- `PURPOSE`
- `STATUS`
- `AUTHORITY`
- `OWNS`
- `DOES_NOT_OWN`
- `INPUTS`
- `OUTPUTS`
- `DEPENDS_ON`
- `PUBLIC_API`
- `CONSUMED_BY`
- `DIAGNOSTIC_OWNER`
- `TEST_OWNER`
- `COMPATIBILITY`
- `SCALABILITY`
- `POCO-REAF`
- `SAFETY`
- `FREEZE_CRITERIA`

Exact imports and public exports MUST be documented in the module's API or implementation contract.

`AST_OWNER`, `SEMANTIC_OWNER`, `IR_OWNER`, `RUNTIME_OWNER`, and `TOOLING_OWNER` MUST be stated wherever they are relevant to prevent responsibility overlap.

### 22.2 Freeze meaning

A file is frozen only when its contract, imports, exports, error behavior, invariants, tests, compatibility requirements, and downstream integration points have been agreed and validated.

A freeze does not prohibit a justified future correction. It prevents avoidable redesign caused by leaving dependencies or semantics undecided.

A genuine contract change MUST be recorded, reviewed, versioned where required, and tested against all declared consumers.

---

## 23. Dependency direction and integration order

The implementation MUST follow dependency direction rather than creating the largest or most visible module first.

```text
Phase 0: Repository audit and normative contract
                    |
                    v
Phase 1: IDs, versions, errors, configuration
                    |
                    v
Phase 2: Diagnostics, provenance, schema primitives
                    |
                    v
Phase 3: Type identity, constraints, values, symbols
                    |
                    v
Phase 4: Core IR structures and invariant definitions
                    |
                    v
Phase 5: Dialect schemas, interfaces, and registry
                    |
                    v
Phase 6: Builders and structural/type/control verification
                    |
                    v
Phase 7: Analysis, pass contracts, and invalidation
                    |
                    v
Phase 8: Serialization, migration, canonicalization, hashing
                    |
                    v
Phase 9: Standard dialects and conformance tests
                    |
                    v
Phase 10: Domain adapters and target lowering
                    |
                    v
Phase 11: End-to-end integration and production hardening
```

### Phase 0 — Audit and architecture freeze

Read the actual repository and record every existing owner, dependency, public API, test, serialized format, and consumer.

Resolve the SSA/value model, identity scopes, operation extension contract, error model, versioning, serialization, and migration policy.

**Gate:** no unresolved ownership conflicts or undocumented semantic assumptions.

### Phase 1 — Independent foundations

Complete and test `ids.rs`, `version.rs`, `errors.rs`, and `config.rs`.

**Gate:** stable types and policies that do not depend on IR node implementations.

### Phase 2 — Schema and diagnostics

Complete diagnostics, source provenance, schema primitives, attributes, metadata, and extension manifests.

**Gate:** schemas can describe extensible operations without depending on builders or concrete program nodes.

### Phase 3 — Types and values

Complete type identity, symbolic constraints, type definitions, literals, constants, expressions, and symbols.

**Gate:** all supported domain types can be described without target-specific representation assumptions.

### Phase 4 — Core IR

Complete the program/module/function/region/block/operation/value/control-flow/effect/ownership/resource contracts and implementations.

**Gate:** a valid program can be represented without relying on builder behavior to hide invalid states.

### Phase 5 — Dialect system

Complete dialect definitions, schemas, interfaces, registry, and compatibility contracts.

**Gate:** a test-only third-party dialect can register a new operation without modifying core IR types.

### Phase 6 — Builders and verification

Implement safe construction and the applicable verification categories.

**Gate:** malformed IR is rejected deterministically with actionable diagnostics.

### Phase 7 — Analysis and transformations

Implement read-only analysis, pass contracts, invalidation, and individually verified transformations.

**Gate:** every pass declares its preservation properties and produces the documented result on success.

### Phase 8 — Persistence and identity

Implement versioned serialization, canonical encoding, migrations, fingerprints, and incremental cache contracts.

**Gate:** supported round trips and canonicalization preserve the declared meaning.

### Phase 9 — Standard dialects

Implement core and classical dialects, then integrate quantum, HDL, tensor, AI, distributed, networking, and cryptographic domains in dependency order.

**Gate:** every implemented dialect passes reusable conformance tests.

### Phase 10 — Real repository integration

Connect the existing semantic model, `src/ir_gen.rs`, `src/ir_verify.rs`, `src/quantum/ir/`, the real classical and HDL implementations, and backend interfaces.

**Gate:** representative source programs pass through the real compiler pipeline with no unverified bypasses or silent semantic loss.

### Phase 11 — Production hardening

Complete property testing, fuzzing, stress tests, deterministic builds, compatibility testing, incremental invalidation, dependency review, and CI enforcement.

**Gate:** the acceptance criteria in this document are demonstrably satisfied for the supported implementation scope.

---

## 24. Mandatory integration contracts

### 24.1 Frontend-to-ZUIR adapter

The adapter MUST consume the validated semantic model and emit ZUIR with explicit type, effect, capability, ownership, resource, and provenance information wherever those facts are available and required.

It MUST NOT redo name resolution independently or infer missing semantic facts through unsafe guesses.

If a source construct has no supported ZUIR mapping, the adapter MUST report the limitation.

### 24.2 Existing classical IR

The current instruction-oriented IR MUST be assessed against ZUIR's requirements.

An adapter MUST define how existing registers, constants, instructions, calls, memory operations, control-flow labels, phi-like operations, and domain-specific instructions map to ZUIR.

Unsupported constructs MUST be reported. Any migration MUST preserve compatibility promises and existing tests or provide an explicit versioned transition.

### 24.3 Quantum IR

The ZUIR quantum adapter MUST map compatible quantum operations and values to the canonical `quantum::ir` model.

It MUST preserve quantum identity rules, measurement semantics, classical feedback, resource requirements, and provenance.

It MUST NOT create a duplicate qubit identity system or treat every quantum program as a flat vector of gates.

### 24.4 HDL and hardware IR

The hardware adapter MUST declare which ZUIR regions, operations, types, and effects map to hardware structures and timing models.

Where the mapping cannot preserve the source semantics, it MUST reject the conversion or report a formally permitted approximation.

### 24.5 Backend integration

A backend adapter MUST declare its supported dialects, operations, types, effects, target capabilities, and unsupported constructs.

Target support MUST be based on explicit capability information rather than assumptions embedded in ZUIR.

### 24.6 Compiler orchestration

The compiler MUST track stage results and verification status explicitly.

A successful parse MUST NOT imply successful semantic analysis. Successful semantic analysis MUST NOT imply verified ZUIR. Verified ZUIR MUST NOT imply successful target lowering or successful runtime execution.

Each stage MUST return its own structured outcome.

---

## 25. Testing and conformance requirements

### 25.1 Unit tests

Each module MUST test:

- Valid ordinary inputs.
- Boundary conditions.
- Invalid inputs.
- Error and diagnostic behavior.
- Overflow and checked arithmetic.
- Missing or unknown extensions.
- Resource-budget exhaustion.
- Relevant deterministic behavior.

### 25.2 Integration tests

Integration tests MUST cover:

- Semantic model to ZUIR.
- ZUIR to the classical representation.
- ZUIR to `quantum::ir`.
- ZUIR to HDL/hardware representations where supported.
- Dialect registration and custom operation verification.
- Serialization and decoding.
- Version compatibility and migration.
- Lowering failures and unsupported target capabilities.
- End-to-end compiler behavior.

### 25.3 Property-based tests

Property tests SHOULD generate valid and invalid programs and verify that:

- Builders preserve documented construction invariants.
- Verification rejects malformed structures.
- Serialization round trips preserve the specified fields and semantics.
- Canonical encoding is stable.
- Transformations preserve their declared properties.
- Incremental invalidation does not reuse stale results.

### 25.4 Fuzz testing

Fuzz targets MUST cover untrusted serialized input, schema decoding, structural verification, dialect registration boundaries, and applicable transformations.

Fuzzing MUST NOT assume that inputs are well-formed.

Panics, uncontrolled resource growth, unsafe behavior, and inconsistent diagnostics on equivalent malformed input MUST be investigated.

### 25.5 Stress and scalability tests

Tests MUST exercise generated programs of increasing size and complexity, constrained only by the test environment and explicit budgets.

They MUST cover large graphs, many modules and operations, deep and recursive structures, symbolic dimensions, extensible operation schemas, and resource exhaustion.

Tests MUST NOT encode an arbitrary universal hardware ceiling as the expected maximum supported size.

### 25.6 Required reference corpus

The repository SHOULD maintain a conformance corpus covering:

- `minimal.zm`
- `classical.zm`
- `quantum.zm`
- `hybrid.zm`
- `hdl.zm`
- `poco-reaf.zm`

These files MUST use syntax accepted by the canonical language specification and grammar. Each example MUST state its expected semantic and lowering status.

### 25.7 Continuous integration

The production pipeline MUST include applicable checks equivalent to:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

Feature combinations and dependency configurations MUST be tested as required by the actual project.

CI MUST enforce the ZUIR safe-Rust policy and test compatibility, determinism, and relevant conformance suites.

A successful command is evidence only for the code and configuration that command actually tested.

---

## 26. Security and trust boundaries

Serialized IR, external-format imports, dialect manifests, and third-party extensions MUST be treated as potentially untrusted inputs.

The implementation MUST:

- Validate references and schemas before trusted execution or lowering.
- Apply explicit resource budgets to expensive operations.
- Avoid executing metadata as code.
- Avoid hidden filesystem, network, or environment access during core verification.
- Keep credentials and secrets outside portable IR unless a separately reviewed secure mechanism explicitly requires otherwise.
- Prevent diagnostic and metadata leakage of sensitive information.
- Report unsupported trust requirements instead of silently weakening them.

A verified IR is structurally and semantically valid under the checks performed; verification alone does not prove that arbitrary extension code is trustworthy.

---

## 27. Compatibility and migration

### 27.1 Public API compatibility

Public Rust APIs MUST have a documented stability policy.

Breaking changes MUST be identified, justified, and migrated deliberately. Compatibility aliases MUST delegate to canonical implementations.

### 27.2 Serialized compatibility

Persisted artifacts MUST declare their schema version.

Readers MUST either support the version, apply an explicit valid migration, preserve permitted opaque data, or reject the artifact with a structured error.

They MUST NOT silently reinterpret an unsupported version.

### 27.3 Migration from existing IR

The repository audit MUST classify existing representations as:

- Canonical and retained.
- Canonical but requiring extension.
- Compatibility representation.
- Adapter input/output.
- Deprecated representation.
- Candidate for retirement after migration.

No existing representation MUST be deleted merely because a new universal representation has been proposed.

A migration MUST include semantic mapping, compatibility behavior, tests, and a rollback or recovery strategy appropriate to the repository's release process.

---

## 28. Production acceptance criteria

ZUIR MUST NOT be labelled production-ready until the applicable criteria below have passing evidence.

| Area | Required evidence |
|---|---|
| Specification | Ownership, identity, SSA/value semantics, effects, types, extensions, versioning, and errors are unambiguous |
| Rust safety | Rust 1.97 baseline passes; ZUIR forbids unsafe code |
| Core representation | Valid programs and all required entities can be represented |
| Extensibility | A test-only external dialect adds operations without editing core operation definitions |
| Verification | Invalid references, types, control flow, effects, and schemas are rejected |
| Resource model | No artificial universal machine limits; explicit budgets and target capabilities are distinct |
| Semantic preservation | Lowering and transformations preserve their documented obligations |
| Quantum integration | The canonical `quantum::ir` boundary is respected |
| Classical integration | Existing classical constructs have verified mappings or explicit unsupported status |
| HDL integration | Supported hardware/timing constructs have validated mappings |
| Serialization | Versioned decode, validation, round trips, and migration tests pass |
| Determinism | Canonical output and declared deterministic passes are reproducible |
| Incremental compilation | Dependency changes invalidate all affected cached results |
| Security | Untrusted inputs and extensions are validated under documented boundaries |
| Testing | Unit, integration, property, fuzz, stress, and conformance suites pass as applicable |
| Documentation | Public APIs, dialect contracts, limitations, and migration guidance are current |
| Repository integration | Real frontend-to-ZUIR-to-domain/backend paths work without silent semantic loss |

A criterion MUST be marked `NOT APPLICABLE` only with a documented architectural reason and an identified reviewer. It MUST NOT be marked complete merely because its planned file exists.

---

## 29. Required audit deliverable before implementation freeze

Before freezing the first implementation file, produce a repository-specific inventory containing:

1. Every existing file under `src/zuir/`.
2. Existing ZUIR types, functions, public APIs, tests, and dependencies.
3. Existing IR specifications and conflicting authority statements.
4. The grammar-to-AST-to-semantic-model mapping for relevant language constructs.
5. Existing classical, quantum, HDL, optimization, compiler, and runtime consumers.
6. Every existing serialized format and compatibility commitment.
7. A file-by-file mapping from current owners to the target responsibilities in §22.
8. A dependency graph showing which contracts must freeze first.
9. A list of missing implementations, duplicate responsibilities, and unsupported constructs.
10. Exact commands and results used to validate the baseline.

The audit MUST distinguish observed repository facts from planned architecture.

It MUST NOT claim that files, APIs, or tests exist without inspecting them.

---

## 30. Final governing rule

ZUIR is the stable, universal representation of computational meaning within Zamani.

Its core MUST remain target-independent, safe, extensible, versioned, verifiable, and explicit about uncertainty and unsupported realization.

Domain-specific computation MUST integrate through declared schemas, semantic contracts, verification interfaces, and lowering adapters. Physical capabilities and constraints MUST be supplied by explicit downstream descriptions rather than embedded as universal assumptions.

POCO-REAF is achieved architecturally by preserving portable program meaning and separating it from target-specific realization—not by pretending every machine has identical capabilities or unlimited resources.

**The production objective is one canonical representation, explicit semantic preservation, extensible domain integration, and no artificial universal hardware ceiling.**

Worked for 4m 27s

### Recommended implementation order after adding the specification

Do not create every proposed file in one pass. Freeze the architecture and then implement in this order:

* `src/zuir/ids.rs` — typed identities and uniqueness contracts.

* `src/zuir/version.rs` — format and compatibility versions.

* `src/zuir/errors.rs` — structured error contracts.

* `src/zuir/config.rs` — explicit policies and resource budgets.

* `src/zuir/diagnostics.rs` and `src/zuir/source_map.rs`.

* `src/zuir/schema/` — schema primitives, attributes, metadata, and extensions.

* `src/zuir/types/` and `src/zuir/values/`.

* `src/zuir/ir/` — canonical data model.

* `src/zuir/dialect/` and `src/zuir/builder/`.

* `src/zuir/verify/` and `src/zuir/analysis/`.

* `src/zuir/serialization/`, `src/zuir/fingerprint/`, and `src/zuir/incremental/`.

* Standard dialects, transformations, and lowering adapters.

* Integration with the actual frontend, classical IR, `quantum::ir`, HDL, and backends.

* Production acceptance tests and CI enforcement.

Important: This is the proposed dependency order, not confirmation that the implementation has passed any of the acceptance gates.

The next engineering action is to complete the repository-specific audit described in §29. That audit will establish the exact files to retain, extend, create, or migrate, preventing the new specification from introducing duplicate ownership or breaking existing consumers.

The specification is ready to serve as the architectural baseline for that audit. The implementation should be considered production-ready only after its contracts are implemented, integrated, and verified against the stated acceptance criteria.




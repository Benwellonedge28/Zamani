Zamani Universal Meta-Compiler

Compilation Pipeline Contract

Canonical path: "src/compiler/documentation/PIPELINE_CONTRACT.md"

Repository: "github.com/Benwellonedge28/Zamani"

Architecture: Universal Meta-Compiler (UMC)

Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

Rust edition: 2021

Minimum Rust version: Rust 1.97 or later

Safety: Safe Rust only. "unsafe" is prohibited.

Status: Normative compiler architecture and integration contract.

---

1. Purpose

This document defines the production compilation pipeline for Zamani.

It establishes the responsibilities, ordering, dependencies, interfaces, integration requirements, correctness guarantees, scalability requirements and completion criteria of every stage of compilation.

The pipeline must support the complete Zamani programming language, including:

- Classical computing.
- Systems programming.
- Parallel and concurrent computing.
- Distributed computing.
- Scientific and numerical computing.
- Quantum computing.
- Quantum-classical hybrid computing.
- Quantum simulation.
- Quantum error correction integration.
- AI and machine learning.
- Deep learning and neural networks.
- Tensor computation.
- Symbolic and probabilistic computation.
- Neuro-symbolic computation.
- Hardware description languages.
- Hardware/software co-design.
- FPGA and ASIC realization.
- GPU and accelerator computing.
- Neuromorphic computing.
- Photonic computing.
- Networking.
- Cryptographic computation.
- Data-intensive computation.
- Embedded and edge computing.
- Future computational domains.

The compiler must support these domains without creating separate, incompatible programming languages or competing semantic and intermediate-representation systems.

The fundamental requirement is:

One Zamani program must retain one defined semantic meaning while being compiled and realized across different compatible computing environments.

The compiler must not require developers to rewrite their source programs merely because the available hardware changes.

However, portability does not mean that every program is physically executable on every machine.

When the requested computation cannot be realized, the compiler must report the incompatibility explicitly.

It must never silently change program meaning to obtain a successful compilation.

---

2. Repository architecture and existing implementation

2.1 Existing compiler infrastructure

The inspected repository already contains the following compiler architecture.

Existing path| Existing responsibility
"src/compiler/pipeline/stage.rs"| Compiler stage contracts
"src/compiler/pipeline/stages.rs"| Standard stage definitions
"src/compiler/pipeline/ordering.rs"| Dependency validation and deterministic ordering
"src/compiler/pipeline/execution.rs"| Stage execution
"src/compiler/pipeline/failure.rs"| Structured pipeline failures
"src/compiler/pipeline/recovery.rs"| Recovery policy and decision contracts
"src/compiler/pipeline/orchestration.rs"| Pipeline lifecycle orchestration
"src/compiler/pipeline/mod.rs"| Pipeline composition and public API
"src/compiler/context/compilation_context.rs"| Shared compilation context
"src/compiler/context/compilation_state.rs"| Compilation lifecycle
"src/compiler/context/dependency_context.rs"| Compilation dependencies
"src/compiler/context/provenance_context.rs"| Compilation provenance
"src/compiler/configuration/budget.rs"| Operational resource budgets
"src/compiler/configuration/config.rs"| Compiler configuration
"src/compiler/configuration/policy.rs"| Compilation policies
"src/compiler/configuration/profile.rs"| Compilation profiles
"src/compiler/configuration/target.rs"| Target descriptors
"src/compiler/configuration/validation.rs"| Configuration validation
"src/compiler/analysis/"| Compiler analysis integration
"src/compiler/ir/"| Canonical IR integration
"src/compiler/passes/"| Compiler-facing pass integration
"src/compiler/api/"| Public compiler API

These are existing components, not proposed replacements.

The pipeline architecture must use them.

2.2 Repository audit qualification

The repository snapshot used for this contract is:

"5cafd4755e71b1638ec495daf823134c74d5b117"

The inspection covered the relevant existing files and grammar authorities, but did not establish a successful complete repository build or a recursive audit of every source file.

The following matters must therefore be verified before declaring the pipeline production-ready:

1. The actual contents and declarations of "src/compiler/mod.rs".
2. The complete registration and integration of compiler stages.
3. Whether all standard stages have real implementations.
4. Whether every required stage is invoked.
5. Whether the legacy compiler entry points delegate to the intended production pipeline.
6. Whether all domain IR adapters perform real transformations.
7. Whether failure and cancellation propagate through all stages.
8. Whether the grammar composition is mechanically valid.
9. Whether the compiler builds and passes its repository-wide tests.

Documentation, module declarations and API tests are not proof that the corresponding implementation is complete.

---

3. Fundamental architectural ownership

The compiler coordinates repository subsystems. It does not recreate them.

The authoritative ownership boundaries are:

Responsibility| Canonical repository owner
Lexical rules| "grammar/lexer/"
Lexer implementation| "src/lexer.rs"
Parser grammar| "grammar/"
Parser implementation| "src/frontend/parser/"
AST| "src/frontend/ast/"
Semantic meaning| "src/frontend/semantic/" and its normative specification
Legacy semantic compatibility| "src/semantic.rs"
Universal intermediate representation| "src/zuir/"
Classical IR| "src/classical/"
Quantum IR| "src/quantum/ir/"
Hardware IR| "src/hdl/hdlir/"
AI computation| "src/ai/"
Hybrid computing| "src/hybrid/"
Distributed computing| "src/distributed/"
Optimization infrastructure| "src/optimizer/" and "src/zuir/pass/"
Backend implementation| "src/backend/"
Runtime execution| "src/runtime/"
Compiler orchestration| "src/compiler/"
Toolchain and packaging| "src/toolchain/"

The compiler must consume the contracts and implementations belonging to these subsystems.

It must not introduce:

- A second semantic model.
- A second AST.
- A second ZUIR.
- A second quantum IR.
- A second hardware IR.
- A second optimizer.
- A second runtime.
- A competing hardware allocation system.

In particular, "ZUIR" is an intermediate representation, not the semantic authority.

The semantic subsystem establishes program meaning. ZUIR represents validated computation for subsequent compiler processing.

---

4. Canonical compilation architecture

The production pipeline follows this architecture.

                    ZAMANI SOURCE
                          |
                          v
                    SOURCE INPUT
                          |
                          v
                    CANONICAL LEXER
                          |
                          v
                    CANONICAL PARSER
                          |
                          v
                    DOMAIN-NEUTRAL AST
                          |
                          v
                 STRUCTURAL VALIDATION
                          |
                          v
                 NAME / MODULE RESOLUTION
                          |
                          v
                 SEMANTIC VALIDATION
                          |
             +------------+-------------+
             |            |             |
             v            v             v
           TYPES        EFFECTS       OWNERSHIP
             |            |             |
             +------------+-------------+
                          |
                          v
              RESOURCE / CAPABILITY ANALYSIS
                          |
                          v
                 VALIDATED PROGRAM MODEL
                          |
                          v
                    CANONICAL ZUIR
                          |
                          v
                 IR VERIFICATION
                          |
                          v
                 COMPILER ANALYSIS
                          |
                          v
                 OPTIMIZATION PLANNING
                          |
                          v
                 SEMANTICS-PRESERVING
                     OPTIMIZATION
                          |
                          v
                 HYBRID WORKLOAD ANALYSIS
                          |
                          v
                CAPABILITY AND FEASIBILITY
                       ANALYSIS
                          |
                          v
                 HETEROGENEOUS PLANNING
                          |
                          v
                 SPECIALIZATION / LOWERING
                          |
                          v
                     LEGALIZATION
                          |
                          v
                TARGET-SPECIFIC BACKENDS
                          |
                          v
                   LINKING / LTO
                          |
                          v
                 ARTIFACT VERIFICATION
                          |
                          v
                  PORTABLE / REALIZED
                       ARTIFACTS
                          |
                          v
                 PACKAGING / DEPLOYMENT
                          |
                          v
                   RUNTIME / HAL
                          |
                          v
                  PHYSICAL EXECUTION

Not every program traverses every domain-specific stage.

The pipeline must be conditional and extensible.

For example:

- A classical-only program must not require quantum lowering.
- A quantum program must use the canonical quantum IR.
- An AI program must use the canonical AI and tensor integration.
- An HDL program must use the canonical HDLIR.
- A hybrid program must preserve dependencies between all participating domains.
- A distributed program must preserve communication and execution contracts.

The absence of a domain must not be treated as an error.

The presence of a domain must not allow its required compilation stages to be silently skipped.

---

5. Compilation invariants

Every pipeline implementation must preserve the following invariants.

5.1 Semantic preservation

The source program has one defined semantic meaning.

Compiler transformations must preserve the applicable observable behavior.

A transformation that cannot establish the required preservation guarantees must be rejected or explicitly classified as an authorized approximation under the program's declared semantic contract.

5.2 Representation correctness

Every intermediate representation must satisfy its own canonical specification.

The compiler must not treat an unverified representation as verified.

5.3 Dependency correctness

A stage may execute only after all required dependencies have completed successfully.

A skipped stage does not count as completed.

A failed stage does not count as completed.

A blocked stage does not count as completed.

5.4 Requirement preservation

The compiler must preserve:

- Mandatory requirements.
- Constraints.
- Effect contracts.
- Ownership requirements.
- Security requirements.
- Resource requirements.
- Domain-specific correctness conditions.
- Provenance obligations.

5.5 Explicit infeasibility

If no supported realization satisfies the program's requirements, compilation must return a structured failure explaining the incompatibility.

The compiler must not silently:

- Remove quantum operations.
- Reduce required precision.
- Change numerical semantics.
- Ignore synchronization.
- Remove distributed dependencies.
- Replace mandatory hardware behavior.
- Violate resource constraints.

5.6 No artificial machine ceilings

The pipeline must not impose universal limits on:

- CPUs.
- Cores.
- Threads.
- GPUs.
- FPGAs.
- ASICs.
- QPUs.
- Accelerators.
- Distributed nodes.
- Memory.
- Storage.
- Registers.
- Register widths.
- Qubits.
- Tensor rank.
- Tensor dimensions.
- Functions.
- Modules.
- IR operations.
- Pipeline stages.
- Dependencies.
- Targets.
- Artifacts.

An implementation may encounter representational, operating-system or resource limitations.

Those limitations must be reported accurately.

---

6. Pipeline stage contract

The existing "src/compiler/pipeline/stage.rs" remains the canonical stage contract.

It must not be replaced with another stage trait.

6.1 Stage responsibilities

Every stage must declare or provide:

1. A stable stage identifier.
2. A stage descriptor.
3. Its required dependencies.
4. Its input contract.
5. Its output contract.
6. Its concurrency declaration.
7. Its determinism declaration.
8. Its execution behavior.
9. Its failure classification.
10. Its cancellation behavior.
11. Its budget-accounting obligations.
12. Its verification obligations.
13. Its provenance requirements.
14. Its applicable security requirements.

6.2 Stage ownership

A stage owns only the implementation of its declared compilation operation.

It does not become the authority for unrelated semantic or domain information.

For example:

A type-analysis stage consumes canonical AST and semantic information.

It must not create another language type system.

An IR verification stage consumes canonical IR.

It must not create another IR.

A backend stage consumes a verified, compatible representation.

It must not allocate physical hardware.

6.3 Stage inputs and outputs

The existing stage input/output contracts must remain the canonical mechanism for exchanging stage results.

The executor must preserve the original compilation input and the outputs of direct dependencies.

A stage must not depend on accidental execution order.

For example, a backend stage must request the appropriate lowered representation through its declared dependency rather than assume that the immediately preceding stage produced it.

Type-erased payloads, where used by the existing API, must be downcast and validated explicitly.

An unexpected payload type must produce a structured integration error.

It must never be interpreted as a valid alternative representation.

6.4 Stage identifiers

Stage identifiers must remain dynamically extensible.

The standard identifiers in "pipeline/stages.rs" must not become a closed list of all possible future compilation stages.

Domain extensions must be able to introduce new identifiers without requiring a modification to the compiler root.

Identifier validation must reject invalid identifiers without imposing arbitrary hardware-independent length ceilings.

---

7. Canonical pipeline stages

The existing "src/compiler/pipeline/stages.rs" owns standard stage identifiers and dependency definitions.

It must remain the canonical source for those definitions.

The following describes the required logical stage groups.

7.1 Source acquisition

Responsibilities:

- Accept source text or source-file input.
- Preserve source identity.
- Preserve encoding information.
- Integrate with the canonical source map.
- Report source-loading failures.
- Preserve source provenance.

Integration:

- "src/compiler/api/"
- "src/compiler/context/"
- "src/source_map/"
- "src/frontend/"

Source acquisition must not select a hardware target.

7.2 Lexical analysis

Responsibilities:

- Tokenize source using the canonical Zamani lexical rules.
- Preserve source positions.
- Produce lexical diagnostics.
- Handle invalid input explicitly.
- Maintain token compatibility.

Integration:

- "grammar/lexer/"
- "grammar/antlr/ZamaniLexer.g4"
- "grammar/lexer/tokens.g4"
- "src/lexer.rs"

The compiler must not implement a competing lexer.

7.3 Parsing

Responsibilities:

- Construct the canonical domain-neutral AST.
- Preserve source spans.
- Validate syntactic structure.
- Produce parser diagnostics.
- Support the registered language domains.

Integration:

- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniParser.g4"
- "src/frontend/parser/"
- "src/frontend/ast/"

7.4 Structural validation

Responsibilities:

- Validate AST structural invariants.
- Validate declaration relationships.
- Detect malformed or inconsistent structures.
- Preserve source locations.

Integration:

- "src/frontend/ast/"
- "src/frontend/semantic/"
- "src/compiler/diagnostics.rs"

7.5 Name and module resolution

Responsibilities:

- Resolve names and qualified identifiers.
- Resolve imports and module relationships.
- Detect missing or ambiguous declarations.
- Establish dependency information.

Integration:

- "src/frontend/semantic/"
- "src/compiler/analysis/dependency.rs"
- "src/compiler/context/dependency_context.rs"
- "src/toolchain/"

The compiler must not independently redefine symbol semantics.

7.6 Semantic validation

Responsibilities:

- Validate types.
- Validate ownership and borrowing.
- Validate effects.
- Validate capabilities and requirements.
- Validate constraints.
- Validate contracts.
- Validate policies.
- Validate provenance.
- Validate domain-specific semantic rules.

Integration:

- "src/frontend/semantic/"
- "src/semantic.rs" during the documented compatibility migration.
- "grammar/specification/semantic-model.md"
- "grammar/validation/semantic-boundaries.md"

The semantic subsystem owns program meaning.

The compiler must not silently bypass failed semantic validation.

7.7 Resource and capability analysis

Responsibilities:

- Consume semantic resource requirements.
- Analyze capability requirements.
- Preserve constraints and preferences separately.
- Establish abstract feasibility information.
- Identify unsupported operations.

Integration:

- "src/compiler/analysis/capabilities.rs"
- "src/compiler/analysis/requirements.rs"
- "src/compiler/analysis/feasibility.rs"
- "src/frontend/semantic/"
- "src/hybrid/resources/"
- "src/backend/"

This stage does not allocate physical resources.

7.8 Canonical IR construction

Responsibilities:

- Integrate validated semantic information with canonical IR construction.
- Preserve semantic identities.
- Preserve effects and dependencies.
- Preserve domain boundaries.
- Reject unsupported conversions.

Integration:

- "src/compiler/ir/"
- "src/zuir/"
- "src/classical/"
- "src/quantum/ir/"
- "src/hdl/hdlir/"
- "src/ai/"

The compiler must not construct a competing universal IR.

7.9 IR verification

Responsibilities:

- Verify structural invariants.
- Verify types and value relationships.
- Verify control flow.
- Verify effects.
- Verify ownership-related invariants.
- Verify domain-specific constraints.
- Verify conversion correctness.
- Verify transformation results.

Integration:

- "src/zuir/verify/"
- "src/ir_verify.rs"
- "src/compiler/ir/"
- Domain-specific verifiers.

Verification must be performed at every transformation boundary where correctness requires it.

7.10 Compiler analysis

Responsibilities:

- Dependency analysis.
- Program analysis.
- Effect analysis.
- Ownership-related IR analysis.
- Cost estimation.
- Capability analysis.
- Feasibility analysis.
- Provenance analysis.

Integration:

- "src/compiler/analysis/"
- "src/frontend/semantic/"
- "src/zuir/"

Compiler analysis may compute derived facts.

It must not replace semantic authority.

7.11 Optimization planning

Responsibilities:

- Identify legal transformations.
- Determine applicable optimization policies.
- Establish required analyses.
- Establish profitability criteria.
- Determine whether transformations are deterministic.
- Preserve compilation provenance.

Integration:

- "src/compiler/optimization/", where present.
- "src/compiler/passes/"
- "src/optimizer/"
- "src/zuir/pass/"

7.12 Optimization execution

Responsibilities:

- Execute registered, legal transformations.
- Validate transformation preconditions.
- Track invalidated analyses.
- Preserve required effects.
- Verify resulting IR.
- Record optimization decisions.

Integration:

- "src/compiler/passes/"
- "src/zuir/pass/"
- "src/optimizer/"
- "src/compiler/instruction_fusion.rs"
- "src/compiler/optimization_strategies.rs"
- "src/compiler/sro.rs"

The compiler must not introduce another independent pass manager.

7.13 Hybrid analysis

Responsibilities:

- Analyze relationships between computational domains.
- Preserve cross-domain dependencies.
- Identify communication requirements.
- Identify synchronization requirements.
- Analyze data representation compatibility.
- Identify domain boundaries.
- Preserve execution constraints.

Integration:

- "src/hybrid/"
- "src/compiler/analysis/"
- "src/zuir/"
- "src/quantum/ir/"
- "src/classical/"
- "src/ai/"
- "src/hdl/hdlir/"
- "src/distributed/"

Hybrid analysis must support arbitrary supported combinations of domains.

It must not assume that quantum and classical computation are the only possible hybrid combination.

7.14 Heterogeneous planning

Responsibilities:

- Discover available target descriptors through authorized providers.
- Match operations to target capabilities.
- Evaluate target compatibility.
- Evaluate resource feasibility.
- Analyze data movement.
- Analyze communication costs.
- Produce abstract partitioning plans.
- Produce abstract scheduling plans.
- Produce alternative feasible plans.
- Explain infeasible plans.

Integration:

- "src/compiler/planning/"
- "src/compiler/hardware_partitioner.rs" if retained in the current tree.
- "src/backend/"
- "src/hybrid/planning/"
- "src/runtime/"

The compiler must not perform physical device allocation or runtime resource reservation.

7.15 Specialization

Responsibilities:

- Specialize generics and permitted compile-time values.
- Preserve specialization provenance.
- Avoid specializing on unknown or unauthorized runtime values.
- Cache specialization results safely.
- Verify specialized representations.

Integration:

- "src/compiler/monomorphizer.rs"
- "src/frontend/semantic/"
- "src/compiler/incremental/"
- "src/zuir/"

7.16 Lowering and legalization

Responsibilities:

- Convert verified representations to compatible lower-level representations.
- Apply legal decompositions.
- Preserve dependencies.
- Preserve observable semantics.
- Reject unsupported operations.
- Validate target compatibility.

Integration:

- "src/compiler/ir/"
- "src/zuir/lowering/"
- "src/classical/"
- "src/quantum/ir/"
- "src/hdl/hdlir/"
- "src/backend/"

Quantum routing, scheduling, resilience and QEC remain with their authoritative quantum subsystems.

7.17 Backend realization

Responsibilities:

- Select compatible backend providers.
- Validate backend capabilities.
- Invoke actual code-generation implementations.
- Verify generated outputs.
- Record backend versions and provenance.
- Report unsupported operations.

Integration:

- "src/compiler/backends/"
- "src/backend/"
- "src/compiler/llvm_backend.rs", if retained.
- "src/compiler/wasm_backend.rs", if retained.
- Other registered providers.

A generated source file or manifest is not automatically an executable artifact.

7.18 Linking

Responsibilities:

- Resolve symbols.
- Validate dependencies.
- Handle relocations.
- Compose compatible artifacts.
- Perform permitted link-time optimization.
- Preserve ABI and version compatibility.
- Report unresolved references.

Integration:

- "src/compiler/linker.rs"
- "src/backend/"
- "src/compiler/artifacts/"
- "src/toolchain/"

7.19 Artifact verification

Responsibilities:

- Verify artifact integrity.
- Verify artifact format.
- Verify representation versions.
- Verify backend compatibility.
- Verify required metadata.
- Verify provenance.
- Verify security properties.
- Distinguish portable artifacts from target-realized artifacts.

Integration:

- "src/compiler/artifacts/"
- "src/compiler/security/"
- "src/backend/"
- "src/toolchain/"

7.20 Packaging and deployment handoff

Responsibilities:

- Assemble verified artifacts.
- Preserve dependency manifests.
- Preserve target requirements.
- Preserve provenance.
- Produce deployment metadata.
- Hand off to the authoritative deployment system.

Integration:

- "src/toolchain/"
- "src/runtime/"
- "src/backend/"

The compiler does not become the runtime or deployment manager.

---

8. Pipeline execution contract

The existing "src/compiler/pipeline/execution.rs" is the authoritative stage executor.

8.1 Deterministic execution

The current implementation executes stages sequentially in deterministic topological order.

This is a valid baseline.

The implementation must preserve deterministic output collection and dependency correctness.

8.2 Parallel execution

The existence of "StageConcurrency" does not establish parallel execution.

Parallel stage execution requires a separately verified implementation.

It must:

- Execute only dependency-independent stages concurrently.
- Respect stage concurrency declarations.
- Share the same compilation budget.
- Share cancellation state.
- Preserve dependency outputs.
- Preserve deterministic result collection where required.
- Propagate failures correctly.
- Avoid data races through safe Rust.
- Avoid duplicate execution.
- Avoid silently abandoning unfinished required work.

Integration:

"src/compiler/parallel_build.rs"

"src/compiler/parallel/", if present.

"src/compiler/context/"

"src/compiler/configuration/budget.rs"

Parallel execution must not introduce arbitrary worker ceilings.

Actual worker availability and explicitly configured operational budgets determine usable concurrency.

8.3 Conditional stages

A stage may be skipped only when its activation policy permits it.

A skipped stage must be recorded as skipped.

A required stage cannot be skipped successfully.

A dependent stage cannot consume output from a skipped prerequisite.

The standard definitions must distinguish:

- Required stages.
- Conditional stages.
- Policy-controlled stages.
- Domain-specific extension stages.

A stage must not decide that it is optional merely because its implementation is unavailable.

8.4 Empty pipelines

An empty optional pipeline may be structurally valid.

However, a successful compilation must not be inferred from an empty stage registry.

The production compiler must establish its mandatory-stage set independently.

Every required compilation obligation must be accounted for.

---

9. Pipeline orchestration

The existing "src/compiler/pipeline/orchestration.rs" owns one pipeline invocation's lifecycle coordination.

It must remain the canonical orchestrator.

Its responsibilities include:

- Establishing the compilation lifecycle.
- Validating the stage graph.
- Establishing required stages.
- Invoking the executor.
- Preserving partial execution reports.
- Recording stage outcomes.
- Propagating failures.
- Preventing invalid successful completion.
- Respecting cancellation.
- Establishing terminal compilation state.

It must not implement another executor.

It must not implement another stage-ordering algorithm.

It must not own semantic analysis or backend code generation.

9.1 Mandatory success conditions

Compilation may report success only when:

1. All mandatory stages are registered.
2. The stage dependency graph is valid.
3. All mandatory stages completed.
4. All required verification stages succeeded.
5. No required output is missing.
6. The compilation lifecycle reached its successful terminal state.
7. No cancellation or unresolved fatal failure remains.
8. The final result contains the required verified outputs.

A successful orchestration report alone is insufficient to certify a correct executable artifact.

The final artifact must independently pass its required verification.

---

10. Failure and recovery contract

The existing modules remain authoritative:

- "src/compiler/pipeline/failure.rs"
- "src/compiler/pipeline/recovery.rs"
- "src/compiler/pipeline/orchestration.rs"

10.1 Failure categories

The pipeline must distinguish:

- Invalid input.
- Lexical failure.
- Parse failure.
- Structural validation failure.
- Semantic failure.
- Type failure.
- Ownership failure.
- Effect failure.
- Dependency failure.
- IR construction failure.
- IR verification failure.
- Transformation failure.
- Unsupported capability.
- Infeasible realization.
- Resource-budget exhaustion.
- Backend incompatibility.
- Backend failure.
- Linking failure.
- Artifact validation failure.
- Security failure.
- Cancellation.
- Internal compiler invariant failure.

Failures must retain their original classification and relevant stage context.

10.2 Partial reports

When a stage fails, the pipeline must preserve:

- Completed stages.
- Skipped stages.
- Blocked stages.
- Failed stage.
- Original failure information.
- Relevant diagnostics.
- Available provenance.

Partial reports must not be mistaken for successful compilation.

10.3 Retry policy

The existing recovery subsystem evaluates retry eligibility.

A retry must require:

- An eligible failure.
- An explicit retry policy.
- Replay-safety evidence.
- An available retry budget.
- A non-cancelled compilation.
- No violated correctness invariant.

A failed stage must not automatically be retried simply because its error is marked retryable.

10.4 Fallback policy

A fallback must preserve:

- Program semantics.
- Mandatory requirements.
- Constraints.
- Applicable effects.
- Security requirements.
- Target compatibility.

The existing "FallbackEvidence" contract must be strengthened so that verification evidence cannot be fabricated by an untrusted caller merely by constructing a value containing successful boolean flags.

Production integration should use trusted, typed verification receipts or another controlled evidence boundary.

An unverified fallback must never be reported as successful.

10.5 Recovery execution

"RecoveryPlanner" produces decisions.

It does not, by itself, implement safe retry or fallback execution.

Actual recovery execution must be integrated into orchestration with:

- Correct lifecycle transitions.
- Replay safety.
- Cancellation checks.
- Retry accounting.
- Provenance.
- Partial-result invalidation.
- Post-recovery verification.

Until that integration exists, the compiler must not advertise automatic recovery as an implemented capability.

---

11. Resource budgets and scalability

The authoritative budget implementation is:

"src/compiler/configuration/budget.rs"

The compilation context shares budget accounting between stages.

11.1 Budget model

A compilation budget may account for:

- Source processing.
- IR processing.
- Memory accounting.
- Worker usage.
- Optimization operations.
- Constant evaluation.
- Diagnostics.
- Artifact generation.
- Stage-specific work.
- Time consumption.
- Extension-defined dimensions.

Budgets are operational policies.

They are not language-level machine-capacity ceilings.

11.2 Unlimited configuration

An absent budget means no explicit compiler budget has been configured for that dimension.

It does not guarantee infinite resources.

The compiler remains subject to:

- Physical memory.
- Available processors.
- Operating-system restrictions.
- Address-space limitations.
- Storage capacity.
- Target capabilities.
- Backend limitations.
- Security policies.

11.3 Mandatory budget behavior

Every stage that consumes a budgeted resource must:

1. Identify the relevant dimension.
2. Measure its consumption.
3. Account for cumulative usage.
4. Handle accounting errors.
5. Stop or recover when exhaustion occurs.
6. Report explicit failure.
7. Preserve partial results where valid.
8. Avoid silent truncation.

The existing budget tracker explicitly accounts for consumption reported by callers.

It does not automatically monitor every allocation or operation.

Stages must not claim automatic enforcement unless they actually implement it.

11.4 Scalability architecture

The compiler must use:

- Dynamic collections.
- Checked arithmetic.
- Configurable operational budgets.
- Extensible registries.
- Streaming where appropriate.
- Incremental compilation.
- Dependency-aware processing.
- Memory-conscious intermediate representations.
- Efficient graph algorithms.
- Safe parallel execution.
- Explicit resource exhaustion reporting.

No arbitrary constant may become a substitute for actual capability or budget evaluation.

---

12. Compilation context and lifecycle integration

The existing context subsystem remains authoritative.

Files:

- "src/compiler/context/mod.rs"
- "src/compiler/context/compilation_context.rs"
- "src/compiler/context/compilation_state.rs"
- "src/compiler/context/dependency_context.rs"
- "src/compiler/context/provenance_context.rs"

12.1 CompilationContext

Every pipeline invocation must receive the appropriate shared compilation context.

It provides:

- Immutable compiler configuration.
- Source identity.
- Shared budget accounting.
- Cancellation.
- Configuration validation.
- Context derivation.

Stages must not independently create fresh budgets for the same compilation.

12.2 CompilationStateTracker

The lifecycle tracker owns:

- Created state.
- Running state.
- Cancellation-requested state.
- Completed state.
- Failed state.
- Cancelled state.
- Valid transitions.

Terminal states must not be reused.

A cancelled or failed compilation cannot be published as successful.

12.3 DependencyContext

The dependency context must provide a consistent snapshot of compilation-unit relationships.

It must be integrated with:

- Name resolution.
- Module resolution.
- Incremental compilation.
- Parallel compilation.
- Linking.

12.4 ProvenanceContext

Provenance must record relevant compilation history.

It must support:

- Source identity.
- Transformation lineage.
- Verification evidence.
- Backend identity.
- Artifact relationships.
- Compilation configuration identity.

Provenance must not be used to redefine program semantics.

---

13. IR integration contract

The existing "src/compiler/ir/" subsystem is the compiler-facing IR integration authority.

It must remain an adapter and orchestration layer.

It must not introduce another IR.

13.1 ZUIR

"src/zuir/" is the universal intermediate-representation authority.

The compiler must consume its canonical structures and verification interfaces.

13.2 Classical IR

"src/classical/" owns the classical representation.

The compiler must integrate it with ZUIR and the relevant domain lowering paths.

13.3 Quantum IR

"src/quantum/ir/" remains the sole canonical quantum semantic representation.

The pipeline must preserve:

- Quantum operations.
- Quantum control flow.
- Quantum data dependencies.
- Measurement semantics.
- Quantum-classical interactions.
- Applicable resource requirements.
- Domain-specific verification contracts.

The pipeline must not introduce a competing quantum IR.

13.4 HDLIR

"src/hdl/hdlir/" owns the canonical hardware-description representation.

Compiler integration must preserve the distinction between:

- Hardware behavior described by the program.
- Abstract hardware requirements.
- Physical implementation choices.
- Vendor-specific realization.

13.5 AI and tensor representations

The compiler must integrate "src/ai/" and the authoritative tensor implementation.

It must preserve:

- Tensor semantics.
- Dynamic shapes.
- Symbolic dimensions.
- Operation dependencies.
- Precision requirements.
- Differentiation semantics.
- AI execution requirements.

13.6 IR conversion

Every representation conversion must declare:

- Input representation.
- Output representation.
- Source and destination versions.
- Preconditions.
- Postconditions.
- Semantic-preservation obligations.
- Information loss, if any.
- Required verification.
- Failure behavior.
- Provenance.

A conversion that loses required semantic information must fail.

---

14. Optimization integration

The compiler must use the existing optimizer and ZUIR pass infrastructure.

Canonical owners:

- "src/optimizer/"
- "src/zuir/pass/"
- "src/compiler/passes/"

The compiler-facing pass modules are adapters and orchestration components.

They must not become a second pass manager.

14.1 Pass contract

Every production pass must declare:

1. Stable identity.
2. Version.
3. Accepted IR representations.
4. Preconditions.
5. Required analyses.
6. Transformation behavior.
7. Preserved analyses.
8. Invalidated analyses.
9. Semantic-preservation contract.
10. Resource requirements.
11. Determinism behavior.
12. Failure behavior.
13. Verification requirements.

14.2 Pass ordering

Pass ordering must be validated.

A pass cannot run before its mandatory dependencies.

The pass manager must not assume that arbitrary registration order is correct.

14.3 Instruction fusion

"src/compiler/instruction_fusion.rs" must integrate with the canonical pass infrastructure.

Fusion must preserve:

- Data dependencies.
- Control dependencies.
- Effects.
- Memory behavior.
- Synchronization.
- Domain-specific semantics.

It must not perform transformations merely because two instructions appear adjacent.

14.4 Telemetry-informed optimization

"src/compiler/sro.rs" must consume trustworthy profiling information.

Telemetry may guide profitability decisions.

It must not override semantic legality or correctness.

---

15. Heterogeneous and hybrid planning

The compiler must support computation distributed across multiple compatible domains and hardware types.

The relevant authorities include:

- "src/compiler/analysis/"
- "src/compiler/planning/"
- "src/hybrid/"
- "src/backend/"
- "src/runtime/"

15.1 Requirements and capabilities

These concepts must remain distinct.

Concept| Meaning
Requirement| What the computation needs
Capability| What an execution environment provides
Constraint| What must be obeyed
Preference| What is desirable
Hint| Optional implementation guidance
Budget| Operational resource allowance
Feasibility| Whether a realization satisfies mandatory conditions

A preference must not silently become a requirement.

A hint must not become a constraint.

A target capability must not become a universal language limit.

15.2 Abstract planning

The compiler may produce:

- Workload partitions.
- Operation-to-capability mappings.
- Abstract execution dependencies.
- Data-movement plans.
- Communication plans.
- Synchronization plans.
- Resource feasibility assessments.
- Candidate target realizations.

These are planning results.

They are not physical resource reservations.

15.3 Runtime boundary

The compiler does not:

- Allocate actual GPUs.
- Reserve physical qubits.
- Create runtime workers.
- Establish live device connections.
- Perform physical resource release.
- Execute distributed tasks.

These remain responsibilities of runtime and hardware-facing subsystems.

---

16. Backend integration

The authoritative backend subsystem is "src/backend/".

The compiler must integrate existing backend contracts rather than establish a competing backend architecture.

Each backend provider must declare:

- Provider identity.
- Provider version.
- Supported IR inputs.
- Supported target descriptors.
- Supported operations.
- Supported data types.
- Supported effects.
- Required capabilities.
- ABI compatibility.
- Artifact formats.
- Verification requirements.
- Failure behavior.
- Determinism properties.

Backend discovery must be extensible.

Adding a new processor architecture must not require editing a central closed hardware enum throughout the compiler.

16.1 LLVM

The LLVM integration must:

- Lower compatible verified IR.
- Validate the LLVM module.
- Preserve required metadata.
- Produce genuine LLVM output.
- Distinguish textual IR from object files and executables.
- Report missing toolchain dependencies.

16.2 WebAssembly

The WASM integration must:

- Validate the supported WebAssembly version.
- Lower compatible representations.
- Preserve control flow.
- Validate memory and calling conventions.
- Produce actual WASM artifacts.
- Report unsupported features.

16.3 Other providers

Future providers may support:

- CPUs.
- GPUs.
- TPUs.
- FPGAs.
- ASICs.
- QPUs.
- Neuromorphic hardware.
- Photonic hardware.
- Distributed accelerators.
- Future architectures.

No provider should require modification of unrelated compiler stages merely to become available.

---

17. Artifact contract

The compiler must distinguish three concepts.

17.1 Portable compilation artifact

A portable artifact represents validated, target-independent computation.

It may contain:

- Verified portable IR.
- Domain representations.
- Semantic contracts.
- Requirements.
- Constraints.
- Dependencies.
- Provenance.
- Version information.
- Realization metadata.

It need not be directly executable.

17.2 Realized artifact

A realized artifact is generated for a compatible target.

Examples:

- Native executable.
- Native library.
- GPU binary.
- FPGA bitstream.
- Quantum execution package.
- WASM module.
- Hybrid deployment package.

17.3 Deployment bundle

A deployment bundle combines compatible artifacts and the metadata required by the deployment environment.

It must preserve:

- Artifact identity.
- Dependency relationships.
- Target compatibility.
- Required capabilities.
- ABI information.
- Version compatibility.
- Provenance.
- Verification results.

The artifact subsystem must never claim that an uncompiled source file or manifest is an executable.

---

18. Incremental compilation

The existing incremental compiler implementation must remain the authoritative compatibility foundation.

Relevant files include:

- "src/compiler/incremental.rs", if retained as the legacy entry point.
- "src/compiler/incremental/"
- "src/compiler/context/dependency_context.rs"

Incremental compilation must:

- Track source dependencies.
- Track semantic dependencies.
- Track IR dependencies.
- Track pass dependencies.
- Track backend dependencies.
- Track configuration dependencies.
- Track relevant toolchain versions.
- Invalidate affected results.
- Preserve unaffected verified results.
- Revalidate outputs when required.

Cache identity must account for all inputs that can change compilation results.

A cache hit is not proof that an artifact remains compatible with a changed target, compiler, IR version, semantic model or backend.

Cached artifacts must be verified against their applicable compatibility contracts.

---

19. Security contract

Security is mandatory throughout compilation.

Relevant owners:

- "src/compiler/security/"
- "src/compiler/safety_guard.rs", if retained.
- "src/zuir/verify/"
- "src/backend/"
- "src/runtime/"

Security checks must cover:

- Source validation.
- Semantic contracts.
- IR validity.
- Transformation legality.
- Backend input.
- Generated artifacts.
- Dependency integrity.
- Provenance.
- Plugin trust.
- Artifact loading.
- Target authorization.

The security system must not rely exclusively on function names or textual inspection.

It must inspect the relevant structured representations.

Compiler plugins must not automatically receive unrestricted access to compiler internals or the host environment.

No unsafe Rust may be introduced to implement these facilities.

---

20. Grammar and compilation-pipeline integration

The compiler pipeline consumes source-level intent through the frontend.

It must not interpret grammar declarations directly.

20.1 Canonical grammar authorities

The relevant existing files include:

- "grammar/DESIGN.md"
- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniLexer.g4"
- "grammar/antlr/ZamaniParser.g4"
- "grammar/lexer/tokens.g4"
- "grammar/compile/README.md"
- "grammar/compile/compile.g4"
- "grammar/compile/compilation.g4"
- "grammar/specification/semantic-model.md"
- "grammar/validation/semantic-boundaries.md"

The lexer and parser must produce the canonical frontend representations.

The compiler must consume validated AST and semantic information.

20.2 Compilation grammar

The compilation grammar describes source-level intent such as:

- Target intent.
- Optimization intent.
- Compilation profiles.
- Specialization.
- Lowering intent.
- Artifact intent.
- Caching.
- Reproducibility.
- Cross-compilation.
- Deployment intent.

It does not implement those operations.

20.3 Composition authority conflict

The inspected documentation gives "grammar/compile/compile.g4" and "grammar/compile/compilation.g4" overlapping composition responsibilities.

This must be resolved explicitly.

The required contract is:

1. "compile.g4" owns the canonical "Compile" parser entry and composition of compilation-intent feature grammars.
2. "compilation.g4" may provide a higher-level compilation-specification composition only if its distinct responsibility and ANTLR integration are formally established.
3. Neither file may independently redefine the same public parser entry.
4. Shared productions must have one owner.
5. The grammar build must mechanically validate imports, rule references and generated parser compatibility.
6. "grammar/compile/README.md" must document the final authority relationship.
7. The frontend parser adapter must consume the canonical generated/parser boundary.

The grammar architecture must not rely solely on comments claiming that composition is valid.

20.4 Semantic interpretation

Compilation declarations must pass through:

Grammar
   |
   v
Lexer
   |
   v
Parser
   |
   v
Domain-neutral AST
   |
   v
Semantic validation
   |
   v
Compilation semantic facts
   |
   v
Compiler configuration and pipeline

The compiler must not bypass semantic validation because a compilation declaration parsed successfully.

---

21. Existing pipeline files: completion contracts

The following files must be completed and integrated without introducing duplicate implementations.

File| Required final responsibility
"src/compiler/mod.rs"| Canonical compiler composition root, module declarations and compatibility exports
"src/compiler/pipeline/mod.rs"| Stable public pipeline facade
"src/compiler/pipeline/stage.rs"| Canonical stage contract
"src/compiler/pipeline/stages.rs"| Standard stage identities and dependencies
"src/compiler/pipeline/ordering.rs"| Validated deterministic dependency ordering
"src/compiler/pipeline/execution.rs"| Correct execution and dependency-output delivery
"src/compiler/pipeline/failure.rs"| Structured failure representation
"src/compiler/pipeline/recovery.rs"| Safe recovery decision contracts
"src/compiler/pipeline/orchestration.rs"| Complete lifecycle orchestration
"src/compiler/context/mod.rs"| Context composition
"src/compiler/context/compilation_context.rs"| Shared invocation context
"src/compiler/context/compilation_state.rs"| Lifecycle state management
"src/compiler/context/dependency_context.rs"| Compilation dependency snapshots
"src/compiler/context/provenance_context.rs"| Provenance tracking
"src/compiler/configuration/mod.rs"| Configuration composition
"src/compiler/configuration/budget.rs"| Shared resource accounting
"src/compiler/configuration/config.rs"| Compiler configuration
"src/compiler/configuration/policy.rs"| Compiler policy contracts
"src/compiler/configuration/profile.rs"| Profile resolution
"src/compiler/configuration/target.rs"| Extensible target descriptors
"src/compiler/configuration/validation.rs"| Configuration validation
"src/compiler/analysis/mod.rs"| Compiler analysis composition
"src/compiler/ir/mod.rs"| IR integration composition
"src/compiler/passes/mod.rs"| Canonical ZUIR pass integration
"src/compiler/api/mod.rs"| Public compilation API
"src/compiler/api/compiler.rs"| Stable compiler service
"src/compiler/api/request.rs"| Compilation request
"src/compiler/api/session.rs"| Session lifecycle
"src/compiler/incremental/"| Incremental dependency and cache integration
"src/compiler/parallel/"| Parallel compiler coordination, where implemented
"src/compiler/artifacts/"| Artifact composition and verification
"src/compiler/backends/"| Backend-provider integration
"src/compiler/planning/"| Heterogeneous planning
"src/compiler/security/"| Security integration
"src/compiler/testing/"| Compiler conformance testing

Existing legacy files must be inspected individually before deciding whether they should be retained, adapted or deprecated.

A historical filename must not be recreated as a duplicate merely because a newer subsystem has been introduced.

---

22. Compiler root integration

"src/compiler/mod.rs" is the composition root.

It must expose the existing compiler modules and preserve the existing public API.

Its responsibilities are:

- Declare compiler modules.
- Expose the canonical compiler API.
- Expose pipeline integration.
- Expose configuration and context.
- Expose analysis and IR integration.
- Expose pass integration.
- Expose artifact and backend integration.
- Expose diagnostics.
- Preserve existing compatibility functions.
- Coordinate the actual compilation driver.

It must not contain the implementation of every compiler subsystem.

The root must not silently default to a host-specific target when a target-independent compilation artifact is requested.

The existing "production_config()" and "CompilerConfig" behavior must be reviewed together.

The existing "CompilationTarget" compatibility contract must be preserved while introducing extensible target realization.

The public entry points must not report success when the production pipeline has not actually executed its mandatory stages.

The "LegacyPipelineDriver" in "src/compiler/api/compiler.rs" must remain visibly transitional until the complete verified pipeline replaces the legacy path.

A stable service interface is not proof of a production-ready underlying compiler.

---

23. Recommended directory ownership

Use the existing directory structure.

Do not create parallel directory trees for responsibilities that already have owners.

The compiler's logical organization should remain:

src/compiler/
│
├── mod.rs
│
├── api/
│
├── configuration/
│
├── context/
│
├── pipeline/
│
├── analysis/
│
├── ir/
│
├── passes/
│
├── optimization/
│
├── planning/
│
├── lowering/
│
├── backends/
│
├── artifacts/
│
├── incremental/
│
├── linking/
│
├── diagnostics/
│
├── security/
│
├── parallel/
│
├── profiling/
│
├── testing/
│
└── documentation/
    ├── ARCHITECTURE.md
    ├── PIPELINE_CONTRACT.md
    ├── INTEGRATION_CONTRACT.md
    └── PORTABILITY_CONTRACT.md

This is a logical target structure.

It is not an instruction to recreate directories already present.

Before adding a directory or file:

1. Inspect the current repository tree.
2. Identify the existing owner.
3. Identify existing implementations.
4. Identify compatibility obligations.
5. Determine whether an adapter is sufficient.
6. Create a new file only when it has a distinct responsibility.

---

24. Production testing requirements

The pipeline must have independent unit tests and complete integration tests.

24.1 Stage tests

Test:

- Valid stage descriptors.
- Invalid identifiers.
- Duplicate identifiers.
- Missing dependencies.
- Cyclic dependencies.
- Valid topological ordering.
- Deterministic ordering.
- Stage input validation.
- Stage output validation.
- Failure propagation.
- Cancellation.
- Budget exhaustion.

24.2 Orchestration tests

Test:

- Successful required stages.
- Optional stages.
- Conditional stages.
- Required-stage skipping.
- Blocked dependencies.
- Failed stages.
- Invalid graphs.
- Partial reports.
- Lifecycle transitions.
- Cancellation during execution.
- Invalid terminal transitions.
- Recovery decision handling.

24.3 Semantic integration tests

Test that:

- Invalid programs never reach successful backend realization.
- Valid semantic information is preserved into IR.
- Ownership requirements are enforced.
- Effects are preserved.
- Capability requirements are preserved.
- Resource constraints are preserved.
- Unsupported operations fail explicitly.

24.4 IR tests

Test:

- Canonical ZUIR construction.
- Classical IR integration.
- Quantum IR integration.
- HDLIR integration.
- AI integration.
- Hybrid dependencies.
- IR conversion.
- Verification.
- Transformation preservation.
- Invalid conversion rejection.

24.5 Portability tests

Compile the same source program for different compatible targets.

Compare:

- Semantic results.
- Observable behavior.
- Required effects.
- Numerical guarantees.
- Domain-specific behavior.
- Artifact compatibility.

A successful backend invocation is not sufficient evidence of portability.

24.6 Parallel and incremental tests

Verify:

- Parallel and sequential results.
- Dependency ordering.
- Shared budgets.
- Cancellation.
- Cache invalidation.
- Dependency changes.
- Configuration changes.
- Backend-version changes.
- Reproducibility.

24.7 Security tests

Verify:

- Malformed source.
- Malformed IR.
- Invalid transformations.
- Untrusted plugins.
- Invalid artifacts.
- Dependency tampering.
- Provenance inconsistency.
- Backend input validation.

---

25. Required integration fixtures

The compiler should use shared repository-level fixtures.

tests/compiler/
│
├── fixtures/
│   ├── minimal.zm
│   ├── classical.zm
│   ├── quantum.zm
│   ├── hybrid.zm
│   ├── hdl.zm
│   ├── ai.zm
│   ├── distributed.zm
│   ├── networking.zm
│   └── poco-reaf.zm
│
├── pipeline/
├── stages/
├── ordering/
├── execution/
├── orchestration/
├── recovery/
├── analysis/
├── ir/
├── optimization/
├── planning/
├── lowering/
├── backends/
├── artifacts/
├── linking/
├── incremental/
├── parallel/
├── portability/
├── security/
└── scalability/

Every fixture must have:

- Expected parsing behavior.
- Expected semantic facts.
- Expected IR properties.
- Expected stage activation.
- Expected verification results.
- Expected compilation outcome.
- Expected portability constraints.

A fixture must not claim universal executability simply because it compiles on one test machine.

---

26. Determinism and reproducibility

The compiler must distinguish:

- Deterministic source parsing.
- Deterministic semantic analysis.
- Deterministic stage ordering.
- Deterministic transformations.
- Deterministic artifact generation.
- Reproducible builds.
- Runtime nondeterminism explicitly permitted by the language.

Where deterministic output is required, the compiler must control:

- Ordering of registered stages.
- Ordering of dependencies.
- Symbol resolution.
- Pass selection.
- Optimization decisions.
- Serialization.
- Artifact metadata.
- Provenance records.

External timestamps, random identifiers, host state and nondeterministic provider responses must not silently affect reproducible compilation.

Where an external toolchain cannot provide deterministic output, the limitation must be documented.

---

27. Rust implementation requirements

The entire compiler pipeline must use:

- Rust 2021 edition.
- Rust 1.97 or later.
- Safe Rust.
- No project-owned "unsafe".
- Checked arithmetic.
- Explicit error propagation.
- Well-defined ownership.
- Thread-safe shared state where required.
- No unchecked assumptions about external artifacts.

Required repository validation:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additional testing must include:

- Grammar generation and validation.
- Parser conformance.
- Semantic conformance.
- IR verification.
- Backend conformance.
- Portability testing.
- Incremental compilation.
- Determinism.
- Fuzzing.
- Security testing.
- Scalability testing.

All tests must be run against the actual repository state being released.

---

28. Implementation and integration order

The pipeline must be implemented in dependency order.

Do not implement the orchestration facade before its foundational contracts are established.

Phase 0 — Repository audit

Establish:

- Complete compiler file inventory.
- Current module declarations.
- Actual public interfaces.
- Existing tests.
- Existing grammar ownership.
- Existing semantic ownership.
- Existing IR ownership.
- Existing backend integration.
- Duplicate implementations.
- Unresolved placeholders.
- Missing module declarations.
- Dependency graph.

Deliverable: verified repository architecture and defect register.

Phase 1 — Independent contracts

Complete and validate:

- Configuration.
- Resource budgets.
- Target descriptors.
- Policies.
- Compilation context.
- Lifecycle state.
- Diagnostics.
- Dependency identity.
- Provenance.

These must not depend on pipeline orchestration.

Phase 2 — Pipeline foundations

Complete and freeze:

- "pipeline/stage.rs"
- "pipeline/stages.rs"
- "pipeline/ordering.rs"
- "pipeline/failure.rs"

Establish all public contracts before implementing dependent orchestration.

Phase 3 — Execution

Complete:

- "pipeline/execution.rs"
- "pipeline/recovery.rs"
- "pipeline/orchestration.rs"
- "pipeline/mod.rs"

Prove required-stage enforcement, cancellation, failure propagation and deterministic execution.

Phase 4 — Compiler API and root

Integrate:

- "compiler/mod.rs"
- "compiler/api/"
- "compiler/context/"
- "compiler/configuration/"

Preserve existing public entry points.

Phase 5 — Frontend and semantic integration

Integrate:

- Lexer.
- Parser.
- AST.
- Name resolution.
- Types.
- Effects.
- Ownership.
- Requirements.
- Capabilities.
- Provenance.

No competing semantic model may be introduced.

Phase 6 — IR integration

Integrate:

- ZUIR.
- ZCIR.
- Quantum IR.
- HDLIR.
- AI and tensor representations.
- Hybrid relationships.

Complete representation verification and conversion.

Phase 7 — Pass management and optimization

Integrate:

- Canonical ZUIR pass manager.
- Existing optimizer.
- Analysis invalidation.
- Transformation legality.
- Instruction fusion.
- Cost models.
- Optimization policies.

Phase 8 — Hybrid and heterogeneous planning

Integrate:

- Capability matching.
- Feasibility.
- Resource planning.
- Partitioning.
- Communication.
- Data movement.
- Abstract scheduling.

Phase 9 — Lowering and backends

Integrate existing backend providers.

Verify that each produces genuine, validated outputs.

Phase 10 — Linking and artifacts

Complete:

- Symbol resolution.
- Relocation.
- LTO.
- Portable artifacts.
- Target-specific artifacts.
- Hybrid bundles.
- Artifact validation.
- Provenance.

Phase 11 — Incremental and parallel execution

Complete:

- Sound fingerprints.
- Invalidation.
- Persistent caching.
- Parallel compilation.
- Shared budget accounting.
- Deterministic collection.

Phase 12 — Production certification

Run the full test suite.

Verify:

- Semantic correctness.
- IR correctness.
- Security.
- Portability.
- Reproducibility.
- Scalability.
- Backend conformance.
- Runtime compatibility.

No phase may be declared complete merely because its files compile.

---

29. Per-file completion and freeze contract

Every compiler file must have an implementation contract before coding begins.

The contract must include:

Item| Required definition
Purpose| Exact responsibility
Ownership| What the file owns
Non-ownership| What it must not implement
Public API| Complete types, functions and traits
Dependencies| Existing and planned dependencies
Upstream integration| Inputs and authorities
Downstream integration| Consumers and outputs
Invariants| Conditions that must always hold
Error handling| Every relevant failure category
Configuration| Policies and operational budgets
Scalability| No artificial machine ceilings
Determinism| Required reproducibility
Security| Trust boundaries
Testing| Unit and integration tests
Compatibility| Existing API obligations
Completion| Objective acceptance criteria

A file may be declared frozen only when:

1. Its implementation is complete.
2. Its public interface is stable.
3. Its dependencies are established.
4. Its tests pass.
5. Its integrations are verified.
6. Its ownership boundaries are respected.
7. Its error behavior is complete.
8. Its scalability requirements are satisfied.
9. No production placeholder remains.
10. Its documentation matches its actual behavior.

A frozen file may still require correction if a genuine defect is discovered.

The purpose of freezing is to prevent architectural redesign caused by incomplete upfront contracts, not to prevent necessary maintenance.

---

30. Production readiness criteria

The compilation pipeline is production-ready only when all the following conditions are satisfied.

Area| Required condition
Frontend| Canonical lexer, parser and AST integration
Semantics| Complete semantic validation
Ownership| Actual ownership enforcement
Effects| Complete effect integration
Requirements| Mandatory requirements preserved
Capabilities| Extensible capability matching
IR| Canonical IR integration
Verification| Verification at required boundaries
Optimization| Semantics-preserving transformations
Hybrid| Cross-domain dependency correctness
Planning| Feasibility and resource planning
Lowering| Verified conversion and legalization
Backends| Genuine provider implementations
Linking| Correct symbols, ABI and relocations
Artifacts| Verified portable and realized artifacts
Incremental| Sound cache invalidation
Parallel| Dependency-safe parallel execution
Recovery| Verified retry and fallback behavior
Security| End-to-end security validation
Determinism| Reproducible results where required
Scalability| No arbitrary hardware ceilings
Compatibility| Existing API preservation
Testing| Complete integration and conformance suite

Until these conditions are demonstrated, the compiler must describe its capabilities accurately rather than claim complete universal portability.

---

31. Final POCO-REAF contract

Zamani's ultimate compilation model is:

                ONE SOURCE PROGRAM
                       |
                       v
              ONE DEFINED MEANING
                       |
                       v
             ONE VALIDATED PROGRAM
                       |
                       v
                CANONICAL IR
                       |
                       v
            PORTABLE COMPILATION MODEL
                       |
                       v
             MULTIPLE REALIZATIONS
                       |
          +------------+-------------+
          |            |             |
          v            v             v
        CPU          GPU           FPGA
          |            |             |
          +------------+-------------+
                       |
              +--------+---------+
              |        |         |
              v        v         v
             ASIC     QPU     DISTRIBUTED
              |        |         |
              +--------+---------+
                       |
                       v
               HYBRID EXECUTION
                       |
                       v
                CONSISTENT
              PROGRAM MEANING

The source program must not be tied to the temporary characteristics of the machine on which it is compiled.

The compiler must determine the appropriate realization from:

- Program semantics.
- Requirements.
- Constraints.
- Capabilities.
- Preferences.
- Policies.
- Available resources.
- Backend compatibility.
- Verification evidence.

A new compatible target must be integrable without redesigning the language, semantic subsystem, canonical IR or entire compiler pipeline.

A future computing domain must be integrable through explicit domain contracts and registered implementations.

The compiler must scale from tiny embedded workloads to the largest workloads supported by available resources.

There is no promise of infinite physical resources or guaranteed execution on incapable hardware.

There must be no artificial universal ceiling imposed by the language or compiler architecture.

The governing principle is:

«One Zamani program. One canonical semantic meaning. One portable compilation architecture. Multiple verified realizations. No unnecessary source rewrites. No artificial hardware ceilings.»

That is the production compilation contract required to make Zamani's POCO-REAF architecture real.
Zamani Toolchain — Agent Integration Contract

File: "src/toolchain/documentation/AGENT_CONTRACT.md"
Repository: "Benwellonedge28/Zamani"
Status: Normative architecture, implementation and production-readiness contract
Language implementation: Rust 1.97 or later
Rust edition: 2021
Safety: Safe Rust only; "unsafe" is prohibited
Portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document establishes the complete production architecture and integration contract for human developers, AI agents, automated development systems, IDEs, CI/CD systems, enterprise automation and future autonomous computing agents interacting with the Zamani toolchain.

The objective is to provide one extensible, secure, consistent and resource-aware interface through which authorized clients can:

- Discover and inspect projects and workspaces.
- Understand source, project and dependency structures.
- Request compilation and validation.
- Request classical, quantum, HDL, AI, hybrid and distributed builds.
- Inspect requirements, capabilities, preferences, constraints and hints.
- Request target compatibility and feasibility assessments.
- Run tests and benchmarks.
- Create, verify, install and publish packages.
- Inspect external toolchains and environments.
- Request deployment planning and execution through authorized providers.
- Inspect diagnostics, provenance, build reports and execution outcomes.
- Continue interrupted workflows where the underlying operation supports safe resumption.
- Interact with the toolchain through structured, versioned interfaces.

The same capabilities must be available through consistent public APIs and command services, regardless of whether the client is a human, an AI agent, an IDE, an automation system or another authorized service.

An AI agent is a client of the Zamani toolchain, not an alternative implementation of the toolchain.

The agent subsystem must not create another:

- Compiler.
- Frontend or semantic analyzer.
- Intermediate representation.
- Package manager.
- Dependency resolver.
- Build executor.
- Target registry.
- Hardware abstraction.
- Runtime.
- Security policy engine.
- Authentication system.
- Authorization engine.
- Cryptographic implementation.
- Audit persistence system.
- Deployment engine.

It must coordinate existing authoritative services through their documented public contracts.

This document is normative for "src/toolchain/agents/" and its integration with the rest of the repository.

It does not claim that every agent capability or every downstream toolchain operation is already implemented merely because its interface is specified.

2. Normative language

The following terms have precise meanings:

- MUST: Mandatory for conformance.
- MUST NOT: Prohibited.
- SHOULD: Recommended unless a documented technical reason justifies deviation.
- SHOULD NOT: Discouraged unless justified.
- MAY: Optional.
- UNKNOWN: The available evidence cannot establish a fact.
- UNRESOLVED: A required evaluation has not established an acceptable result.
- UNSUPPORTED: The authoritative implementation does not support the requested operation.
- INFEASIBLE: The requested operation cannot satisfy its mandatory requirements under the evaluated conditions.
- DENIED: Authorization has explicitly rejected an operation.
- INDETERMINATE: Authorization or validation could not establish a reliable decision.

These statuses MUST remain distinguishable throughout request processing, execution, reporting and auditing.

A missing result must never be silently interpreted as success.

3. Repository architecture and ownership

3.1 Current agent subsystem

The inspected repository already contains the following agent implementation files:

src/toolchain/agents/
├── mod.rs
├── context.rs
├── requests.rs
├── results.rs
├── permissions.rs
├── audit.rs
└── compliance.rs

The responsibilities are:

File| Canonical responsibility
"mod.rs"| Agent subsystem composition and public module boundaries
"context.rs"| Validated, provenance-aware agent context
"requests.rs"| Agent request envelope and canonical API request validation
"results.rs"| Structured agent workflow outcomes and advisory next actions
"permissions.rs"| Operation-specific authorization integration
"audit.rs"| Agent operation audit events and audit-sink integration
"compliance.rs"| Fail-closed workflow compliance evaluation

All seven files MUST retain these single-authority boundaries.

The agent architecture MUST be expanded through these existing files before creating additional abstractions.

A new file is justified only when it introduces a genuinely independent responsibility that cannot be correctly owned by an existing subsystem.

3.2 Canonical repository authorities

The following ownership boundaries are mandatory.

Responsibility| Authoritative repository location
Source grammar| "grammar/"
Lexing and parsing| "src/frontend/"
AST| "src/frontend/ast/"
Semantic analysis| "src/frontend/semantic/"
Classical computation| "src/classical/"
Quantum computation and quantum IR| "src/quantum/"
HDL and hardware description| "src/hdl/"
AI computation| "src/ai/"
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"
Universal IR| "src/zuir/"
Compilation| "src/compiler/"
Target-specific code generation| "src/backend/"
External process execution| "src/backend/process.rs"
External tool discovery| "src/backend/toolchain.rs"
Physical hardware integration| "src/zhal/"
Runtime and application execution| "src/zur/"
Project configuration| "src/zamani_project_config/"
Toolchain API| "src/toolchain/api/"
Command registration and dispatch| "src/toolchain/commands/"
Security policy evaluation| "src/toolchain/security/policy.rs"
Aggregate security validation| "src/toolchain/security/validation.rs"
Toolchain agent integration| "src/toolchain/agents/"

The agent subsystem MUST consume these authorities without redefining their responsibilities.

3.3 Toolchain-level ownership

The toolchain agent subsystem owns:

1. Agent-facing context contracts.
2. Agent request adaptation.
3. Operation-specific authorization integration.
4. Agent workflow result composition.
5. Agent audit-event composition.
6. Agent compliance evaluation.
7. Agent/client integration documentation.
8. Structured interaction with canonical toolchain services.

It does not own the actual computation requested by an agent.

For example:

AI agent
   |
   v
Agent request
   |
   v
Canonical toolchain API
   |
   v
Command authorization
   |
   v
Compliance and security validation
   |
   v
Canonical command dispatcher
   |
   v
Build / Package / Test / Deploy service
   |
   +--> Compiler
   +--> Backend
   +--> ZHAL
   +--> ZUR
   +--> Registered providers

An agent MUST NOT bypass this pipeline.

4. Fundamental human/AI parity contract

4.1 One public toolchain interface

Human developers and AI agents MUST use the same canonical toolchain operations.

The following interfaces are shared:

- "crate::toolchain::api"
- "crate::toolchain::commands"
- "crate::toolchain::configuration"
- "crate::toolchain::build"
- "crate::toolchain::dependencies"
- "crate::toolchain::packages"
- "crate::toolchain::registries"
- "crate::toolchain::targets"
- "crate::toolchain::deployment"
- "crate::toolchain::security"

An agent-facing wrapper MAY add contextual metadata, evidence references, correlation information and workflow reporting.

It MUST NOT change the underlying meaning of a canonical operation.

For example, an AI-requested build and a human-requested build MUST use the same:

- Build request type.
- Manifest interpretation.
- Dependency resolution.
- Build-plan validation.
- Compiler API.
- Target compatibility rules.
- Security checks.
- Artifact verification.
- Error semantics.
- Execution handoff.

There must not be an unrestricted agent-only build path.

4.2 Client type is not authenticated identity

The system MUST distinguish:

1. Client type.
2. Claimed client identity.
3. Authenticated principal.
4. Delegated authority.
5. Authorization decision.

A client may identify itself as an AI agent, IDE, automation process or human-facing application.

That declaration is descriptive metadata.

It MUST NOT establish authentication or authorization.

Authenticated identity MUST come from an authoritative authentication provider.

The following MUST NOT grant permissions:

- A client-supplied username.
- An agent-supplied identity string.
- An agent-generated role.
- A claimed administrator status.
- An agent-generated authorization token.
- A context fact.
- A successful compliance report.
- A registered provider name.

4.3 Equal enforcement

Human/AI parity means equal enforcement of the same security and operation contracts.

It does not mean that every client must use an identical authentication mechanism.

A human session and an authenticated autonomous agent may have different identity mechanisms, but both MUST resolve to authenticated principals and pass through the same applicable authorization, validation and command-enforcement boundaries.

Delegated agent authority MUST be explicit, scoped, revocable and auditable.

An agent MUST NOT inherit all the permissions of its human creator merely because the human created or configured it.

5. Agent trust and security model

5.1 Zero implicit trust

Every agent request, context value, metadata field, recommendation, provider response and remotely supplied result MUST be treated as untrusted until validated by the appropriate authority.

Agent-generated text MUST NOT be interpreted as an instruction to override:

- Security policy.
- Authorization decisions.
- Compiler validation.
- Semantic requirements.
- Target feasibility.
- Resource constraints.
- Runtime restrictions.
- Deployment policy.
- User approval requirements.

Prompt injection, malicious project documentation, hostile source comments, malicious package metadata and untrusted provider responses MUST NOT modify toolchain security policy.

Agent reasoning is not an authorization mechanism.

5.2 Authentication

Authentication is owned by the authoritative identity provider.

"agents/permissions.rs" MUST consume an authenticated principal through its existing provider boundary.

It MUST NOT implement credential verification.

The identity integration MUST support:

- Human principals.
- AI-agent principals.
- Service principals.
- Delegated principals.
- Enterprise identity providers.
- Future authentication providers.

Identity identifiers MUST be open-ended.

The architecture MUST NOT impose a universal maximum number of users, agents, service identities or delegated principals.

Credentials, tokens, private keys, session secrets and authentication material MUST NOT be included in agent context, public results, diagnostic messages or audit payloads.

Only approved opaque evidence references may be recorded.

5.3 Authorization

Authorization MUST be:

- Explicit.
- Operation-specific.
- Resource-specific.
- Default-deny.
- Based on authenticated identity.
- Evaluated against authoritative policy.
- Bound to the actual command being dispatched.

The existing "agents/permissions.rs" implementation MUST integrate with:

- "commands::CommandAuthorizer".
- "commands::CommandDescriptor".
- "commands::CommandRequest".
- "security::policy".
- "security::trust".
- "security::validation".
- An authoritative authenticated-principal provider.
- An authoritative protected-resource resolver.
- An authoritative authorization-attribute provider.

The existing authorization receipt MUST preserve the binding between:

- Authenticated principal.
- Command identity.
- Protected resource.
- Correlation identity.
- Authentication evidence reference.
- Explicit policy decision.

Only an explicit canonical Permit decision may authorize the operation.

The following MUST never authorize an operation:

- Missing policy.
- Empty policy set.
- Missing identity.
- Unknown identity.
- Indeterminate policy evaluation.
- Evaluation-budget exhaustion.
- Unverified security evidence.
- Unresolved protected-resource identity.
- Mismatched command identity.
- Mismatched resource identity.

An agent's requested action MUST be checked against the exact operation eventually dispatched.

An authorization receipt for one command MUST NOT be reused to authorize another command.

Authorization MUST be revalidated where required by policy, particularly when resource identity, permission revision, delegation validity or protected state changes between authorization and execution.

5.4 Capabilities are not permissions

The architecture MUST preserve the following distinctions:

Concept| Meaning
Identity| Who or what is authenticated
Permission| An authorization rule describing allowed actions
Authorization| A decision for a principal, action and resource
Capability| What an implementation can provide
Requirement| What an operation must satisfy
Constraint| What an operation must not violate
Preference| What outcome is preferred
Hint| Advisory information
Feasibility| Whether the requested realization can satisfy its requirements
Evidence| Information supporting a claim or decision
Provenance| Where information or an artifact originated

A GPU capability does not authorize GPU access.

An authorization to use a GPU does not prove that a GPU is available.

A requested quantum capability does not prove that a compatible QPU exists.

An agent MUST NOT infer feasibility from authorization, or authorization from feasibility.

6. Agent context contract

6.1 Ownership

"src/toolchain/agents/context.rs" owns the agent-facing context representation and its construction.

It MUST remain an immutable, validated, provenance-aware snapshot.

It MUST NOT become another:

- Project discovery service.
- Workspace manager.
- Manifest parser.
- Semantic analyzer.
- Capability evaluator.
- Target registry.
- Resource allocator.
- Compiler configuration authority.
- Hardware inventory.
- Runtime context authority.

6.2 Context information

An agent context MAY contain:

- Project identity.
- Workspace identity.
- Project scope.
- Build identity.
- Build request references.
- Toolchain API version.
- Source fingerprints.
- Dependency fingerprints.
- Environment fingerprints.
- Validated requirements.
- Validated capabilities.
- Preferences.
- Constraints.
- Hints.
- Operational budgets.
- Diagnostics.
- Evidence references.
- Evidence status.
- Context completeness.
- Context visibility classifications.

These values MUST retain their original meaning and provenance.

6.3 Evidence requirements

Every evidence-backed fact MUST identify its authoritative source.

An evidence reference SHOULD include:

- Stable evidence identity.
- Authority identity.
- Evidence reference.
- Relevant revision or version.
- Verification status.
- Scope.
- Applicable freshness or validity information where supported.

The agent context MUST distinguish:

- Verified.
- Unverified.
- Stale.
- Incomplete.
- Rejected.
- Unavailable.
- Unknown.

An unverified fact MUST NOT become verified merely because it is copied into a context snapshot.

The context builder MUST reject missing evidence references where evidence is mandatory.

6.4 Information classification

Context facts MUST preserve their visibility and classification.

At minimum, the implementation must distinguish:

- Public information.
- Restricted information.
- Secret information.

Secret values MUST NOT be inserted into agent context.

Restricted facts MUST only be included when the authenticated principal is independently authorized to access them.

The context builder MUST NOT perform authorization itself.

It MUST consume authorization-filtered information from the appropriate authority.

6.5 Context completeness

A context marked complete MUST satisfy its declared completeness contract.

An incomplete context MUST remain explicitly incomplete.

The absence of a fact MUST NOT be interpreted as proof that the fact does not exist.

The absence of a capability MUST NOT be interpreted as proof that a capability is impossible.

The absence of an error MUST NOT be interpreted as successful validation.

6.6 Context scalability

Context construction MUST support incremental and resource-aware collection.

It MUST support:

- Large projects.
- Large workspaces.
- Numerous dependency records.
- Large evidence collections.
- Multiple computational domains.
- Large target matrices.
- Distributed environments.
- Future providers.

The implementation MUST NOT silently truncate facts or evidence to satisfy a collection budget.

Where collection exceeds a configured budget, it MUST return an explicit incomplete or budget-exhausted outcome.

Context budgets MUST be configurable.

The implementation MUST use fallible allocation where allocation failure is reasonably recoverable.

Literal infinite memory is not promised.

7. Agent request contract

7.1 Ownership

"src/toolchain/agents/requests.rs" owns the agent-facing request wrapper.

It MUST reuse the canonical request types from:

"src/toolchain/api/request.rs"

It MUST NOT create competing build, check, run, test, package, install, publish, dependency, target or deployment request models.

7.2 Canonical request envelope

The canonical "RequestEnvelope" remains the source of operation identity and request semantics.

The existing "AgentRequest" wraps:

- The canonical request envelope.
- The validated agent context.

It MUST preserve the original request identity, correlation metadata, project scope, operation identity and canonical request contents.

It MUST reject mismatched request and context identities.

7.3 Request validation

Every request MUST pass validation before dispatch.

Validation MUST include:

1. Request structure.
2. Required fields.
3. Identifier validity.
4. API-version compatibility.
5. Operation identity.
6. Project and workspace scope.
7. Target-selection syntax and structure.
8. Operational budget validity.
9. Context-to-request binding.
10. Correlation consistency.
11. Idempotency metadata where applicable.

Deserialization is not validation.

Every deserialized or remotely received request MUST be validated again before execution.

Structural validation MUST NOT pretend to establish filesystem access, authorization, capability availability or target feasibility.

Those checks belong to the appropriate authorities.

7.4 Idempotency and retries

Mutating requests SHOULD carry an idempotency key.

Examples include:

- Package installation.
- Package publishing.
- Registry modification.
- Deployment.
- External state changes.
- Resource provisioning.

The receiving service MUST define the scope and retention semantics of idempotency keys.

Repeated requests with the same key and equivalent operation identity MUST NOT accidentally produce duplicate external effects.

A retry MUST NOT be assumed safe merely because the original response was lost.

Operations without reliable idempotency support MUST return an explicit uncertain or unresolved outcome when their completion cannot be established.

7.5 Request integrity

Where requests cross trust boundaries, the implementation MUST provide a documented mechanism to bind the authenticated session, request identity, operation and protected resource.

A request MUST NOT be modified after authorization without revalidation.

An agent MUST NOT alter the target, resource scope, deployment destination or operation arguments while reusing an authorization receipt.

7.6 Long-running requests

Long-running operations MUST support structured status and correlation.

Where implemented, they SHOULD support:

- Cancellation.
- Progress reporting.
- Checkpoint references.
- Safe resumption.
- Partial results.
- Explicit terminal outcomes.

A submitted request is not a successful operation.

8. Permission and command integration

8.1 Canonical dispatcher

All agent operations MUST pass through the canonical command infrastructure:

"src/toolchain/commands/mod.rs"

The command registry and dispatcher remain authoritative for:

- Command registration.
- Command identity.
- Command discovery.
- Command descriptors.
- Authorization integration.
- Audit integration.
- Handler dispatch.
- Structured command errors.

Agents MUST NOT call internal command handlers directly to avoid authorization or auditing.

They MUST NOT invoke private compiler, backend, ZHAL or ZUR implementations directly.

8.2 Command mutability

Every command MUST declare its mutability and audit requirements through its canonical descriptor.

The authorization and audit policies MUST be applied according to the command's actual effects.

Examples:

Operation| Security considerations
Inspect project| Protected project-read scope
Read restricted files| Resource-specific read permission
Build| Project and execution permissions
Clean| Generated-output deletion scope
Run| Application execution permission
Test| Test execution permission
Install| Package installation and filesystem scope
Publish| Registry publication permission
Deploy| Deployment and external-state permissions
Modify configuration| Configuration-write permission
Manage credentials| Dedicated privileged permission

An operation MUST NOT be considered read-only merely because the client describes it as inspection.

Its actual effects determine its security classification.

8.3 Command execution guarantees

The command dispatcher MUST guarantee:

- Authorization before protected execution.
- Resource binding.
- Mandatory audit acceptance before execution.
- No handler invocation after denied or indeterminate authorization.
- No success response for failed handlers.
- No silent loss of required audit events.
- Explicit handling of failures after external side effects.

If mandatory pre-execution auditing fails, execution MUST NOT begin.

If a required final audit record fails after an external operation has already completed, the result MUST distinguish the completed external operation from the audit failure.

The system MUST NOT falsely claim that the operation never happened.

The existing command infrastructure's enforcement is authoritative. The agent compliance layer MUST NOT attempt to replace it.

9. Agent result contract

9.1 Ownership

"src/toolchain/agents/results.rs" owns agent workflow result composition.

It MUST reuse the canonical response from:

"src/toolchain/api/response.rs"

It MUST NOT invent alternative compiler artifacts, diagnostics, authorization decisions, feasibility reports or execution results.

9.2 Structured outcomes

An agent result MUST distinguish:

- Overall request outcome.
- Individual workflow-stage outcomes.
- Canonical toolchain response.
- Diagnostics.
- Evidence references.
- Provenance references.
- Partial results.
- Cancellation.
- Resource exhaustion.
- Advisory next actions.

A successful workflow stage MUST contain the evidence required by its contract.

An unresolved stage MUST NOT become successful simply because a later stage completed.

9.3 Suggested actions

Suggested next actions MUST remain advisory.

Examples:

- Install a missing external tool.
- Resolve a dependency conflict.
- Inspect an unsupported target.
- Request additional authorization.
- Select a compatible provider.
- Retry an interrupted operation.
- Review a diagnostic.

Suggestions MUST NOT execute automatically.

A suggested operation MUST be submitted as a new canonical request and pass the full security and authorization lifecycle.

9.4 Result truthfulness

An agent result MUST never claim:

- A build succeeded when only a build plan was constructed.
- A target is feasible when feasibility is unknown.
- A package is trusted when verification was not performed.
- A deployment succeeded when only a deployment plan exists.
- A program executed when only an artifact was generated.
- A hardware resource was allocated when only a provider was discovered.
- A cancelled operation completed successfully.

Results MUST preserve the difference between planning, validation, execution, verification and completion.

9.5 Machine-readable interoperability

Results MUST be serializable through the canonical API serialization contract.

They SHOULD support:

- Structured JSON responses.
- Stable operation identifiers.
- Stable error codes.
- Diagnostic references.
- Evidence references.
- Correlation identifiers.
- Version information.
- Pagination or streaming for large result sets.

Human-readable rendering MUST be a presentation of the structured result, not a separate source of truth.

10. Audit contract

10.1 Ownership

"src/toolchain/agents/audit.rs" owns agent audit-event representation and integration with the authoritative audit sink.

It does not own:

- Authentication.
- Authorization.
- Policy evaluation.
- Cryptographic signing.
- Hash generation.
- Provenance verification.
- Storage implementation.
- Retention policy.
- Log rotation.
- Runtime enforcement.

Those remain with their respective authorities.

10.2 Mandatory audit information

Where required by policy, an audit event MUST preserve:

- Authenticated principal identity.
- Client type.
- Command identity.
- Protected resource identity.
- Request identity.
- Correlation identity.
- Authentication evidence reference.
- Authorization outcome.
- Actual operation outcome.
- Event identity.
- Trusted occurrence-time information.
- Applicable policy or authorization revision.
- Relevant provenance references.

Client-supplied metadata MUST be identified as non-authoritative.

10.3 Audit lifecycle

Audit records MUST cover the relevant lifecycle:

1. Request received.
2. Authentication established or rejected.
3. Authorization evaluated.
4. Compliance evaluated.
5. Operation accepted or rejected.
6. Operation started.
7. Operation completed, failed, cancelled or became uncertain.
8. Final outcome recorded.

The precise events required depend on the applicable security policy.

The system MUST preserve denial and failure events.

It MUST NOT record credentials, access tokens, private keys, raw secrets or unnecessary sensitive source contents.

10.4 Audit reliability

The audit sink MUST report whether an event was accepted.

An event MUST NOT be treated as persisted merely because it was placed in an in-memory queue.

The required durability, integrity, retention and tamper-evidence guarantees MUST be supplied by the configured authoritative audit-storage and security providers.

If the policy requires durable audit acceptance before an operation, the command dispatcher MUST enforce that requirement.

The agent layer MUST NOT silently downgrade mandatory audit to best-effort logging.

10.5 Audit scalability

Audit events MUST be capable of being streamed, batched or persisted incrementally.

The design MUST NOT require loading the complete audit history into memory.

No arbitrary universal maximum may be imposed on the total number of audit records.

Storage budgets, retention, batching, indexing and throughput controls MUST be configurable operational policies.

11. Compliance contract

11.1 Ownership

"src/toolchain/agents/compliance.rs" owns agent workflow compliance checks and aggregate compliance reporting.

It MUST reuse the authoritative implementations of:

- Request validation.
- Authorization.
- Security validation.
- Result validation.
- Audit integration.

It MUST NOT implement a second authorization engine.

11.2 Fail-closed behavior

Compliance MUST fail closed whenever a mandatory check:

- Is missing.
- Is rejected.
- Is unresolved.
- Is indeterminate.
- Cannot be evaluated.
- Exhausts its evaluation budget.
- Has invalid or mismatched evidence.

Only an explicitly verified mandatory condition may satisfy that condition.

Compliance MUST distinguish required checks from advisory checks.

An advisory failure MUST NOT silently override a mandatory authorization decision, and advisory success MUST NOT replace mandatory validation.

11.3 Compliance is not authorization

A successful compliance report does not grant permission.

Only the canonical authorization authority establishes authorization.

A compliance report MUST preserve the authorization decision it evaluated.

It MUST reject mismatched command identities, request identities, project scopes and correlation identifiers.

11.4 Extensibility

Compliance controls MUST support registered extensions.

Extensions MUST have stable identities and documented contracts.

Untrusted remote agents MUST NOT register executable in-process compliance extensions.

Only trusted, authorized integration code may register such extensions.

Remote evidence from an extension MUST be validated and authenticated before it is accepted.

New computational domains MUST NOT require changing the compliance model merely to introduce another domain identifier.

12. Complete agent workflow

The normative request lifecycle is:

Human / AI agent / IDE / automation
                 |
                 v
          Client connection
                 |
                 v
        Request decoding
                 |
                 v
       API version negotiation
                 |
                 v
       Canonical request validation
                 |
                 v
       Authentication provider
                 |
                 v
       Authenticated principal
                 |
                 v
        Agent context validation
                 |
                 v
      Request/context correlation
                 |
                 v
      Protected resource resolution
                 |
                 v
       Canonical authorization
                 |
                 v
        Compliance evaluation
                 |
                 v
      Mandatory security validation
                 |
                 v
       Required audit acceptance
                 |
                 v
       Canonical command dispatcher
                 |
                 v
        Authoritative service
                 |
                 v
      Operation outcome collection
                 |
                 v
       Final audit recording
                 |
                 v
        Structured API response
                 |
                 v
         Agent result composition
                 |
                 v
       Human / machine presentation

The dispatcher and authoritative operation services MUST enforce their own mandatory checks.

The diagram does not permit a caller to perform these steps independently and then claim that execution is authorized.

The ordering of authentication, authorization, compliance and security checks MUST respect the dependencies of their authoritative implementations.

For example, authorization cannot precede obtaining an authenticated principal, and dispatch cannot precede the required authorization and audit checks.

12.1 Cancellation

Cancellation MUST propagate through the canonical task and command execution infrastructure.

An agent MUST NOT report cancellation as successful completion.

The final outcome MUST distinguish:

- Cancellation requested.
- Cancellation accepted.
- Cancellation in progress.
- Cancellation completed.
- Cancellation unsupported.
- Cancellation failed.
- External operation outcome uncertain.

A cancellation request MUST NOT erase audit evidence.

12.2 Timeouts

Timeouts MUST be explicit operational budgets.

A timeout MUST NOT silently become success.

For an external operation, timeout does not necessarily mean the external system stopped performing the operation.

Where completion cannot be established, the result MUST be uncertain or unresolved.

12.3 Concurrency

Concurrent requests MUST preserve their individual:

- Authenticated principals.
- Authorization receipts.
- Project scopes.
- Correlation identifiers.
- Idempotency identities.
- Operational budgets.
- Audit events.
- Result identities.

No mutable authorization state may accidentally leak between concurrent clients.

The agent subsystem MUST NOT introduce a competing scheduler or worker pool.

It MUST reuse the canonical task and command infrastructure.

13. POCO-REAF and computational-domain neutrality

13.1 One agent interface for all computing

The agent architecture MUST support the complete Zamani computational ecosystem through the same canonical toolchain interface.

It must accommodate:

- Classical computing.
- Embedded and systems programming.
- Parallel and concurrent computing.
- Scientific and high-performance computing.
- Quantum computing.
- Quantum simulation.
- Quantum hardware integration.
- HDL and hardware/software co-design.
- FPGA and ASIC workflows.
- AI and machine learning.
- Neural networks and tensor computation.
- Symbolic and probabilistic computation.
- Hybrid computation.
- Distributed computation.
- Networking.
- Cryptography.
- Accelerators.
- Future computational domains.

The agent subsystem MUST NOT create separate agent APIs for each domain.

13.2 Open-world domain identification

Domain identities MUST be extensible.

The agent architecture MUST NOT rely on a permanently closed enum of computational domains.

New domains and providers MUST be represented through canonical registered identifiers and capability contracts.

Adding a domain MUST NOT require rebuilding the agent authorization model around a new hard-coded vendor or device list.

13.3 Semantic portability

POCO-REAF requires preservation of program semantics across compatible realizations.

It does not promise that every physical target can execute every program.

An agent MUST understand the difference between:

- Source portability.
- Semantic compatibility.
- Compilation compatibility.
- Artifact compatibility.
- Runtime compatibility.
- Physical resource availability.
- Deployment compatibility.

For example, one Zamani source project may produce:

- A native executable.
- A WebAssembly artifact.
- An accelerator-specific artifact.
- A quantum circuit artifact.
- An HDL synthesis input.

These are distinct realizations of a compatible logical project, not necessarily interchangeable machine-code outputs.

An agent MUST NOT promise a realization without authoritative evidence.

13.4 Requirements and target feasibility

The agent may communicate:

- Requirements.
- Capabilities.
- Preferences.
- Constraints.
- Hints.
- Operational budgets.

It MUST preserve these categories without conflating them.

Target feasibility MUST be established by the canonical compiler, capability, backend, hardware and runtime authorities as appropriate.

The agent may request an assessment.

It may not certify its own feasibility decision.

14. Scalability and resource-parametric operation

14.1 Fundamental rule

The agent subsystem MUST scale from the smallest supported development environment to arbitrarily large computational systems, subject to actual available resources and implementation capabilities.

This is a resource-scalability objective, not a claim of literal infinite computation or storage.

No arbitrary universal ceilings may be introduced for:

- Number of agents.
- Number of users.
- Number of projects.
- Number of workspaces.
- Number of requests.
- Number of context facts.
- Number of evidence records.
- Number of computational domains.
- Number of targets.
- Number of providers.
- Number of packages.
- Number of dependencies.
- Number of hardware devices.
- Number of CPUs.
- Number of GPUs.
- Number of FPGAs.
- Number of QPUs.
- Number of qubits.
- Number of distributed participants.
- Memory capacity.
- Tensor rank.
- Artifact count.
- Audit-event count.

The implementation MUST NOT introduce arbitrary "MAX_*" constants that turn current hardware capacities into permanent system limits.

14.2 Operational budgets

Operational safeguards are permitted and required.

They MUST be configurable rather than semantic or hardware ceilings.

Examples include:

- Request-processing time.
- Context-collection time.
- Maximum request payload size.
- Maximum response payload size.
- Maximum context collection size.
- Maximum concurrent requests.
- Maximum queued operations.
- Maximum audit batch size.
- Maximum diagnostic retention.
- Maximum memory allocated to one operation.
- Maximum temporary storage.
- Network transfer budgets.
- Retry budgets.
- Policy-evaluation budgets.

The actual set of budgets MUST be owned by the relevant configuration and execution authorities.

The agent subsystem MUST NOT introduce a competing global budget system.

A budget MUST identify:

1. The protected resource or operation.
2. Its configured value.
3. Its scope.
4. Its enforcement authority.
5. Its exhaustion behavior.
6. Whether the limit is mandatory or advisory.

A budget exhaustion result MUST be explicit.

It MUST NOT be reported as a semantic incompatibility or a successful operation.

14.3 Tiny and large environments

The architecture MUST support constrained environments without requiring all agent infrastructure to be active simultaneously.

It SHOULD support:

- Minimal local operation.
- Optional agent integration.
- Remote agent clients.
- Enterprise authentication.
- Distributed agent coordination.
- Streaming workflows.
- Incremental context collection.
- Large workspace inspection.
- Large dependency graphs.
- Long-running builds.
- Large audit histories.

Optional features MUST be activated through explicit configuration and feature boundaries.

A lightweight local developer workflow MUST NOT require a distributed agent service.

A distributed agent deployment MUST NOT require changing the meaning of canonical toolchain operations.

14.4 Memory and allocation

Agent context, results, audit events and diagnostic collections MUST use resource-conscious representations.

Where appropriate, implementations SHOULD use:

- Streaming.
- Pagination.
- Incremental collection.
- Fallible reservation.
- Bounded operational queues.
- Backpressure.
- Shared immutable data.
- Stable references instead of unnecessary copies.

The implementation MUST NOT silently discard security-relevant evidence to reduce memory use.

Allocation failure MUST produce an explicit failure.

Rust's finite address space, operating-system limits and actual physical memory remain implementation constraints.

15. Grammar and semantic integration

15.1 Grammar authority

The agent subsystem MUST respect the existing grammar architecture.

The grammar defines source-level language syntax and semantic intent.

It does not implement authentication, authorization enforcement, cryptography, runtime security or toolchain command execution.

The agent subsystem MUST NOT introduce another agent permission grammar or independently parse security declarations.

15.2 Relevant existing security grammars

The following files are authoritative source-syntax integration points.

Grammar file| Responsibility
"grammar/security/identity.g4"| Identity-related source syntax
"grammar/security/permissions.g4"| Permission declarations and references
"grammar/security/authorization.g4"| Authorization intent
"grammar/security/policies.g4"| Security policy composition
"grammar/security/capabilities.g4"| Capability-related syntax
"grammar/security/trust.g4"| Trust intent
"grammar/security/audit.g4"| Audit intent
"grammar/security/provenance.g4"| Provenance intent
"grammar/security/sandbox.g4"| Sandbox-related security intent
"grammar/security/secrets.g4"| Secret references and security boundaries
"grammar/policies/policy.g4"| General policy syntax
"grammar/policies/permissions.g4"| Policy permission composition
"grammar/policies/requirements.g4"| Requirement declarations
"grammar/policies/constraints.g4"| Constraint declarations
"grammar/policies/preferences.g4"| Preference declarations
"grammar/policies/deployment.g4"| Deployment policy intent

The grammar composition and normative semantics must be reconciled through:

- "grammar/README.md"
- "grammar/security/README.md"
- "grammar/spec/security.md"
- "grammar/CONFORMANCE.md"
- "grammar/DEPENDENCIES.yaml"
- "grammar/OWNERSHIP.yaml"
- "grammar/MANIFEST.yaml"
- "grammar/STATUS.yaml"

These files must remain consistent.

15.3 Compilation and execution intent

The following additional grammar authorities matter when agents request builds and deployments:

Grammar file| Responsibility
"grammar/compile/intent.g4"| Compilation-intent integration boundary
"grammar/compile/profiles.g4"| Compilation profile syntax
"grammar/compile/target-selection.g4"| Target-selection policy
"grammar/compile/deployment.g4"| Compilation deployment integration
"grammar/execution/deployment.g4"| Canonical deployment syntax
"grammar/policies/deployment.g4"| Deployment policy intent

The agent layer MUST consume validated semantic and toolchain configuration data.

It MUST NOT interpret source grammar directly as permission to execute an operation.

15.4 Semantic ownership

The frontend semantic authority MUST determine the meaning of source declarations.

In particular:

- A permission declaration is not an authorization decision.
- A capability declaration is not proof of actual capability.
- A requirement is not proof of feasibility.
- A target-selection preference is not a mandatory requirement.
- A hint cannot override a constraint.
- Deployment intent is not deployment execution.
- Audit syntax is not audit persistence.

The agent subsystem MUST consume the canonical semantic results and preserve their evidence and provenance.

It MUST NOT construct a competing semantic model.

15.5 Grammar changes

Any future grammar change affecting agent-visible security, permission, capability, requirement, constraint, preference, deployment or provenance semantics MUST be reviewed against this contract.

Such a change MUST:

1. Identify the canonical grammar owner.
2. Preserve the dependency direction.
3. Update the normative semantic specification.
4. Update grammar conformance evidence.
5. Preserve compatibility or document a migration.
6. Identify the affected frontend semantic adapter.
7. Identify the affected toolchain API contract.
8. Add relevant integration tests.

The agent implementation MUST NOT be used to compensate for inconsistent grammar or semantic authority.

16. Detailed file-by-file implementation contracts

The following contracts define what each existing agent file must own and how it integrates with the repository.

16.1 "src/toolchain/agents/mod.rs"

Role: Agent subsystem composition root.

MUST own:

- Public child-module declarations.
- Agent subsystem documentation.
- Publicly selected agent API organization.
- Cross-module ownership boundaries.
- Shared agent integration invariants.

MUST expose the existing modules:

pub mod context;
pub mod requests;
pub mod results;
pub mod permissions;
pub mod audit;
pub mod compliance;

MUST NOT expose implementation internals indiscriminately.

MUST NOT introduce duplicate shared types or service implementations.

Integration:

- "toolchain/api/"
- "toolchain/commands/"
- "toolchain/errors.rs"
- "toolchain/result.rs"
- "toolchain/security/"
- All six child modules.

Freeze criteria:

- All child modules compile.
- All public exports are intentional.
- Documentation matches actual implementation.
- No circular ownership is introduced.
- All security invariants have corresponding tests.

16.2 "src/toolchain/agents/context.rs"

Role: Agent context snapshots.

MUST own:

- Context fact representation.
- Evidence references.
- Visibility classifications.
- Context completeness.
- Project context.
- Build context.
- Toolchain context.
- Context builder.
- Context validation.
- Deterministic context ordering.
- Configurable context collection limits.

Integration:

- "toolchain/api/request.rs"
- "toolchain/ids.rs"
- "toolchain/errors.rs"
- "toolchain/result.rs"
- "toolchain/version.rs"
- "toolchain/workspace/"
- "toolchain/build/"
- "toolchain/agents/permissions.rs"
- "toolchain/agents/requests.rs"
- "frontend/"
- "compiler/"
- "backend/"
- "zhal/"
- "zur/"

MUST NOT authenticate, authorize, discover hardware or evaluate target feasibility.

Freeze criteria:

- All six intent categories are preserved.
- Evidence status cannot be upgraded without verification.
- Secret facts are rejected.
- Restricted facts require authorized collection.
- Request and context API versions are compatible.
- Context construction is deterministic.
- Configured collection budgets are enforced without silent truncation.
- Allocation failures are explicit.

16.3 "src/toolchain/agents/requests.rs"

Role: Agent request boundary.

MUST own:

- Agent request wrapper.
- Canonical envelope integration.
- Context/request binding.
- Project-scope validation.
- Operation identity preservation.
- Request correlation.
- Deserialized request revalidation.

Integration:

- "toolchain/api/request.rs"
- "toolchain/api/versioning.rs"
- "toolchain/api/compatibility.rs"
- "toolchain/agents/context.rs"
- "toolchain/errors.rs"
- "toolchain/commands/"
- "toolchain/ids.rs"

MUST reuse canonical request types.

MUST NOT execute commands or grant authorization.

Freeze criteria:

- A human and agent request for the same operation preserve identical canonical operation semantics.
- Context from another request is rejected.
- Scope mismatches are rejected.
- Deserialized requests must pass validation.
- Correlation metadata is preserved.
- Mutating requests preserve idempotency metadata.
- Unsupported API versions are rejected explicitly.

16.4 "src/toolchain/agents/permissions.rs"

Role: Authenticated client authorization integration.

MUST own:

- Authenticated-principal provider integration.
- Command-to-action mapping.
- Protected-resource resolution.
- Authorization attribute acquisition.
- Policy evaluation orchestration.
- Authorization receipt composition.
- Authorization/request binding.

Integration:

- "toolchain/commands/"
- "toolchain/security/policy.rs"
- "toolchain/security/trust.rs"
- "toolchain/security/validation.rs"
- "toolchain/agents/requests.rs"
- "toolchain/agents/audit.rs"
- "toolchain/errors.rs"

MUST NOT own authentication implementation or policy evaluation algorithms.

Freeze criteria:

- Default-deny behavior.
- Explicit Permit required.
- Deny and Indeterminate preserved.
- Unknown evidence cannot grant permission.
- Command mismatch rejected.
- Resource mismatch rejected.
- Unauthenticated identities rejected.
- Authorization receipts are immutable.
- Human and AI callers follow identical command authorization rules.
- Evaluation exhaustion cannot authorize an operation.

16.5 "src/toolchain/agents/results.rs"

Role: Structured agent workflow results.

MUST own:

- Stage outcome composition.
- Canonical API response integration.
- Agent result validation.
- Evidence references.
- Advisory next actions.
- Correlation validation.
- Workflow completion classification.

Integration:

- "toolchain/api/response.rs"
- "toolchain/agents/context.rs"
- "toolchain/agents/requests.rs"
- "toolchain/diagnostics/"
- "toolchain/errors.rs"
- "toolchain/result.rs"

MUST NOT generate fake artifact, compiler, authorization or feasibility evidence.

Freeze criteria:

- Unknown and unresolved stages prevent full success.
- Successful stages contain required evidence.
- Request and result identities match.
- Suggestions remain advisory.
- Partial outcomes are preserved.
- Serialization round trips preserve meaning.
- Failure cannot become success through result aggregation.

16.6 "src/toolchain/agents/audit.rs"

Role: Agent audit event and sink integration.

MUST own:

- Agent audit event model.
- Authorization outcome representation.
- Operation outcome representation.
- Audit event validation.
- Audit sink integration.
- Audit recording orchestration.

Integration:

- "toolchain/agents/permissions.rs"
- "toolchain/agents/requests.rs"
- "toolchain/agents/results.rs"
- "toolchain/agents/context.rs"
- "toolchain/commands/"
- "toolchain/security/"
- Canonical audit storage providers.

MUST NOT implement cryptographic signing, hashing, audit database storage or authentication.

Freeze criteria:

- Authenticated identity is preserved.
- Authorization decisions are preserved.
- Actual command outcomes are recorded.
- Credentials and secrets are excluded.
- Required audit acceptance is enforced.
- Event validation rejects malformed data.
- Large audit streams do not require full-history memory loading.
- Sink failures remain explicit.

16.7 "src/toolchain/agents/compliance.rs"

Role: Agent workflow compliance coordinator.

MUST own:

- Compliance control contracts.
- Compliance evidence.
- Compliance reports.
- Control aggregation.
- Fail-closed compliance decisions.
- Request/result correlation checks.

Integration:

- "agents/context.rs"
- "agents/requests.rs"
- "agents/results.rs"
- "agents/permissions.rs"
- "agents/audit.rs"
- "commands/"
- "security/validation.rs"
- "security/policy.rs"

MUST NOT authenticate, authorize, execute operations or independently establish feasibility.

Freeze criteria:

- Missing mandatory checks cannot pass.
- Indeterminate checks cannot pass.
- Explicit denial cannot pass.
- Mismatched command authorization is rejected.
- Request and result correlation is enforced.
- Remote evidence is validated.
- Advisory controls cannot override mandatory security decisions.
- Compliance never grants authorization.

17. Required integration with other toolchain files

The agent subsystem is not independently production-ready unless its public boundaries are connected correctly.

The following integrations are mandatory.

Existing file or subsystem| Required integration
"src/toolchain/api/request.rs"| Canonical requests, identity metadata, project scope and operation identity
"src/toolchain/api/response.rs"| Canonical operation responses
"src/toolchain/api/versioning.rs"| API version negotiation
"src/toolchain/api/compatibility.rs"| Contract compatibility
"src/toolchain/api/capabilities.rs"| Capability descriptions, without treating them as authorization
"src/toolchain/commands/mod.rs"| Mandatory shared command dispatch
"src/toolchain/security/policy.rs"| Canonical policy decisions
"src/toolchain/security/trust.rs"| Trust assessment
"src/toolchain/security/validation.rs"| Aggregate security validation
"src/toolchain/security/provenance.rs"| Provenance references
"src/toolchain/security/integrity.rs"| Integrity verification
"src/toolchain/configuration/policies.rs"| Applicable security policies
"src/toolchain/configuration/budgets.rs"| Configurable operational budgets
"src/toolchain/workspace/"| Canonical workspace identity and scope
"src/toolchain/build/"| Build requests, status and reports
"src/toolchain/tasks/"| Cancellation and task execution status
"src/toolchain/dependencies/"| Dependency-resolution evidence
"src/toolchain/packages/"| Package verification and installation results
"src/toolchain/registries/"| Registry operations and authenticated access
"src/toolchain/targets/"| Target intent and compatibility
"src/toolchain/deployment/"| Deployment plans and provider outcomes
"src/toolchain/diagnostics/"| Structured diagnostic references
"src/backend/process.rs"| Safe external process infrastructure, through owning services
"src/compiler/"| Canonical compilation service
"src/backend/"| Artifact realization
"src/zhal/"| Hardware capability and resource authority
"src/zur/"| Runtime and execution authority

The agent subsystem MUST NOT directly replace or bypass these services.

18. Public API, feature and crate integration

18.1 "src/toolchain/mod.rs"

The existing composition root MUST expose the agent subsystem through its deliberate public module structure.

The agent module must be available only through the intended toolchain API.

It MUST NOT create a separate top-level agent subsystem outside the toolchain.

18.2 "src/lib.rs"

The inspected current "src/lib.rs" does not declare "pub mod toolchain;".

This is an integration gap.

The toolchain's intended public exposure and Cargo feature policy MUST be reconciled.

The existing Cargo manifest defines:

[features]
default = []
lsp = []
buildsystem = []
full = []
omniversal = []

The "zamani-pkg" executable requires the "buildsystem" feature.

The implementation MUST establish a consistent feature-gating policy so that:

- Toolchain consumers can access the intended public API.
- Agent integration does not accidentally become available through an unsupported feature configuration.
- The package-manager binary can import the toolchain API.
- Documentation and tests use the same feature configuration.
- Lightweight consumers do not acquire unnecessary build-system functionality.

A possible library boundary is:

#[cfg(feature = "buildsystem")]
pub mod toolchain;

This is an integration proposal, not a claim that the current source already contains it.

The final feature decision MUST be validated against every existing binary, integration test and downstream library consumer before freezing.

18.3 "Cargo.toml"

The agent integration MUST preserve:

- Rust 1.97 or later.
- Rust 2021.
- Safe Rust only.
- Existing dependency ownership.
- Optional feature boundaries.
- Compatibility with the package-manager binary.

New authentication, network, transport, cryptographic or persistence dependencies MUST NOT be added merely because an agent feature might eventually use them.

A dependency must have a documented owner, necessity, security assessment and feature requirement.

18.4 "src/bin/zamani_pkg_main.rs"

The package-manager CLI MUST remain a thin interface over canonical command services.

The same command implementation must be accessible to human developers and authorized agent clients.

The CLI MUST NOT implement a second agent permission model or bypass the dispatcher.

19. Integration with classical, quantum, HDL, AI, hybrid and distributed computing

Agent requests MUST remain domain-neutral.

The same request architecture must coordinate domain-specific operations through canonical services.

19.1 Classical

Integrate with:

- "src/classical/"
- "src/frontend/"
- "src/compiler/"
- "src/backend/"

Agents may request compilation, testing, benchmarking and execution.

The compiler and backend establish the actual compatible realization.

19.2 Quantum

Integrate with:

- "src/quantum/"
- "src/quantum/ir/"
- Canonical quantum backend providers.
- "src/zhal/"
- "src/zur/"

Agents may request quantum compilation, simulation or execution.

They MUST NOT independently allocate qubits, route circuits, perform QEC or claim QPU availability.

19.3 HDL and hardware

Integrate with:

- "src/hdl/"
- "src/zhal/"
- "src/backend/"

Agents may request synthesis, simulation, validation and hardware artifact generation.

Actual synthesis, placement, routing, timing closure and physical realization belong to their authoritative implementations and providers.

19.4 AI and tensor

Integrate with:

- "src/ai/"
- Canonical tensor infrastructure.
- "src/zuir/"
- "src/compiler/"
- "src/backend/"
- "src/zur/"

Agents may request training, inference, model packaging and deployment.

They MUST NOT establish hardware feasibility merely by reasoning about an available accelerator.

19.5 Hybrid

Integrate with:

- "src/hybrid/"
- Classical IR.
- Quantum IR.
- HDL and hardware representations.
- AI and tensor infrastructure.
- Canonical compiler orchestration.

A hybrid request may produce several coordinated artifacts.

The agent result must preserve each artifact's identity, compatibility and provenance.

19.6 Distributed

Integrate with:

- "src/distributed/"
- "src/toolchain/tasks/"
- "src/toolchain/deployment/"
- "src/zur/"
- Authorized distributed providers.

Agents may request distributed builds and deployment.

They MUST NOT allocate cluster nodes, schedule physical work or assume a successful local build establishes distributed execution feasibility.

19.7 Future domains

New computational domains MUST integrate through open-ended canonical contracts.

The agent subsystem MUST NOT require a source-code change for every new provider or vendor merely to represent a new compatible capability.

20. Interoperability and external agent clients

20.1 Transport independence

The agent architecture MUST not depend on one permanently fixed transport.

It SHOULD support adapters for:

- In-process clients.
- Local command-line clients.
- IDE clients.
- Local automation.
- Remote authenticated services.
- Enterprise agent systems.
- Future registered communication protocols.

Transport adapters MUST terminate at the canonical API and command boundaries.

They MUST NOT create independent execution or authorization semantics.

20.2 Protocol versioning

Agent-facing contracts MUST be versioned independently from:

- Zamani language version.
- Grammar version.
- Compiler version.
- IR version.
- Backend version.
- Runtime version.
- Package-format version.
- Registry protocol version.

The existing "agents/requests.rs" and "agents/results.rs" contract versions MUST be documented and checked for compatibility.

Unsupported versions MUST produce explicit compatibility failures.

The system MUST NOT silently reinterpret an old request using new semantics.

20.3 Serialization

Serialized requests and results MUST have:

- Defined schemas.
- Explicit versions.
- Stable operation identities.
- Validated identifiers.
- Well-defined optional-field semantics.
- Defined unknown-field compatibility behavior.
- Explicit error representations.

Serialization MUST NOT be treated as authentication.

Remote responses MUST be validated before being trusted.

20.4 Large responses

Large context and result sets SHOULD support pagination, streaming or stable references.

A client MUST be able to distinguish:

- Complete response.
- Partial response.
- Paginated response.
- Truncated response.
- Unavailable response.
- Failed response.

Silent truncation is prohibited.

21. Human approval and consequential operations

Some operations may have consequences outside the toolchain process.

Examples:

- Publishing packages.
- Deleting project data.
- Modifying protected configuration.
- Installing untrusted dependencies.
- Changing security policies.
- Accessing protected credentials.
- Deploying applications.
- Provisioning infrastructure.
- Triggering physical hardware operations.
- Submitting work to paid external services.

The applicable policy MUST determine whether additional human approval is required.

An agent MUST NOT bypass such approval by:

- Changing its client identity.
- Submitting an equivalent request through another command.
- Splitting a prohibited operation into smaller requests.
- Reusing a previous authorization receipt.
- Invoking a lower-level service directly.

Approval MUST be bound to the intended operation and protected resource.

Approval for one action MUST NOT automatically authorize a different action.

The toolchain MUST record the relevant approval evidence where required.

22. Provenance and reproducibility

Agent-originated operations MUST preserve sufficient provenance to reconstruct what was requested and what actually happened.

Where applicable, provenance SHOULD include:

- Project identity.
- Source fingerprint.
- Manifest fingerprint.
- Dependency fingerprint.
- Build identity.
- Toolchain API version.
- Compiler version.
- Backend/provider identities.
- Target realization identity.
- Configuration and policy revisions.
- Agent or human principal.
- Operation identity.
- Artifact references.
- Verification evidence.
- Execution and deployment outcomes.

Agent reasoning traces MUST NOT be substituted for compiler provenance.

Agent-generated explanations are not authoritative build evidence.

Reproducibility MUST be established by the canonical build and artifact systems.

23. Failure and recovery semantics

Every failure MUST preserve its actual cause and classification.

At minimum, the architecture must distinguish:

- Invalid request.
- Unsupported API version.
- Authentication failure.
- Authorization denial.
- Authorization indeterminacy.
- Compliance rejection.
- Security validation failure.
- Missing capability.
- Unknown capability.
- Infeasible target.
- Dependency failure.
- Build failure.
- Provider failure.
- Resource exhaustion.
- Timeout.
- Cancellation.
- Audit failure.
- Serialization failure.
- External operation uncertainty.

A failure MUST NOT be converted into a successful result through fallback behavior.

Fallbacks MUST preserve mandatory requirements and constraints.

An agent MUST NOT recommend silently weakening security, correctness, reproducibility or program semantics to obtain a successful build.

Recovery actions MUST be submitted through the same canonical request, validation and authorization lifecycle.

24. Testing and conformance requirements

Agent integration is not production-ready until the implementation passes unit, integration, security, scalability and downstream compatibility tests.

24.1 Existing unit-test coverage

The current agent source already contains tests for important invariants, including:

- Open-ended computational domains.
- All six intent categories.
- Evidence validation.
- Restricted and secret facts.
- Context completeness.
- Deterministic context ordering.
- Context serialization.
- Request/context mismatch rejection.
- Human/agent request parity.
- Client identity versus authentication.
- Explicit Permit.
- Explicit Deny.
- Default-deny behavior.
- Command binding.
- Indeterminate authorization.
- Structured result validation.
- Compliance fail-closed behavior.

These tests MUST be preserved and expanded.

24.2 Required additional integration tests

Create or extend:

tests/toolchain/
└── agents/
    ├── mod.rs
    ├── human_agent_parity.rs
    ├── authentication.rs
    ├── authorization.rs
    ├── request_validation.rs
    ├── context_security.rs
    ├── compliance.rs
    ├── audit.rs
    ├── command_dispatch.rs
    ├── api_compatibility.rs
    ├── cancellation.rs
    ├── idempotency.rs
    ├── concurrency.rs
    ├── serialization.rs
    ├── resource_budgets.rs
    ├── provenance.rs
    ├── portability.rs
    ├── domain_interoperability.rs
    ├── failure_recovery.rs
    └── security_regression.rs

This is a proposed test structure. Before creating it, inspect the current test tree and reuse existing test modules where their ownership already matches.

24.3 Mandatory security scenarios

Tests MUST verify that:

1. An agent cannot authenticate itself by supplying an identity string.
2. An unregistered identity cannot execute protected commands.
3. A registered agent is not automatically trusted.
4. A missing policy denies protected execution.
5. An indeterminate policy decision denies execution.
6. Evaluation-budget exhaustion cannot authorize execution.
7. An authorization receipt cannot authorize another command.
8. An authorization receipt cannot authorize another resource.
9. A context fact cannot establish authorization.
10. Unverified evidence cannot establish feasibility.
11. Secret values cannot enter agent context.
12. Restricted context cannot leak to unauthorized clients.
13. Malicious serialized requests cannot bypass validation.
14. Prompt-injection content cannot change authorization.
15. Agent suggestions cannot execute automatically.
16. Failed mandatory audit acceptance prevents execution.
17. Failed final audit recording is not reported as if the external operation never happened.
18. Cancellation does not erase audit evidence.
19. Retry does not duplicate non-idempotent external operations.
20. A provider cannot declare itself trusted through its own response.

24.4 Human/agent parity tests

For each supported command, construct equivalent human and AI requests with equivalent authenticated principals and permissions.

Verify that both requests receive the same:

- Structural validation.
- Command classification.
- Authorization requirements.
- Security checks.
- Service invocation semantics.
- Compiler behavior.
- Artifact verification.
- Error classification.

Client-specific metadata may differ.

The underlying operation semantics MUST NOT.

24.5 Domain interoperability tests

The agent API MUST be tested with representative projects:

Fixture| Required evidence
"minimal.zm"| Basic project discovery and request validation
"classical.zm"| Classical compilation request
"quantum.zm"| Quantum provider request and honest feasibility
"hdl.zm"| Hardware workflow integration
"hybrid.zm"| Cross-domain request and artifact coordination
"distributed.zm"| Distributed provider and deployment integration
"ai.zm"| AI and tensor workflow integration

Tests MUST distinguish simulated provider responses from real provider execution.

24.6 Scalability tests

Test increasing workloads without introducing language-level ceilings.

Measure:

- Context construction.
- Evidence collection.
- Request validation.
- Policy evaluation.
- Compliance aggregation.
- Audit throughput.
- Result serialization.
- Concurrent requests.
- Large project scopes.
- Large target matrices.
- Large dependency references.

Test under deliberately constrained resources as well as larger environments.

The expected result under resource exhaustion is a truthful, structured failure, not silent data loss or corrupted authorization.

25. Rust implementation requirements

Every agent implementation file MUST satisfy:

- Rust 1.97 or later.
- Rust 2021 edition.
- No unsafe Rust.
- Explicit error handling.
- Documented public APIs.
- Stable and validated identifiers.
- Safe serialization and deserialization.
- No hidden global mutable state.
- No implicit privileged behavior.
- No fabricated successful results.
- No hard-coded hardware limits.

Each agent module MUST include:

1. Purpose.
2. Ownership.
3. Non-ownership.
4. Canonical integration dependencies.
5. Public API documentation.
6. Error behavior.
7. Security invariants.
8. Scalability behavior.
9. Concurrency behavior.
10. Unit tests.
11. Integration requirements.
12. Freeze criteria.

The following checks MUST pass:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Build-system feature checks:

cargo check --all-targets --features buildsystem

cargo test --all-targets --features buildsystem

The actual package-manager binary MUST also be validated:

cargo build --release \
    --features buildsystem \
    --bin zamani-pkg

Where feature combinations are supported, test them independently.

Passing Rust compilation does not prove authorization correctness, audit reliability, security or provider interoperability. Those require their own evidence.

26. Production-readiness acceptance matrix

An agent subsystem MUST NOT be declared production-ready until the following conditions are demonstrated.

Area| Acceptance condition
Architecture| All ownership boundaries enforced
Public API| Canonical API reused
Human/AI parity| Equivalent operation semantics
Authentication| Authoritative identity integration
Authorization| Explicit, default-deny decisions
Resource binding| Authorization bound to actual resource
Context| Validated, immutable, provenance-aware
Request validation| Deserialized requests revalidated
Compliance| Mandatory checks fail closed
Audit| Required events accepted and outcomes preserved
Results| Structured and truthful
Versioning| Explicit compatibility validation
Idempotency| Safe mutation retry semantics
Cancellation| Propagated and accurately reported
Concurrency| No cross-request identity or permission leakage
Scalability| Configurable resource budgets
Portability| Domain-neutral agent API
Security| Prompt injection and untrusted input defenses
Provenance| Source-to-operation evidence preserved
Testing| Unit, integration and security tests pass
Rust safety| Rust 1.97+, no unsafe
Documentation| All contracts and integration dependencies documented

27. File-freeze and implementation discipline

The user-facing objective of this architecture is that each file can be completed and frozen without repeatedly redesigning it when downstream files are implemented.

The following discipline is mandatory.

Before implementing a file, record:

1. Its single responsibility.
2. Its public types.
3. Its public functions and traits.
4. Its error contract.
5. Its serialization contract.
6. Its ownership boundaries.
7. Its exact upstream dependencies.
8. Its expected downstream consumers.
9. Its concurrency requirements.
10. Its security requirements.
11. Its resource-budget behavior.
12. Its unit tests.
13. Its integration tests.
14. Its freeze criteria.

The implementation sequence must follow dependency direction.

Recommended order:

Foundation

1. "agents/context.rs"
2. "agents/requests.rs"
3. "agents/results.rs"

Security integration

4. "agents/permissions.rs"
5. "agents/audit.rs"
6. "agents/compliance.rs"

Composition and integration

7. "agents/mod.rs"
8. Canonical API and command-dispatch integration
9. Feature and library exposure
10. Integration tests

The current files already contain implementations, so this order means auditing and completing their contracts, not blindly replacing existing code.

A file is frozen only when:

- Its public API is complete.
- Its dependencies are known.
- Its error behavior is established.
- Its security invariants are tested.
- Its downstream integration contracts are documented.
- Its tests pass.
- Its consumers compile against it.

A later defect may justify a deliberate correction to the owning file. Freeze does not mean refusing necessary security or correctness fixes.

28. Prohibited architectural shortcuts

The following are prohibited:

1. A special unrestricted AI-agent command dispatcher.
2. Treating an agent's claimed identity as authenticated.
3. Treating context as authorization.
4. Treating capability as permission.
5. Treating a target request as proof of feasibility.
6. Treating source grammar as runtime enforcement.
7. Allowing an agent to bypass the canonical compiler API.
8. Allowing agents to invoke private backend or hardware implementations directly.
9. Creating another policy evaluator.
10. Creating another audit persistence implementation.
11. Trusting agent-generated success reports.
12. Silently dropping evidence or audit records.
13. Silently weakening mandatory requirements.
14. Treating timeout as successful cancellation.
15. Retrying external side effects without defined idempotency.
16. Embedding secrets in prompts, context, results or logs.
17. Hard-coding a fixed list of supported computational domains.
18. Hard-coding arbitrary hardware or project capacity limits.
19. Introducing an independent scheduler.
20. Declaring the subsystem production-ready without real integration evidence.

29. Final architectural contract

The Zamani agent subsystem MUST provide one secure and extensible interaction boundary for human developers, AI agents and automated systems.

It must enable agents to understand projects, formulate requests, coordinate compilation, inspect results, propose corrections and request authorized operations.

However, the agent is never the authority for:

- Source-language meaning.
- Semantic correctness.
- Authorization.
- Security policy.
- Hardware capabilities.
- Resource availability.
- Target feasibility.
- Compilation success.
- Artifact integrity.
- Runtime execution.
- Deployment success.

Those decisions remain with their canonical repository authorities.

The complete relationship is:

              ONE ZAMANI PROJECT
                      |
                      v
            Canonical project model
                      |
                      v
             Canonical toolchain API
                      |
           +----------+----------+
           |                     |
           v                     v
      Human client           AI agent
           |                     |
           +----------+----------+
                      |
                      v
             Shared authorization
                      |
                      v
            Shared command services
                      |
                      v
             Canonical compiler
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
       Classical   Quantum       HDL
          |           |           |
          +-----------+-----------+
                      |
                AI / Hybrid /
                 Distributed
                      |
                      v
             Backend and ZHAL
                      |
                      v
                 ZUR / Runtime
                      |
                      v
          Verified execution outcomes
                      |
                      v
             Audit and provenance

A single logical project and its semantic requirements must remain independent of the number, type and size of available computational resources.

Different environments may produce different compatible realizations.

The agent architecture must preserve the same project identity, source identity, dependency identity, build intent, authorization rules, security requirements and provenance across those realizations.

It must scale from constrained machines to large distributed systems without turning current hardware limitations into permanent language restrictions.

It must also be honest: when resources, capabilities, compatibility or authorization cannot be established, the agent must report that condition rather than invent success.

The final principle is:

«Humans and AI agents may share the same power to develop Zamani programs, but neither may bypass the authorities that establish what those programs mean, what operations are permitted, what resources are available, what can be compiled and what has actually executed.»

That is the agent-integration foundation required for POCO-REAF.

---

30. Repository integration checklist for this contract

This checklist is part of the implementation specification. It is not a declaration that all items have already been completed.

- [ ] Add this document at "src/toolchain/documentation/AGENT_CONTRACT.md".
- [ ] Reconcile "src/toolchain/agents/mod.rs" with this contract.
- [ ] Audit "context.rs" against the evidence and information-classification requirements.
- [ ] Audit "requests.rs" against canonical request, version and idempotency requirements.
- [ ] Audit "permissions.rs" against the command dispatcher and resource-binding requirements.
- [ ] Audit "results.rs" against truthful outcome and evidence requirements.
- [ ] Audit "audit.rs" against command-level audit enforcement and persistence guarantees.
- [ ] Audit "compliance.rs" against mandatory fail-closed behavior.
- [ ] Reconcile "src/toolchain/commands/mod.rs" with the shared human/agent enforcement lifecycle.
- [ ] Reconcile "src/toolchain/api/request.rs" and "response.rs".
- [ ] Reconcile API compatibility and versioning.
- [ ] Reconcile "src/toolchain/security/policy.rs" and "validation.rs".
- [ ] Reconcile identity, trust, integrity and provenance authorities.
- [ ] Verify "src/lib.rs" exposes the toolchain under the intended Cargo feature policy.
- [ ] Verify "Cargo.toml" and all relevant feature combinations.
- [ ] Reconcile the relevant grammar ownership and conformance records.
- [ ] Add missing agent integration and security regression tests.
- [ ] Validate the actual CLI and downstream client integration.
- [ ] Run all required Rust checks.
- [ ] Record evidence in "src/toolchain/documentation/READINESS_MATRIX.md" when that document is established.

Completion rule: This contract is complete as documentation only when its ownership, dependencies, interfaces, security behavior, integration requirements and acceptance tests are internally consistent. The agent subsystem itself is production-ready only when implementation and test evidence demonstrate conformance.
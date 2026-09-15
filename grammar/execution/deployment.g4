/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/execution/deployment.g4
* 
* Grammar:
* Deployment
* 
* Status:
* Production-ready deployment-intent parser grammar
* 
* Baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust only
* no unsafe
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar defines SOURCE-LEVEL DEPLOYMENT INTENT.
* 
* Deployment describes how a compiled/semantic Zamani computation is intended
* to be made available to an execution environment.
* 
* It expresses portable intent such as:
* 
* - deployment identity;
* - deployment artifact references;
* - deployment environment intent;
* - deployment requirements;
* - deployment constraints;
* - deployment preferences;
* - deployment capabilities;
* - deployment policies;
* - deployment lifecycle intent;
* - rollout intent;
* - availability intent;
* - portability intent;
* - recovery/failure intent;
* - observability intent;
* - deployment metadata;
* - deployment parameters;
* - future deployment properties.
* 
* It does NOT perform deployment.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* Core parser
*      |
*      v
* Deployment parser
*      |
*      v
* frontend AST
*      |
*      +--> name resolution
*      +--> type analysis
*      +--> capability analysis
*      +--> resource analysis
*      +--> target resolution
*      +--> deployment semantic validation
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL / hardware representation
*      +--> distributed representation
*      |
*      v
* compilation / lowering
*      |
*      +--> optimization
*      +--> routing
*      +--> scheduling
*      +--> resilience
*      +--> hardware HAL
*      |
*      v
* deployment planning
*      |
*      v
* runtime / dispatcher / deployment system
* 
* ============================================================================
* CORE PRINCIPLE
* ============================================================================
* 
* This grammar describes:
* 
* WHAT deployment outcome is intended.
* 
* It does NOT describe:
* 
* HOW a particular provider, operating system, cluster manager, cloud
* platform, QPU service, FPGA toolchain, container runtime, or deployment
* engine implements that outcome.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - deployment declaration syntax;
* - deployment subject syntax;
* - deployment context syntax;
* - deployment intent categories;
* - deployment lifecycle intent;
* - rollout intent;
* - deployment availability intent;
* - deployment portability intent;
* - deployment policy intent;
* - deployment artifact references;
* - deployment parameter syntax;
* - deployment requirement/constraint/preference/hint composition;
* - deployment metadata syntax;
* - open-ended deployment properties.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifiers;
* - qualified names;
* - general expressions;
* - types;
* - modules;
* - package management;
* - compilation;
* - code generation;
* - target compilation;
* - resource discovery;
* - resource allocation;
* - hardware discovery;
* - hardware calibration;
* - topology;
* - placement algorithms;
* - routing;
* - scheduling;
* - dispatch implementation;
* - runtime implementation;
* - cloud/provider APIs;
* - container implementation;
* - orchestration implementation;
* - networking implementation;
* - authentication;
* - authorization;
* - QEC;
* - ZQN;
* - quantum operations;
* - quantum::ir;
* - classical IR;
* - HDL IR;
* - resilience algorithms.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* This grammar imports Core and ExecutionContext.
* 
* Core owns:
* 
* expression
* identifier
* qualifiedName
* argumentList
* literals
* punctuation
* canonical language names
* 
* ExecutionContext owns:
* 
* executionContext
* executionContextEntry
* executionContextValue
* executionContextProperty/value composition
* 
* Deployment MUST reuse those definitions.
* 
* Deployment MUST NOT create another expression language, name language,
* literal language, or generic execution-context language.
* 
* Dependency direction:
* 
* ZamaniLexer
*      |
*      v
*    Core
*      |
*      v
* ExecutionContext
*      |
*      v
*  Deployment
* 
* The reverse dependency is forbidden.
* 
* ============================================================================
* INTEGRATION WITH EXECUTION.G4
* ============================================================================
* 
* "execution.g4" owns generic execution declarations.
* 
* This grammar owns deployment-specific syntax.
* 
* The execution composition layer should expose deployment through:
* 
* executionDeployment
*     : deploymentDeclaration
*     ;
* 
* or an equivalent delegation rule.
* 
* "execution.g4" MUST NOT duplicate deployment rules.
* 
* ============================================================================
* INTEGRATION WITH EXECUTION-CONTEXT.G4
* ============================================================================
* 
* Deployment may be embedded inside an execution context.
* 
* Example semantic shape:
* 
* deploy computation {
*     target: portable;
*     resources: required_resources;
*     rollout: rollout_policy;
* }
* 
* The deployment grammar owns the deployment structure.
* 
* Generic nested property semantics remain compatible with
* ExecutionContext.
* 
* ============================================================================
* INTEGRATION WITH RUNTIME-CAPABILITIES.G4
* ============================================================================
* 
* Deployment capability requirements are references to the runtime capability
* model.
* 
* This grammar does not enumerate:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* simulator
* cluster
* cloud
* 
* as a closed vocabulary.
* 
* Capability analysis resolves those names against the runtime capability
* system.
* 
* ============================================================================
* INTEGRATION WITH PLACEMENT.G4
* ============================================================================
* 
* Deployment may express placement intent.
* 
* It MUST NOT implement placement.
* 
* Deployment can say that a placement policy is required, preferred, or
* constrained.
* 
* Placement grammar and semantic infrastructure determine how placement is
* realized.
* 
* ============================================================================
* INTEGRATION WITH SCHEDULING.G4
* ============================================================================
* 
* Deployment may express lifecycle or timing-related deployment intent.
* 
* It MUST NOT define:
* 
* ASAP
* ALAP
* critical path
* RCPSP
* pulse timing
* operation scheduling
* quantum scheduling
* 
* Those remain scheduling concerns.
* 
* ============================================================================
* INTEGRATION WITH DISPATCH.G4
* ============================================================================
* 
* Deployment produces intent consumed by dispatch.
* 
* Deployment does NOT:
* 
* select a backend;
* open a device connection;
* submit a job;
* start a process;
* allocate a node;
* select a QPU;
* communicate with a provider.
* 
* Dispatch owns the actual handoff.
* 
* ============================================================================
* INTEGRATION WITH HARDWARE / RESOURCES
* ============================================================================
* 
* Deployment properties may refer to resource and hardware capabilities using
* expressions and open semantic names.
* 
* Example:
* 
* resources {
*     processing: required_processing;
*     memory: required_memory;
*     quantum: required_quantum_resources;
* }
* 
* This does not allocate those resources.
* 
* Actual resource availability belongs to resource management, hardware HAL,
* target resolution, and runtime infrastructure.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* A deployment declaration MUST NOT make a portable program permanently
* dependent on one physical environment unless that dependency is explicitly
* part of the program's semantic contract.
* 
* Deployment therefore distinguishes:
* 
* requirement
* constraint
* preference
* hint
* capability
* resource
* target
* placement
* policy
* metadata
* 
* These are interpreted downstream.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* This grammar contains no finite machine or deployment limits.
* 
* It contains no:
* 
* MAX_DEVICES
* MAX_NODES
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QUBITS
* MAX_REPLICAS
* MAX_INSTANCES
* MAX_REGIONS
* MAX_JOBS
* MAX_DEPLOYMENTS
* MAX_RESOURCES
* MAX_ARTIFACTS
* MAX_TARGETS
* 
* Quantities are expressions.
* 
* Collections use parser repetition.
* 
* Actual limits belong to:
* 
* - resource analysis;
* - target capabilities;
* - deployment policy;
* - runtime policy;
* - operating-system limits;
* - provider limits;
* - explicit user constraints.
* 
* Therefore the grammar can describe deployment from a tiny execution
* environment to arbitrarily large environments subject only to downstream
* resource availability and explicit semantic constraints.
* 
* ============================================================================
* NO HARD-CODED TOPOLOGY
* ============================================================================
* 
* Deployment MUST NOT encode:
* 
* device IDs;
* physical addresses;
* node names as required syntax;
* cloud regions as language keywords;
* fixed cluster shapes;
* fixed accelerator counts;
* fixed quantum topology;
* fixed CPU topology;
* fixed network topology.
* 
* Such information may appear as semantic data only where explicitly required
* by a deployment contract and must remain represented through expressions or
* qualified names rather than hard-coded grammar alternatives.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no semantic actions;
* - no Rust code;
* - no predicates;
* - no filesystem access;
* - no network access;
* - no hardware discovery;
* - no resource discovery;
* - no random behavior;
* - no mutable global state.
* 
* Given the same canonical token stream, parsing is deterministic.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* The grammar contains no Rust implementation code.
* 
* The implementation using this grammar must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* safe Rust only
* no unsafe
* 
* ============================================================================
  */

/*

* ============================================================================
* PARSER GRAMMAR
* ============================================================================
  */

parser grammar Deployment;

import Core, ExecutionContext;

/*

* ============================================================================
* PUBLIC ENTRY POINT
* ============================================================================
* 
* This is the only deployment root intended for integration by execution.g4
* and other execution-domain grammars.
* ============================================================================
  */

deploymentDeclaration
: DEPLOY deploymentSubject deploymentBody?
deploymentTerminator?
;

/*

* ============================================================================
* DEPLOYMENT SUBJECT
* ============================================================================
* 
* The subject is intentionally an expression.
* 
* It can therefore represent:
* 
* a function;
* a program;
* a module entry;
* a pipeline;
* a circuit;
* a classical computation;
* a quantum computation;
* a hybrid computation;
* a hardware/software computation;
* a future computation kind.
* 
* No closed list is required.
* ============================================================================
  */

deploymentSubject
: expression
;

/*

* ============================================================================
* DEPLOYMENT BODY
* ============================================================================
* 
* The body is a deployment-specific semantic object.
* ============================================================================
  */

deploymentBody
: LBRACE deploymentClause+ RBRACE
;

/*

* ============================================================================
* DEPLOYMENT CLAUSES
* ============================================================================
  */

deploymentClause
: deploymentArtifact
| deploymentEnvironment
| deploymentRequirement
| deploymentConstraint
| deploymentPreference
| deploymentHint
| deploymentCapability
| deploymentResource
| deploymentTarget
| deploymentPlacement
| deploymentPolicy
| deploymentLifecycle
| deploymentRollout
| deploymentAvailability
| deploymentPortability
| deploymentRecovery
| deploymentObservability
| deploymentParameter
| deploymentProperty
;

/*

* ============================================================================
* ARTIFACT
* ============================================================================
* 
* An artifact is a semantic reference to something deployable.
* 
* This grammar does not define artifact storage, packaging, hashing,
* downloading, signing, or verification.
* ============================================================================
  */

deploymentArtifact
: deploymentArtifactKey deploymentValue deploymentTerminator?
;

deploymentArtifactKey
: qualifiedName
;

/*

* ============================================================================
* ENVIRONMENT
* ============================================================================
* 
* Environment is intentionally abstract.
* 
* It can describe:
* 
* local;
* remote;
* embedded;
* distributed;
* cloud;
* edge;
* quantum;
* heterogeneous;
* future execution environments.
* 
* No environment is enumerated by this grammar.
* ============================================================================
  */

deploymentEnvironment
: deploymentEnvironmentKey deploymentValue deploymentTerminator?
;

deploymentEnvironmentKey
: qualifiedName
;

/*

* ============================================================================
* REQUIREMENT
* ============================================================================
* 
* A requirement is mandatory.
* 
* Example semantic forms:
* 
* requires quantum;
* requires capability;
* requires environment;
* requires signed_artifact;
* 
* The semantic layer determines whether it is satisfiable.
* ============================================================================
  */

deploymentRequirement
: REQUIRES deploymentValue deploymentTerminator?
;

/*

* ============================================================================
* CONSTRAINT
* ============================================================================
* 
* Constraints restrict acceptable deployment realizations.
* 
* The expression remains open-ended.
* ============================================================================
  */

deploymentConstraint
: deploymentConstraintKey
deploymentConstraintOperator
expression
deploymentTerminator?
;

deploymentConstraintKey
: qualifiedName
;

deploymentConstraintOperator
: EQUAL_EQUAL
| NOT_EQUAL
| LESS_THAN
| LESS_EQUAL
| GREATER_THAN
| GREATER_EQUAL
;

/*

* ============================================================================
* PREFERENCE
* ============================================================================
* 
* Preferences are advisory.
* 
* Failure to satisfy a preference MUST NOT automatically imply semantic
* deployment failure.
* ============================================================================
  */

deploymentPreference
: deploymentPreferenceKey
deploymentValue
deploymentTerminator?
;

deploymentPreferenceKey
: qualifiedName
;

/*

* ============================================================================
* HINT
* ============================================================================
* 
* Hints provide advisory information to downstream implementation layers.
* 
* A hint MUST NOT silently become a semantic requirement.
* ============================================================================
  */

deploymentHint
: deploymentHintKey
deploymentValue
deploymentTerminator?
;

deploymentHintKey
: qualifiedName
;

/*

* ============================================================================
* CAPABILITY
* ============================================================================
* 
* Capability names are open-ended.
* 
* Examples:
* 
* quantum
* distributed
* realtime
* secure_execution
* tensor_compute
* custom.future.capability
* 
* The grammar does not enumerate them.
* ============================================================================
  */

deploymentCapability
: deploymentCapabilityKey
deploymentValue
deploymentTerminator?
;

deploymentCapabilityKey
: qualifiedName
;

/*

* ============================================================================
* RESOURCE
* ============================================================================
* 
* Resource requirements remain symbolic.
* 
* Examples:
* 
* memory: required_memory;
* quantum: required_quantum_resources;
* processing: required_processing;
* 
* A numeric quantity is an expression, not a grammar-level machine limit.
* ============================================================================
  */

deploymentResource
: deploymentResourceKey
deploymentValue
deploymentTerminator?
;

deploymentResourceKey
: qualifiedName
;

/*

* ============================================================================
* TARGET
* ============================================================================
* 
* Target identifies an abstract target expression.
* 
* It does not necessarily identify a physical device.
* ============================================================================
  */

deploymentTarget
: deploymentTargetKey
deploymentValue
deploymentTerminator?
;

deploymentTargetKey
: qualifiedName
;

/*

* ============================================================================
* PLACEMENT
* ============================================================================
* 
* Placement is expressed as intent.
* 
* Actual placement belongs to placement/routing/resource infrastructure.
* ============================================================================
  */

deploymentPlacement
: deploymentPlacementKey
deploymentValue
deploymentTerminator?
;

deploymentPlacementKey
: qualifiedName
;

/*

* ============================================================================
* POLICY
* ============================================================================
* 
* Policies are named semantic policies.
* 
* The grammar does not enumerate provider or deployment-engine policies.
* ============================================================================
  */

deploymentPolicy
: deploymentPolicyKey
deploymentPolicyValue
deploymentTerminator?
;

deploymentPolicyKey
: qualifiedName
;

deploymentPolicyValue
: expression
| deploymentPropertyBlock
;

/*

* ============================================================================
* LIFECYCLE
* ============================================================================
* 
* Lifecycle is intentionally open-ended.
* 
* Examples of semantic values include:
* 
* start
* stop
* pause
* resume
* suspend
* retire
* replace
* 
* These are data/semantic names, not a closed machine-specific enumeration.
* ============================================================================
  */

deploymentLifecycle
: deploymentLifecycleKey
deploymentValue
deploymentTerminator?
;

deploymentLifecycleKey
: qualifiedName
;

/*

* ============================================================================
* ROLLOUT
* ============================================================================
* 
* Rollout describes desired deployment transition behavior.
* 
* It does not implement rolling updates, blue/green deployment, canary
* execution, orchestration, or traffic management.
* 
* Such strategies remain downstream policy/runtime concerns.
* ============================================================================
  */

deploymentRollout
: deploymentRolloutKey
deploymentValue
deploymentTerminator?
;

deploymentRolloutKey
: qualifiedName
;

/*

* ============================================================================
* AVAILABILITY
* ============================================================================
* 
* Availability intent is open-ended.
* 
* It can express semantic availability requirements without defining a
* particular infrastructure mechanism.
* ============================================================================
  */

deploymentAvailability
: deploymentAvailabilityKey
deploymentValue
deploymentTerminator?
;

deploymentAvailabilityKey
: qualifiedName
;

/*

* ============================================================================
* PORTABILITY
* ============================================================================
* 
* Portability expresses deployment portability intent.
* 
* It does not itself guarantee that all targets can execute the program.
* Capability analysis and target resolution establish actual portability.
* ============================================================================
  */

deploymentPortability
: deploymentPortabilityKey
deploymentValue
deploymentTerminator?
;

deploymentPortabilityKey
: qualifiedName
;

/*

* ============================================================================
* RECOVERY
* ============================================================================
* 
* Recovery intent is declarative.
* 
* Actual failure diagnosis, recovery algorithms, retries, rollback, checkpoint
* reconstruction, backend switching, and resilience decisions belong to the
* resilience/runtime layers.
* ============================================================================
  */

deploymentRecovery
: deploymentRecoveryKey
deploymentValue
deploymentTerminator?
;

deploymentRecoveryKey
: qualifiedName
;

/*

* ============================================================================
* OBSERVABILITY
* ============================================================================
* 
* Observability describes desired deployment telemetry/visibility intent.
* 
* It does not implement telemetry.
* ============================================================================
  */

deploymentObservability
: deploymentObservabilityKey
deploymentValue
deploymentTerminator?
;

deploymentObservabilityKey
: qualifiedName
;

/*

* ============================================================================
* PARAMETERS
* ============================================================================
* 
* Deployment parameters are semantic values.
* 
* They are not fixed infrastructure parameters.
* ============================================================================
  */

deploymentParameter
: deploymentParameterKey
deploymentParameterOperator
deploymentValue
deploymentTerminator?
;

deploymentParameterKey
: qualifiedName
;

deploymentParameterOperator
: COLON
| ASSIGN
;

/*

* ============================================================================
* OPEN DEPLOYMENT PROPERTY
* ============================================================================
* 
* Future deployment concepts can be represented without modifying this
* grammar.
* 
* Example:
* 
* future::deployment_mode: future_expression;
* 
* This is essential for POCO-REAF and long-term language evolution.
* ============================================================================
  */

deploymentProperty
: deploymentPropertyKey
COLON
deploymentValue
deploymentTerminator?
;

deploymentPropertyKey
: qualifiedName
;

/*

* ============================================================================
* GENERIC DEPLOYMENT VALUE
* ============================================================================
* 
* Values reuse the canonical expression language.
* 
* Structured objects are provided for deployment-specific metadata.
* ============================================================================
  */

deploymentValue
: expression
| deploymentPropertyBlock
| executionContextValue
;

/*

* ============================================================================
* DEPLOYMENT PROPERTY BLOCK
* ============================================================================
* 
* Nested deployment metadata remains dynamically extensible.
* ============================================================================
  */

deploymentPropertyBlock
: LBRACE deploymentPropertyEntry+ RBRACE
;

deploymentPropertyEntry
: deploymentPropertyKey
deploymentPropertyAssignment
deploymentValue
deploymentTerminator?
;

deploymentPropertyAssignment
: COLON
| ASSIGN
;

/*

* ============================================================================
* DEPLOYMENT LIST
* ============================================================================
* 
* This rule is provided for deployment extensions requiring explicit
* collections.
* 
* There is deliberately no finite collection size.
* ============================================================================
  */

deploymentList
: LBRACKET deploymentListElements? RBRACKET
;

deploymentListElements
: deploymentValue
(COMMA deploymentValue)*
COMMA?
;

/*

* ============================================================================
* DEPLOYMENT INVOCATION
* ============================================================================
* 
* Named deployment policies/extensions may receive arguments.
* 
* Ordinary function calls remain owned by Core.
* ============================================================================
  */

deploymentInvocation
: qualifiedName
LPAREN
deploymentArgumentList?
RPAREN
;

deploymentArgumentList
: deploymentArgument
(COMMA deploymentArgument)*
COMMA?
;

deploymentArgument
: expression
| qualifiedName
COLON
expression
;

/*

* ============================================================================
* DEPLOYMENT TERMINATOR
* ============================================================================
* 
* Deployment statements use the canonical semicolon token.
* ============================================================================
  */

deploymentTerminator
: SEMICOLON
;

/*

* ============================================================================
* EXECUTION-CONTEXT ADAPTER
* ============================================================================
* 
* This adapter gives execution.g4 and other execution grammars a stable
* delegation point without importing or redefining deployment syntax.
* ============================================================================
  */

executionDeployment
: deploymentDeclaration
;
/*

* ============================================================================
* END
* ============================================================================
  */
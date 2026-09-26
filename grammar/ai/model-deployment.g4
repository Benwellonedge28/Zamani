/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/ai/model-deployment.g4
* 
* Grammar:
* ModelDeployment
* 
* Status:
* CANONICAL AI MODEL-DEPLOYMENT LEAF GRAMMAR
* 
* Baseline:
* Rust 1.97 / Rust 1.97.1
* Rust edition 2021
* Safe Rust only
* No unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file defines the SOURCE-LEVEL SYNTAX for deployment intent of AI/ML
* models and model-derived computations.
* 
* Deployment syntax describes:
* 
* - what model is to be deployed;
* - what deployment interface is exposed;
* - what inputs and outputs are exposed;
* - what execution mode is requested;
* - what scaling policy is desired;
* - what resource requirements exist;
* - what capabilities are required;
* - what constraints apply;
* - what preferences are expressed;
* - what rollout/lifecycle intent exists;
* - what resilience/availability properties are required;
* - what security policy references apply;
* - what observability intent exists;
* - what interoperability boundary is required.
* 
* This file does NOT implement:
* 
* - model execution;
* - model serving;
* - process creation;
* - container creation;
* - VM creation;
* - device discovery;
* - accelerator discovery;
* - hardware allocation;
* - resource scheduling;
* - network provisioning;
* - load balancing;
* - autoscaling algorithms;
* - placement algorithms;
* - orchestration;
* - security enforcement;
* - authentication;
* - authorization;
* - model serialization;
* - model storage;
* - runtime execution;
* - compiler backend selection.
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
* ZamaniParser
*      |
*      v
* AI composition
*      |
*      v
* ModelDeployment
*      |
*      v
* domain-neutral AST
*      |
*      v
* structural validation
*      |
*      v
* semantic analysis
*      |
*      +-----------------------------+
*      |                             |
*      v                             v
* AI/model semantics       resource/capability semantics
*      |                             |
*      +-------------+---------------+
*                    |
*                    v
*          canonical semantic model
*                    |
*      +-------------+-------------+
*      |             |             |
*      v             v             v
*  Classical      quantum::ir   HDL/Hardware
*      |             |             |
*      +-------------+-------------+
*                    |
*                    v
*               compilation
*                    |
*              optimization
*                    |
*         routing / scheduling
*                    |
*             deployment plan
*                    |
*                   HAL
*                    |
*                  runtime
* 
* Deployment is therefore a SOURCE-LEVEL INTENT layer.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - deployment declarations;
* - deployment invocations;
* - deployment members;
* - deployment endpoints;
* - deployment inputs/outputs;
* - deployment execution intent;
* - deployment scaling intent;
* - deployment rollout intent;
* - deployment lifecycle intent;
* - deployment resilience intent;
* - deployment observability intent;
* - deployment resource/capability contracts;
* - deployment security references;
* - deployment interoperability boundaries;
* - deployment-local extension points.
* 
* THIS FILE DOES NOT OWN:
* 
* - identifiers;
* - qualified-name lexical rules;
* - literals;
* - general expressions;
* - general types;
* - statements;
* - model declaration structure;
* - tensor declaration structure;
* - training;
* - inference;
* - accelerator implementation;
* - hardware;
* - networking implementation;
* - distributed scheduling;
* - resource discovery;
* - physical placement;
* - compiler IR;
* - runtime behavior.
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* General types are owned by:
* 
* Types
* 
* General expressions are owned by:
* 
* Expressions
* 
* General statements are owned by:
* 
* Statements
* 
* Model declarations are owned by:
* 
* AIModels / models.g4
* 
* Inference semantics are owned by:
* 
* Inference / inference.g4
* 
* Resource semantics are owned by:
* 
* Resources
* 
* Hardware intent is owned by:
* 
* Hardware
* 
* Networking semantics are owned by:
* 
* Networking
* 
* Security semantics are owned by:
* 
* Security
* 
* Compilation semantics are owned by:
* 
* Compile
* 
* Execution semantics are owned by:
* 
* Execution
* 
* This file references those domains through source-level expressions,
* declarations, identifiers, and semantic contracts. It does not duplicate
* their implementations.
* 
* ============================================================================
* LEXICAL POLICY
* ============================================================================
* 
* This grammar intentionally does NOT introduce deployment-specific lexer
* keywords.
* 
* Deployment roles use the existing:
* 
* AT
* identifier
* 
* mechanism.
* 
* Examples:
* 
* @deployment
* @deploy
* @endpoint
* @scale
* @replicas
* @resources
* @capability
* @constraint
* @prefer
* @rollout
* @security
* 
* The lexical representation is deliberately open.
* 
* Semantic analysis determines whether an annotation name is registered,
* legal, deprecated, experimental, or supplied by a dialect.
* 
* This prevents deployment syntax from becoming permanently coupled to a
* closed vocabulary of today's orchestration systems.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* This grammar imposes NO universal limits on:
* 
* models
* deployments
* endpoints
* inputs
* outputs
* replicas
* workers
* instances
* nodes
* regions
* zones
* devices
* accelerators
* CPUs
* GPUs
* FPGAs
* QPUs
* memory
* storage
* tensor dimensions
* tensor rank
* request count
* throughput
* concurrency
* pipeline stages
* rollout stages
* 
* It MUST NOT contain:
* 
* MAX_REPLICAS
* MAX_DEPLOYMENTS
* MAX_ENDPOINTS
* MAX_WORKERS
* MAX_NODES
* MAX_DEVICES
* MAX_GPUS
* MAX_CPUS
* MAX_MEMORY
* MAX_STORAGE
* 
* or equivalent language-level ceilings.
* 
* Repetition is therefore structural:
* 
* *
* +
* 
* where appropriate.
* 
* Actual limits belong to:
* 
* - compiler resources;
* - runtime resources;
* - deployment environment;
* - target capabilities;
* - resource policy;
* - explicit program constraints.
* 
* ============================================================================
* RESOURCE / CAPABILITY SEPARATION
* ============================================================================
* 
* Deployment syntax distinguishes:
* 
* requirement
* capability
* constraint
* preference
* hint
* realization
* 
* Requirement:
* 
* requires ...
* 
* Capability:
* 
* capability ...
* 
* Constraint:
* 
* constraint ...
* 
* Preference:
* 
* prefer ...
* 
* Hint:
* 
* hint ...
* 
* None of these selects a physical machine.
* 
* For example, a deployment may require:
* 
* capability("tensor.compute")
* 
* without selecting:
* 
* GPU 0
* 
* A deployment may require:
* 
* memory >= required_memory
* 
* without assuming:
* 
* RAM = 64GB
* 
* A deployment may express:
* 
* replicas >= desired_replicas
* 
* without creating a language-level maximum or choosing physical nodes.
* 
* ============================================================================
* DEPLOYMENT IS NOT PLACEMENT
* ============================================================================
* 
* A deployment describes an execution/service intent.
* 
* It does NOT automatically describe:
* 
* physical machine;
* physical CPU;
* physical GPU;
* physical FPGA;
* physical QPU;
* physical memory bank;
* physical network interface;
* cloud instance;
* rack;
* host identifier.
* 
* Physical realization is determined downstream.
* 
* ============================================================================
* TARGET INDEPENDENCE
* ============================================================================
* 
* A deployment may ultimately be realized on:
* 
* embedded hardware
* CPU
* multicore CPU
* GPU
* FPGA
* ASIC
* NPU
* TPU
* accelerator
* QPU-assisted systems
* distributed systems
* heterogeneous systems
* edge systems
* cloud systems
* HPC systems
* future execution systems
* 
* The grammar does not need to change merely because a new target appears.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* A deployed model may consume or produce values associated with quantum
* computation.
* 
* This grammar MUST NOT define:
* 
* - qubits;
* - quantum gates;
* - physical qubits;
* - topology;
* - routing;
* - QEC;
* - ZQN;
* - quantum scheduling.
* 
* Quantum semantics remain owned by the quantum subsystem and ultimately:
* 
* quantum::ir
* 
* If deployment requires quantum capabilities, they are represented through
* the general capability/resource boundary.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Deployment may refer semantically to hardware capabilities.
* 
* It MUST NOT define:
* 
* - register widths;
* - bus widths;
* - FPGA resource counts;
* - memory-bank counts;
* - accelerator counts;
* - physical wiring;
* - clock topology;
* - chip identifiers.
* 
* Those belong downstream to hardware/HDL/resource/compiler/runtime systems.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Deployment may express:
* 
* - replication intent;
* - distribution intent;
* - partitioning intent;
* - availability intent;
* - communication requirements;
* - scaling intent.
* 
* It does not implement:
* 
* - consensus;
* - scheduling;
* - service discovery;
* - load balancing;
* - node allocation;
* - network provisioning.
* 
* ============================================================================
* SECURITY INTEGRATION
* ============================================================================
* 
* Deployment may reference security policies and capabilities.
* 
* It does not perform:
* 
* authentication;
* authorization;
* key management;
* secret storage;
* cryptographic operations.
* 
* Security enforcement remains owned by the security/runtime layers.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* Every deployment construct MUST preserve sufficient structure for downstream
* semantic analysis.
* 
* At minimum the AST representation must preserve:
* 
* - source span;
* - deployment annotation;
* - deployment name;
* - model expression/reference;
* - optional type;
* - member order;
* - endpoint declarations;
* - input declarations;
* - output declarations;
* - scaling expressions;
* - replica expressions;
* - resource expressions;
* - capability expressions;
* - constraint expressions;
* - preference expressions;
* - hint expressions;
* - rollout members;
* - lifecycle members;
* - security references;
* - observability references;
* - interoperability references;
* - extension directives;
* - nested regions;
* - source provenance.
* 
* The AST remains domain-neutral.
* 
* This file does not require a deployment-specific IR.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis is responsible for:
* 
* - validating deployment annotation roles;
* - resolving model references;
* - checking model availability;
* - checking input/output compatibility;
* - resolving types;
* - resolving expressions;
* - checking deployment contracts;
* - validating scaling semantics;
* - validating replica semantics;
* - validating resource requirements;
* - validating capabilities;
* - validating constraints;
* - preserving preference/hint weakness;
* - checking security references;
* - checking interoperability;
* - checking portability;
* - checking determinism;
* - checking lifecycle legality;
* - checking rollout legality;
* - checking target feasibility;
* - checking runtime feasibility.
* 
* Parser-level acceptance MUST NOT be confused with deployment feasibility.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar produces NO IR.
* 
* Deployment syntax lowers through the existing domain-neutral semantic model.
* 
* The implementation MUST NOT create:
* 
* AIDeploymentIR
* 
* merely because this grammar exists.
* 
* If the compiler requires an internal deployment representation, it belongs
* outside grammar/ and must be defined by the compiler/semantic architecture.
* 
* If deployment interacts with quantum computation:
* 
* deployment
*      |
*      v
* semantic analysis
*      |
*      v
* quantum semantics
*      |
*      v
* quantum::ir
* 
* There MUST NOT be a second quantum IR owned by deployment.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing is deterministic for a fixed:
* 
* source
* lexer
* grammar
* parser configuration
* 
* Deployment discovery, resource discovery, runtime state, target state,
* network state, randomness, autoscaling and orchestration do not participate
* in parsing.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* This file contains:
* 
* - no embedded Rust;
* - no semantic predicates;
* - no target-language actions;
* - no filesystem access;
* - no network access;
* - no hardware discovery;
* - no runtime execution;
* - no randomness;
* - no unsafe implementation.
* 
* The Rust implementation consuming this grammar must remain:
* 
* Rust 2021
* Rust 1.97 / Rust 1.97.1
* safe Rust
* 
* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* Canonical lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Canonical parser composition:
* 
* grammar/antlr/ZamaniParser.g4
* 
* AI composition:
* 
* grammar/ai/ai.g4
* 
* Existing model grammar:
* 
* grammar/ai/models.g4
* 
* Existing inference grammar:
* 
* grammar/ai/inference.g4
* 
* Existing accelerator grammar:
* 
* grammar/ai/ai-accelerators.g4
* 
* Resource grammar:
* 
* grammar/resources/
* 
* Hardware grammar:
* 
* grammar/hardware/
* 
* Compilation grammar:
* 
* grammar/compile/
* 
* Execution grammar:
* 
* grammar/execution/
* 
* Networking grammar:
* 
* grammar/networking/
* 
* Security grammar:
* 
* grammar/security/
* 
* This file is a LEAF grammar.
* 
* It must NOT become another composition root.
* 
* ============================================================================
  */

parser grammar ModelDeployment;

options {
tokenVocab = ZamaniLexer;
}

import Types,
Expressions,
Statements;

/* ============================================================================

* 1. PUBLIC ENTRY POINT
* ========================================================================== */

/**

* Complete model-deployment construct.
* 
* Deployment source crosses an explicit annotation boundary.
* 
* Ordinary expressions are not automatically classified as deployments.
  */
  deploymentConstruct
  : deploymentDeclaration
  | deploymentInvocation
  ;

/* ============================================================================

* 2. DEPLOYMENT DECLARATION
* ========================================================================== */

/**

* Canonical deployment declaration.
* 
* Examples:
* 
* @deployment serving {
*     @model classifier;
* }
* 
* @deploy classifier {
*     @endpoint predict;
* }
* 
* The annotation text is intentionally open and is validated semantically.
  */
  deploymentDeclaration
  : AT
  deploymentAnnotation
  identifier
  deploymentType?
  deploymentInitializer?
  deploymentBody
  ;

deploymentAnnotation
: identifier
;

deploymentType
: COLON
typeExpression
;

deploymentInitializer
: ASSIGN
expression
;

deploymentBody
: LBRACE
deploymentMember*
RBRACE
;

/* ============================================================================

* 3. DEPLOYMENT INVOCATION
* ========================================================================== */

/**

* Deployment operation/invocation.
* 
* Examples:
* 
* @deploy(classifier);
* 
* @deploy classifier(input);
* 
* @deploy pipeline;
* 
* The semantic layer determines whether the invocation represents creation,
* update, promotion, execution, or another registered deployment operation.
  */
  deploymentInvocation
  : AT
  deploymentInvocationAnnotation
  deploymentInvocationTarget?
  deploymentInvocationArguments?
  SEMI?
  ;

deploymentInvocationAnnotation
: identifier
;

deploymentInvocationTarget
: expression
;

deploymentInvocationArguments
: LPAREN
argumentList?
RPAREN
;

/* ============================================================================

* 4. DEPLOYMENT MEMBERS
* ========================================================================== */

/**

* Deployment members are deliberately separated by structural role.
* 
* Generic extension directives remain available without requiring a lexer
* change for every future deployment feature.
  */
  deploymentMember
  : deploymentModelBinding
  | deploymentEndpoint
  | deploymentInput
  | deploymentOutput
  | deploymentExecution
  | deploymentScaling
  | deploymentReplication
  | deploymentDistribution
  | deploymentRollout
  | deploymentLifecycle
  | deploymentResourceContract
  | deploymentCapabilityContract
  | deploymentConstraintContract
  | deploymentPreferenceContract
  | deploymentHintContract
  | deploymentSecurity
  | deploymentObservability
  | deploymentInteroperability
  | deploymentConfiguration
  | deploymentExtension
  | statement
  ;

/* ============================================================================

* 5. MODEL BINDING
* ========================================================================== */

/**

* Associates deployment intent with a model expression/reference.
* 
* Examples:
* 
* @model classifier;
* 
* @model = classifier;
* 
* @model selected_model;
* 
* Model resolution belongs to semantic analysis.
  */
  deploymentModelBinding
  : AT
  deploymentModelAnnotation
  deploymentBindingPayload?
  SEMI?
  ;

deploymentModelAnnotation
: identifier
;

deploymentBindingPayload
: identifier
| ASSIGN
expression
| expression
;

/* ============================================================================

* 6. ENDPOINT
* ========================================================================== */

/**

* Defines a logical deployment endpoint.
* 
* It does not define:
* 
* TCP port;
* IP address;
* physical network interface;
* host;
* load balancer;
* service implementation.
* 
* Those are downstream concerns.
  */
  deploymentEndpoint
  : AT
  deploymentEndpointAnnotation
  identifier?
  deploymentEndpointPayload?
  SEMI?
  ;

deploymentEndpointAnnotation
: identifier
;

deploymentEndpointPayload
: deploymentDirectiveCall
| deploymentBinding
| deploymentTypedBinding
| deploymentAssignment
| deploymentTarget
| deploymentRegion
;

/* ============================================================================

* 7. INPUT
* ========================================================================== */

deploymentInput
: AT
deploymentInputAnnotation
identifier
deploymentTypedDeclaration?
deploymentDefaultValue?
SEMI?
;

deploymentInputAnnotation
: identifier
;

deploymentTypedDeclaration
: COLON
typeExpression
;

deploymentDefaultValue
: ASSIGN
expression
;

/* ============================================================================

* 8. OUTPUT
* ========================================================================== */

deploymentOutput
: AT
deploymentOutputAnnotation
identifier
deploymentTypedDeclaration?
deploymentDefaultValue?
SEMI?
;

deploymentOutputAnnotation
: identifier
;

/* ============================================================================

* 9. EXECUTION INTENT
* ========================================================================== */

/**

* Execution intent may express:
* 
* online
* offline
* streaming
* batch
* interactive
* asynchronous
* synchronous
* adaptive
* 
* These are semantic roles rather than a closed enumeration.
  */
  deploymentExecution
  : AT
  deploymentExecutionAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentExecutionAnnotation
: identifier
;

deploymentExecutionPayload
: deploymentDirectivePayload
;

/* ============================================================================

* 10. SCALING
* ========================================================================== */

/**

* Scaling syntax is expression-based.
* 
* This supports:
* 
* workload-dependent scaling;
* symbolic scaling;
* runtime-derived scaling;
* policy-driven scaling.
* 
* No maximum or minimum machine size is embedded in the grammar.
  */
  deploymentScaling
  : AT
  deploymentScalingAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentScalingAnnotation
: identifier
;

/* ============================================================================

* 11. REPLICATION
* ========================================================================== */

deploymentReplication
: AT
deploymentReplicationAnnotation
deploymentDirectivePayload?
SEMI?
;

deploymentReplicationAnnotation
: identifier
;

/* ============================================================================

* 12. DISTRIBUTION
* ========================================================================== */

/**

* Distribution expresses intent, not physical placement.
  */
  deploymentDistribution
  : AT
  deploymentDistributionAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentDistributionAnnotation
: identifier
;

/* ============================================================================

* 13. ROLLOUT
* ========================================================================== */

/**

* Rollout syntax represents deployment lifecycle intent.
* 
* It does not implement:
* 
* canary scheduling;
* blue/green orchestration;
* traffic shifting;
* cluster management.

*/
deploymentRollout
: AT
deploymentRolloutAnnotation
deploymentDirectivePayload?
SEMI?
;

deploymentRolloutAnnotation
: identifier
;

/* ============================================================================

* 14. LIFECYCLE
* ========================================================================== */

deploymentLifecycle
: AT
deploymentLifecycleAnnotation
deploymentDirectivePayload?
SEMI?
;

deploymentLifecycleAnnotation
: identifier
;

/* ============================================================================

* 15. RESOURCE CONTRACT
* ========================================================================== */

/**

* Mandatory resource requirement.
* 
* Examples:
* 
* requires memory >= required_memory;
* requires throughput >= required_throughput;
* requires availability >= required_availability;
* 
* The expression is evaluated semantically.
  */
  deploymentResourceContract
  : REQUIRES
  expression
  SEMI?
  ;

/* ============================================================================

* 16. CAPABILITY CONTRACT
* ========================================================================== */

/**

* Capability requirement.
* 
* Examples:
* 
* capability("tensor.compute");
* capability("streaming");
* capability("quantum.measurement");
* 
* Capability names are open semantic values.
  */
  deploymentCapabilityContract
  : CAPABILITY
  expression
  SEMI?
  ;

/* ============================================================================

* 17. CONSTRAINT CONTRACT
* ========================================================================== */

deploymentConstraintContract
: CONSTRAINT
expression
SEMI?
;

/* ============================================================================

* 18. PREFERENCE CONTRACT
* ========================================================================== */

/**

* Preferences are non-binding optimization guidance.
* 
* A preference MUST NOT silently become a requirement.
  */
  deploymentPreferenceContract
  : PREFER
  expression
  SEMI?
  ;

/* ============================================================================

* 19. HINT CONTRACT
* ========================================================================== */

/**

* Hints are advisory.
* 
* An implementation may ignore a hint without changing required program
* semantics.
  */
  deploymentHintContract
  : HINT
  expression
  SEMI?
  ;

/* ============================================================================

* 20. SECURITY
* ========================================================================== */

/**

* Security integration is intentionally reference-oriented.
* 
* The grammar does not implement authentication or authorization.
  */
  deploymentSecurity
  : AT
  deploymentSecurityAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentSecurityAnnotation
: identifier
;

/* ============================================================================

* 21. OBSERVABILITY
* ========================================================================== */

deploymentObservability
: AT
deploymentObservabilityAnnotation
deploymentDirectivePayload?
SEMI?
;

deploymentObservabilityAnnotation
: identifier
;

/* ============================================================================

* 22. INTEROPERABILITY
* ========================================================================== */

/**

* Deployment may expose or consume external interfaces.
* 
* Actual FFI/network/serialization behavior belongs to interoperability and
* networking subsystems.
  */
  deploymentInteroperability
  : AT
  deploymentInteroperabilityAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentInteroperabilityAnnotation
: identifier
;

/* ============================================================================

* 23. CONFIGURATION
* ========================================================================== */

/**

* Configuration is expression-based.
* 
* This prevents a deployment-specific configuration language from becoming a
* second general-purpose expression language.
  */
  deploymentConfiguration
  : AT
  deploymentConfigurationAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentConfigurationAnnotation
: identifier
;

/* ============================================================================

* 24. EXTENSION / DIALECT BOUNDARY
* ========================================================================== */

/**

* Future deployment dialects may add directives without modifying the lexer.
* 
* Semantic/dialect validation determines whether an extension is legal.
  */
  deploymentExtension
  : AT
  deploymentExtensionAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentExtensionAnnotation
: identifier
;

/* ============================================================================

* 25. SHARED DEPLOYMENT DIRECTIVE PAYLOAD
* ========================================================================== */

/**

* Shared payload grammar.
* 
* The payload deliberately reuses canonical expressions and types.
* 
* It supports:
* 
* @scale(workload)
* @scale min = lower_bound
* @endpoint predict
* @resources memory >= required_memory
* @config value = expression
* @policy(policy_reference)
* 
* Semantic interpretation belongs downstream.
  */
  deploymentDirectivePayload
  : deploymentDirectiveCall
  | deploymentBinding
  | deploymentTypedBinding
  | deploymentAssignment
  | deploymentTarget
  | deploymentRegion
  ;

deploymentDirectiveCall
: LPAREN
argumentList?
RPAREN
;

deploymentBinding
: identifier
deploymentDirectiveCall?
;

deploymentTypedBinding
: identifier
COLON
typeExpression
deploymentDirectiveCall?
;

deploymentAssignment
: identifier?
ASSIGN
expression
;

deploymentTarget
: expression
;

deploymentRegion
: LBRACE
deploymentMember*
RBRACE
;

/* ============================================================================

* 26. EXPLICIT DEPLOYMENT ROLE ANCHORS
* ========================================================================== */

/**

* These stable parser rules provide tooling/AST anchors without introducing
* dedicated lexer keywords.
* 
* They are intentionally structural rather than semantic predicates.
  */

deploymentEndpointDirective
: AT
deploymentEndpointDirectiveName
deploymentDirectivePayload?
SEMI?
;

deploymentEndpointDirectiveName
: identifier
;

deploymentModelDirective
: AT
deploymentModelDirectiveName
deploymentDirectivePayload?
SEMI?
;

deploymentModelDirectiveName
: identifier
;

deploymentInputDirective
: AT
deploymentInputDirectiveName
deploymentDirectivePayload?
SEMI?
;

deploymentInputDirectiveName
: identifier
;

deploymentOutputDirective
: AT
deploymentOutputDirectiveName
deploymentDirectivePayload?
SEMI?
;

deploymentOutputDirectiveName
: identifier
;

/* ============================================================================

* 27. SCALING POLICY ANCHORS
* ========================================================================== */

/**

* Scaling policy names remain open semantic identifiers.
* 
* Examples:
* 
* @autoscale
* @elastic
* @adaptive
* @scale
* @scale_to_workload
* 
* The parser does not enumerate them.
  */
  deploymentScalePolicy
  : AT
  deploymentScalePolicyName
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentScalePolicyName
: identifier
;

/* ============================================================================

* 28. AVAILABILITY / RESILIENCE
* ========================================================================== */

/**

* Availability and resilience are expressed as semantic contracts.
* 
* The parser does not implement failure detection or recovery.
  */
  deploymentResilience
  : AT
  deploymentResilienceName
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentResilienceName
: identifier
;

/* ============================================================================

* 29. STATE / VERSION
* ========================================================================== */

/**

* Deployment versioning is a semantic reference.
* 
* It does not define package-version semantics.
* 
* Module/package versioning remains owned by modules/compatibility.
  */
  deploymentVersion
  : AT
  deploymentVersionName
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentVersionName
: identifier
;

/* ============================================================================

* 30. DATA / MODEL ARTIFACT REFERENCE
* ========================================================================== */

/**

* Deployment may refer to a model artifact through an ordinary expression.
* 
* Artifact resolution belongs to model/module/interoperability/deployment
* semantics.
  */
  deploymentArtifact
  : AT
  deploymentArtifactName
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentArtifactName
: identifier
;

/* ============================================================================

* 31. QUANTUM CAPABILITY BRIDGE
* ========================================================================== */

/**

* This is intentionally only a syntactic capability boundary.
* 
* Examples:
* 
* @capability("quantum.measurement");
* @capability("quantum.mid_circuit_measurement");
* 
* No quantum operation or QEC syntax is introduced here.
  */
  deploymentQuantumCapability
  : AT
  deploymentQuantumCapabilityName
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentQuantumCapabilityName
: identifier
;

/* ============================================================================

* 32. HARDWARE CAPABILITY BRIDGE
* ========================================================================== */

/**

* Hardware capability references remain symbolic.
* 
* Examples:
* 
* @capability("tensor.compute");
* @capability("accelerator.compute");
* @capability("hardware.reconfigurable");

*/
deploymentHardwareCapability
: AT
deploymentHardwareCapabilityName
deploymentDirectivePayload?
SEMI?
;

deploymentHardwareCapabilityName
: identifier
;

/* ============================================================================

* 33. DISTRIBUTED CAPABILITY BRIDGE
* ========================================================================== */

deploymentDistributedCapability
: AT
deploymentDistributedCapabilityName
deploymentDirectivePayload?
SEMI?
;

deploymentDistributedCapabilityName
: identifier
;

/* ============================================================================

* 34. DEPLOYMENT CONTRACT REGION
* ========================================================================== */

/**

* Explicit contract region.
* 
* This allows related requirements, capabilities, constraints, preferences
* and hints to be grouped without creating a second contract language.
  */
  deploymentContractRegion
  : AT
  deploymentContractAnnotation
  deploymentContractBody
  ;

deploymentContractAnnotation
: identifier
;

deploymentContractBody
: LBRACE
deploymentContractMember*
RBRACE
;

deploymentContractMember
: deploymentResourceContract
| deploymentCapabilityContract
| deploymentConstraintContract
| deploymentPreferenceContract
| deploymentHintContract
;

/* ============================================================================

* 35. DEPLOYMENT POLICY REGION
* ========================================================================== */

/**

* Policy regions are syntactic containers.
* 
* Their semantics are supplied by the appropriate policy subsystem.
  */
  deploymentPolicyRegion
  : AT
  deploymentPolicyAnnotation
  deploymentPolicyBody
  ;

deploymentPolicyAnnotation
: identifier
;

deploymentPolicyBody
: LBRACE
deploymentMember*
RBRACE
;

/* ============================================================================

* 36. DEPLOYMENT ENVIRONMENT REFERENCE
* ========================================================================== */

/**

* Environment is a semantic reference, not a machine identifier.
  */
  deploymentEnvironment
  : AT
  deploymentEnvironmentAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentEnvironmentAnnotation
: identifier
;

/* ============================================================================

* 37. DEPLOYMENT SERVICE / APPLICATION BOUNDARY
* ========================================================================== */

/**

* A deployment may expose a logical service/application boundary.
* 
* No network implementation is defined here.
  */
  deploymentService
  : AT
  deploymentServiceAnnotation
  identifier?
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentServiceAnnotation
: identifier
;

/* ============================================================================

* 38. DEPLOYMENT DATA BOUNDARY
* ========================================================================== */

/**

* Data references remain ordinary semantic expressions.
* 
* Dataset semantics remain owned by the AI/data subsystems.
  */
  deploymentDataBinding
  : AT
  deploymentDataAnnotation
  identifier?
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentDataAnnotation
: identifier
;

/* ============================================================================

* 39. DEPLOYMENT CHECK / VALIDATION INTENT
* ========================================================================== */

/**

* Deployment checks express validation intent.
* 
* The parser does not execute the check.
  */
  deploymentCheck
  : AT
  deploymentCheckAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentCheckAnnotation
: identifier
;

/* ============================================================================

* 40. DEPLOYMENT EXTENSION CONTAINER
* ========================================================================== */

/**

* Explicit extension container.
* 
* Future deployment dialects may place structured syntax here while preserving
* the canonical deployment boundary.
  */
  deploymentExtensionRegion
  : AT
  deploymentExtensionRegionAnnotation
  deploymentExtensionRegionBody
  ;

deploymentExtensionRegionAnnotation
: identifier
;

deploymentExtensionRegionBody
: LBRACE
deploymentMember*
RBRACE
;

/* ============================================================================

* 41. PORTABILITY ANCHOR
* ========================================================================== */

/**

* Portability declarations express source-level portability requirements.
* 
* They do not enumerate current hardware.
  */
  deploymentPortability
  : AT
  deploymentPortabilityAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentPortabilityAnnotation
: identifier
;

/* ============================================================================

* 42. DETERMINISM ANCHOR
* ========================================================================== */

/**

* Deployment determinism is semantic.
* 
* Parsing itself remains deterministic regardless of deployment policy.
  */
  deploymentDeterminism
  : AT
  deploymentDeterminismAnnotation
  deploymentDirectivePayload?
  SEMI?
  ;

deploymentDeterminismAnnotation
: identifier
;

/* ============================================================================

* 43. COMPLETION CONTRACT
* ============================================================================
* 
* This grammar is complete as an independently owned source-syntax layer when:
* 
* [x] deployment has an explicit parser boundary;
* [x] ordinary expressions remain ordinary expressions;
* [x] ordinary statements are reused;
* [x] ordinary types are reused;
* [x] model references remain semantic references;
* [x] resource contracts remain separate from capabilities;
* [x] capabilities remain open-world;
* [x] preferences remain weaker than requirements;
* [x] hints remain advisory;
* [x] deployment does not select physical devices;
* [x] deployment does not define hardware topology;
* [x] deployment does not define quantum gates;
* [x] deployment does not define QEC;
* [x] deployment does not define ZQN;
* [x] deployment does not define networking implementation;
* [x] deployment does not define security implementation;
* [x] no artificial capacity constants exist;
* [x] no fixed replica count exists;
* [x] no fixed node count exists;
* [x] no fixed accelerator count exists;
* [x] no fixed memory size exists;
* [x] no fixed tensor size exists;
* [x] no unsafe implementation exists;
* [x] no embedded target-language actions exist;
* [x] no second deployment IR is created by the grammar;
* [x] source spans can be preserved by the frontend;
* [x] dialect extension remains possible;
* [x] downstream ownership is explicit.
* 
* ============================================================================
  */
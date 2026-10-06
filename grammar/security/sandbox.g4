/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/sandbox.g4
 *
 * GRAMMAR
 * -------
 * SecuritySandbox
 *
 * STATUS
 * ------
 * CANONICAL SOURCE-LEVEL SANDBOX INTENT GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines source-level sandbox intent.
 *
 * A sandbox is a declarative security/execution boundary that constrains
 * what a computation may require, provide, perform, access, invoke, modify,
 * communicate with, or adapt to.
 *
 * This grammar does NOT implement sandbox enforcement.
 *
 * It does NOT:
 *
 *     authenticate
 *     authorize
 *     allocate resources
 *     inspect hardware
 *     inspect the filesystem
 *     inspect the network
 *     execute programs
 *     invoke FFI
 *     invoke native code
 *     evaluate policies
 *     resolve capabilities
 *     resolve resources
 *     perform reflection
 *     perform adaptation
 *     create credentials
 *     create secrets
 *     select a target
 *     select a device
 *     create an IR
 *     execute an IR
 *
 * It represents SOURCE INTENT only.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     SecuritySandbox
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     security semantic model
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 *     classical                    quantum::ir
 *       |                              |
 *       +--------------+---------------+
 *                      |
 *                      v
 *               optimization/lowering
 *                      |
 *                      v
 *              execution planning
 *                      |
 *                      v
 *              target realization
 *
 * Sandbox syntax therefore remains target-independent.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Grammar technology:
 *
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *
 *     Rust 1.97+
 *
 * Rust edition:
 *
 *     Rust 2021
 *
 * Safety:
 *
 *     This grammar contains no embedded Rust actions.
 *     Generated frontend integration must remain safe Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - sandbox statement syntax;
 *     - sandbox target syntax;
 *     - sandbox configuration syntax;
 *     - sandbox directives;
 *     - sandbox allow/forbid intent;
 *     - sandbox permission intent;
 *     - sandbox requirement intent;
 *     - sandbox contract intent;
 *     - sandbox preference intent;
 *     - sandbox fallback intent;
 *     - sandbox effect restrictions;
 *     - sandbox capability restrictions;
 *     - sandbox resource restrictions;
 *     - sandbox native/foreign restrictions;
 *     - sandbox reflection restrictions;
 *     - sandbox adaptation restrictions;
 *     - sandbox network restrictions;
 *     - sandbox property metadata;
 *     - sandbox provenance metadata;
 *     - sandbox audit/trace intent;
 *     - nested sandbox configuration.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - generic capabilities;
 *     - resource requirements;
 *     - effect semantics;
 *     - authorization;
 *     - identity;
 *     - trust;
 *     - credentials;
 *     - cryptography;
 *     - policy evaluation;
 *     - runtime enforcement;
 *     - hardware selection;
 *     - resource allocation;
 *     - execution scheduling;
 *     - quantum operations;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend selection.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/*
 *     grammar/types/*
 *     grammar/expressions/*
 *
 * Canonical imported grammar owners:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * EXPORTS:
 *
 *     sandboxStatement
 *     sandboxTarget
 *     sandboxBody
 *     sandboxMember
 *     sandboxDirective
 *     sandboxSubject
 *     sandboxRequirement
 *     sandboxContract
 *     sandboxProperty
 *
 * CONSUMED_BY:
 *
 *     grammar/statements/statements.g4
 *
 * AST_OWNER:
 *
 *     repository frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     repository security/policy semantic analysis.
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/*
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/core/capabilities.g4
 *     grammar/security/capabilities.g4
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/*
 *
 * POLICY_OWNER:
 *
 *     grammar/security/policy.g4
 *     grammar/policies/*
 *
 * PROVENANCE_OWNER:
 *
 *     grammar/security/provenance.g4
 *     grammar/spec/provenance.md
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR.
 *
 * Quantum programs eventually cross:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/security/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER:
 *
 *     grammar/DESIGN.md
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the sole grammar owner of:
 *
 *     sandboxStatement
 *
 * No other grammar may define another competing sandbox statement.
 *
 * In particular:
 *
 *     grammar/statements/statements.g4
 *
 * MUST NOT duplicate sandbox syntax.
 *
 * It imports this grammar and consumes:
 *
 *     sandboxStatement
 *
 * exactly once.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar consumes tokens supplied by:
 *
 *     grammar/lexer/tokens.g4
 *
 * through:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * Existing canonical tokens include:
 *
 *     SANDBOX
 *     WITH
 *     ALLOW
 *     FORBID
 *     PERMIT
 *     DENY
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *     ASSUME
 *     GUARANTEE
 *     PREFER
 *     FALLBACK
 *     EFFECT
 *     CAPABILITY
 *     RESOURCE
 *     NETWORK
 *     NATIVE
 *     FOREIGN
 *     REFLECTION
 *     ADAPTATION
 *     POLICY
 *     AUDIT
 *     TRACE
 *     PROVENANCE
 *     SOURCE
 *     PROPERTY
 *
 * No additional sandbox-specific keyword is required for the canonical
 * sandbox model.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Sandbox subjects are deliberately open-ended.
 *
 * The grammar MUST NOT enumerate a finite universe of:
 *
 *     effects
 *     capabilities
 *     resources
 *     devices
 *     filesystems
 *     networks
 *     protocols
 *     hardware
 *     vendors
 *     processors
 *     accelerators
 *     quantum systems
 *     policies
 *     execution environments
 *
 * A new capability or effect must be representable without changing this
 * grammar merely because its name is new.
 *
 * For example:
 *
 *     capability("future.compute.mode");
 *     capability("vendor.future.feature");
 *     effect("future.effect");
 *     resource("future.resource");
 *
 * remain syntactically representable.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO language-level capacity constants.
 *
 * It MUST NOT define limits for:
 *
 *     sandboxes
 *     nested sandboxes
 *     directives
 *     policies
 *     capabilities
 *     effects
 *     resources
 *     expressions
 *     targets
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     threads
 *     actors
 *     processes
 *     memory
 *     storage
 *     network size
 *     topology size
 *     tensor rank
 *     register width
 *
 * Repetition is represented through ANTLR repetition operators.
 *
 * Practical limits may be imposed by:
 *
 *     compiler resources
 *     parser configuration
 *     runtime resources
 *     operating-system limits
 *     deployment policy
 *     target resources
 *
 * Those are NOT language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A sandbox may constrain:
 *
 *     embedded execution
 *     classical execution
 *     parallel execution
 *     distributed execution
 *     accelerator execution
 *     quantum execution
 *     HDL simulation
 *     hardware interaction
 *     AI/model execution
 *     data processing
 *     networking
 *     simulation
 *
 * without changing the grammar.
 *
 * Physical target selection belongs downstream.
 *
 * ============================================================================
 * SECURITY MODEL
 * ============================================================================
 *
 * The source program may state:
 *
 *     allow X
 *     forbid X
 *     permit X
 *     deny X
 *
 * These are DECLARATIVE INTENT.
 *
 * They do not themselves establish authorization.
 *
 * Semantic/security analysis must determine:
 *
 *     whether X exists;
 *     whether X is meaningful;
 *     whether X is authorized;
 *     whether X is compatible with enclosing policy;
 *     whether X conflicts with another rule;
 *     whether X can be enforced;
 *     whether the target can preserve the requested boundary.
 *
 * Runtime enforcement belongs outside the parser.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Sandbox restrictions may constrain effects such as:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * The effect system remains authoritative for effect semantics.
 *
 * This grammar only expresses restrictions.
 *
 * Example:
 *
 *     sandbox {
 *         forbid effect("network");
 *         forbid effect("native");
 *     }
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Sandbox capability restrictions reference capabilities symbolically.
 *
 * Example:
 *
 *     sandbox {
 *         forbid capability("native.execute");
 *         require capability("security.isolation");
 *     }
 *
 * The sandbox grammar does not determine whether a capability exists.
 *
 * Capability resolution belongs downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Sandboxes may constrain resources.
 *
 * Examples:
 *
 *     sandbox {
 *         require resource("memory") <= memory_budget;
 *         require resource("network.bandwidth") <= bandwidth_budget;
 *     }
 *
 * Resource expressions remain symbolic.
 *
 * The grammar does not allocate resources.
 *
 * It does not select a device.
 *
 * It does not establish a universal capacity.
 *
 * ============================================================================
 * NETWORK INTEGRATION
 * ============================================================================
 *
 * Network restrictions may be represented through the open-world subject model.
 *
 * Examples:
 *
 *     forbid network;
 *     forbid network::external;
 *     forbid network("external");
 *
 * The semantic layer determines what the selected representation means.
 *
 * No finite protocol catalogue is embedded here.
 *
 * ============================================================================
 * FILESYSTEM INTEGRATION
 * ============================================================================
 *
 * Filesystem concepts remain symbolic.
 *
 * For example:
 *
 *     forbid filesystem::write;
 *     forbid filesystem::external;
 *
 * The identifier namespace remains open.
 *
 * The grammar does not define:
 *
 *     POSIX
 *     Windows
 *     ext4
 *     NTFS
 *     FAT
 *     device paths
 *
 * as a closed language universe.
 *
 * ============================================================================
 * NATIVE / FOREIGN INTEGRATION
 * ============================================================================
 *
 * Native and foreign operations may be restricted:
 *
 *     forbid native;
 *     forbid foreign;
 *     forbid native::execute;
 *     forbid foreign::call;
 *
 * FFI and ABI semantics remain owned by:
 *
 *     grammar/interoperability/*
 *
 * The sandbox merely expresses a boundary.
 *
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection can be restricted without defining a reflection implementation.
 *
 * Examples:
 *
 *     forbid reflection;
 *     forbid reflection::write;
 *     forbid reflection::code_generation;
 *
 * Reflection semantics remain owned by metaprogramming/reflection systems.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may be restricted:
 *
 *     forbid adaptation;
 *     forbid adaptation::code;
 *     forbid adaptation::policy;
 *
 * Sandbox policy MUST NOT be interpreted as permission for unrestricted
 * self-modifying execution.
 *
 * Adaptation semantics remain downstream.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Sandbox members may express:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *
 * These constructs participate in the universal contract system.
 *
 * The sandbox grammar does not redefine contract semantics.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Sandbox declarations may be constrained by policies.
 *
 * Example:
 *
 *     sandbox {
 *         policy security::restricted_execution;
 *     }
 *
 * Policy evaluation belongs to the policy subsystem.
 *
 * This grammar records the reference.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Sandbox intent must remain traceable.
 *
 * The AST/source model must preserve source spans for:
 *
 *     sandbox
 *     target
 *     directives
 *     subjects
 *     requirements
 *     contracts
 *     policies
 *     properties
 *     provenance declarations
 *     audit/trace declarations
 *
 * Provenance consumers may record:
 *
 *     source
 *     sandbox identity
 *     inherited sandbox
 *     policy source
 *     capability decision
 *     resource decision
 *     effect decision
 *     enforcement result
 *     fallback
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A sandbox may constrain quantum execution.
 *
 * Example:
 *
 *     sandbox {
 *         forbid capability("quantum.hardware.control");
 *         forbid effect("network");
 *         require capability("quantum.measurement");
 *     }
 *
 * The sandbox grammar MUST NOT define:
 *
 *     physical qubits
 *     physical qubit IDs
 *     gate catalogues
 *     coupling maps
 *     calibration
 *     pulse schedules
 *     QEC codes
 *     routing
 *     scheduling
 *
 * Quantum semantics eventually cross:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A sandbox may constrain hardware interaction.
 *
 * Examples:
 *
 *     forbid capability("hardware.configure");
 *     forbid native::hardware_access;
 *
 * Physical hardware selection remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A sandbox may constrain distributed behavior:
 *
 *     forbid capability("distributed.spawn");
 *     forbid network;
 *     require capability("distributed.isolation");
 *
 * The number of nodes, services, actors, or processes is not fixed.
 *
 * ============================================================================
 * AI / MODEL INTEGRATION
 * ============================================================================
 *
 * A sandbox may constrain model execution:
 *
 *     forbid capability("model.external_data");
 *     forbid network;
 *     forbid reflection;
 *
 * Learning/adaptation semantics remain in their appropriate domain systems.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * A sandbox may allow or forbid simulation:
 *
 *     allow simulation;
 *     forbid simulation;
 *
 * Whether simulation can satisfy another semantic requirement is decided
 * downstream by execution planning.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     resource availability
 *     wall-clock time
 *     random state
 *     target availability
 *     runtime state
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT introduce syntax for:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     API keys
 *     authentication secrets
 *     recovery secrets
 *     raw credential material
 *
 * Symbolic references to security objects are permitted.
 *
 * Secret material belongs to secure credential/key-management infrastructure.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Recommended frontend representation:
 *
 *     SandboxStatement {
 *         attributes
 *         visibility
 *         target
 *         configuration
 *         source_span
 *     }
 *
 * Configuration should preserve ordered source members:
 *
 *     SandboxMember
 *
 * with semantic variants such as:
 *
 *     SandboxPermission
 *     SandboxRequirement
 *     SandboxContract
 *     SandboxPreference
 *     SandboxFallback
 *     SandboxPolicyReference
 *     SandboxProperty
 *     SandboxProvenance
 *     SandboxAudit
 *     SandboxNested
 *
 * The exact Rust representation belongs to the frontend AST.
 *
 * This grammar must preserve enough parse structure for the AST to distinguish
 * each source form without reconstructing information from raw text.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     resolve sandbox target;
 *     resolve names;
 *     normalize directives;
 *     resolve capabilities;
 *     resolve effects;
 *     resolve resources;
 *     resolve policy references;
 *     validate contract expressions;
 *     detect contradictory restrictions;
 *     calculate inherited restrictions;
 *     validate nesting;
 *     determine enforcement requirements;
 *     preserve provenance;
 *     determine whether requested isolation is realizable.
 *
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * ============================================================================
 * CONFLICT SEMANTICS
 * ============================================================================
 *
 * This grammar deliberately does not decide conflicts such as:
 *
 *     allow X;
 *     forbid X;
 *
 * or:
 *
 *     require capability("x");
 *     forbid capability("x");
 *
 * Such conflicts are semantic/policy diagnostics.
 *
 * The parser must accept structurally valid input and preserve the ordering
 * needed for downstream conflict analysis.
 *
 * ============================================================================
 * NESTING SEMANTICS
 * ============================================================================
 *
 * Nested sandboxes are supported.
 *
 * Example:
 *
 *     sandbox {
 *         forbid network;
 *
 *         sandbox {
 *             forbid native;
 *         }
 *     }
 *
 * Nested restrictions normally compose toward the more restrictive effective
 * boundary, but the exact conflict/inheritance semantics belong to policy
 * analysis.
 *
 * The grammar does not impose a nesting-depth limit.
 *
 * ============================================================================
 * TARGET SEMANTICS
 * ============================================================================
 *
 * The optional target is an expression.
 *
 * Examples:
 *
 *     sandbox execution_context {
 *         forbid network;
 *     }
 *
 *     sandbox workload {
 *         forbid native;
 *     }
 *
 *     sandbox expression_for_target {
 *         ...
 *     }
 *
 * The target is NOT:
 *
 *     CPU number
 *     GPU number
 *     QPU number
 *     node number
 *     physical address
 *     device identifier
 *
 * Physical target resolution belongs downstream.
 *
 * ============================================================================
 * WITH OPTIONS
 * ============================================================================
 *
 * Sandbox configuration may carry open-ended named options:
 *
 *     sandbox with (
 *         mode = security::strict,
 *         policy = security::restricted
 *     ) {
 *         ...
 *     }
 *
 * The grammar does not enumerate option names.
 *
 * This permits future configuration dimensions without grammar redesign.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Stable public rules:
 *
 *     sandboxStatement
 *     sandboxTarget
 *     sandboxBody
 *     sandboxMember
 *     sandboxDirective
 *     sandboxSubject
 *     sandboxRequirement
 *     sandboxContract
 *     sandboxProperty
 *
 * ============================================================================
 */

parser grammar SecuritySandbox;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * STANDALONE TEST ENTRY
 * ============================================================================
 *
 * This is useful for isolated grammar testing.
 *
 * The production parser consumes sandboxStatement through Statements.
 * ============================================================================
 */

sandboxFile
    : sandboxStatement* EOF
    ;


/*
 * ============================================================================
 * CANONICAL SANDBOX STATEMENT
 * ============================================================================
 *
 * Canonical minimal form:
 *
 *     sandbox {
 *         forbid effect("network");
 *         forbid capability("native.execute");
 *     }
 *
 * Targeted form:
 *
 *     sandbox execution_context {
 *         forbid network;
 *     }
 *
 * Configured form:
 *
 *     sandbox with (
 *         mode = security::restricted
 *     ) {
 *         forbid native;
 *     }
 *
 * Target + configuration:
 *
 *     sandbox execution_context
 *         with (
 *             mode = security::restricted
 *         )
 *     {
 *         forbid network;
 *     }
 */
sandboxStatement
    : attributes*
      visibility?
      SANDBOX
      sandboxTarget?
      sandboxWithClause?
      sandboxBody
      SEMI?
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * The target is intentionally an expression.
 *
 * This allows symbolic target references without enumerating target classes.
 */
sandboxTarget
    : expression
    ;


/*
 * ============================================================================
 * OPTIONAL CONFIGURATION
 * ============================================================================
 *
 * Syntax:
 *
 *     with (
 *         option = expression,
 *         another_option = expression
 *     )
 *
 * Option names are open-world identifiers/qualified names.
 */
sandboxWithClause
    : WITH
      LPAREN
      sandboxOptionList?
      RPAREN
    ;


sandboxOptionList
    : sandboxOption
      (
          COMMA
          sandboxOption
      )*
      COMMA?
    ;


sandboxOption
    : qualifiedName
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * SANDBOX BODY
 * ============================================================================
 */

sandboxBody
    : LBRACE
      sandboxMember*
      RBRACE
    ;


/*
 * ============================================================================
 * SANDBOX MEMBER
 * ============================================================================
 *
 * Ordering is preserved by the parse tree.
 */
sandboxMember
    : sandboxDirective
    | sandboxRequirement
    | sandboxContract
    | sandboxPreference
    | sandboxFallback
    | sandboxPolicyReference
    | sandboxProvenanceDirective
    | sandboxAuditDirective
    | sandboxTraceDirective
    | sandboxProperty
    | sandboxNested
    ;


/*
 * ============================================================================
 * ALLOW / FORBID / PERMIT / DENY
 * ============================================================================
 *
 * These are declarations of security/execution intent.
 *
 * They do not perform authorization.
 */
sandboxDirective
    : sandboxDirectiveOperator
      sandboxSubjectList
      SEMI
    ;


sandboxDirectiveOperator
    : ALLOW
    | FORBID
    | PERMIT
    | DENY
    ;


sandboxSubjectList
    : sandboxSubject
      (
          COMMA
          sandboxSubject
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * SANDBOX SUBJECT
 * ============================================================================
 *
 * Subject syntax deliberately combines:
 *
 *     canonical domain references
 *     qualified names
 *     expressions
 *
 * with explicit forms for keyword-tokenized semantic categories.
 *
 * This avoids requiring a new lexer keyword whenever a new security-relevant
 * capability/effect/resource appears.
 */
sandboxSubject
    : sandboxEffectSubject
    | sandboxCapabilitySubject
    | sandboxResourceSubject
    | sandboxNetworkSubject
    | sandboxNativeSubject
    | sandboxForeignSubject
    | sandboxReflectionSubject
    | sandboxAdaptationSubject
    | sandboxSimulationSubject
    | sandboxQualifiedSubject
    | sandboxExpressionSubject
    ;


sandboxEffectSubject
    : EFFECT
      LPAREN
      expression
      RPAREN
    ;


sandboxCapabilitySubject
    : CAPABILITY
      LPAREN
      expression
      RPAREN
    ;


sandboxResourceSubject
    : RESOURCE
      LPAREN
      expression
      RPAREN
    ;


sandboxNetworkSubject
    : NETWORK
    | NETWORK
      LPAREN
      expression
      RPAREN
    ;


sandboxNativeSubject
    : NATIVE
    | NATIVE
      LPAREN
      expression
      RPAREN
    | NATIVE
      qualifiedName
    ;


sandboxForeignSubject
    : FOREIGN
    | FOREIGN
      LPAREN
      expression
      RPAREN
    | FOREIGN
      qualifiedName
    ;


sandboxReflectionSubject
    : REFLECTION
    | REFLECTION
      LPAREN
      expression
      RPAREN
    | REFLECTION
      qualifiedName
    ;


sandboxAdaptationSubject
    : ADAPTATION
    | ADAPTATION
      LPAREN
      expression
      RPAREN
    | ADAPTATION
      qualifiedName
    ;


sandboxSimulationSubject
    : SIMULATION
    | SIMULATION
      LPAREN
      expression
      RPAREN
    | SIMULATION
      qualifiedName
    ;


sandboxQualifiedSubject
    : qualifiedName
    ;


sandboxExpressionSubject
    : LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Examples:
 *
 *     require capability("security.isolation");
 *     require resource("memory") <= memory_budget;
 *
 * REQUIRES is the canonical language token.
 *
 * A requirement is not an allocation.
 */
sandboxRequirement
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * These reuse the universal contract vocabulary.
 */
sandboxContract
    : ENSURES
      expression
      SEMI
    | INVARIANT
      expression
      SEMI
    | ASSUME
      expression
      SEMI
    | GUARANTEE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences do not override requirements or prohibitions.
 *
 * Semantic precedence is determined downstream by policy analysis.
 */
sandboxPreference
    : PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * FALLBACK
 * ============================================================================
 *
 * A fallback describes alternative execution/security intent.
 *
 * It does not perform fallback execution.
 */
sandboxFallback
    : FALLBACK
      expression
      SEMI
    ;


/*
 * ============================================================================
 * POLICY REFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     policy security::restricted_execution;
 *
 * The policy subsystem owns policy semantics.
 */
sandboxPolicyReference
    : POLICY
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Generic provenance references remain open-world.
 *
 * Examples:
 *
 *     provenance security::policy_source;
 *     provenance source::deployment_profile;
 */
sandboxProvenanceDirective
    : PROVENANCE
      sandboxReferenceOrExpression
      SEMI
    ;


sandboxReferenceOrExpression
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * AUDIT
 * ============================================================================
 *
 * Audit intent is declarative.
 *
 * It does not itself create or persist an audit record.
 */
sandboxAuditDirective
    : AUDIT
      sandboxAuditSpecification?
      SEMI
    ;


sandboxAuditSpecification
    : expression
    ;


/*
 * ============================================================================
 * TRACE
 * ============================================================================
 *
 * Trace intent is declarative.
 */
sandboxTraceDirective
    : TRACE
      sandboxTraceSpecification?
      SEMI
    ;


sandboxTraceSpecification
    : expression
    ;


/*
 * ============================================================================
 * GENERIC PROPERTIES
 * ============================================================================
 *
 * Open-world metadata.
 *
 * Example:
 *
 *     property security::classification = security::restricted;
 *
 * Property names are not enumerated.
 */
sandboxProperty
    : PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * NESTED SANDBOX
 * ============================================================================
 *
 * Nested sandbox configuration is recursive.
 *
 * No finite nesting depth is encoded.
 */
sandboxNested
    : sandboxStatement
    ;


/*
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics should cover:
 *
 *     missing sandbox body
 *     malformed target
 *     malformed with clause
 *     malformed option
 *     malformed directive
 *     malformed subject
 *     malformed requirement
 *     malformed contract
 *     malformed policy reference
 *     malformed property
 *     malformed nested sandbox
 *     missing statement terminator
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown capability
 *     unknown effect
 *     unknown resource
 *     contradictory allow/forbid rules
 *     unsatisfied requirement
 *     forbidden capability
 *     invalid policy
 *     invalid target
 *     invalid nesting semantics
 *     unenforceable isolation requirement
 *     incompatible target
 *     insufficient resources
 *
 * ============================================================================
 * SEMANTIC NON-RESPONSIBILITY
 * ============================================================================
 *
 * The parser MUST NOT reject:
 *
 *     capability names it does not know;
 *     effect names it does not know;
 *     resource names it does not know;
 *     future policy names;
 *     future hardware capabilities;
 *     future execution environments;
 *     future security mechanisms;
 *
 * merely because they are unknown to the current parser.
 *
 * Such issues are semantic resolution questions.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_SANDBOXES
 *     MAX_NESTING
 *     MAX_RULES
 *     MAX_CAPABILITIES
 *     MAX_EFFECTS
 *     MAX_RESOURCES
 *     MAX_POLICIES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_NETWORK_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * It contains no:
 *
 *     vendor catalogue
 *     device catalogue
 *     physical device identifiers
 *     physical topology
 *     fixed resource capacity
 *     fixed quantum capacity
 *     fixed hardware width
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must progressively increase:
 *
 *     sandbox count
 *     member count
 *     directive count
 *     subject count
 *     expression complexity
 *     qualified-name depth
 *     nested sandbox depth
 *     option count
 *     policy references
 *     resource expressions
 *     cross-domain restrictions
 *
 * The grammar itself must remain unchanged as test scale increases.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum:
 *
 *     sandbox {
 *         forbid effect("network");
 *         forbid capability("native.execute");
 *     }
 *
 *     sandbox {
 *         forbid network;
 *     }
 *
 *     sandbox {
 *         forbid native;
 *         forbid foreign;
 *         forbid reflection;
 *         forbid adaptation;
 *     }
 *
 *     sandbox {
 *         require capability("security.isolation");
 *         require resource("memory") <= memory_budget;
 *     }
 *
 *     sandbox execution_context {
 *         forbid network;
 *     }
 *
 *     sandbox with (
 *         mode = security::restricted
 *     ) {
 *         forbid native;
 *     }
 *
 *     sandbox {
 *         policy security::restricted_execution;
 *         property security::classification = security::restricted;
 *         audit security::events;
 *         trace security::decisions;
 *     }
 *
 *     sandbox {
 *         forbid filesystem::write;
 *         forbid network::external;
 *         forbid foreign::call;
 *     }
 *
 *     sandbox {
 *         sandbox {
 *             forbid native;
 *         }
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The suite must reject structurally malformed forms such as:
 *
 *     sandbox
 *
 *     sandbox {
 *
 *     sandbox with {
 *
 *     sandbox with ();
 *
 *     sandbox with (mode);
 *
 *     sandbox {
 *         forbid;
 *     }
 *
 *     sandbox {
 *         require;
 *     }
 *
 *     sandbox {
 *         property;
 *     }
 *
 *     sandbox {
 *         property security::mode;
 *     }
 *
 *     sandbox {
 *         policy;
 *     }
 *
 * Semantic impossibilities such as an unavailable capability MUST be tested
 * downstream rather than converted into parser errors.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations of:
 *
 *     sandbox + classical computation
 *     sandbox + quantum computation
 *     sandbox + hybrid computation
 *     sandbox + HDL
 *     sandbox + hardware intent
 *     sandbox + accelerator
 *     sandbox + AI/model execution
 *     sandbox + distributed execution
 *     sandbox + networking
 *     sandbox + FFI
 *     sandbox + reflection
 *     sandbox + adaptation
 *     sandbox + simulation
 *     sandbox + contracts
 *     sandbox + policies
 *     sandbox + provenance
 *     sandbox + resource requirements
 *     sandbox + capabilities
 *     sandbox + effects
 *
 * ============================================================================
 * CROSS-DOMAIN TEST
 * ============================================================================
 *
 * At least one conformance fixture should combine:
 *
 *     classical execution
 *     quantum execution
 *     resource requirements
 *     capability requirements
 *     effect restrictions
 *     network restrictions
 *     native/foreign restrictions
 *     policy references
 *     provenance
 *     contracts
 *
 * in one source program.
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     grammar version
 *     lexer version
 *     parser configuration
 *
 * must produce equivalent parse-tree structure.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     resource availability
 *     target selection
 *     filesystem state
 *     network state
 *     wall-clock time
 *     random state
 *     runtime state
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable public entry:
 *
 *     sandboxStatement
 *
 * Existing token names are reused.
 *
 * No new lexical keyword is introduced by this grammar.
 *
 * Future sandbox subjects should normally be represented through:
 *
 *     qualifiedName
 *     expression
 *     capability(...)
 *     effect(...)
 *     resource(...)
 *
 * rather than adding a new keyword.
 *
 * ============================================================================
 * INTEGRATION WITH STATEMENTS.G4
 * ============================================================================
 *
 * grammar/statements/statements.g4
 *
 * MUST add this grammar to its imports:
 *
 *     SecuritySandbox
 *
 * and add exactly one alternative:
 *
 *     | sandboxStatement
 *
 * to the universal statement rule.
 *
 * It MUST NOT copy any sandbox production into statements.g4.
 *
 * The resulting composition is:
 *
 *     Statements.statement
 *          |
 *          +--> sandboxStatement
 *                    |
 *                    v
 *               SecuritySandbox
 *
 * ============================================================================
 * INTEGRATION WITH CORE
 * ============================================================================
 *
 * Core owns:
 *
 *     attributes
 *     visibility
 *     qualifiedName
 *     names
 *     generic core syntax
 *
 * This file consumes those rules.
 *
 * ============================================================================
 * INTEGRATION WITH TYPES
 * ============================================================================
 *
 * Type syntax remains owned by Types.
 *
 * Sandbox expressions may contain typed expressions, but sandbox.g4 does not
 * redefine type syntax.
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * Expression semantics remain owned by Expressions.
 *
 * This file uses expression as an opaque source-level semantic expression.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * Sandbox restrictions are consumed by effect analysis.
 *
 * Example:
 *
 *     forbid effect("network");
 *
 * becomes semantic intent equivalent to a restriction over the canonical
 * effect model.
 *
 * The effect grammar does not need to know sandbox syntax.
 *
 * ============================================================================
 * INTEGRATION WITH CAPABILITIES
 * ============================================================================
 *
 * Capability restrictions are consumed by capability analysis.
 *
 * Example:
 *
 *     forbid capability("native.execute");
 *
 * The sandbox grammar does not duplicate capability declaration syntax.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource restrictions and requirements flow into resource analysis.
 *
 * They remain symbolic.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * INTEGRATION WITH SECURITY POLICY
 * ============================================================================
 *
 * Policy references are passed to the security/policy semantic layer.
 *
 * The sandbox grammar does not evaluate policies.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Sandbox source spans and policy references must remain available to
 * provenance/audit infrastructure.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Sandbox metadata may accompany quantum semantic operations.
 *
 * The canonical path remains:
 *
 *     sandbox intent
 *          |
 *          v
 *     security/policy semantics
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience/QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * Sandbox syntax MUST NOT create another quantum IR.
 *
 * ============================================================================
 * INTEGRATION WITH HDL / HARDWARE
 * ============================================================================
 *
 * Sandbox restrictions may accompany HDL/hardware intent.
 *
 * They remain security metadata and semantic constraints.
 *
 * They do not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED EXECUTION
 * ============================================================================
 *
 * Sandbox restrictions may constrain:
 *
 *     node communication
 *     service access
 *     actor creation
 *     distributed execution
 *     external networking
 *
 * The number of nodes is not part of the grammar's universe.
 *
 * ============================================================================
 * INTEGRATION WITH FFI / ABI
 * ============================================================================
 *
 * Sandbox restrictions may forbid:
 *
 *     native
 *     foreign
 *     foreign::call
 *
 * FFI/ABI details remain owned by interoperability grammars.
 *
 * ============================================================================
 * INTEGRATION WITH METAPROGRAMMING
 * ============================================================================
 *
 * Sandbox restrictions may forbid:
 *
 *     reflection
 *     reflection::write
 *     reflection::code_generation
 *
 * Metaprogramming semantics remain downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Sandbox information may become:
 *
 *     security metadata
 *     semantic constraints
 *     capability constraints
 *     effect restrictions
 *     resource constraints
 *     policy metadata
 *     provenance metadata
 *     deployment constraints
 *
 * It may accompany:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representations
 *     distributed representations
 *     accelerator representations
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated/consuming frontend implementation must:
 *
 *     compile with Rust 1.97+
 *     use Rust 2021
 *     remain memory-safe
 *     remain deterministic at the parser layer
 *     contain no unsafe Rust requirement
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] SecuritySandbox is the sole owner of sandboxStatement.
 *
 * [ ] No lexer rule is defined here.
 *
 * [ ] Existing canonical lexer tokens are reused.
 *
 * [ ] No new sandbox keyword is required for ordinary new subjects.
 *
 * [ ] sandbox { ... } parses.
 *
 * [ ] sandbox <target> { ... } parses.
 *
 * [ ] sandbox with (...) { ... } parses.
 *
 * [ ] nested sandboxes parse.
 *
 * [ ] allow/forbid/permit/deny parse.
 *
 * [ ] effect restrictions parse.
 *
 * [ ] capability restrictions parse.
 *
 * [ ] resource restrictions parse.
 *
 * [ ] network restrictions parse.
 *
 * [ ] native restrictions parse.
 *
 * [ ] foreign restrictions parse.
 *
 * [ ] reflection restrictions parse.
 *
 * [ ] adaptation restrictions parse.
 *
 * [ ] simulation restrictions parse.
 *
 * [ ] symbolic qualified-name restrictions parse.
 *
 * [ ] generic expression restrictions parse.
 *
 * [ ] requirements parse.
 *
 * [ ] contracts parse.
 *
 * [ ] preferences parse.
 *
 * [ ] fallbacks parse.
 *
 * [ ] policy references parse.
 *
 * [ ] provenance metadata parses.
 *
 * [ ] audit metadata parses.
 *
 * [ ] trace metadata parses.
 *
 * [ ] arbitrary properties parse.
 *
 * [ ] no finite capability universe exists.
 *
 * [ ] no finite effect universe exists.
 *
 * [ ] no finite resource universe exists.
 *
 * [ ] no hardware capacity is encoded.
 *
 * [ ] no device catalogue is encoded.
 *
 * [ ] no physical target is selected.
 *
 * [ ] no resource is allocated.
 *
 * [ ] no policy is evaluated.
 *
 * [ ] no runtime operation occurs.
 *
 * [ ] no IR is created.
 *
 * [ ] quantum semantics remain downstream.
 *
 * [ ] quantum::ir remains the canonical quantum IR.
 *
 * [ ] AST mapping is defined.
 *
 * [ ] semantic ownership is defined.
 *
 * [ ] effect integration is defined.
 *
 * [ ] capability integration is defined.
 *
 * [ ] resource integration is defined.
 *
 * [ ] policy integration is defined.
 *
 * [ ] provenance integration is defined.
 *
 * [ ] positive tests exist.
 *
 * [ ] negative tests exist.
 *
 * [ ] boundary tests exist.
 *
 * [ ] scalability tests exist.
 *
 * [ ] cross-domain tests exist.
 *
 * [ ] determinism tests exist.
 *
 * [ ] Rust 1.97+ integration succeeds.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file describes:
 *
 *     WHAT SECURITY BOUNDARY IS REQUESTED
 *
 * It does not describe:
 *
 *     HOW A PARTICULAR MACHINE IMPLEMENTS THAT BOUNDARY.
 *
 * Therefore:
 *
 *     Zamani source
 *          |
 *          v
 *     sandbox intent
 *          |
 *          v
 *     security/policy analysis
 *          |
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL
 *          +--> hardware
 *          +--> distributed
 *          +--> accelerator
 *          +--> future domains
 *          |
 *          v
 *     execution planning
 *          |
 *          v
 *     target realization
 *
 * The sandbox grammar therefore does not create a ceiling on the size,
 * technology, or computational domain of Zamani programs.
 *
 * ============================================================================
 */
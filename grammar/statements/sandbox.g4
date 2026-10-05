/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/sandbox.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar identity:
 *     Sandbox
 *
 * Status:
 *     CANONICAL STATEMENT-LEVEL SANDBOX COMPOSITION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar-only.
 *
 *     No embedded Rust.
 *     No semantic predicates.
 *     No parser actions.
 *     No filesystem access.
 *     No network access.
 *     No hardware access.
 *     No runtime execution.
 *     No dynamic policy evaluation.
 *     No capability discovery.
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL STATEMENT SYNTAX for establishing a
 * sandboxed execution boundary.
 *
 * A sandbox statement expresses portable execution intent such as:
 *
 *     - which computation is placed inside the sandbox;
 *     - which sandbox configuration is requested;
 *     - which policy/reference/configuration expressions apply;
 *     - which body of Zamani computation is subject to the boundary.
 *
 * The sandbox statement does NOT implement isolation.
 *
 * It does NOT:
 *
 *     - create an operating-system container;
 *     - select a hypervisor;
 *     - select a VM;
 *     - select an enclave;
 *     - select a process;
 *     - select a CPU;
 *     - select a GPU;
 *     - select an FPGA;
 *     - select an ASIC;
 *     - select a QPU;
 *     - select a cloud provider;
 *     - select a node;
 *     - allocate memory;
 *     - allocate network interfaces;
 *     - enforce permissions;
 *     - evaluate security policy;
 *     - discover hardware;
 *     - perform capability negotiation;
 *     - perform resource negotiation;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform quantum mapping;
 *     - perform QEC;
 *     - perform ZQN;
 *     - perform HAL operations.
 *
 * Those responsibilities belong to downstream semantic, security, resource,
 * capability, effect, execution, compiler, runtime, and target layers.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The sandbox construct participates in:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     sandbox statement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *       effects           capabilities          resources
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                           policies
 *                              |
 *                              v
 *                         security model
 *                              |
 *                              v
 *                       contract validation
 *                              |
 *                              v
 *                         provenance
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *        classical IR      quantum::ir    HDL/hardware
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                       scheduling / routing
 *                              |
 *                    resilience / recovery
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                             target
 *
 * Sandbox syntax therefore remains ABOVE target realization.
 *
 * ============================================================================
 * 3. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Sandbox syntax MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program describes security/execution intent.
 *
 * The implementation determines how that intent is realized on the available
 * target.
 *
 * The same source may therefore be considered for:
 *
 *     tiny embedded execution
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed environment
 *     cloud environment
 *     future computational substrate
 *
 * provided that the required security properties and execution conditions can
 * be satisfied.
 *
 * A target that cannot satisfy the sandbox contract MUST produce a semantic
 * diagnostic or an explicit execution outcome.
 *
 * It MUST NOT silently weaken the requested sandbox semantics.
 *
 * ============================================================================
 * 4. OPEN-WORLD / SCALABILITY RULE
 * ============================================================================
 *
 * This grammar intentionally contains no fixed universe of:
 *
 *     sandboxes
 *     policies
 *     principals
 *     permissions
 *     effects
 *     capabilities
 *     resources
 *     devices
 *     nodes
 *     processes
 *     threads
 *     memories
 *     networks
 *     targets
 *
 * Repetition is represented through ANTLR '*' and '+' operators.
 *
 * There are no language-level limits such as:
 *
 *     MAX_SANDBOXES
 *     MAX_POLICIES
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_EFFECTS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * Physical limits are runtime/compiler/resource concerns.
 *
 * ============================================================================
 * 5. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     sandboxStatement
 *     sandboxTarget
 *     sandboxConfiguration
 *     sandboxOption
 *     sandboxBody
 *     sandboxItem
 *
 * It owns only the composition of these concepts at statement level.
 *
 * ============================================================================
 * 6. DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     generic capabilities
 *     generic requirements
 *     resource requirements
 *     resource constraints
 *     effects
 *     security capabilities
 *     permissions
 *     authorization
 *     security policies
 *     identities
 *     trust
 *     cryptography
 *     provenance
 *     contracts
 *     execution scheduling
 *     adaptive execution
 *     simulation
 *     quantum operations
 *     HDL operations
 *     hardware topology
 *     target selection
 *     runtime enforcement
 *
 * Those remain owned by their canonical subsystems.
 *
 * ============================================================================
 * 7. DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/
 *
 * The parser consumes the canonical ZamaniLexer.
 *
 * The expression grammar supplies domain-neutral expressions that can appear
 * as sandbox subjects, option values, and sandbox body items.
 *
 * IMPORTANT:
 *
 * This file deliberately does NOT import security policy, permission,
 * capability, resource, effect, or execution grammars.
 *
 * Reason:
 *
 *     sandbox.g4
 *         -> statement-level composition
 *
 * while:
 *
 *     grammar/security/
 *         -> security-domain syntax
 *
 *     grammar/core/capabilities.g4
 *         -> computational capabilities
 *
 *     grammar/resources/
 *         -> resources and resource requirements
 *
 *     grammar/effects/
 *         -> effects
 *
 *     grammar/execution/
 *         -> execution semantics
 *
 *     grammar/security/policies.g4
 *         -> security policy declarations
 *
 * This avoids circular imports and prevents this file from becoming a second
 * owner of those constructs.
 *
 * ============================================================================
 * 8. EXPORTS
 * ============================================================================
 *
 * Primary public rule:
 *
 *     sandboxStatement
 *
 * Reusable public rules:
 *
 *     sandboxTarget
 *     sandboxConfiguration
 *     sandboxOption
 *     sandboxBody
 *     sandboxItem
 *
 * ============================================================================
 * 9. CONSUMED BY
 * ============================================================================
 *
 * Expected consumers:
 *
 *     grammar/statements/statement.g4
 *     grammar/Zamani.g4
 *
 * Domain-specific systems may consume the resulting AST/semantic model
 * without importing this grammar directly.
 *
 * Security systems consume the semantic representation rather than owning
 * this statement grammar.
 *
 * ============================================================================
 * 10. AST CONTRACT
 * ============================================================================
 *
 * The parser contexts produced here MUST be lowered into a domain-neutral
 * sandbox statement representation.
 *
 * Conceptual AST shape:
 *
 *     SandboxStatement
 *     {
 *         target,
 *         configuration,
 *         body,
 *         source_span
 *     }
 *
 * Where:
 *
 *     target
 *         Optional expression identifying the computation/value/execution
 *         subject to the sandbox.
 *
 *     configuration
 *         Zero or more symbolic configuration entries.
 *
 *     body
 *         Ordered sandbox-contained source constructs.
 *
 *     source_span
 *         Exact source location.
 *
 * The AST MUST preserve source meaning.
 *
 * The AST MUST NOT contain:
 *
 *     physical CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     ASIC IDs
 *     QPU IDs
 *     physical qubit IDs
 *     cloud provider IDs
 *     process IDs
 *     machine-specific memory addresses
 *     backend-specific handles
 *
 * ============================================================================
 * 11. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis interprets sandbox configuration against the canonical
 * Zamani semantic systems.
 *
 * Conceptually:
 *
 *     SandboxStatement
 *          |
 *          +--> target analysis
 *          |
 *          +--> option analysis
 *          |
 *          +--> effect analysis
 *          |
 *          +--> capability analysis
 *          |
 *          +--> resource analysis
 *          |
 *          +--> policy analysis
 *          |
 *          +--> contract analysis
 *          |
 *          +--> provenance
 *          |
 *          v
 *     SandboxExecutionContract
 *
 * The semantic layer decides whether an option is meaningful and valid.
 *
 * Unknown sandbox options MUST NOT automatically become security permissions.
 *
 * They must be resolved through the semantic capability/policy system.
 *
 * ============================================================================
 * 12. CONFIGURATION MODEL
 * ============================================================================
 *
 * Configuration is deliberately represented as:
 *
 *     IDENTIFIER ASSIGN expression
 *
 * rather than an exhaustive keyword catalog.
 *
 * This permits future sandbox properties without changing the universal
 * statement grammar.
 *
 * Examples of semantic configuration concepts that MAY be represented by
 * downstream-defined option names include:
 *
 *     policy
 *     capability
 *     effect
 *     resource
 *     network
 *     filesystem
 *     native
 *     foreign
 *     reflection
 *     randomness
 *     determinism
 *     provenance
 *     trust
 *     isolation
 *     permissions
 *     identity
 *
 * These names are semantic vocabulary, not grammar-level universal limits.
 *
 * A semantic implementation MUST validate each option against the appropriate
 * subsystem.
 *
 * ============================================================================
 * 13. SECURITY BOUNDARY
 * ============================================================================
 *
 * Sandbox syntax MUST NOT be treated as equivalent to authorization.
 *
 * The distinction is:
 *
 *     sandbox
 *         = execution isolation intent
 *
 *     capability
 *         = computational ability
 *
 *     security capability
 *         = authority representation
 *
 *     permission
 *         = permitted action
 *
 *     policy
 *         = rule governing behavior
 *
 *     effect
 *         = observable/semantic consequence
 *
 *     resource requirement
 *         = required computational resource
 *
 * These concepts may interact but MUST remain distinct in the AST and
 * semantic model.
 *
 * ============================================================================
 * 14. EFFECT CONTRACT
 * ============================================================================
 *
 * Entering a sandbox does not itself imply that all effects are permitted.
 *
 * Effects such as:
 *
 *     filesystem
 *     network
 *     native
 *     foreign
 *     reflection
 *     mutation
 *     randomness
 *     distributed
 *     quantum measurement
 *
 * remain governed by the canonical effect system and applicable policies.
 *
 * A sandbox may constrain effects.
 *
 * It MUST NOT silently remove an effect from the semantic model.
 *
 * ============================================================================
 * 15. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Sandbox configuration MAY refer semantically to capabilities.
 *
 * Example intent:
 *
 *     sandbox with
 *         capability = capability_reference
 *     {
 *         ...
 *     }
 *
 * Capability resolution belongs to:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/capabilities.g4
 *     security capability subsystem
 *
 * depending on the semantic kind of capability involved.
 *
 * This file MUST NOT redefine capability syntax.
 *
 * ============================================================================
 * 16. RESOURCE CONTRACT
 * ============================================================================
 *
 * Sandbox execution may require resources.
 *
 * Examples include:
 *
 *     memory
 *     compute
 *     storage
 *     network
 *     accelerator
 *     quantum resources
 *     distributed resources
 *
 * Resource requirements remain symbolic and open-ended.
 *
 * This grammar MUST NOT encode physical capacity assumptions.
 *
 * Example semantic intent:
 *
 *     sandbox with
 *         resource = required_resource_expression
 *     {
 *         ...
 *     }
 *
 * The resource subsystem determines whether the requirement can be satisfied.
 *
 * ============================================================================
 * 17. POLICY CONTRACT
 * ============================================================================
 *
 * Sandbox configuration may reference a policy:
 *
 *     sandbox with
 *         policy = security::restricted_execution
 *     {
 *         ...
 *     }
 *
 * The policy declaration itself belongs to the security/policy subsystem.
 *
 * This grammar merely permits the statement to carry a policy reference as
 * an expression-valued configuration.
 *
 * Policy evaluation is downstream.
 *
 * ============================================================================
 * 18. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Sandbox statements may be subject to:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Those constructs remain owned by the validation/contract subsystem.
 *
 * This grammar MUST NOT duplicate contract syntax.
 *
 * The semantic representation of a sandbox boundary must preserve applicable
 * contract relationships so that validation can verify them.
 *
 * ============================================================================
 * 19. PROVENANCE CONTRACT
 * ============================================================================
 *
 * A sandbox boundary is a semantically significant execution boundary.
 *
 * The semantic model SHOULD preserve:
 *
 *     source location
 *     enclosing module
 *     configuration expressions
 *     policy references
 *     capability references
 *     resource references
 *     relevant effects
 *     contract associations
 *     transformation history
 *     execution decision
 *
 * Provenance storage belongs to the canonical provenance subsystem.
 *
 * This grammar only preserves the source structure required to construct it.
 *
 * ============================================================================
 * 20. IR CONTRACT
 * ============================================================================
 *
 * Sandbox statements MUST NOT lower directly to:
 *
 *     LLVM-specific constructs
 *     vendor security APIs
 *     container APIs
 *     hypervisor APIs
 *     enclave APIs
 *     cloud-specific sandbox APIs
 *     QPU-specific mechanisms
 *     FPGA-specific mechanisms
 *
 * Instead:
 *
 *     sandbox statement
 *          |
 *          v
 *     semantic sandbox contract
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical lowering
 *          +--> quantum lowering
 *          +--> HDL/hardware lowering
 *          +--> distributed lowering
 *          +--> target-specific security realization
 *
 * ============================================================================
 * 21. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Sandbox syntax is domain-neutral.
 *
 * If the sandbox encloses quantum computation, semantic analysis may associate
 * the resulting sandbox contract with quantum semantic operations.
 *
 * Quantum-specific operations continue to lower through:
 *
 *     quantum::ir
 *
 * The sandbox grammar MUST NOT contain:
 *
 *     physical qubit
 *     hardware coupling map
 *     calibration
 *     routing
 *     QEC
 *     vendor gate catalog
 *
 * Those remain outside this statement grammar.
 *
 * ============================================================================
 * 22. HDL BOUNDARY
 * ============================================================================
 *
 * If a sandbox contains HDL/co-design constructs, the sandbox boundary remains
 * a source-level execution/security intent.
 *
 * HDL semantics remain owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains downstream.
 *
 * No fixed bus width, device count, register width, or hardware topology may
 * be introduced here.
 *
 * ============================================================================
 * 23. EXECUTION BOUNDARY
 * ============================================================================
 *
 * Sandbox is NOT itself a scheduler, executor, simulator, retry mechanism, or
 * recovery mechanism.
 *
 * It may constrain those systems semantically.
 *
 * For example:
 *
 *     sandbox with
 *         policy = execution_policy
 *     {
 *         ...
 *     }
 *
 * may produce a semantic policy association consumed by execution planning.
 *
 * Execution remains owned by:
 *
 *     grammar/execution/
 *
 * and its semantic/runtime layers.
 *
 * ============================================================================
 * 24. NESTING
 * ============================================================================
 *
 * Sandboxes may be nested.
 *
 * This is important for:
 *
 *     composition
 *     isolation domains
 *     delegated execution
 *     multi-agent execution
 *     distributed computation
 *     hybrid computation
 *     simulation
 *
 * No fixed nesting depth is imposed by the grammar.
 *
 * Practical parser/runtime limits are implementation/resource concerns.
 *
 * Nested sandbox semantics MUST be validated for:
 *
 *     policy compatibility
 *     capability attenuation
 *     effect restrictions
 *     resource requirements
 *     inheritance
 *     provenance
 *     contract consistency
 *
 * ============================================================================
 * 25. DIAGNOSTICS
 * ============================================================================
 *
 * Grammar-level diagnostics should be limited to structural errors.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples of semantic errors include:
 *
 *     - referenced policy does not exist;
 *     - referenced capability is unavailable;
 *     - requested effect is prohibited;
 *     - resource requirement cannot be satisfied;
 *     - sandbox configuration conflicts with enclosing policy;
 *     - nested sandbox attempts to exceed an enclosing restriction;
 *     - required isolation guarantee cannot be realized;
 *     - target cannot satisfy the sandbox contract.
 *
 * The compiler MUST distinguish:
 *
 *     parse failure
 *
 * from:
 *
 *     semantic sandbox failure
 *
 * from:
 *
 *     resource/capability infeasibility
 *
 * from:
 *
 *     runtime enforcement failure.
 *
 * ============================================================================
 * 26. COMPATIBILITY
 * ============================================================================
 *
 * Existing:
 *
 *     sandbox
 *
 * remains the canonical sandbox statement introducer.
 *
 * This file must not introduce an alternative spelling such as:
 *
 *     isolate
 *     secure_block
 *     container
 *     enclave
 *     jail
 *
 * merely for implementation-specific mechanisms.
 *
 * Such mechanisms may be represented through policies, capabilities, dialects,
 * or target-specific lowering.
 *
 * ============================================================================
 * 27. TEST CONTRACT
 * ============================================================================
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/security/
 *     grammar/tests/resources/
 *     grammar/tests/effects/
 *     grammar/tests/policies/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/negative/
 *
 * ============================================================================
 * 28. REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * Minimal:
 *
 *     sandbox {
 *     }
 *
 * Targeted:
 *
 *     sandbox computation {
 *     }
 *
 * Configuration:
 *
 *     sandbox computation with
 *         policy = security::restricted_execution
 *     {
 *     }
 *
 * Multiple options:
 *
 *     sandbox computation with
 *         policy = security::restricted_execution
 *         capability = execution::isolated
 *         resource = required_resource
 *     {
 *     }
 *
 * Nested:
 *
 *     sandbox outer {
 *         sandbox inner {
 *         }
 *     }
 *
 * Expression-valued target:
 *
 *     sandbox some_expression {
 *     }
 *
 * ============================================================================
 * 29. REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * Reject structurally malformed forms such as:
 *
 *     sandbox
 *
 *     sandbox {
 *
 *     sandbox with {
 *
 *     sandbox target with policy {
 *
 *     sandbox target with = value {
 *     }
 *
 * Semantic tests must additionally reject:
 *
 *     unknown policy references
 *     invalid capability references
 *     prohibited effects
 *     unsatisfied resource requirements
 *     conflicting sandbox policies
 *
 * ============================================================================
 * 30. REQUIRED BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     sandbox + classical
 *     sandbox + quantum
 *     sandbox + hybrid
 *     sandbox + HDL
 *     sandbox + AI
 *     sandbox + distributed
 *     sandbox + networking
 *     sandbox + FFI
 *     sandbox + reflection
 *     sandbox + simulation
 *     sandbox + adaptive execution
 *
 * Verify that the sandbox boundary remains domain-neutral.
 *
 * ============================================================================
 * 31. REQUIRED SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite MUST include:
 *
 *     many nested sandbox scopes
 *     many configuration options
 *     symbolic resource expressions
 *     symbolic capability expressions
 *     large policy references
 *     deeply composed expressions
 *     large source programs containing sandbox boundaries
 *
 * No test may assume a universal maximum number of:
 *
 *     sandboxes
 *     options
 *     resources
 *     capabilities
 *     policies
 *     devices
 *     nodes
 *     threads
 *     qubits
 *
 * ============================================================================
 * 32. REQUIRED PORTABILITY TEST
 * ============================================================================
 *
 * The same source-level sandbox program should be semantically analyzable
 * without changing its source syntax for:
 *
 *     embedded
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *
 * The target-specific implementation may differ.
 *
 * The source-level meaning must not.
 *
 * ============================================================================
 * 33. REQUIRED DETERMINISM / REPRODUCIBILITY TEST
 * ============================================================================
 *
 * Parsing the same source with the same grammar and language version MUST
 * produce the same structural parse.
 *
 * Semantic sandbox resolution must separately record any runtime-dependent
 * policy/capability/resource decision through provenance.
 *
 * Hardware discovery MUST NOT be performed by this grammar.
 *
 * ============================================================================
 * 34. REQUIRED SECURITY INVARIANTS
 * ============================================================================
 *
 * The implementation MUST preserve these invariants:
 *
 *     sandbox != authorization
 *     sandbox != capability
 *     sandbox != resource
 *     sandbox != effect
 *     sandbox != policy
 *     sandbox != target
 *
 * A sandbox statement MUST NOT silently grant authority.
 *
 * A sandbox statement MUST NOT silently create capabilities.
 *
 * A sandbox statement MUST NOT silently allocate resources.
 *
 * A sandbox statement MUST NOT silently suppress effects.
 *
 * A sandbox statement MUST NOT silently weaken an enclosing security policy.
 *
 * ============================================================================
 * 35. INTEGRATION WITH statement.g4
 * ============================================================================
 *
 * AFTER THIS FILE IS COMPLETE:
 *
 *     grammar/statements/statement.g4
 *
 * should import:
 *
 *     Sandbox
 *
 * and add:
 *
 *     | sandboxStatement
 *
 * to the statement dispatcher.
 *
 * The integration should be:
 *
 *     Statement
 *         |
 *         +--> Sandbox
 *                  |
 *                  +--> sandboxStatement
 *
 * This file itself should NOT modify statement.g4.
 *
 * That keeps this file independently completable.
 *
 * ============================================================================
 * 36. INTEGRATION WITH Zamani.g4
 * ============================================================================
 *
 * The canonical composition root:
 *
 *     grammar/Zamani.g4
 *
 * should continue to own only root composition and dispatch.
 *
 * Sandbox syntax should reach the root through the normal statement grammar
 * import graph.
 *
 * Zamani.g4 MUST NOT duplicate sandboxStatement.
 *
 * ============================================================================
 * 37. INTEGRATION WITH SECURITY
 * ============================================================================
 *
 * Security semantic integration belongs downstream with:
 *
 *     grammar/security/security.g4
 *     grammar/security/policies.g4
 *     grammar/security/permissions.g4
 *     grammar/security/capabilities.g4
 *     grammar/security/trust.g4
 *
 * No security rule is duplicated here.
 *
 * ============================================================================
 * 38. INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource integration belongs downstream with:
 *
 *     grammar/resources/requirements.g4
 *     grammar/resources/capabilities.g4
 *     grammar/resources/constraints.g4
 *     grammar/resources/preferences.g4
 *     grammar/resources/negotiation.g4
 *
 * Sandbox syntax may reference the resulting semantic resource expressions,
 * but does not own their grammar.
 *
 * ============================================================================
 * 39. INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * Effect integration belongs downstream with:
 *
 *     grammar/effects/
 *
 * Sandbox analysis may restrict or permit effects, but the sandbox grammar
 * does not redefine effect syntax.
 *
 * ============================================================================
 * 40. INTEGRATION WITH EXECUTION
 * ============================================================================
 *
 * Execution planning may consume sandbox contracts for:
 *
 *     isolation
 *     scheduling
 *     placement
 *     fallback
 *     recovery
 *     simulation
 *     adaptive execution
 *
 * Sandbox syntax itself remains independent of those mechanisms.
 *
 * ============================================================================
 * 41. INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Sandbox semantic decisions should be traceable through the repository's
 * provenance model.
 *
 * Provenance may record:
 *
 *     source sandbox
 *     policy resolution
 *     capability resolution
 *     resource resolution
 *     effect analysis
 *     target realization
 *     enforcement result
 *
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It parses the canonical sandbox statement forms.
 *
 * [ ] It uses ZamaniLexer.
 *
 * [ ] It owns no lexer rules.
 *
 * [ ] It owns no security-policy implementation.
 *
 * [ ] It owns no authorization semantics.
 *
 * [ ] It owns no resource allocation.
 *
 * [ ] It owns no hardware selection.
 *
 * [ ] It owns no target-specific isolation mechanism.
 *
 * [ ] It permits open-ended configuration through expressions.
 *
 * [ ] It supports nesting without a fixed depth.
 *
 * [ ] It contains no universal hardware/resource limits.
 *
 * [ ] It does not duplicate security grammar rules.
 *
 * [ ] It does not duplicate execution grammar rules.
 *
 * [ ] It does not duplicate policy grammar rules.
 *
 * [ ] It integrates through statement.g4.
 *
 * [ ] It integrates through Zamani.g4 without duplicate ownership.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Semantic security tests pass.
 *
 * [ ] Resource/capability tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Portability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Rust implementation remains Rust 1.97 / 1.97.1 compatible.
 *
 * [ ] No unsafe Rust is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Sandbox;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Generic form:
 *
 *     sandbox <optional-target> <optional-configuration> {
 *         ...
 *     }
 *
 * The target is an expression rather than an enumerated target vocabulary.
 *
 * This is essential for POCO-REAF because the sandbox may protect:
 *
 *     a value
 *     a computation
 *     a call
 *     a task
 *     an agent
 *     a quantum computation
 *     a simulation
 *     an HDL operation
 *     a distributed computation
 *     another sandbox
 *     a future construct
 *
 * without changing this grammar.
 */
sandboxStatement
    : SANDBOX sandboxTarget? sandboxConfiguration? sandboxBody
    ;


/*
 * ============================================================================
 * SANDBOX TARGET
 * ============================================================================
 *
 * The target is intentionally an expression.
 *
 * It is NOT:
 *
 *     CPU
 *     GPU
 *     QPU
 *     process
 *     container
 *     VM
 *     enclave
 *     device
 *
 * Those are implementation concepts and must remain outside this grammar.
 */
sandboxTarget
    : expression
    ;


/*
 * ============================================================================
 * SANDBOX CONFIGURATION
 * ============================================================================
 *
 * Configuration is introduced with WITH.
 *
 * Each option is:
 *
 *     name = expression
 *
 * The name remains an identifier so that the language can evolve without
 * repeatedly changing the universal sandbox grammar.
 *
 * Semantic validation determines whether an option is:
 *
 *     valid
 *     deprecated
 *     unsupported
 *     policy-controlled
 *     capability-controlled
 *     resource-controlled
 *     target-dependent
 */
sandboxConfiguration
    : WITH sandboxOption+
    ;


/*
 * ============================================================================
 * SANDBOX OPTION
 * ============================================================================
 *
 * Examples:
 *
 *     policy = security::restricted_execution
 *     capability = execution::isolated
 *     resource = required_resource
 *     effect = restricted_effect
 *     network = disabled
 *     filesystem = restricted
 *     provenance = required
 *
 * These names are NOT hard-coded by this grammar.
 */
sandboxOption
    : IDENTIFIER ASSIGN expression
    ;


/*
 * ============================================================================
 * SANDBOX BODY
 * ============================================================================
 *
 * The body is deliberately independent from a particular execution domain.
 *
 * It may therefore contain:
 *
 *     classical expressions
 *     quantum constructs
 *     hybrid constructs
 *     AI constructs
 *     HDL constructs
 *     distributed constructs
 *     nested sandbox statements
 *
 * through the normal statement/expression dispatch architecture.
 */
sandboxBody
    : LBRACE sandboxItem* RBRACE
    ;


/*
 * ============================================================================
 * SANDBOX ITEM
 * ============================================================================
 *
 * Nested sandbox statements are explicitly supported.
 *
 * Expressions are allowed because the sandbox must be able to contain
 * domain-neutral computations without importing every domain grammar here.
 *
 * An optional semicolon permits normal statement-style termination where the
 * surrounding lexical/parser architecture uses semicolon termination.
 */
sandboxItem
    : sandboxStatement
    | expression SEMICOLON?
    ;
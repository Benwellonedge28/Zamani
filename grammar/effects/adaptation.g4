/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/effects/adaptation.g4
 *
 * Grammar:
 *     AdaptationEffects
 *
 * Status:
 *     CANONICAL ADAPTATION-EFFECT DOMAIN ADAPTER
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No hardware discovery.
 *     - No resource discovery.
 *     - No target selection.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser-level adapter for the semantic
 * "adaptation" effect domain.
 *
 * Adaptation represents a semantic request or operation that may change an
 * already-established computational state, model, strategy, policy state,
 * execution strategy, learned state, configuration, or other explicitly
 * authorized semantic state.
 *
 * This file does NOT define the source-level:
 *
 *     adapt
 *
 * statement.
 *
 * The source-level statement is owned by:
 *
 *     grammar/statements/adapt.g4
 *
 * This file instead connects generic effect syntax with the adaptation
 * semantic domain.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Adaptation is NOT unrestricted self-modification.
 *
 * Semantic adaptation must remain subject to:
 *
 *     type checking
 *     ownership rules
 *     effect checking
 *     capability checking
 *     resource checking
 *     contract checking
 *     policy checking
 *     authorization
 *     provenance
 *     reproducibility requirements
 *     execution-state validation
 *
 * This grammar only preserves the source structure required by those
 * downstream systems.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Generic effect syntax remains owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effect-types.g4
 *     grammar/effects/effect-polymorphism.g4
 *     grammar/effects/effect-composition.g4
 *     grammar/effects/custom-effects.g4
 *     grammar/effects/effects.g4
 *
 * This file MUST NOT redefine any of those constructs.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectSetBody
 *     effectInvocation
 *     effectOperationReference
 *     effectOperationUse
 *     effectOperationCall
 *     effectHandler
 *     effectComposition
 *     effectPolymorphicConstruct
 *
 * ============================================================================
 * SOURCE-LEVEL ADAPT STATEMENT
 * ============================================================================
 *
 * The source-level:
 *
 *     adapt ...
 *
 * statement remains exclusively owned by:
 *
 *     grammar/statements/adapt.g4
 *
 * The intended architecture is:
 *
 *     adapt statement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic adaptation operation
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     adaptation effect     other effects
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             semantic model
 *
 * Therefore this file MUST NOT define:
 *
 *     adaptStatement
 *     adaptationTarget
 *     adaptationSourceClause
 *     adaptationContextClause
 *
 * ============================================================================
 * LEARNING / ADAPTATION SEPARATION
 * ============================================================================
 *
 * Learning and adaptation are related but distinct.
 *
 * Learning may produce information that is later used by adaptation.
 *
 * Adaptation may consume:
 *
 *     learned information
 *     evidence
 *     observations
 *     measurements
 *     feedback
 *     policy decisions
 *     resource observations
 *     execution observations
 *     simulation results
 *     reasoning results
 *
 * But:
 *
 *     learning != adaptation
 *
 * A learning operation MUST NOT automatically imply unrestricted adaptation.
 *
 * Likewise:
 *
 *     adapt
 *
 * MUST NOT automatically imply learning.
 *
 * Semantic analysis determines the complete effect set.
 *
 * ============================================================================
 * OPEN-WORLD ADAPTATION MODEL
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate adaptation operations.
 *
 * Forbidden closed-world designs include:
 *
 *     adapt_model
 *     adapt_strategy
 *     adapt_policy
 *     adapt_hardware
 *     adapt_quantum
 *     adapt_network
 *     adapt_scheduler
 *     adapt_tensor
 *     adapt_agent
 *
 * as universal grammar keywords or operation alternatives.
 *
 * Those are semantic/library/dialect vocabulary.
 *
 * A future domain may introduce:
 *
 *     domain::adaptation
 *
 * without modifying this grammar, provided it can use the existing generic
 * effect syntax.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT IDENTITIES
 * ============================================================================
 *
 * Possible semantic identities include:
 *
 *     adaptation
 *     adaptation::update
 *     adaptation::strategy
 *     adaptation::model
 *     adaptation::policy
 *     adaptation::execution
 *     learning::adaptation
 *     ai::adaptation
 *     distributed::adaptation
 *     quantum::adaptation
 *     hardware::adaptation
 *     future::adaptation::operation
 *     vendor::extension::adaptation
 *
 * These examples DO NOT establish built-in effects.
 *
 * The parser accepts source structure.
 *
 * Semantic resolution determines whether a referenced identity:
 *
 *     exists;
 *     is accessible;
 *     is classified as an adaptation effect;
 *     has a valid signature;
 *     is authorized;
 *     has the required capabilities;
 *     has sufficient resources;
 *     satisfies applicable policies.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     Core
 *     Expressions
 *     EffectOperations
 *     EffectSets
 *
 * Core supplies canonical:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     names
 *     shared source constructs
 *
 * Expressions supplies canonical:
 *
 *     expression
 *     argumentList
 *     expression-level structures
 *
 * EffectOperations supplies:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *
 * EffectSets supplies:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * This file exports domain adapters only.
 *
 * Public rules:
 *
 *     adaptationEffectReference
 *     adaptationEffectReferenceList
 *     adaptationEffectSet
 *     adaptationEffectOperationReference
 *     adaptationEffectInvocation
 *     adaptationEffectOperationUse
 *
 * There is intentionally NO broad:
 *
 *     adaptationEffect
 *
 * union rule that combines overlapping generic alternatives.
 *
 * This avoids an unnecessary ambiguity layer because:
 *
 *     effectReference
 *     effectInvocation
 *     effectOperationUse
 *
 * share the same initial syntactic structure.
 *
 * Consumers should select the adapter corresponding to the syntactic context
 * they already know they are parsing.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Potential consumers:
 *
 *     grammar/ai/
 *     grammar/execution/
 *     grammar/policies/
 *     grammar/security/
 *     grammar/expressions/
 *     semantic effect analysis
 *     adaptation semantic analysis
 *     effect inference
 *     compiler conformance tooling
 *
 * The generic effect root MUST NOT import this file merely to enumerate
 * adaptation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * It does NOT define AST classes.
 *
 * The frontend AST must preserve, where applicable:
 *
 *     effect identity
 *     operation identity
 *     qualified-name segments
 *     operation arguments
 *     effect-set membership
 *     source ordering
 *     source spans
 *     surrounding semantic context
 *
 * The semantic layer may subsequently represent the operation using the
 * existing domain-neutral effect representation.
 *
 * No adaptation-specific backend AST is required merely because the effect
 * belongs to the adaptation domain.
 *
 * The AST MUST NOT require:
 *
 *     physical device identifiers
 *     physical qubit identifiers
 *     GPU identifiers
 *     CPU identifiers
 *     FPGA identifiers
 *     node identifiers
 *     vendor instruction identifiers
 *
 * unless explicitly introduced later by a target-specific realization layer.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser acceptance establishes syntax only.
 *
 * Semantic analysis MUST determine:
 *
 *     1. effect identity;
 *     2. operation identity;
 *     3. declaration resolution;
 *     4. accessibility;
 *     5. operation signature;
 *     6. argument type compatibility;
 *     7. produced result type;
 *     8. adaptation classification;
 *     9. effect propagation;
 *    10. required capabilities;
 *    11. required resources;
 *    12. ownership permissions;
 *    13. applicable contracts;
 *    14. applicable policies;
 *    15. authorization;
 *    16. provenance requirements;
 *    17. reproducibility requirements;
 *    18. determinism requirements;
 *    19. execution-context compatibility;
 *    20. domain-crossing validity.
 *
 * A syntactically valid adaptation effect MUST NOT be treated as automatically
 * executable.
 *
 * ============================================================================
 * ADAPTATION SAFETY MODEL
 * ============================================================================
 *
 * Adaptation must be explicitly controlled.
 *
 * The semantic model should distinguish at least:
 *
 *     requested adaptation
 *     authorized adaptation
 *     validated adaptation
 *     planned adaptation
 *     applied adaptation
 *     rejected adaptation
 *
 * This grammar does not encode those states.
 *
 * They belong to semantic analysis and execution planning.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * An adaptation operation may combine with other effects, including:
 *
 *     learning
 *     mutation
 *     randomness
 *     IO
 *     network
 *     distributed
 *     native
 *     foreign
 *     reflection
 *     code generation
 *     simulation
 *     measurement
 *     quantum
 *     hardware
 *     security
 *     memory
 *     persistence
 *
 * This file does not infer those effects.
 *
 * The complete effect set is determined by semantic analysis.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Adaptation may require capabilities such as:
 *
 *     model.modify
 *     strategy.modify
 *     policy.modify
 *     execution.adapt
 *     runtime.adapt
 *     state.modify
 *     learning.update
 *     distributed.reconfigure
 *     quantum.dynamic_control
 *     hardware.reconfigure
 *
 * These names are illustrative semantic identities.
 *
 * They are NOT grammar keywords and are not a closed capability catalogue.
 *
 * Capability resolution belongs to:
 *
 *     grammar/resources/
 *     grammar/security/
 *     semantic capability analysis
 *
 * This grammar MUST NOT authorize an adaptation operation.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Adaptation may consume or require:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     distributed resources
 *     energy
 *     execution time
 *
 * Resource requirements are determined downstream.
 *
 * This file MUST NOT encode:
 *
 *     fixed memory capacities
 *     fixed worker counts
 *     fixed device counts
 *     fixed node counts
 *     fixed qubit counts
 *     fixed tensor dimensions
 *     fixed register widths
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Adaptation itself is not a resource requirement.
 *
 * A semantic adaptation may subsequently produce:
 *
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *
 * For example, a semantic layer may determine that an adaptation requires a
 * capability or resource.
 *
 * Such declarations remain owned by:
 *
 *     grammar/resources/
 *     grammar/core/
 *     grammar/policies/
 *
 * This file does not duplicate them.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Adaptation may be governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This grammar MUST NOT redefine contract syntax.
 *
 * Semantic adaptation analysis may associate contracts with an adaptation
 * operation.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Adaptation is particularly sensitive to policy.
 *
 * Applicable policy domains may include:
 *
 *     authorization
 *     security
 *     model modification
 *     data use
 *     execution
 *     deployment
 *     resource use
 *     network access
 *     native access
 *     foreign access
 *     reflection
 *     persistence
 *     reproducibility
 *
 * Policy syntax remains owned by:
 *
 *     grammar/policies/
 *     grammar/security/
 *
 * A policy MUST NOT be bypassed merely because an adaptation operation parsed
 * successfully.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Adaptation should be traceable where the semantic environment requires
 * provenance.
 *
 * Downstream provenance may record:
 *
 *     request
 *     source span
 *     prior state
 *     source of adaptation
 *     evidence
 *     reason
 *     authorization
 *     policy
 *     capability decision
 *     resource decision
 *     transformation
 *     resulting state
 *     verification
 *     execution realization
 *
 * This grammar preserves source structure but does not create provenance
 * records.
 *
 * ============================================================================
 * EXPLAINABILITY / DECISION CONTRACT
 * ============================================================================
 *
 * Adaptation decisions may be explained through the common explanation and
 * decision-record systems.
 *
 * Possible semantic information includes:
 *
 *     why adaptation was requested;
 *     which evidence supported it;
 *     which policy authorized it;
 *     which constraints were considered;
 *     which alternatives were rejected;
 *     which realization was selected;
 *     whether the adaptation succeeded.
 *
 * Explanation syntax is not defined here.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning may produce information consumed by adaptation.
 *
 * Example semantic flow:
 *
 *     learning
 *        |
 *        v
 *     learned information
 *        |
 *        v
 *     adaptation request
 *        |
 *        v
 *     policy / capability / contract checks
 *        |
 *        v
 *     adaptation plan
 *
 * The learning effect remains independently represented.
 *
 * There is no implicit:
 *
 *     learning -> adaptation
 *
 * rule in this grammar.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume:
 *
 *     inference
 *     deduction
 *     reasoning
 *     causal analysis
 *     evidence
 *     decision records
 *
 * The reasoning grammar and semantic system remain authoritative.
 *
 * This file MUST NOT define:
 *
 *     infer
 *     deduce
 *     reason
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume uncertain information:
 *
 *     probability
 *     distribution
 *     confidence
 *     belief
 *     interval
 *     evidence strength
 *
 * Uncertainty syntax and types remain owned by:
 *
 *     grammar/ai/
 *     grammar/types/
 *
 * This file imposes no precision, width, cardinality, or tensor limits.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Adaptation may be evaluated through simulation before realization.
 *
 * Semantic flow may be:
 *
 *     adaptation request
 *          |
 *          v
 *       simulation
 *          |
 *          v
 *       verification
 *          |
 *          v
 *       policy/contract check
 *          |
 *          v
 *       realization
 *
 * Simulation syntax remains owned by:
 *
 *     grammar/execution/
 *
 * This file does not create simulation syntax.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION INTEGRATION
 * ============================================================================
 *
 * Adaptation may participate in adaptive execution together with:
 *
 *     detection
 *     evaluation
 *     selection
 *     fallback
 *     retry
 *     recovery
 *     specialization
 *     migration
 *
 * These are execution-planning concepts.
 *
 * This grammar only represents adaptation-effect syntax.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Adaptation may influence a hybrid or quantum computation.
 *
 * Example semantic flow:
 *
 *     adaptation
 *        |
 *        v
 *     semantic quantum operation
 *        |
 *        v
 *     quantum::ir
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     routing
 *        |
 *        v
 *     scheduling
 *        |
 *        v
 *     resilience / QEC / ZQN
 *        |
 *        v
 *     HAL
 *
 * This file MUST NOT define:
 *
 *     qubit identifiers
 *     physical qubit mappings
 *     gate catalogues
 *     coupling maps
 *     calibration
 *     pulse schedules
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * No adaptation-specific quantum IR is permitted.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Adaptation may influence hardware intent or implementation strategy.
 *
 * Possible semantic realizations include:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     quantum processor
 *     simulator
 *     distributed system
 *     future computational substrate
 *
 * The grammar does not select any realization.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Adaptation may compose with:
 *
 *     distributed state
 *     synchronization
 *     messaging
 *     replication
 *     migration
 *     reconfiguration
 *
 * Distributed syntax remains owned by:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *
 * No worker/node/device limit is encoded.
 *
 * ============================================================================
 * REFLECTION / METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Adaptation must not automatically bypass reflection or metaprogramming
 * safety boundaries.
 *
 * If an adaptation is implemented through:
 *
 *     reflection
 *     generated code
 *     compile-time computation
 *     runtime code generation
 *
 * those effects must be represented independently.
 *
 * The corresponding syntax remains owned by:
 *
 *     grammar/metaprogramming/
 *     grammar/macros/
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Adaptation may cross:
 *
 *     FFI
 *     ABI
 *     data formats
 *     model formats
 *     vendor dialects
 *     accelerator interfaces
 *     external services
 *
 * Such boundaries remain owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This grammar does not duplicate:
 *
 *     FFI
 *     ABI
 *     SQL
 *     JSON
 *     XML
 *     OpenQASM
 *     HDL
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Adaptation syntax describes portable semantic intent.
 *
 * It MUST NOT identify a particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     device
 *     memory bank
 *     physical qubit
 *     network interface
 *
 * unless a separate explicit target/deployment language is being used.
 *
 * The same source-level adaptation structure must remain syntactically valid
 * across:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum processors
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * Actual feasibility belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limits on:
 *
 *     adaptation operations
 *     effect references
 *     effect-set cardinality
 *     argument count
 *     qualified-name depth
 *     nesting
 *     program size
 *     model size
 *     data size
 *     resource quantity
 *     machine size
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     node count
 *     network size
 *     tensor rank
 *     tensor dimensions
 *
 * No universal maximum is encoded.
 *
 * Practical compiler/runtime limits are implementation-resource concerns and
 * must not become language semantics.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * Physical execution cannot be assumed to be literally infinite.
 *
 * Production scalability means that this grammar introduces no artificial
 * finite machine-size ceiling.
 *
 * Execution feasibility is determined by:
 *
 *     available resources
 *     available capabilities
 *     physical constraints
 *     compiler resources
 *     runtime resources
 *     deployment policy
 *     target feasibility
 *
 * Failure must therefore be reported downstream as an appropriate semantic,
 * resource, capability, policy, or feasibility result.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is a pure syntactic operation.
 *
 * Given identical:
 *
 *     source
 *     token stream
 *     lexer vocabulary
 *     grammar version
 *     parser configuration
 *     dialect configuration
 *
 * the parser must produce equivalent parse structures.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     model availability
 *     resource availability
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *     target identity
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an adaptation effect MUST NOT execute adaptation.
 *
 * For example, parsing:
 *
 *     adaptation::update(model, feedback)
 *
 * MUST NOT:
 *
 *     modify a model;
 *     modify persistent state;
 *     invoke hardware;
 *     allocate resources;
 *     access a network;
 *     invoke native code;
 *     invoke foreign code;
 *     load external data;
 *     change compiler state;
 *     change runtime state.
 *
 * All such behavior occurs only after semantic validation and execution
 * planning.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors are structural errors only.
 *
 * Examples:
 *
 *     malformed qualified name
 *     malformed invocation
 *     malformed argument list
 *     malformed effect set
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unresolved effect
 *     unresolved operation
 *     operation is not an adaptation effect
 *     invalid argument type
 *     missing capability
 *     insufficient resources
 *     unauthorized adaptation
 *     forbidden policy
 *     violated contract
 *     unsupported realization
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parser MUST preserve, through ordinary ANTLR parse contexts:
 *
 *     source ordering
 *     source spans
 *     qualified-name structure
 *     operation arguments
 *     effect-set membership
 *
 * This grammar MUST NOT:
 *
 *     sort;
 *     deduplicate;
 *     evaluate;
 *     normalize semantic identities;
 *     resolve aliases;
 *     select targets;
 *     evaluate policies.
 *
 * Semantic normalization occurs downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The intended path is:
 *
 *     source
 *        |
 *        v
 *     parser
 *        |
 *        v
 *     domain-neutral AST
 *        |
 *        v
 *     semantic adaptation model
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--> classical representation
 *        +--> quantum::ir
 *        +--> HDL/hardware representation
 *        +--> distributed representation
 *        +--> accelerator representation
 *        +--> future domain representation
 *
 * No:
 *
 *     AdaptationIR
 *     AdaptationQuantumIR
 *     AdaptationHardwareIR
 *     AdaptationGPUIR
 *
 * is introduced here.
 *
 * ============================================================================
 * QUANTUM IR INVARIANT
 * ============================================================================
 *
 * If adaptation affects quantum computation:
 *
 *     adaptation syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic adaptation model
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum representation.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust source.
 *
 * The generated frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The consuming implementation must use safe Rust only.
 *
 * This grammar requires no unsafe code.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical production lexer vocabulary used by the existing effect
 * operation grammar is:
 *
 *     ZamaniTokens
 *
 * Therefore this grammar intentionally uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * It must not introduce lexer rules.
 *
 * The canonical lexer/token registry remains responsible for all lexical
 * tokens.
 *
 * No adaptation-specific keyword is required here.
 *
 * ============================================================================
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Core
 *     Expressions
 *     EffectOperations
 *     EffectSets
 *
 * It does not import:
 *
 *     Statements
 *     AI
 *     Execution
 *     Quantum
 *     Hardware
 *     Distributed
 *     Networking
 *     Security
 *     Runtime
 *
 * merely to obtain generic syntax.
 *
 * This prevents circular grammar dependencies.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AdaptationEffects;

options {
    tokenVocab = ZamaniTokens;
}

import
    Core,
    Expressions,
    EffectOperations,
    EffectSets
    ;


/*
 * ============================================================================
 * 1. ADAPTATION EFFECT REFERENCE
 * ============================================================================
 *
 * Use this rule where the consuming grammar already knows that it is parsing
 * an effect reference.
 *
 * Examples of source shape:
 *
 *     adaptation
 *     adaptation::update
 *     future::adaptation::operation
 *
 * These names are semantic identities, not built-in operation declarations.
 */

adaptationEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 2. ADAPTATION EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Delegates collection syntax to the generic effect subsystem.
 *
 * Source order is preserved by the parse tree.
 */

adaptationEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * 3. ADAPTATION EFFECT SET
 * ============================================================================
 *
 * Delegates set syntax to the generic effect subsystem.
 *
 * Example source shape:
 *
 *     {
 *         adaptation,
 *         learning,
 *         distributed::synchronize
 *     }
 *
 * The referenced identities are semantically classified downstream.
 */

adaptationEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 4. ADAPTATION OPERATION REFERENCE
 * ============================================================================
 *
 * Delegates operation identity to the generic effect-operation grammar.
 *
 * No adaptation-operation catalogue is introduced.
 */

adaptationEffectOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * 5. ADAPTATION EFFECT INVOCATION
 * ============================================================================
 *
 * Delegates invocation syntax to the canonical generic effect operation
 * grammar.
 *
 * Examples of source shape:
 *
 *     adaptation::update(model, feedback)
 *
 *     future::adaptation::operation(state)
 *
 * The examples do not establish built-in operations.
 */

adaptationEffectInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * 6. ADAPTATION EFFECT OPERATION USE
 * ============================================================================
 *
 * This is the preferred adapter for an actual effect-operation occurrence.
 *
 * The generic operation-use grammar remains authoritative.
 */

adaptationEffectOperationUse
    : effectOperationUse
    ;


/*
 * ============================================================================
 * 7. INTEGRATION INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 *
 * This grammar defines no lexer rules.
 *
 * INVARIANT 2
 *
 * This grammar defines no adaptation statement.
 *
 * INVARIANT 3
 *
 * This grammar defines no generic effect declaration.
 *
 * INVARIANT 4
 *
 * This grammar defines no generic effect operation declaration.
 *
 * INVARIANT 5
 *
 * This grammar defines no generic effect set.
 *
 * INVARIANT 6
 *
 * This grammar defines no generic effect handler.
 *
 * INVARIANT 7
 *
 * This grammar defines no generic expression hierarchy.
 *
 * INVARIANT 8
 *
 * This grammar defines no generic type system.
 *
 * INVARIANT 9
 *
 * This grammar defines no capability catalogue.
 *
 * INVARIANT 10
 *
 * This grammar defines no resource catalogue.
 *
 * INVARIANT 11
 *
 * This grammar defines no policy language.
 *
 * INVARIANT 12
 *
 * This grammar defines no contract language.
 *
 * INVARIANT 13
 *
 * This grammar defines no provenance language.
 *
 * INVARIANT 14
 *
 * This grammar defines no learning operation catalogue.
 *
 * INVARIANT 15
 *
 * This grammar defines no reasoning operation catalogue.
 *
 * INVARIANT 16
 *
 * This grammar defines no quantum operation catalogue.
 *
 * INVARIANT 17
 *
 * This grammar defines no hardware catalogue.
 *
 * INVARIANT 18
 *
 * This grammar defines no physical target selection.
 *
 * INVARIANT 19
 *
 * This grammar defines no runtime behavior.
 *
 * INVARIANT 20
 *
 * This grammar creates no IR.
 *
 * INVARIANT 21
 *
 * Quantum semantic realization continues through quantum::ir.
 *
 * INVARIANT 22
 *
 * Adaptation remains subject to downstream authorization and policy.
 *
 * INVARIANT 23
 *
 * No universal physical capacity is encoded.
 *
 * INVARIANT 24
 *
 * Parsing has no observable execution side effects.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of universal language ceilings.
 *
 * In particular, it must contain no language-level constants or grammar
 * alternatives representing:
 *
 *     maximum adaptation count
 *     maximum operation count
 *     maximum argument count
 *     maximum effect count
 *     maximum model count
 *     maximum parameter count
 *     maximum worker count
 *     maximum device count
 *     maximum node count
 *     maximum memory
 *     maximum tensor rank
 *     maximum tensor dimension
 *     maximum CPU count
 *     maximum GPU count
 *     maximum FPGA count
 *     maximum accelerator count
 *     maximum QPU count
 *     maximum qubit count
 *
 * It must also contain no physical identities such as:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     DEVICE_0
 *     NODE_0
 *
 * Numeric values occurring in user source remain ordinary program values.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following are structural examples.
 *
 * They do NOT declare built-in operations.
 *
 * --------------------------------------------------------------------------
 * EFFECT REFERENCES
 * --------------------------------------------------------------------------
 *
 *     adaptation
 *
 *     adaptation::update
 *
 *     future::adaptation::operation
 *
 *     vendor::extension::adaptation
 *
 * --------------------------------------------------------------------------
 * OPERATION REFERENCES
 * --------------------------------------------------------------------------
 *
 *     adaptation::update
 *
 *     adaptation::strategy
 *
 *     future::domain::adaptation
 *
 * --------------------------------------------------------------------------
 * INVOCATIONS
 * --------------------------------------------------------------------------
 *
 *     adaptation::update(model, feedback)
 *
 *     adaptation::update(state)
 *
 *     future::adaptation::operation(state, evidence)
 *
 *     vendor::extension::adaptation(value)
 *
 * --------------------------------------------------------------------------
 * EFFECT SETS
 * --------------------------------------------------------------------------
 *
 *     {
 *         adaptation
 *     }
 *
 *     {
 *         adaptation,
 *         learning
 *     }
 *
 *     {
 *         adaptation,
 *         learning,
 *         distributed::synchronize
 *     }
 *
 *     {
 *         adaptation,
 *         quantum::measurement
 *     }
 *
 *     {
 *         adaptation,
 *         hdl::simulation
 *     }
 *
 * --------------------------------------------------------------------------
 * CROSS-DOMAIN SHAPES
 * --------------------------------------------------------------------------
 *
 *     adaptation::update(classical_state, evidence)
 *
 *     adaptation::update(learned_model, feedback)
 *
 *     adaptation::update(quantum_result, observation)
 *
 *     adaptation::update(simulation_result, verification)
 *
 *     adaptation::update(distributed_state, resource_observation)
 *
 *     adaptation::update(hardware_intent, measurement)
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These malformed forms must be rejected by the canonical generic grammar
 * when the corresponding adapter is invoked.
 *
 *     adaptation::
 *
 *     ::adaptation::update
 *
 *     adaptation::update(
 *
 *     adaptation::update(,)
 *
 *     adaptation::update(,)
 *
 *     adaptation::update(a,,b)
 *
 *     adaptation::update(a b)
 *
 *     {
 *         adaptation,
 *         ,
 *     }
 *
 * Semantic-invalid cases MUST NOT be converted into parser tests.
 *
 * Examples of semantic-invalid cases:
 *
 *     unresolved adaptation effect
 *     unresolved operation
 *     operation not classified as adaptation
 *     unauthorized adaptation
 *     missing capability
 *     insufficient resources
 *     violated policy
 *     violated contract
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test independently:
 *
 *     zero-argument invocation
 *     one-argument invocation
 *     many arguments
 *     deeply qualified names
 *     nested expressions
 *     nested effect contexts
 *     empty effect sets where permitted by EffectSets
 *     single-entry effect sets
 *     large effect sets
 *     mixed-domain effect sets
 *     generic effect references
 *     future namespaces
 *     vendor namespaces
 *
 * Values used for scalability tests are test data, not language limits.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Vary:
 *
 *     source size
 *     qualified-name depth
 *     effect-set cardinality
 *     operation argument count
 *     nesting depth
 *     number of adaptation occurrences
 *     number of modules
 *     number of semantic domains
 *
 * No grammar modification is permitted merely because a larger test fixture
 * is introduced.
 *
 * Scaling the source must be handled through:
 *
 *     repetition
 *     recursion
 *     existing collection rules
 *     semantic representations
 *
 * rather than finite enumerations.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     lexer vocabulary
 *     grammar version
 *     parser configuration
 *     dialect configuration
 *
 * must produce equivalent parse structures.
 *
 * Parsing must not inspect:
 *
 *     hardware
 *     models
 *     datasets
 *     resources
 *     capabilities
 *     network state
 *     runtime state
 *     scheduler state
 *     wall-clock time
 *     randomness
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Adaptation effect references must be usable in semantic contexts involving:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     accelerator
 *     simulation
 *     future domains
 *
 * The syntax remains unchanged across these domains.
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same source structure:
 *
 *     adaptation::update(state, evidence)
 *
 * must remain syntactically independent of whether the eventual realization
 * occurs on:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     quantum processor
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud
 *     future substrate
 *
 * Target feasibility is not a parser responsibility.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where formatter/printer support exists:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     formatter
 *       |
 *       v
 *     parser
 *
 * must preserve:
 *
 *     effect identity
 *     operation identity
 *     argument order
 *     effect-set membership
 *     qualified-name structure
 *
 * Equivalent whitespace normalization is permitted.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing generic effect syntax remains authoritative.
 *
 * Adding a new semantic adaptation operation must NOT require a grammar
 * change if its source structure is already expressible through:
 *
 *     effectReference
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *     effectSet
 *
 * A grammar change is required only when a genuinely new source syntax is
 * introduced.
 *
 * This preserves long-term source compatibility and keeps semantic
 * extensibility separate from grammar evolution.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 * ----------
 *
 * No lexer modification is required solely for this file.
 *
 * `adaptation` is intentionally not required to be a reserved keyword.
 *
 * If source-level:
 *
 *     adapt
 *
 * is needed, that keyword is already owned by the canonical lexer and consumed
 * by:
 *
 *     grammar/statements/adapt.g4
 *
 * This file must not introduce another lexical definition.
 *
 * --------------------------------------------------------------------------
 *
 * 2. GENERIC EFFECT SYSTEM
 * ------------------------
 *
 * This file consumes:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *
 * Generic effect semantics remain authoritative there.
 *
 * --------------------------------------------------------------------------
 *
 * 3. EFFECT COMPOSITION ROOT
 * --------------------------
 *
 * `grammar/effects/effects.g4` remains domain-neutral.
 *
 * It MUST NOT import this file merely to enumerate adaptation effects.
 *
 * Adaptation effect identities are already expressible through the generic
 * open-world effect machinery.
 *
 * --------------------------------------------------------------------------
 *
 * 4. ADAPT STATEMENT
 * ------------------
 *
 * `grammar/statements/adapt.g4` remains the owner of:
 *
 *     adaptStatement
 *
 * It does not need to import this file merely to parse `adapt`.
 *
 * Semantic analysis associates an `adapt` operation with the adaptation effect
 * where appropriate.
 *
 * --------------------------------------------------------------------------
 *
 * 5. AI SUBSYSTEM
 * ---------------
 *
 * AI semantic components may consume:
 *
 *     adaptationEffectReference
 *     adaptationEffectInvocation
 *     adaptationEffectOperationUse
 *
 * where an explicit adaptation-effect classification is required.
 *
 * AI remains responsible for AI-domain semantics.
 *
 * --------------------------------------------------------------------------
 *
 * 6. EXECUTION SUBSYSTEM
 * ----------------------
 *
 * Execution analysis may consume adaptation semantic information for:
 *
 *     adaptive execution
 *     fallback
 *     retry
 *     recovery
 *     specialization
 *     migration
 *     dynamic strategy selection
 *
 * Execution planning remains downstream.
 *
 * --------------------------------------------------------------------------
 *
 * 7. SECURITY
 * -----------
 *
 * Security analysis must be able to inspect adaptation operations for:
 *
 *     authorization
 *     sandbox restrictions
 *     capability requirements
 *     policy restrictions
 *     provenance requirements
 *
 * This grammar does not perform those checks.
 *
 * --------------------------------------------------------------------------
 *
 * 8. RESOURCES
 * ------------
 *
 * Resource analysis may derive requirements from a resolved adaptation
 * operation.
 *
 * No resource quantity is embedded in this grammar.
 *
 * --------------------------------------------------------------------------
 *
 * 9. CONTRACTS
 * -----------
 *
 * Validation may associate:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * with adaptation semantics.
 *
 * This file does not duplicate those grammar rules.
 *
 * --------------------------------------------------------------------------
 *
 * 10. PROVENANCE
 * --------------
 *
 * Provenance analysis may record:
 *
 *     adaptation request
 *     source
 *     evidence
 *     decision
 *     authorization
 *     transformation
 *     resulting state
 *
 * Source spans originate from the parser contexts supplied here.
 *
 * --------------------------------------------------------------------------
 *
 * 11. QUANTUM
 * -----------
 *
 * If adaptation affects quantum computation:
 *
 *     adaptation
 *        |
 *        v
 *     semantic quantum model
 *        |
 *        v
 *     quantum::ir
 *
 * No adaptation-specific quantum IR may be introduced.
 *
 * --------------------------------------------------------------------------
 *
 * 12. HDL / HARDWARE
 * ------------------
 *
 * Adaptation may influence hardware intent, but hardware realization remains
 * downstream.
 *
 * No physical hardware identity belongs in this grammar.
 *
 * --------------------------------------------------------------------------
 *
 * 13. DISTRIBUTED
 * ---------------
 *
 * Distributed adaptation may interact with:
 *
 *     synchronization
 *     messaging
 *     replication
 *     migration
 *     reconfiguration
 *
 * Distributed semantics remain independently owned.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] It is an ANTLR4 parser grammar.
 *     [x] Grammar name is AdaptationEffects.
 *     [x] Filename is adaptation.g4.
 *     [x] It consumes ZamaniTokens.
 *     [x] It defines no lexer rules.
 *     [x] It defines no semantic predicates.
 *     [x] It contains no embedded Rust.
 *     [x] It requires no unsafe Rust.
 *     [x] It imports only generic dependencies.
 *     [x] It defines no duplicate generic effect syntax.
 *     [x] It defines no adapt statement.
 *     [x] It defines no expression hierarchy.
 *     [x] It defines no type hierarchy.
 *     [x] It defines no capability catalogue.
 *     [x] It defines no resource catalogue.
 *     [x] It defines no policy language.
 *     [x] It defines no contract language.
 *     [x] It defines no provenance language.
 *     [x] It defines no learning catalogue.
 *     [x] It defines no quantum catalogue.
 *     [x] It defines no hardware catalogue.
 *     [x] It creates no IR.
 *     [x] It preserves open-world effect identity.
 *     [x] It introduces no universal machine-size limits.
 *     [x] It introduces no physical device identities.
 *     [x] It introduces no vendor-specific realization.
 *     [x] It preserves quantum::ir as the quantum boundary.
 *     [x] It keeps parsing side-effect free.
 *
 * Repository integration is complete when:
 *
 *     [ ] ANTLR generation accepts the grammar.
 *     [ ] Its imports resolve against the current grammar build.
 *     [ ] `adaptStatement` remains owned exactly once by statements/adapt.g4.
 *     [ ] Semantic adaptation analysis recognizes the exported contexts where
 *         explicit classification is needed.
 *     [ ] Adaptation effects participate in the common effect model.
 *     [ ] Capability checking consumes the resolved semantic effect.
 *     [ ] Resource checking consumes the resolved semantic effect.
 *     [ ] Contract checking consumes the resolved semantic effect.
 *     [ ] Policy checking consumes the resolved semantic effect.
 *     [ ] Provenance consumes source spans and semantic adaptation records.
 *     [ ] Adaptive execution consumes the semantic adaptation model.
 *     [ ] Quantum adaptations cross through quantum::ir where applicable.
 *     [ ] Classical adaptations lower through the canonical classical path.
 *     [ ] HDL/hardware adaptations remain target-independent until realization.
 *     [ ] Distributed adaptations use the existing distributed/concurrency
 *         architecture.
 *     [ ] Positive fixtures pass.
 *     [ ] Negative fixtures pass.
 *     [ ] Boundary fixtures pass.
 *     [ ] Cross-domain fixtures pass.
 *     [ ] Scalability fixtures pass.
 *     [ ] Determinism fixtures pass.
 *     [ ] Compatibility fixtures pass.
 *     [ ] Round-trip fixtures pass where formatting exists.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Adaptation effect syntax:
 *
 *     generic effect syntax
 *             |
 *             v
 *     adaptation classification
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic adaptation model
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *       v            v                v
 *   classical    quantum::ir     other domains
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                    v
 *             canonical semantic IR
 *                    |
 *             optimization
 *                    |
 *          specialization/lowering
 *                    |
 *          routing/scheduling
 *                    |
 *          resilience/recovery
 *                    |
 *                 ZQN/HAL
 *                    |
 *              realization
 *
 * This file therefore provides the adaptation-effect boundary without
 * creating a second effect language, second AST, second IR, or second
 * hardware model.
 *
 * ============================================================================
 */
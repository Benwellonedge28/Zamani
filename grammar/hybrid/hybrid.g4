/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/hybrid.g4
 *
 * Grammar:
 *     Hybrid
 *
 * Status:
 *     CANONICAL HYBRID DOMAIN COMPOSITION GRAMMAR
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no target-language actions, predicates, embedded
 *     code, filesystem access, network access, hardware discovery, or runtime
 *     execution.
 *
 *     The Rust implementation consuming this grammar MUST remain safe Rust.
 *     No `unsafe` Rust is required by this grammar.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * Hybrid is a FIRST-CLASS COMPOSITION DOMAIN of Zamani.
 *
 * It describes source-level computation whose semantic dependency graph may
 * cross or combine computational domains, including:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hardware
 *     HDL
 *     distributed
 *     AI/data
 *     networking
 *     future domains
 *
 * Hybrid does NOT create another language.
 *
 * Hybrid does NOT create another type system.
 *
 * Hybrid does NOT create another expression language.
 *
 * Hybrid does NOT create another statement language.
 *
 * Hybrid does NOT create another quantum IR.
 *
 * Hybrid is a composition boundary over the canonical Zamani language.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Hybrid
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical              quantum                HDL /
 *      semantics              semantics             hardware
 *          |                      |                      |
 *          |                      v                      |
 *          |                  quantum::ir               |
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                        canonical semantic model
 *                                 |
 *                    +------------+------------+
 *                    |            |            |
 *                    v            v            v
 *                optimize      routing      scheduling
 *                                             |
 *                                             v
 *                                       QEC / ZQN /
 *                                        resilience
 *                                             |
 *                                             v
 *                                            HAL
 *                                             |
 *                                             v
 *                                      target realization
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * This file owns ONLY hybrid-domain composition.
 *
 * General syntax remains owned by the canonical grammar domains:
 *
 *     Core
 *     Expressions
 *     Types
 *     Statements
 *     Functions
 *     Resources
 *     Quantum
 *     Classical
 *     HDL
 *     Hardware
 *
 * Hybrid MUST NOT redefine:
 *
 *     expression
 *     statement
 *     block
 *     typeExpression
 *     typeAnnotation
 *     qualifiedName
 *     argumentList
 *     functionDeclaration
 *     resourceItem
 *     quantum operation syntax
 *     quantum measurement syntax
 *     HDL syntax
 *     hardware syntax
 *
 * The hybrid grammar only composes those concepts.
 *
 * ============================================================================
 * CANONICAL PARSER COMPOSITION
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4
 *          |
 *          +--> Hybrid
 *                   |
 *                   +--> HybridFunctions
 *                   +--> HybridResources
 *                   +--> Expressions
 *                   +--> Types
 *                   +--> Statements
 *                   +--> Functions
 *                   +--> Resources
 *
 * Hybrid MUST NOT import ZamaniParser.
 *
 * There is exactly one direction:
 *
 *     leaf/domain grammar
 *          ->
 *     Hybrid
 *          ->
 *     ZamaniParser
 *
 * Never:
 *
 *     Hybrid -> ZamaniParser
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore uses:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file MUST NOT define lexer rules.
 *
 * IMPORTANT:
 *
 * The current repository lexer vocabulary does NOT define:
 *
 *     HYBRID
 *     CONVERT
 *     TO
 *     SYNCHRONIZE
 *
 * Therefore this grammar deliberately does not reference those nonexistent
 * tokens.
 *
 * `hybrid` is treated as a contextual source marker through `identifier`.
 * Semantic validation must require its exact spelling where the hybrid-region
 * construct is used.
 *
 * This avoids adding a new lexical dependency merely for this domain.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Hybrid syntax expresses:
 *
 *     computation
 *     data dependency
 *     domain dependency
 *     control dependency
 *     semantic boundaries
 *     resource requirements
 *     capabilities
 *     constraints
 *     preferences
 *     implementation hints
 *
 * It MUST NOT encode universal hardware limits.
 *
 * This grammar contains no language-level limits for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *     nodes
 *     processes
 *     tasks
 *     channels
 *     memory
 *     storage
 *     registers
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     devices
 *     timelines
 *     circuit depth
 *     operation count
 *
 * A requirement such as:
 *
 *     requires qubits >= n;
 *
 * is semantic/resource intent.
 *
 * It is NOT a parser-imposed machine limit.
 *
 * Likewise:
 *
 *     requires capability("quantum.measurement");
 *
 * is a capability requirement.
 *
 * It does NOT select a physical QPU.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts are deliberately kept distinct.
 *
 * requirement:
 *     mandatory semantic/resource property.
 *
 * constraint:
 *     condition that must remain satisfied.
 *
 * preference:
 *     advisory optimization/deployment preference.
 *
 * hint:
 *     non-semantic implementation guidance.
 *
 * capability:
 *     named property used by semantic/resource analysis.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * CONTEXTUAL HYBRID MARKER
 * ============================================================================
 *
 * The repository's current lexical vocabulary does not reserve `hybrid`.
 *
 * Therefore:
 *
 *     hybridMarker
 *         : identifier
 *
 * is intentional.
 *
 * The semantic validator MUST enforce:
 *
 *     identifier spelling == "hybrid"
 *
 * for a `hybridRegion`.
 *
 * This gives Zamani a contextual domain marker without requiring another
 * globally reserved keyword.
 *
 * It also keeps this grammar compatible with future lexical evolution:
 *
 *     current:
 *         hybrid -> IDENTIFIER
 *
 *     future lexical policy:
 *         hybrid -> dedicated keyword
 *
 * If the latter is adopted, that is a language-version compatibility change,
 * not a reason to duplicate hybrid syntax here.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * The canonical parser depends on these rules:
 *
 *     hybridDeclaration
 *     hybridStatement
 *     hybridExpression
 *
 * `hybridConstruct` is the principal internal composition boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR DECLARATION
 * ============================================================================
 */

parser grammar Hybrid;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Types,
    Statements,
    HybridFunctions,
    HybridResources
;


/*
 * ============================================================================
 * 1. PUBLIC HYBRID DECLARATION
 * ============================================================================
 *
 * A hybrid declaration establishes an explicitly hybrid source region or
 * delegates to an existing canonical declaration/function/resource construct.
 *
 * Ordinary declarations remain owned by Declarations.
 *
 * ============================================================================
 */

hybridDeclaration
    : hybridRegion
    | hybridFunctionDeclaration
    | hybridResourceConstruct
    ;


/*
 * ============================================================================
 * 2. PUBLIC HYBRID STATEMENT
 * ============================================================================
 *
 * Hybrid statements are source-level composition constructs.
 *
 * Ordinary statements remain owned by Statements.
 *
 * ============================================================================
 */

hybridStatement
    : hybridInvocation
    | hybridBinding
    | hybridControl
    | hybridRequirement
    | hybridCapability
    | hybridPreference
    | hybridConstraint
    | hybridHint
    ;


/*
 * ============================================================================
 * 3. PUBLIC HYBRID EXPRESSION
 * ============================================================================
 *
 * Hybrid expressions reuse the canonical expression grammar.
 *
 * A separate precedence hierarchy is intentionally forbidden.
 *
 * ============================================================================
 */

hybridExpression
    : hybridInvocationExpression
    | expression
    ;


/*
 * ============================================================================
 * 4. HYBRID CONSTRUCT
 * ============================================================================
 *
 * This is the central hybrid composition rule.
 *
 * It contains only constructs whose presence carries hybrid-domain structural
 * meaning.
 *
 * ============================================================================
 */

hybridConstruct
    : hybridInvocation
    | hybridBinding
    | hybridControl
    | hybridRequirement
    | hybridCapability
    | hybridPreference
    | hybridConstraint
    | hybridHint
    | hybridResourceConstruct
    | hybridFunctionDeclaration
    ;


/*
 * ============================================================================
 * 5. HYBRID REGION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     hybrid {
 *         ...
 *     }
 *
 * Because `hybrid` is currently a contextual identifier, the parser recognizes
 * an identifier followed by a block. Semantic analysis MUST validate that the
 * identifier text is exactly `hybrid`.
 *
 * This rule has no finite cardinality.
 *
 * ============================================================================
 */

hybridRegion
    : hybridMarker hybridBlock
    ;


hybridMarker
    : identifier
    ;


hybridBlock
    : LBRACE hybridRegionItem* RBRACE
    ;


hybridRegionItem
    : hybridConstruct
    | statement
    ;


/*
 * ============================================================================
 * 6. DOMAIN-QUALIFIED HYBRID INVOCATION
 * ============================================================================
 *
 * Domain-qualified invocation is the principal explicit hybrid boundary.
 *
 * Examples:
 *
 *     quantum::prepare(...)
 *     quantum::measure(...)
 *     classical::postprocess(...)
 *     accelerator::compute(...)
 *     future_domain::operation(...)
 *
 * Operation names remain open-world identifiers.
 *
 * No fixed operation inventory is encoded here.
 *
 * In particular this grammar does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     CUDA operations
 *     FPGA primitives
 *     vendor operations
 *     accelerator instructions
 *
 * Those remain semantic operations, library operations, dialect operations,
 * or target-level operations.
 *
 * ============================================================================
 */

hybridInvocation
    : hybridDomainQualifier
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


hybridInvocationExpression
    : hybridDomainQualifier
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


hybridDomainQualifier
    : hybridDomainName
      DOUBLE_COLON
    ;


/*
 * `quantum` is currently a reserved lexer token, while future/custom domains
 * remain identifiers.
 *
 * Additional globally reserved domain words can be added here only when they
 * already exist in the canonical lexer vocabulary.
 *
 * No vendor/device names are listed.
 */

hybridDomainName
    : QUANTUM
    | GPU
    | identifier
    ;


/*
 * ============================================================================
 * 7. HYBRID BINDING
 * ============================================================================
 *
 * A hybrid binding establishes a named source-level value whose semantic type
 * or provenance may cross computational domains.
 *
 * The grammar does not decide whether the value is:
 *
 *     classical
 *     quantum
 *     measurement-derived
 *     accelerator-produced
 *     hardware-produced
 *     distributed
 *     AI/data
 *
 * That belongs to semantic analysis.
 *
 * ============================================================================
 */

hybridBinding
    : LET
      identifier
      typeAnnotation?
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. HYBRID CONTROL
 * ============================================================================
 *
 * `when` is already part of Zamani's canonical keyword vocabulary.
 *
 * This rule provides an explicit hybrid control boundary without introducing
 * another `if` grammar.
 *
 * Example:
 *
 *     when measurement_result {
 *         quantum::correction(...);
 *     }
 *
 * The condition may be evaluated:
 *
 *     - on a host;
 *     - on a quantum control system;
 *     - in an accelerator;
 *     - in FPGA/control hardware;
 *     - in a simulator;
 *     - by a compiler transformation;
 *     - by a future execution substrate.
 *
 * The grammar does not choose the realization.
 *
 * ============================================================================
 */

hybridControl
    : WHEN
      expression
      block
    ;


/*
 * ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * This is intentionally a lightweight adapter over the canonical expression
 * model.
 *
 * Resource-specific requirements are also available through HybridResources.
 *
 * ============================================================================
 */

hybridRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. CAPABILITY
 * ============================================================================
 *
 * Capability names are open-world semantic names.
 *
 * Example:
 *
 *     capability("quantum.measurement");
 *
 * No finite capability registry belongs in this grammar.
 *
 * ============================================================================
 */

hybridCapability
    : CAPABILITY
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     prefer capability("accelerated.compute");
 *
 * This is advisory.
 *
 * It MUST NOT silently become physical device selection.
 *
 * ============================================================================
 */

hybridPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 *
 * Constraints describe semantic restrictions.
 *
 * They are not placement directives.
 *
 * ============================================================================
 */

hybridConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. HINT
 * ============================================================================
 *
 * Hints are advisory implementation information.
 *
 * Hints MUST NOT change program meaning.
 *
 * ============================================================================
 */

hybridHint
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. DOMAIN BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally structural rather than target-specific.
 *
 * A domain boundary is represented by a domain-qualified operation or a
 * semantically typed binding.
 *
 * No physical transfer primitive is introduced here.
 *
 * ============================================================================
 */

hybridDomainBoundary
    : hybridInvocationExpression
    | hybridBindingExpression
    ;


hybridBindingExpression
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 15. CLASSICAL -> QUANTUM
 * ============================================================================
 *
 * No separate syntax is required.
 *
 * Example:
 *
 *     quantum::prepare(classical_value);
 *
 * The semantic layer determines:
 *
 *     - whether `classical_value` is classical;
 *     - whether the quantum operation accepts it;
 *     - conversion requirements;
 *     - ownership/lifetime;
 *     - representation;
 *     - timing;
 *     - capability requirements.
 *
 * ============================================================================
 */

classicalToQuantum
    : hybridInvocation
    ;


classicalToQuantumExpression
    : hybridInvocationExpression
    ;


/*
 * ============================================================================
 * 16. QUANTUM -> CLASSICAL
 * ============================================================================
 *
 * The inverse boundary is represented by a hybrid binding or invocation
 * expression.
 *
 * Example:
 *
 *     let result = quantum::measure(...);
 *
 * The resulting type remains a semantic concern.
 *
 * ============================================================================
 */

quantumToClassical
    : hybridBinding
    ;


quantumToClassicalExpression
    : hybridBindingExpression
    ;


/*
 * ============================================================================
 * 17. HYBRID VALUE
 * ============================================================================
 *
 * A hybrid value is intentionally represented by the canonical expression
 * system.
 *
 * There is no HybridValue type in the grammar.
 *
 * ============================================================================
 */

hybridValue
    : hybridInvocationExpression
    | expression
    ;


/*
 * ============================================================================
 * 18. HYBRID SEQUENCE
 * ============================================================================
 *
 * Unbounded source-level composition.
 *
 * The grammar never imposes a finite number of:
 *
 *     operations
 *     domains
 *     values
 *     boundaries
 *     statements
 *     controls
 *     resources
 *
 * ============================================================================
 */

hybridConstructSequence
    : hybridConstruct*
    ;


hybridConstructSequenceNonEmpty
    : hybridConstruct+
    ;


/*
 * ============================================================================
 * 19. HYBRID GROUP
 * ============================================================================
 *
 * Structural grouping only.
 *
 * It does not imply:
 *
 *     thread
 *     process
 *     device
 *     queue
 *     scheduling region
 *     hardware controller
 *     synchronization primitive
 *
 * ============================================================================
 */

hybridGroup
    : LBRACE
      hybridConstructSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 20. HYBRID ITEM
 * ============================================================================
 *
 * Stable tooling-facing item boundary.
 *
 * ============================================================================
 */

hybridItem
    : hybridConstruct
    | statement
    ;


/*
 * ============================================================================
 * 21. HYBRID SOURCE ELEMENT
 * ============================================================================
 */

hybridSourceElement
    : hybridConstruct
    ;


/*
 * ============================================================================
 * 22. HYBRID DOMAIN PAIR
 * ============================================================================
 *
 * Used by tooling and semantic analysis to identify a pair of participating
 * logical domains.
 *
 * Example:
 *
 *     quantum::classical
 *
 * The names remain open-world.
 *
 * ============================================================================
 */

hybridDomainPair
    : hybridDomainName
      DOUBLE_COLON
      hybridDomainName
    ;


/*
 * ============================================================================
 * 23. HYBRID ARGUMENT
 * ============================================================================
 *
 * Arguments are ordinary Zamani expressions.
 *
 * ============================================================================
 */

hybridArgument
    : expression
    ;


hybridArgumentList
    : hybridArgument
      (COMMA hybridArgument)*
    ;


/*
 * ============================================================================
 * 24. HYBRID RETURN VALUE
 * ============================================================================
 *
 * No hybrid-specific return type exists.
 *
 * ============================================================================
 */

hybridReturnValue
    : expression
    ;


/*
 * ============================================================================
 * 25. HYBRID CALL
 * ============================================================================
 *
 * Alias for tooling.
 *
 * It does not create another invocation implementation.
 *
 * ============================================================================
 */

hybridCall
    : hybridInvocationExpression
    ;


/*
 * ============================================================================
 * 26. DOMAIN ANNOTATION
 * ============================================================================
 *
 * An annotation is represented using the canonical annotation marker.
 *
 * The domain remains an open semantic name.
 *
 * ============================================================================
 */

hybridDomainAnnotation
    : AT
      hybridDomainName
    ;


/*
 * ============================================================================
 * 27. HYBRID CONDITION
 * ============================================================================
 */

hybridCondition
    : expression
    ;


/*
 * ============================================================================
 * 28. HYBRID TARGET EXPRESSION
 * ============================================================================
 *
 * Target here means semantic execution intent only.
 *
 * It does NOT mean a physical device.
 *
 * Physical target selection belongs to hardware/compile/execution.
 *
 * ============================================================================
 */

hybridTargetExpression
    : expression
    ;


/*
 * ============================================================================
 * 29. HYBRID RESOURCE ADAPTERS
 * ============================================================================
 *
 * Resource syntax is owned by HybridResources/Resources.
 *
 * These adapters provide stable hybrid-domain names without duplicating the
 * resource grammar.
 *
 * ============================================================================
 */

hybridResource
    : hybridResourceConstruct
    ;


hybridRequirementResource
    : hybridRequirementDeclaration
    ;


hybridConstraintResource
    : hybridConstraintDeclaration
    ;


hybridPreferenceResource
    : hybridPreferenceDeclaration
    ;


hybridHintResource
    : hybridHintDeclaration
    ;


hybridCapabilityResource
    : hybridCapabilityDeclaration
    ;


/*
 * ============================================================================
 * 30. FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function syntax remains owned by Functions.
 *
 * HybridFunctions provides adapters:
 *
 *     hybridFunctionDeclaration
 *     hybridFunctionDefinition
 *     hybridFunctionPrototype
 *     hybridFunctionSignature
 *
 * No function syntax is duplicated here.
 *
 * ============================================================================
 */

hybridFunction
    : hybridFunctionDeclaration
    ;


hybridFunctionBody
    : hybridFunctionDefinition
    ;


/*
 * ============================================================================
 * 31. QUANTUM INTEGRATION
 * ============================================================================
 *
 * This grammar deliberately does not import or redefine individual quantum
 * operations.
 *
 * Quantum source constructs enter through:
 *
 *     domain-qualified invocation
 *
 * and ordinary canonical statements/expressions.
 *
 * The semantic pipeline remains:
 *
 *     hybrid source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * No:
 *
 *     HybridQuantumIR
 *     HybridCircuitIR
 *     HybridGateIR
 *
 * may be introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical constructs remain ordinary Zamani expressions/statements.
 *
 * Hybrid composition may therefore surround arbitrary classical computation
 * without creating a second classical language.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hybrid may invoke hardware/HDL semantic operations through open domain
 * names.
 *
 * Example:
 *
 *     accelerator::compute(...);
 *
 * The grammar does NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical device
 *     vendor
 *     device index
 *     topology
 *     memory bank
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed computation remains a domain of the same language.
 *
 * Example:
 *
 *     distributed::map(...);
 *
 * The grammar does not encode:
 *
 *     node count
 *     node identifiers
 *     cluster size
 *     fixed topology
 *     provider
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Hybrid may compose AI/data operations without introducing framework-specific
 * syntax.
 *
 * Example:
 *
 *     ai::infer(model, input);
 *
 * Framework names remain semantic/library identifiers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. NETWORKING INTEGRATION
 * ============================================================================
 *
 * Networking remains downstream from semantic source composition.
 *
 * Hybrid syntax can describe a logical operation or dependency but does not
 * encode physical network topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. ERROR BOUNDARIES
 * ============================================================================
 *
 * The grammar distinguishes structural syntax from later validation.
 *
 * Syntax errors:
 *     handled by the parser.
 *
 * Semantic errors:
 *     handled by semantic analysis.
 *
 * Capability errors:
 *     handled by capability/resource analysis.
 *
 * Resource errors:
 *     handled against an actual target/resource environment.
 *
 * Target errors:
 *     handled during lowering/placement/scheduling/deployment.
 *
 * Runtime errors:
 *     handled by execution infrastructure.
 *
 * A syntactically valid hybrid program therefore does NOT imply that every
 * available target can execute it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * Every hybrid construct must map into the existing domain-neutral frontend
 * AST.
 *
 * The AST must preserve, where applicable:
 *
 *     source span
 *     source ordering
 *     nesting
 *     participating domains
 *     domain boundary
 *     input expressions
 *     output expressions
 *     control dependencies
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *
 * The AST MUST NOT manufacture target-specific objects such as:
 *
 *     PhysicalQubitId
 *     PhysicalCpuId
 *     PhysicalGpuId
 *     PhysicalFpgaId
 *     PhysicalDeviceId
 *     BackendId
 *     ScheduleSlot
 *     HardwareAddress
 *
 * Those belong to downstream representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     domain classification
 *     name resolution
 *     type checking
 *     effect checking
 *     ownership/lifetime validation
 *     domain crossing validation
 *     measurement dependency validation
 *     resource requirement validation
 *     capability validation
 *     constraint validation
 *     preference handling
 *     implementation-hint handling
 *     determinism analysis
 *     target feasibility
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Quantum constructs participating in a hybrid program lower through the
 * repository's canonical:
 *
 *     quantum::ir
 *
 * This grammar creates no quantum IR.
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuit
 *     QuantumRegister
 *     QubitId
 *     PhysicalQubitId
 *
 * as parser-level semantic models.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. QEC / ZQN / RESILIENCE CONTRACT
 * ============================================================================
 *
 * This grammar does not implement:
 *
 *     QEC
 *     syndrome extraction
 *     decoding
 *     logical-error analysis
 *     noise models
 *     fault models
 *     calibration
 *     retries
 *     recovery
 *     rerouting
 *     rescheduling
 *     backend switching
 *
 * Those systems consume the canonical semantic/IR representation downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Hybrid grammar may express dependency.
 *
 * It MUST NOT express scheduler implementation.
 *
 * It does not own:
 *
 *     latency calculation
 *     queue selection
 *     clock alignment
 *     pulse timing
 *     feedback timing
 *     device synchronization
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. HARDWARE / TARGET CONTRACT
 * ============================================================================
 *
 * The grammar never discovers hardware.
 *
 * It never selects:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *
 * It never encodes:
 *
 *     fixed topology
 *     fixed coupling maps
 *     physical addresses
 *     memory-bank IDs
 *     vendor-specific hardware assumptions
 *
 * Target realization occurs downstream:
 *
 *     semantic model
 *          |
 *          v
 *     capability/resource evaluation
 *          |
 *          v
 *     target selection
 *          |
 *          v
 *     placement/routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     lowering
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately contains no artificial finite language ceiling.
 *
 * The same grammar model can represent:
 *
 *     one classical value + one quantum operation
 *
 * through:
 *
 *     arbitrarily large hybrid dependency graphs
 *
 * subject to:
 *
 *     source representation
 *     parser implementation resources
 *     compiler resources
 *     target capabilities
 *     runtime resources
 *     physical resources
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-level finite machine-size ceiling.
 *
 * It does NOT mean an implementation can physically allocate infinite
 * resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer version
 *     grammar version
 *
 * This grammar contains no:
 *
 *     randomness
 *     filesystem access
 *     network access
 *     hardware discovery
 *     environment inspection
 *     clock-dependent behavior
 *     mutable global parser state
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no command execution
 *     no filesystem access
 *     no network access
 *     no credential access
 *     no hardware access
 *     no runtime calls
 *
 * Malformed source is therefore handled entirely through parser diagnostics
 * and recovery.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust.
 *
 * The repository's generated and handwritten Rust frontend must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must remain safe Rust.
 *
 * This file requires no:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe impl
 *     unsafe block
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal grammar assumptions include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also forbidden as universal source semantics:
 *
 *     qpu0
 *     gpu0
 *     cpu0
 *     fpga0
 *     fixed physical qubit numbers
 *     fixed memory addresses
 *     fixed register widths
 *     fixed accelerator counts
 *     fixed topology sizes
 *
 * Numeric literals remain valid program data.
 *
 * Example:
 *
 *     let n = 1024;
 *
 * is a program value.
 *
 * It is NOT a compiler capacity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar preserves the existing public parser entry points:
 *
 *     hybridDeclaration
 *     hybridStatement
 *     hybridExpression
 *
 * Existing specialized hybrid grammars are NOT silently redefined here.
 *
 * In particular, this file does not import the legacy competing grammars:
 *
 *     ClassicalQuantum
 *     HybridQuantumClassical
 *     ClassicalQuantumBoundary
 *     QuantumClassicalControl
 *
 * because several of those files currently overlap in ownership and/or
 * contain legacy vocabulary.
 *
 * They remain migration/reference surfaces until individually normalized.
 *
 * The canonical production path is:
 *
 *     Hybrid
 *        |
 *        +--> HybridFunctions
 *        +--> HybridResources
 *        +--> canonical shared grammars
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. FILE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] parser grammar declaration exists;
 * [x] canonical tokenVocab is used;
 * [x] canonical parser dependencies are explicit;
 * [x] no lexer rules are duplicated;
 * [x] no undefined legacy K_* tokens are used;
 * [x] no nonexistent HYBRID token is required;
 * [x] no nonexistent CONVERT token is required;
 * [x] no nonexistent TO token is required;
 * [x] no nonexistent SYNCHRONIZE token is required;
 * [x] no second expression grammar is created;
 * [x] no second statement grammar is created;
 * [x] no second type grammar is created;
 * [x] no fixed quantum gate inventory is created;
 * [x] no hardware limits are encoded;
 * [x] no physical device identifiers are encoded;
 * [x] no second quantum IR is created;
 * [x] resource semantics remain delegated;
 * [x] function syntax remains delegated;
 * [x] quantum semantics remain delegated;
 * [x] HDL/hardware realization remains downstream;
 * [x] QEC remains downstream;
 * [x] ZQN remains downstream;
 * [x] routing remains downstream;
 * [x] scheduling remains downstream;
 * [x] target discovery remains downstream;
 * [x] deterministic parsing is preserved;
 * [x] safe-Rust compatibility is preserved;
 * [x] unbounded source cardinality is preserved;
 * [x] contextual hybrid syntax is documented;
 * [x] AST integration is defined;
 * [x] semantic integration is defined;
 * [x] IR integration is defined;
 * [x] compatibility boundaries are defined.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. INTEGRATION CHECKLIST FOR THE NEXT FILES
 * ============================================================================
 *
 * This file must NOT require later edits merely because another domain file
 * is completed.
 *
 * The following integration contracts are therefore fixed now:
 *
 * 1. grammar/antlr/ZamaniParser.g4
 *
 *    Continues to import:
 *
 *        Hybrid
 *
 *    and continues to expose:
 *
 *        hybridElement
 *
 *    through:
 *
 *        hybridDeclaration
 *        hybridStatement
 *        hybridExpression
 *
 * 2. grammar/lexer/*
 *
 *    No lexical change is required for this file.
 *
 * 3. grammar/expressions/*
 *
 *    `expression` and `argumentList` remain the canonical expression
 *    interfaces.
 *
 * 4. grammar/types/*
 *
 *    `typeAnnotation` remains canonical.
 *
 * 5. grammar/statements/*
 *
 *    `statement` and `block` remain canonical.
 *
 * 6. grammar/functions/*
 *
 *    Function syntax remains canonical through HybridFunctions.
 *
 * 7. grammar/resources/*
 *
 *    Resource semantics remain canonical through HybridResources.
 *
 * 8. grammar/quantum/*
 *
 *    Quantum operation and measurement semantics remain owned by Quantum.
 *    Hybrid only composes their use.
 *
 * 9. grammar/classical/*
 *
 *    Classical computation remains ordinary Zamani computation.
 *
 * 10. grammar/hdl/*
 *
 *     HDL remains target-independent hardware intent.
 *
 * 11. grammar/hardware/*
 *
 *     Physical realization remains downstream.
 *
 * 12. src/frontend/ast/*
 *
 *     Hybrid must map to domain-neutral AST structures rather than a
 *     hybrid-specific IR.
 *
 * 13. semantic analysis
 *
 *     Must validate domain crossing, type compatibility, effects, resources,
 *     capabilities, ownership, lifetime, and target feasibility.
 *
 * 14. quantum::ir
 *
 *     Remains the sole canonical quantum IR boundary.
 *
 * 15. optimization/routing/scheduling/QEC/ZQN/HAL
 *
 *     Consume downstream semantic representations and never become grammar
 *     dependencies.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. TESTING CONTRACT
 * ============================================================================
 *
 * Required acceptance categories:
 *
 * POSITIVE:
 *
 *     hybrid {
 *         quantum::prepare(input);
 *         classical::postprocess(result);
 *     }
 *
 *     hybrid {
 *         let result = quantum::measure(state);
 *         when result {
 *             quantum::correction(result);
 *         }
 *     }
 *
 *     hybrid {
 *         requires qubits >= required_qubits;
 *         requires capability("quantum.measurement");
 *         prefer capability("accelerated.compute");
 *     }
 *
 * NEGATIVE:
 *
 *     malformed hybrid marker
 *     malformed qualified invocation
 *     missing closing block
 *     missing invocation parenthesis
 *     malformed binding
 *     malformed requirement
 *
 * BOUNDARY:
 *
 *     empty hybrid block
 *     one construct
 *     many constructs
 *     deeply nested hybrid blocks
 *     long argument lists
 *     many domain crossings
 *
 * SCALABILITY:
 *
 *     symbolic resource quantities
 *     arbitrary qubit requirements
 *     arbitrary tensor dimensions
 *     arbitrary operation counts
 *     arbitrary domain counts
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + HDL
 *     quantum + accelerator
 *     AI + quantum
 *     distributed + quantum
 *     classical + quantum + HDL
 *     classical + quantum + HDL + hardware
 *
 * DETERMINISM:
 *
 *     repeated parsing of identical source must produce equivalent parse
 *     structures.
 *
 * HARD-CODING:
 *
 *     tests must demonstrate that increasing a resource quantity does not
 *     require changing this grammar.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 *     grammar/macros/safety.g4
 *
 * GRAMMAR
 *     safety
 *
 * STATUS
 *     Canonical macro-safety syntax component
 *
 * PURPOSE
 *     Defines the parser-level boundary for safety metadata associated with
 *     macro declarations, invocations, expansion, token generation, syntax
 *     generation, and compile-time capabilities.
 *
 * LANGUAGE
 *     Zamani
 *
 * GRAMMAR TECHNOLOGY
 *     ANTLR4 parser grammar
 *
 * COMPILER BASELINE
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * SAFETY
 *     Safe Rust only.
 *     No unsafe Rust is required.
 *     No embedded Rust actions.
 *     No executable grammar predicates.
 *     No host-language execution.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This file defines SOURCE SYNTAX ONLY.
 *
 * It does not implement:
 *
 *     macro expansion
 *     macro execution
 *     capability authorization
 *     sandboxing
 *     filesystem access
 *     network access
 *     process execution
 *     secret access
 *     hardware access
 *     target selection
 *     resource allocation
 *     name resolution
 *     hygiene
 *     provenance allocation
 *     type checking
 *     effect checking
 *     semantic analysis
 *     IR generation
 *     quantum compilation
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     HAL
 *
 * The parser recognizes the syntactic presence and structure of safety
 * metadata. Downstream semantic infrastructure decides whether that metadata
 * is valid, authorized, compatible, deterministic, and safe.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     annotation
 *     annotationList
 *     annotationName
 *     annotationArguments
 *     annotationArgument
 *     annotationValue
 *
 * Those productions belong to the canonical annotation grammar.
 *
 * This file consumes the canonical annotation rule:
 *
 *     annotation
 *
 * This prevents the macro safety subsystem from creating a second annotation
 * language.
 *
 * ============================================================================
 * MACRO OWNERSHIP
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     macroDeclaration
 *     macroParameter
 *     macroInvocation
 *     macroExpression
 *     macroPath
 *     macroExpansion
 *     macroHygiene
 *     macroTokenStream
 *     macroSyntaxTree
 *     macroDiagnostic
 *
 * Existing macro components remain authoritative:
 *
 *     declarations.g4
 *     parameters.g4
 *     invocations.g4
 *     expansion.g4
 *     hygiene.g4
 *     token-stream.g4
 *     syntax-tree.g4
 *     diagnostics.g4
 *
 * This file only provides safety integration points for those components.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Safety metadata is target-independent.
 *
 * A macro safety contract must remain meaningful when generated source is
 * eventually compiled for:
 *
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum simulators
 *     accelerators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * Safety syntax MUST NOT encode:
 *
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_EXPANSION_DEPTH
 *     MAX_GENERATED_NODES
 *
 * Implementation resource budgets are not language limits.
 *
 * ============================================================================
 * SAFETY MODEL
 * ============================================================================
 *
 * Macro safety consists of several independent semantic concerns:
 *
 *     1. Capability authorization
 *     2. Effect authorization
 *     3. Resource authorization
 *     4. Expansion policy
 *     5. Determinism policy
 *     6. Provenance policy
 *     7. Hygiene policy
 *     8. Environment isolation
 *     9. Target-independence
 *    10. Diagnostic/provenance preservation
 *
 * These are intentionally NOT encoded as separate reserved keywords.
 *
 * Instead, the canonical annotation system carries declarative metadata.
 *
 * This avoids keyword proliferation and allows future safety facilities to
 * evolve through the semantic annotation registry without repeatedly changing
 * the lexer.
 *
 * ============================================================================
 * SECURITY INVARIANT
 * ============================================================================
 *
 * Merely writing a safety annotation MUST NEVER grant authority.
 *
 * For example, the parser accepting:
 *
 *     @macro.safety(...)
 *
 * does NOT grant:
 *
 *     filesystem access
 *     network access
 *     subprocess execution
 *     environment access
 *     secret access
 *     hardware access
 *     compiler mutation
 *
 * Authorization is performed downstream.
 *
 * ============================================================================
 * CAPABILITY / REQUIREMENT DISTINCTION
 * ============================================================================
 *
 * Safety metadata must preserve the distinction between:
 *
 *     requirement
 *     permission
 *     capability
 *     prohibition
 *     preference
 *     policy
 *     implementation decision
 *
 * The grammar therefore does not encode these concepts as fixed semantic
 * enums.
 *
 * Their meaning is determined by the canonical semantic annotation registry.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing safety metadata is deterministic.
 *
 * Given identical:
 *
 *     source
 *     lexer configuration
 *     language version
 *     grammar version
 *
 * the parser must produce equivalent structural results.
 *
 * Safety annotations must not cause:
 *
 *     random behavior
 *     time-dependent parsing
 *     hardware-dependent parsing
 *     environment-dependent parsing
 *     network-dependent parsing
 *
 * ============================================================================
 * NO EXECUTABLE MACROS
 * ============================================================================
 *
 * This grammar does not provide a syntax through which a macro can execute
 * arbitrary host-language code.
 *
 * In particular, this file does not introduce:
 *
 *     eval
 *     exec
 *     shell
 *     process
 *     command
 *     spawn-host
 *     read-file
 *     write-file
 *     fetch
 *     network-call
 *     reflection-host
 *
 * Any future compile-time capability must be introduced through the
 * specification/semantic capability system and explicitly authorized.
 *
 * ============================================================================
 * RESOURCE SAFETY
 * ============================================================================
 *
 * The language must remain scalable from tiny programs to programs limited
 * only by available implementation resources.
 *
 * Therefore this grammar contains no fixed resource ceilings.
 *
 * A compiler may enforce configurable budgets for:
 *
 *     source size
 *     token count
 *     AST size
 *     macro expansion work
 *     generated syntax
 *     compilation time
 *     compiler memory
 *     recursion depth
 *     diagnostic volume
 *
 * Such budgets are compiler/resource policy.
 *
 * They are not part of the source-language meaning.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic safety errors belong to semantic analysis.
 *
 * Resource exhaustion belongs to compiler resource diagnostics.
 *
 * Capability failures belong to capability analysis.
 *
 * Target incompatibility belongs to target/resource analysis.
 *
 * A macro safety annotation must never turn a semantic failure into a parser
 * failure merely because the requested capability is unavailable.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces ordinary canonical annotation contexts.
 *
 * The frontend AST must preserve:
 *
 *     annotation identity
 *     annotation arguments
 *     source span
 *     attachment context
 *     macro declaration/invocation relationship
 *     source provenance
 *
 * This file MUST NOT require a second:
 *
 *     MacroSafetyAst
 *     MacroSafetyNode
 *     MacroSafetyIR
 *
 * hierarchy.
 *
 * Safety metadata remains part of the canonical annotation representation
 * until semantic analysis converts it into the repository's canonical
 * semantic model.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Downstream semantic analysis is responsible for:
 *
 *     resolving safety annotation names;
 *     validating annotation arguments;
 *     checking attachment points;
 *     checking capability requests;
 *     checking capability authorization;
 *     checking effect permissions;
 *     checking resource requirements;
 *     checking expansion policy;
 *     checking determinism;
 *     checking provenance requirements;
 *     checking hygiene interactions;
 *     checking security policy;
 *     checking dialect permissions;
 *     checking interoperability restrictions;
 *     producing structured diagnostics.
 *
 * An unknown annotation does not become a safety permission merely because
 * it appears in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Safety metadata is resolved before target-specific lowering.
 *
 * After semantic validation, generated source proceeds through the existing
 * canonical pipeline:
 *
 *     frontend AST
 *          |
 *          v
 *     macro resolution
 *          |
 *          v
 *     controlled expansion
 *          |
 *          v
 *     hygiene / provenance
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> other domain representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     ZQN / HAL / backend
 *
 * No macro safety IR is introduced.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Safety metadata may surround macros that generate quantum source.
 *
 * This file does not know:
 *
 *     qubit count
 *     physical qubit identity
 *     gate set
 *     topology
 *     calibration
 *     QPU
 *     QEC code
 *     ZQN implementation
 *
 * Example source-level intent may eventually describe safety requirements
 * around generated quantum operations, but the actual quantum semantics remain
 * downstream and ultimately cross:
 *
 *     quantum::ir
 *
 * No second quantum IR is created.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Safety annotations may be attached to macros generating:
 *
 *     HDL
 *     hardware intent
 *     accelerators
 *     memory descriptions
 *     communication structures
 *
 * The safety grammar does not determine:
 *
 *     register width
 *     bus width
 *     memory capacity
 *     device count
 *     topology
 *     clock frequency
 *     FPGA resources
 *     ASIC technology
 *
 * Those are downstream target concerns.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Safety metadata may apply to macros producing distributed or networking
 * constructs.
 *
 * The grammar does not grant:
 *
 *     network access
 *     endpoint access
 *     node access
 *     credential access
 *     service access
 *
 * Any such operation requires explicit downstream capability authorization.
 *
 * ============================================================================
 * MACRO EXPANSION BOUNDARY
 * ============================================================================
 *
 * The required lifecycle is:
 *
 *     source
 *       |
 *       v
 *     parse safety metadata
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     resolve macro
 *       |
 *       v
 *     validate safety policy
 *       |
 *       v
 *     controlled expansion
 *       |
 *       v
 *     hygiene / provenance
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical IR
 *
 * The parser MUST NOT perform expansion.
 *
 * The parser MUST NOT execute the macro.
 *
 * The parser MUST NOT grant capabilities.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * This file is a parser grammar and consumes:
 *
 *     ZamaniLexer
 *
 * It is intended to be imported by:
 *
 *     grammar/macros/macros.g4
 *
 * after the existing macro grammar components.
 *
 * The intended composition becomes:
 *
 *     import declarations,
 *            invocations,
 *            expansion,
 *            hygiene,
 *            safety;
 *
 * `macros.g4` remains the macro composition root.
 *
 * This file does not import:
 *
 *     declarations.g4
 *     invocations.g4
 *     expansion.g4
 *     hygiene.g4
 *
 * because the composition root owns that dependency graph.
 *
 * ============================================================================
 * RULE OWNERSHIP
 * ============================================================================
 *
 * macroSafetyAnnotation
 *     safety.g4
 *
 * macroSafetyAnnotations
 *     safety.g4
 *
 * optionalMacroSafetyAnnotations
 *     safety.g4
 *
 * macroDeclarationSafety
 *     safety.g4
 *
 * macroParameterSafety
 *     safety.g4
 *
 * macroInvocationSafety
 *     safety.g4
 *
 * macroExpansionSafety
 *     safety.g4
 *
 * macroGeneratedTokenSafety
 *     safety.g4
 *
 * macroGeneratedSyntaxSafety
 *     safety.g4
 *
 * macroCompileTimeSafety
 *     safety.g4
 *
 * macroSafetyPrefix
 *     safety.g4
 *
 * No other macro grammar file should redefine these rules.
 *
 * ============================================================================
 * ANNOTATION OWNERSHIP
 * ============================================================================
 *
 * `annotation` remains the canonical annotation rule.
 *
 * This file only wraps it.
 *
 * Therefore the following are intentionally absent:
 *
 *     SAFETY
 *     CAPABILITY
 *     PERMISSION
 *     DENY
 *     ALLOW
 *     SANDBOX
 *     PURE
 *     DETERMINISTIC
 *
 * as lexer tokens.
 *
 * These concepts must remain extensible through canonical annotation names and
 * the semantic registry.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No fixed limits are encoded for:
 *
 *     number of safety annotations
 *     number of macro parameters
 *     number of macro invocations
 *     generated source size
 *     generated node count
 *     capability declarations
 *     resource requirements
 *     nested macro structure
 *
 * Repetition uses standard ANTLR repetition constructs.
 *
 * Practical compiler limits remain configurable implementation policy.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The AST/source-map system must preserve the original source locations for
 * every safety annotation.
 *
 * This is necessary for:
 *
 *     diagnostics
 *     IDE support
 *     debugging
 *     provenance
 *     reproducible compilation
 *     macro expansion tracing
 *     security auditing
 *     compatibility tooling
 *
 * This grammar does not allocate source IDs or provenance IDs.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding this grammar component does not introduce new lexer tokens.
 *
 * Existing source without safety annotations remains syntactically valid.
 *
 * Existing annotation syntax remains owned by the canonical annotation
 * grammar.
 *
 * Any newly recognized safety annotation semantics must follow the normal
 * language compatibility process.
 *
 * Safety annotation names must not silently become reserved keywords.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics include only malformed syntax.
 *
 * Semantic diagnostics must distinguish:
 *
 *     unknown safety annotation
 *     invalid annotation arguments
 *     invalid attachment point
 *     unauthorized capability
 *     prohibited compile-time effect
 *     forbidden environment access
 *     forbidden host execution
 *     invalid resource requirement
 *     nondeterministic expansion policy
 *     provenance violation
 *     hygiene violation
 *     target-specific safety violation
 *     incompatible dialect policy
 *
 * This grammar does not manufacture those semantic errors.
 *
 * ============================================================================
 * SECURITY INVARIANTS
 * ============================================================================
 *
 * The following invariant must hold:
 *
 *     syntax != authority
 *
 * Therefore:
 *
 *     @some.capability(...)
 *
 * does not itself grant the capability.
 *
 * The semantic capability system must independently verify:
 *
 *     declaration
 *     authorization
 *     scope
 *     policy
 *     provenance
 *     compatibility
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests must include:
 *
 *     one safety annotation;
 *     multiple safety annotations;
 *     optional safety metadata;
 *     declaration safety;
 *     parameter safety;
 *     invocation safety;
 *     expansion safety;
 *     generated-token safety;
 *     generated-syntax safety;
 *     compile-time safety;
 *     qualified annotation names;
 *     annotation arguments;
 *     nested annotation values;
 *
 * Examples are intentionally generic because semantic annotation names belong
 * to the canonical specification rather than this grammar.
 *
 * Negative syntax tests must include:
 *
 *     malformed annotation;
 *     malformed annotation arguments;
 *     malformed attachment structure;
 *     missing required annotation;
 *     malformed nested annotation value;
 *
 * Semantic negative tests, outside this grammar, must include:
 *
 *     unauthorized capability;
 *     forbidden filesystem capability;
 *     forbidden network capability;
 *     forbidden process capability;
 *     forbidden secret access;
 *     prohibited compiler mutation;
 *     nondeterministic expansion;
 *     invalid provenance;
 *     invalid hygiene interaction;
 *     incompatible target-specific request.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Safety metadata must be tested around macros which eventually produce:
 *
 *     classical constructs;
 *     quantum constructs;
 *     hybrid constructs;
 *     HDL;
 *     hardware intent;
 *     distributed constructs;
 *     AI/data constructs;
 *     networking constructs;
 *     security constructs;
 *     interoperability constructs;
 *     future dialect constructs.
 *
 * The macro safety grammar itself remains domain-neutral.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The parser must produce equivalent structure for repeated parsing of
 * identical source.
 *
 * Safety annotations must not make parsing dependent on:
 *
 *     target hardware;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     environment variables.
 *
 * Semantic safety evaluation must separately define deterministic behavior
 * for reproducible builds.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain universal constants such as:
 *
 *     MAX_MACRO_SAFETY_ANNOTATIONS
 *     MAX_CAPABILITIES
 *     MAX_PERMISSIONS
 *     MAX_EFFECTS
 *     MAX_EXPANSION_DEPTH
 *     MAX_GENERATED_NODES
 *     MAX_GENERATED_TOKENS
 *     MAX_MACRO_SIZE
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Configurable compiler resource budgets are permitted outside grammar
 * semantics.
 *
 * ============================================================================
 * NO UNSAFE RUST
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe impl
 *     unsafe trait
 *     unsafe blocks
 *
 * in the Zamani compiler implementation.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a parser grammar.
 * [x] It uses the canonical ZamaniLexer vocabulary.
 * [x] It introduces no lexer tokens.
 * [x] It introduces no executable grammar actions.
 * [x] It does not redefine annotation syntax.
 * [x] It does not redefine macro declarations.
 * [x] It does not redefine macro invocations.
 * [x] It does not redefine macro expansion.
 * [x] It does not redefine hygiene.
 * [x] It does not redefine token streams.
 * [x] It does not redefine syntax trees.
 * [x] It does not create a safety IR.
 * [x] It does not create a second AST.
 * [x] It does not select hardware.
 * [x] It does not perform resource allocation.
 * [x] It does not execute macros.
 * [x] It does not grant capabilities.
 * [x] It does not access external resources.
 * [x] It has no fixed scalability ceiling.
 * [x] It remains domain-neutral.
 * [x] It preserves POCO-REAF.
 * [x] It remains compatible with the canonical quantum::ir boundary.
 * [x] It is compatible with safe Rust 1.97.1 infrastructure.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Macro safety is a semantic/security property.
 *
 * This grammar only preserves the source-level declaration of that intent.
 *
 * Therefore:
 *
 *     safety syntax
 *          !=
 *     safety authority
 *
 *     capability declaration
 *          !=
 *     capability grant
 *
 *     macro syntax
 *          !=
 *     macro execution
 *
 *     compile-time capability
 *          !=
 *     unrestricted host access
 *
 *     resource requirement
 *          !=
 *     hardware limit
 *
 *     source safety
 *          !=
 *     target-specific implementation
 *
 * The canonical architecture remains:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Forever
 *
 * subject to program semantics, explicit policy, available capabilities,
 * available resources, and valid target realization.
 *
 * ============================================================================
 */

parser grammar safety;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CANONICAL SAFETY ANNOTATION
 * ============================================================================
 *
 * `annotation` is owned by the canonical annotation grammar.
 *
 * This rule intentionally does not inspect its name or arguments.
 *
 * Semantic registration determines whether a particular annotation is a
 * macro-safety annotation.
 */

macroSafetyAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * 2. REQUIRED SAFETY ANNOTATION SEQUENCE
 * ============================================================================
 *
 * One or more canonical annotations.
 *
 * There is no language-level finite maximum.
 */

macroSafetyAnnotations
    : macroSafetyAnnotation+
    ;


/*
 * ============================================================================
 * 3. OPTIONAL SAFETY ANNOTATION SEQUENCE
 * ============================================================================
 */

optionalMacroSafetyAnnotations
    : macroSafetyAnnotation*
    ;


/*
 * ============================================================================
 * 4. REUSABLE SAFETY PREFIX
 * ============================================================================
 *
 * A consumer grammar decides whether the prefix is legal at its particular
 * attachment point.
 */

macroSafetyPrefix
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 5. DECLARATION SAFETY
 * ============================================================================
 *
 * The macro declaration grammar owns `macroDeclaration`.
 *
 * This rule only provides an independently reusable safety metadata boundary.
 *
 * A consumer may compose it before a macro declaration without this file
 * redefining the declaration itself.
 */

macroDeclarationSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 6. PARAMETER SAFETY
 * ============================================================================
 *
 * The parameter grammar owns the parameter structure.
 *
 * This rule only represents optional/required safety metadata associated with
 * a parameter-level macro safety contract.
 */

macroParameterSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 7. INVOCATION SAFETY
 * ============================================================================
 *
 * The invocation grammar owns `macroInvocation` and `macroExpression`.
 *
 * This rule supplies the safety metadata boundary only.
 */

macroInvocationSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 8. EXPANSION SAFETY
 * ============================================================================
 *
 * Expansion structure remains owned by expansion.g4.
 *
 * Safety analysis occurs downstream.
 */

macroExpansionSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 9. GENERATED TOKEN SAFETY
 * ============================================================================
 *
 * This rule does NOT parse token streams.
 *
 * token-stream.g4 owns token-stream syntax.
 *
 * This rule only provides a safety metadata attachment boundary for a
 * construct whose semantics concern generated tokens.
 */

macroGeneratedTokenSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 10. GENERATED SYNTAX SAFETY
 * ============================================================================
 *
 * syntax-tree.g4 owns syntax-tree representation.
 *
 * This rule supplies only the safety metadata boundary.
 */

macroGeneratedSyntaxSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 11. COMPILE-TIME SAFETY
 * ============================================================================
 *
 * This does NOT grant compile-time capabilities.
 *
 * It merely preserves canonical safety metadata for semantic validation.
 */

macroCompileTimeSafety
    : macroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 12. OPTIONAL DECLARATION SAFETY
 * ============================================================================
 *
 * Convenience integration boundary.
 *
 * Empty input is allowed only when the consuming grammar explicitly permits
 * optional safety metadata.
 */

optionalMacroDeclarationSafety
    : optionalMacroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 13. OPTIONAL PARAMETER SAFETY
 * ============================================================================
 */

optionalMacroParameterSafety
    : optionalMacroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 14. OPTIONAL INVOCATION SAFETY
 * ============================================================================
 */

optionalMacroInvocationSafety
    : optionalMacroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 15. OPTIONAL EXPANSION SAFETY
 * ============================================================================
 */

optionalMacroExpansionSafety
    : optionalMacroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 16. OPTIONAL GENERATED TOKEN SAFETY
 * ============================================================================
 */

optionalMacroGeneratedTokenSafety
    : optionalMacroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 17. OPTIONAL GENERATED SYNTAX SAFETY
 * ============================================================================
 */

optionalMacroGeneratedSyntaxSafety
    : optionalMacroSafetyAnnotations
    ;


/*
 * ============================================================================
 * 18. OPTIONAL COMPILE-TIME SAFETY
 * ============================================================================
 */

optionalMacroCompileTimeSafety
    : optionalMacroSafetyAnnotations
    ;
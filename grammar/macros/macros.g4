/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/macros.g4
 *
 * Grammar:
 *     macros
 *
 * Role:
 *     Canonical composition boundary for the Zamani macro grammar.
 *
 * ============================================================================
 *
 * ARCHITECTURAL STATUS
 * ============================================================================
 *
 * This file is the composition root for macro syntax.
 *
 * It is deliberately NOT another implementation of:
 *
 *     macroDeclaration
 *     macroParameterList
 *     macroParameter
 *     macroBody
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * Those productions have dedicated owners:
 *
 *     declarations.g4
 *         owns macro declarations.
 *
 *     invocations.g4
 *         owns macro invocation syntax.
 *
 *     expansion.g4
 *         owns grammar-level syntax associated with expansion metadata,
 *         where applicable.
 *
 *     hygiene.g4
 *         owns grammar-level syntax associated with hygiene/provenance
 *         metadata, where applicable.
 *
 * This file composes those components into one macro grammar boundary.
 *
 * ============================================================================
 *
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Exactly one grammar component owns each parser production.
 *
 * This file therefore MUST NOT redefine productions owned by imported
 * components.
 *
 * The purpose of this file is to provide:
 *
 *     lexer vocabulary boundary
 *             |
 *             v
 *     macro grammar composition
 *             |
 *             +--> declarations
 *             +--> invocations
 *             +--> expansion syntax
 *             +--> hygiene syntax
 *             |
 *             v
 *     canonical Zamani parser
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Macro syntax describes portable source-level structure.
 *
 * Macro syntax MUST NOT encode a particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     device
 *     processor count
 *     core count
 *     thread count
 *     qubit count
 *     register width
 *     memory capacity
 *     machine topology
 *     quantum topology
 *     gate set
 *     scheduler
 *     routing strategy
 *     backend
 *
 * Physical and execution requirements belong to the appropriate semantic,
 * capability, resource, target, scheduling, routing, and runtime layers.
 *
 * Therefore:
 *
 *     source program
 *          |
 *          v
 *     macro syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     macro resolution / controlled expansion
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> future domain representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / target lowering
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * This grammar introduces no language-level finite limits for:
 *
 *     macro declarations
 *     macro parameters
 *     macro arguments
 *     namespace depth
 *     body size
 *     invocation count
 *     expansion result size
 *     source size
 *
 * Repetition and recursion are delegated to the canonical grammar components.
 *
 * Compiler implementations MAY impose configurable admission/resource
 * policies for:
 *
 *     source bytes
 *     token count
 *     parser memory
 *     AST nodes
 *     expansion depth
 *     expansion steps
 *     generated nodes
 *     generated source
 *     compilation time
 *     compiler memory
 *
 * Such limits are implementation/resource policies.
 *
 * They MUST NOT become language semantics encoded in this grammar.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Nothing in this grammar may require knowledge of:
 *
 *     hardware topology
 *     physical qubits
 *     logical qubits
 *     device identifiers
 *     calibration data
 *     native gate sets
 *     processor widths
 *     memory capacities
 *     network topology
 *     accelerator inventory
 *
 * A macro can generate syntax that eventually participates in any Zamani
 * computation domain, but this grammar does not determine the physical
 * realization.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Macros may generate quantum source syntax.
 *
 * This grammar does NOT:
 *
 *     create quantum IR;
 *     modify quantum::ir;
 *     select a QPU;
 *     select a gate set;
 *     allocate qubits;
 *     perform routing;
 *     perform scheduling;
 *     perform QEC;
 *     describe ZQN faults;
 *     select calibration data;
 *     choose a quantum backend.
 *
 * Quantum semantics produced after macro expansion eventually cross the
 * canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This preserves the repository architecture:
 *
 *     grammar
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization / routing / scheduling / resilience / execution
 *
 * ============================================================================
 *
 * CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The macro grammar is domain-neutral.
 *
 * A macro may eventually generate syntax for:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     distributed computation
 *     AI/ML
 *     data processing
 *     networking
 *     cryptography
 *     scientific computing
 *     accelerators
 *     future Zamani domains
 *
 * The macro grammar does not need separate copies of macro syntax for each
 * domain.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * Compiler implementation target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The compiler MUST use safe Rust.
 *
 * This grammar contains no executable Rust actions and therefore introduces
 * no Rust unsafe requirement.
 *
 * A Zamani source-level `unsafe` construct, if supported elsewhere, does NOT
 * authorize `unsafe` Rust in the compiler implementation.
 *
 * ============================================================================
 *
 * SIDE-EFFECT BOUNDARY
 * ============================================================================
 *
 * Parsing this grammar MUST have no external side effects.
 *
 * This grammar MUST NOT:
 *
 *     read files
 *     write files
 *     access networks
 *     execute processes
 *     inspect hardware
 *     access secrets
 *     invoke compiler plugins
 *     execute macro bodies
 *     expand macros
 *     mutate compiler-global state
 *
 * Macro execution/expansion is a downstream compiler operation governed by
 * explicit semantic, capability, security, determinism, provenance and
 * resource policies.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The grammar composition itself is deterministic.
 *
 * Repeated parsing of the same canonical token stream must result in the same
 * structural parse result.
 *
 * Deterministic macro resolution/expansion is NOT implemented here.
 *
 * The downstream macro system must separately guarantee deterministic:
 *
 *     name resolution
 *     overload/parameter matching
 *     expansion ordering
 *     hygiene
 *     provenance
 *     generated-source identity
 *     diagnostics
 *     reproducibility
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This file does not define AST structures.
 *
 * Imported grammar components provide parser contexts that the frontend AST
 * lowers into the repository's canonical source representation.
 *
 * There MUST NOT be:
 *
 *     MacroAst
 *     MacroInvocationAst
 *     MacroExpansionAst
 *
 * as a competing second AST hierarchy merely because these grammar files are
 * separated.
 *
 * The canonical frontend AST remains authoritative.
 *
 * Macro declaration/invocation nodes must preserve enough information for
 * downstream processing to retain:
 *
 *     source span
 *     source identity
 *     declaration identity
 *     invocation identity
 *     qualified path
 *     ordered parameters
 *     ordered arguments
 *     generic arguments where applicable
 *     body/source structure
 *     provenance
 *
 * The precise NodeId/span representation belongs to the frontend AST layer.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes syntactic validity only.
 *
 * These are downstream semantic questions:
 *
 *     Does a macro exist?
 *     Is it visible?
 *     Is the path valid?
 *     Is the macro imported?
 *     Are arguments compatible?
 *     Are generic constraints satisfied?
 *     Is expansion permitted?
 *     Is expansion deterministic?
 *     Is expansion hygienic?
 *     Is the expansion resource-safe?
 *     Does the generated program satisfy capabilities?
 *     Does the generated program satisfy resource constraints?
 *     Does generated quantum syntax have valid semantics?
 *     Does generated hardware syntax satisfy target capabilities?
 *
 * None of those questions belong in this grammar.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexical vocabulary is supplied by ZamaniLexer.
 *
 * Imported grammars therefore reuse:
 *
 *     MACRO
 *     BANG
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     COLON
 *     ASSIGN
 *     DOUBLE_COLON
 *     and other canonical tokens
 *
 * without redefining them.
 *
 * Shared parser rules such as:
 *
 *     identifier
 *     qualifiedName
 *     visibility
 *     genericParameters
 *     typeExpression
 *     expression
 *     parameterList
 *     argumentList
 *     block
 *
 * remain owned by the canonical parser/core grammar.
 *
 * ============================================================================
 *
 * IMPORTANT: NO DUPLICATE RULE OWNERSHIP
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/macros/declarations.g4
 *     grammar/macros/invocations.g4
 *     grammar/macros/expansion.g4
 *     grammar/macros/hygiene.g4
 *
 * and also contains macro-related productions in:
 *
 *     grammar/antlr/Core.g4
 *     grammar/antlr/Meta.g4
 *
 * The production architecture requires exactly one canonical owner for every
 * macro production.
 *
 * Therefore:
 *
 *     declarations.g4
 *         -> canonical macro declaration owner
 *
 *     invocations.g4
 *         -> canonical macro invocation owner
 *
 *     expansion.g4
 *         -> canonical expansion grammar owner
 *
 *     hygiene.g4
 *         -> canonical hygiene/provenance grammar owner
 *
 *     macros.g4
 *         -> canonical macro grammar composition root
 *
 * Core.g4 and Meta.g4 must not independently redefine those same productions
 * after the composition migration is complete.
 *
 * ============================================================================
 *
 * WHY THIS FILE CONTAINS NO MACRO RULES
 * ============================================================================
 *
 * A common failure mode is to create:
 *
 *     macros.g4
 *     declarations.g4
 *     invocations.g4
 *
 * and then place copies of the same rules in all three files.
 *
 * That creates:
 *
 *     ambiguous ownership
 *     parser divergence
 *     incompatible AST assumptions
 *     duplicate generated contexts
 *     difficult versioning
 *     difficult diagnostics
 *     fragile integration
 *
 * This file intentionally avoids that architecture.
 *
 * It is a composition boundary, not a duplicate grammar.
 *
 * ============================================================================
 *
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The imported parser grammars are:
 *
 *     declarations
 *     invocations
 *     expansion
 *     hygiene
 *
 * They must remain parser grammars using the canonical Zamani lexer vocabulary.
 *
 * Their shared-rule dependencies are resolved by the final canonical parser
 * composition.
 *
 * ============================================================================
 *
 * RULE OWNERSHIP TABLE
 * ============================================================================
 *
 * macroDeclaration
 *     declarations.g4
 *
 * macroParameterList
 *     declarations.g4
 *
 * macroParameter
 *     declarations.g4
 *
 * macroParameterDefault
 *     declarations.g4
 *
 * macroBody
 *     declarations.g4
 *
 * macroPath
 *     invocations.g4
 *
 * macroInvocation
 *     invocations.g4
 *
 * macroExpression
 *     invocations.g4
 *
 * expansion-specific syntax
 *     expansion.g4
 *
 * hygiene/provenance-specific syntax
 *     hygiene.g4
 *
 * canonical identifier
 *     core/name grammar
 *
 * canonical qualifiedName
 *     core/name grammar
 *
 * canonical expression
 *     core/expression grammar
 *
 * canonical typeExpression
 *     core/type grammar
 *
 * canonical argumentList
 *     core/expression/call grammar
 *
 * ============================================================================
 *
 * NO DOMAIN DUPLICATION
 * ============================================================================
 *
 * This file must never become the place where the following are introduced:
 *
 *     quantumMacro
 *     classicalMacro
 *     gpuMacro
 *     qpuMacro
 *     fpgaMacro
 *     hdlMacro
 *     hardwareMacro
 *     distributedMacro
 *
 * unless a future language specification establishes a genuinely different
 * source-level syntax with a separately justified ownership boundary.
 *
 * Ordinary macros are domain-neutral.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A macro may eventually produce a resource requirement or capability
 * declaration.
 *
 * This grammar does not interpret it.
 *
 * For example, the distinction between:
 *
 *     requires quantum
 *
 * and:
 *
 *     use device X
 *
 * remains outside macro grammar composition.
 *
 * Likewise, a macro invocation must not silently transform a semantic
 * capability requirement into a physical placement decision.
 *
 * ============================================================================
 *
 * EXPANSION BOUNDARY
 * ============================================================================
 *
 * The lifecycle is:
 *
 *     parse
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     resolve macro
 *       |
 *       v
 *     validate arguments
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
 * Expansion MUST NOT be performed by ANTLR parser actions.
 *
 * ============================================================================
 *
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * A macro system may eventually expose explicitly controlled compile-time
 * capabilities.
 *
 * Such capabilities must be represented and authorized outside this grammar.
 *
 * In particular, syntax such as:
 *
 *     macro ...
 *
 * must never implicitly grant:
 *
 *     filesystem access
 *     network access
 *     subprocess execution
 *     secret access
 *     hardware access
 *     arbitrary host-language execution
 *
 * The grammar only establishes source syntax.
 *
 * ============================================================================
 *
 * VERSIONING
 * ============================================================================
 *
 * `macros.g4` is part of the language syntax composition contract.
 *
 * Adding an imported macro grammar component is compatible only when:
 *
 *     - it does not duplicate existing rule names;
 *     - it does not change existing parse meaning;
 *     - it does not steal ordinary identifiers unexpectedly;
 *     - it does not introduce parser ambiguity;
 *     - its AST destination is defined;
 *     - its semantic destination is defined.
 *
 * Removing an imported component is a language-compatibility change and must
 * follow the language version/migration policy.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * This composition grammar must be validated through the canonical parser.
 *
 * Required positive coverage includes:
 *
 *     macro empty() { }
 *     macro one(value) { }
 *     macro many(a, b, c) { }
 *     macro trailing(a, b,) { }
 *     macro typed(value: T) { }
 *     macro defaulted(value = expression) { }
 *     macro typedDefaulted(value: T = expression) { }
 *     macro generic<T>(value: T) { }
 *
 * and invocation forms such as:
 *
 *     build!()
 *     build!(value)
 *     build!(a, b)
 *     math::build!(value)
 *     package::math::build!(a, b)
 *
 * Cross-domain parser tests must verify that macro syntax can surround or
 * invoke constructs belonging to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI/data
 *     networking
 *     future dialects
 *
 * Negative coverage must include malformed declaration/invocation structures
 * as owned by the imported grammar components.
 *
 * Composition tests must additionally verify:
 *
 *     - exactly one macroDeclaration parse rule;
 *     - exactly one macroInvocation parse rule;
 *     - no duplicate token definitions;
 *     - no duplicate canonical identifier definitions;
 *     - no duplicate qualifiedName definitions;
 *     - no parser ambiguity introduced by macro composition.
 *
 * ============================================================================
 *
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must not test arbitrary finite maxima as language semantics.
 *
 * Boundary testing should instead use implementation-configurable resource
 * budgets at the compiler/test harness level.
 *
 * Test dimensions include:
 *
 *     increasing parameter count
 *     increasing argument count
 *     increasing body size
 *     increasing namespace depth
 *     increasing invocation nesting
 *     increasing generated syntax size
 *
 * The grammar itself contains no MAX_* constants.
 *
 * ============================================================================
 *
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given the same source and same lexer configuration:
 *
 *     parse(source)
 *
 * must produce structurally equivalent results on repeated runs.
 *
 * The test must not depend on:
 *
 *     wall-clock time
 *     filesystem ordering
 *     network state
 *     hardware availability
 *     backend availability
 *     random expansion state
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `grammar/macros/macros.g4` is complete when:
 *
 * 1. It is the sole composition root for the macro grammar.
 *
 * 2. It does not duplicate macro declarations.
 *
 * 3. It does not duplicate macro invocations.
 *
 * 4. It does not duplicate lexer rules.
 *
 * 5. It uses the canonical Zamani lexer vocabulary.
 *
 * 6. It composes the dedicated declaration and invocation grammars.
 *
 * 7. It composes expansion/hygiene grammar components where those grammars
 *    expose parser-owned syntax.
 *
 * 8. Core/shared parser rules remain owned by their canonical components.
 *
 * 9. Macro syntax remains domain-neutral.
 *
 * 10. No machine/resource/hardware limits are encoded.
 *
 * 11. No Rust executable actions are present.
 *
 * 12. No unsafe Rust is required.
 *
 * 13. No filesystem/network/process/hardware access is possible during
 *     parsing.
 *
 * 14. The composed parser has exactly one owner for every macro production.
 *
 * 15. AST and semantic destinations are documented and stable.
 *
 * 16. `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * 17. Positive, negative, cross-domain, scalability and determinism tests
 *     pass through the canonical parser.
 *
 * 18. The grammar remains compatible with Rust 1.97 / 1.97.1 compiler
 *     infrastructure.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * GRAMMAR COMPOSITION
 * ============================================================================
 *
 * `macros` is intentionally a parser composition grammar.
 *
 * The imported grammars provide the actual productions.
 *
 * ANTLR combines their parser rules into the importing grammar while retaining
 * the canonical lexer vocabulary supplied below.
 * ============================================================================
 */

parser grammar macros;

options {
    tokenVocab = ZamaniLexer;
}

import declarations, invocations, expansion, hygiene;
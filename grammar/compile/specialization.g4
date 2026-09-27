/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/specialization.g4
 *
 * Grammar:
 *     CompileSpecialization
 *
 * Status:
 *     Production compilation-specialization parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Generated compiler integration MUST use safe Rust.
 *     Rust `unsafe` MUST NOT be required by this grammar or its compiler
 *     integration.
 *
 * Portability:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL COMPILATION SPECIALIZATION POLICY.
 *
 * It describes how compilation may specialize an already-valid semantic
 * program or compilation artifact according to semantic information available
 * to the compiler.
 *
 * This file does NOT perform specialization.
 *
 * It does NOT:
 *
 *     - monomorphize;
 *     - infer generic arguments;
 *     - solve generic constraints;
 *     - evaluate constants;
 *     - select a physical device;
 *     - allocate hardware;
 *     - choose physical qubits;
 *     - choose CPU cores;
 *     - choose GPU devices;
 *     - choose FPGA resources;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN;
 *     - construct quantum::ir;
 *     - construct a second IR;
 *     - execute code;
 *     - inspect the host environment;
 *     - inspect filesystem state;
 *     - inspect network state.
 *
 * The compiler and semantic layers own those operations.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> generic resolution
 *          +--> type analysis
 *          +--> constraint solving
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> portability analysis
 *          |
 *          v
 *     specialization planning
 *          |
 *          +--> no specialization
 *          +--> deferred specialization
 *          +--> partial specialization
 *          +--> generic specialization
 *          +--> target-aware specialization
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum::ir           HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                            optimization
 *                                 |
 *                            routing/scheduling
 *                                 |
 *                            resilience/QEC
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * Specialization is therefore a semantic/compiler concern, not a hardware
 * allocation mechanism.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - compilation-specialization declarations;
 *     - specialization planning intent;
 *     - specialization mode;
 *     - specialization conditions;
 *     - specialization requirements;
 *     - specialization constraints;
 *     - specialization preferences;
 *     - specialization hints;
 *     - specialization capabilities;
 *     - specialization resources;
 *     - specialization portability policy;
 *     - specialization determinism policy;
 *     - specialization reproducibility policy;
 *     - specialization fallback policy;
 *     - specialization provenance intent;
 *     - specialization cache/reuse intent;
 *     - specialization scope;
 *     - specialization policy composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - explicit generic specialization request syntax;
 *     - generic declarations;
 *     - generic parameter declarations;
 *     - generic type applications;
 *     - type inference;
 *     - monomorphization;
 *     - constant evaluation;
 *     - partial evaluation implementation;
 *     - optimization algorithms;
 *     - optimization pass implementation;
 *     - target declarations;
 *     - target selection;
 *     - hardware descriptions;
 *     - resource declarations;
 *     - capability declarations;
 *     - physical placement;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime dispatch.
 *
 * ============================================================================
 * SPECIALIZATION AUTHORITY SEPARATION
 * ============================================================================
 *
 * Explicit source specialization is owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Compile-time control is owned by:
 *
 *     grammar/compile/compile-time.g4
 *
 * Target-selection policy is owned by:
 *
 *     grammar/compile/target-selection.g4
 *
 * Optimization intent is owned by:
 *
 *     grammar/compile/optimization.g4
 *
 * Generic declarations are owned by:
 *
 *     grammar/functions/
 *     grammar/types/
 *
 * This file provides the compilation-policy boundary between those systems.
 *
 * It MUST NOT redefine:
 *
 *     specializationRequest
 *     specializationTarget
 *     specializationTypeArguments
 *     specializationValueArguments
 *     genericParameters
 *     genericParameter
 *     typeExpression
 *     expression
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Specialization MUST preserve the semantic meaning of the source program.
 *
 * A specialization may change implementation representation while preserving
 * the observable semantic contract.
 *
 * Valid conceptual intent:
 *
 *     compile specialize target
 *     compile specialize target with (...)
 *     compile specialize target if condition
 *
 * The exact specialization strategy is resolved downstream.
 *
 * The grammar MUST NOT require:
 *
 *     a particular CPU;
 *     a particular GPU;
 *     a particular FPGA;
 *     a particular QPU;
 *     a particular ASIC;
 *     a particular cloud provider;
 *     a particular node;
 *     a particular physical qubit;
 *     a particular memory bank;
 *     a particular register width;
 *     a particular topology.
 *
 * ============================================================================
 * NO ARTIFICIAL RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar contains no universal limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_SPECIALIZATIONS
 *     MAX_TYPE_ARGUMENTS
 *     MAX_VALUE_ARGUMENTS
 *
 * Nor may it encode disguised equivalents.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * rather than finite enumerations.
 *
 * Actual compiler resource protection belongs to explicit compiler policy,
 * semantic analysis, and implementation configuration.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts are deliberately distinct.
 *
 * REQUIREMENT:
 *
 *     mandatory semantic property.
 *
 * CONSTRAINT:
 *
 *     condition restricting an otherwise valid specialization.
 *
 * PREFERENCE:
 *
 *     non-mandatory desired property.
 *
 * HINT:
 *
 *     non-binding information for downstream planning.
 *
 * CAPABILITY:
 *
 *     semantic capability requirement.
 *
 * RESOURCE:
 *
 *     resource requirement or resource expression.
 *
 * They MUST NOT be collapsed into a generic compiler-options map.
 *
 * ============================================================================
 * EXAMPLE SEMANTIC DISTINCTION
 * ============================================================================
 *
 * Valid:
 *
 *     requires qubits >= n
 *
 * Valid:
 *
 *     requires memory >= required_memory
 *
 * Valid:
 *
 *     requires capability("quantum.measurement")
 *
 * Valid:
 *
 *     requires capability("tensor.compute")
 *
 * These describe semantic requirements.
 *
 * They do NOT mean:
 *
 *     use physical qubit 17
 *     use GPU 0
 *     use CPU core 7
 *     use memory bank 3
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * CANONICAL IR RULE
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It MUST NOT introduce:
 *
 *     SpecializationIR
 *     CompilationSpecializationIR
 *     QuantumSpecializationIR
 *     HardwareSpecializationIR
 *
 * Specialization lowers through the existing semantic representation.
 *
 * Quantum specialization MUST ultimately reach:
 *
 *     quantum::ir
 *
 * There is exactly one canonical quantum IR boundary.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CompileSpecialization;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Core owns names, paths, attributes and blocks.
 *
 * Types owns canonical type expressions.
 *
 * Expressions owns canonical expression syntax.
 *
 * This file consumes those contracts rather than redefining them.
 */

import Core,
       Types,
       Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the public compilation-specialization boundary.
 *
 * It intentionally has a distinct name from:
 *
 *     specializationDeclaration
 *
 * in grammar/metaprogramming/specialization.g4.
 *
 * This prevents the two grammars from competing for ownership.
 */

compileSpecializationDeclaration
    : COMPILE K_SPECIALIZE
      compileSpecializationTarget
      compileSpecializationClause*
      SEMI?
    ;


/*
 * ============================================================================
 * 2. TARGET
 * ============================================================================
 *
 * A specialization target is symbolic.
 *
 * It may refer to:
 *
 *     function;
 *     type;
 *     module;
 *     algorithm;
 *     kernel;
 *     circuit;
 *     model;
 *     hardware abstraction;
 *     domain-neutral semantic entity.
 *
 * Semantic analysis determines whether the target is actually specializable.
 */

compileSpecializationTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. SPECIALIZATION CLAUSE
 * ============================================================================
 *
 * Clauses are open-ended in number.
 *
 * Their semantic categories remain explicit.
 */

compileSpecializationClause
    : compileSpecializationArguments
    | compileSpecializationCondition
    | compileSpecializationMode
    | compileSpecializationRequirement
    | compileSpecializationConstraint
    | compileSpecializationPreference
    | compileSpecializationHint
    | compileSpecializationCapability
    | compileSpecializationResource
    | compileSpecializationPortability
    | compileSpecializationDeterminism
    | compileSpecializationReproducibility
    | compileSpecializationFallback
    | compileSpecializationCachePolicy
    | compileSpecializationScope
    | compileSpecializationMetadata
    ;


/*
 * ============================================================================
 * 4. ARGUMENTS
 * ============================================================================
 *
 * Explicit generic/type/value specialization arguments remain semantically
 * compatible with the canonical metaprogramming specialization system.
 *
 * This grammar does not redefine generic declarations.
 *
 * Type arguments are canonical type expressions.
 *
 * Value arguments are canonical expressions.
 *
 * The semantic layer determines whether the supplied values are legal
 * compile-time specialization parameters.
 */

compileSpecializationArguments
    : K_WITH
      LPAREN
      compileSpecializationArgumentList?
      RPAREN
    ;


compileSpecializationArgumentList
    : compileSpecializationArgument
      (
          COMMA
          compileSpecializationArgument
      )*
      COMMA?
    ;


compileSpecializationArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 5. CONDITION
 * ============================================================================
 *
 * The condition determines whether the specialization request is applicable.
 *
 * It is a semantic expression.
 *
 * The parser does not evaluate it.
 */

compileSpecializationCondition
    : IF expression
    ;


/*
 * ============================================================================
 * 6. SPECIALIZATION MODE
 * ============================================================================
 *
 * Mode is represented symbolically.
 *
 * The grammar deliberately does not enumerate compiler implementation
 * strategies.
 *
 * Examples may include:
 *
 *     eager
 *     deferred
 *     partial
 *     adaptive
 *     target_aware
 *
 * These are semantic policy values, not parser-level algorithms.
 */

compileSpecializationMode
    : identifier
    ;


/*
 * ============================================================================
 * 7. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory.
 */

compileSpecializationRequirement
    : REQUIRES expression
    ;


/*
 * ============================================================================
 * 8. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts the legal specialization space.
 */

compileSpecializationConstraint
    : CONSTRAIN expression
    ;


/*
 * ============================================================================
 * 9. PREFERENCE
 * ============================================================================
 *
 * A preference is non-binding.
 */

compileSpecializationPreference
    : PREFER expression
    ;


/*
 * ============================================================================
 * 10. HINT
 * ============================================================================
 *
 * A hint is non-binding implementation guidance.
 */

compileSpecializationHint
    : HINT expression
    ;


/*
 * ============================================================================
 * 11. CAPABILITY
 * ============================================================================
 *
 * Capability names remain open-ended.
 *
 * Examples:
 *
 *     capability("quantum.measurement")
 *     capability("gpu.compute")
 *     capability("tensor.compute")
 *
 * The grammar does not enumerate capability names.
 */

compileSpecializationCapability
    : identifier
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 12. RESOURCE
 * ============================================================================
 *
 * Resource expressions are symbolic and semantic.
 *
 * This grammar does not define resource capacities.
 */

compileSpecializationResource
    : identifier
      expression
    ;


/*
 * ============================================================================
 * 13. PORTABILITY
 * ============================================================================
 *
 * Portability policy allows the programmer to state that specialization must
 * remain portable.
 */

compileSpecializationPortability
    : identifier
      expression
    ;


/*
 * ============================================================================
 * 14. DETERMINISM
 * ============================================================================
 *
 * Determinism is a semantic/compiler policy.
 *
 * The grammar only preserves the request.
 */

compileSpecializationDeterminism
    : identifier
      expression
    ;


/*
 * ============================================================================
 * 15. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility is separate from semantic determinism.
 *
 * The semantic/compiler layer determines whether the requested policy can
 * actually be guaranteed.
 */

compileSpecializationReproducibility
    : identifier
      expression
    ;


/*
 * ============================================================================
 * 16. FALLBACK
 * ============================================================================
 *
 * Fallback permits a specialization strategy to provide an alternative when
 * a specialization cannot be realized.
 *
 * Fallback is semantic policy.
 *
 * It is NOT runtime exception handling.
 */

compileSpecializationFallback
    : identifier
      compileSpecializationFallbackTarget
    ;


compileSpecializationFallbackTarget
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 17. CACHE / REUSE POLICY
 * ============================================================================
 *
 * Specialization may be reusable.
 *
 * This grammar expresses intent only.
 *
 * Cache implementation remains owned by compiler infrastructure.
 */

compileSpecializationCachePolicy
    : identifier
      expression?
    ;


/*
 * ============================================================================
 * 18. SCOPE
 * ============================================================================
 *
 * Scope identifies where specialization intent applies semantically.
 *
 * It does not define a new lexical scope.
 */

compileSpecializationScope
    : identifier
      qualifiedName
    ;


/*
 * ============================================================================
 * 19. METADATA
 * ============================================================================
 *
 * Attributes remain owned by Core.
 */

compileSpecializationMetadata
    : attribute
    ;


/*
 * ============================================================================
 * 20. POLICY BLOCK
 * ============================================================================
 *
 * A structured policy is useful for large specialization specifications.
 *
 * This rule is deliberately separate from the primary declaration so that
 * future composition layers can embed the policy without duplicating its
 * contents.
 */

compileSpecializationPolicyBlock
    : LBRACE
      compileSpecializationPolicyEntry*
      RBRACE
    ;


compileSpecializationPolicyEntry
    : compileSpecializationPolicyAssignment
    | compileSpecializationRequirement
    | compileSpecializationConstraint
    | compileSpecializationPreference
    | compileSpecializationHint
    | compileSpecializationCapability
    | compileSpecializationResource
    | compileSpecializationPortability
    | compileSpecializationDeterminism
    | compileSpecializationReproducibility
    | compileSpecializationFallback
    | compileSpecializationCachePolicy
    | compileSpecializationScope
    | compileSpecializationMetadata
    ;


compileSpecializationPolicyAssignment
    : identifier ASSIGN expression SEMI?
    | identifier COLON expression SEMI?
    ;


/*
 * ============================================================================
 * 21. BLOCK FORM
 * ============================================================================
 *
 * This form is intended for compilation orchestration where specialization
 * contains many independently expressed policies.
 *
 * It does not introduce a second block grammar.
 */

compileSpecializationBlockDeclaration
    : COMPILE K_SPECIALIZE
      compileSpecializationTarget
      compileSpecializationPolicyBlock
    ;


/*
 * ============================================================================
 * 22. COMPLETE DISPATCH
 * ============================================================================
 *
 * This is the preferred composition point for compile.g4 / compilation.g4.
 *
 * A composition grammar should reference this rule rather than reproduce its
 * alternatives.
 */

compileSpecialization
    : compileSpecializationDeclaration
    | compileSpecializationBlockDeclaration
    ;


/*
 * ============================================================================
 * 23. EXPRESSION BRIDGE
 * ============================================================================
 *
 * Explicitly named bridge for downstream grammar components that need to
 * embed specialization policy values.
 *
 * No expression syntax is redefined.
 */

compileSpecializationExpression
    : expression
    ;


/*
 * ============================================================================
 * 24. TYPE BRIDGE
 * ============================================================================
 *
 * Type semantics remain owned by Types.
 *
 * This bridge intentionally does not create a specialization-specific type
 * grammar.
 */

compileSpecializationType
    : typeExpression
    ;


/*
 * ============================================================================
 * 25. NAME BRIDGE
 * ============================================================================
 *
 * Name resolution remains owned by Core.
 */

compileSpecializationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The parser MUST lower these productions into the repository's
 * domain-neutral frontend AST.
 *
 * The AST representation should preserve at least:
 *
 *     target;
 *     arguments;
 *     argument ordering;
 *     clauses;
 *     clause ordering;
 *     conditions;
 *     source spans;
 *     metadata;
 *     source provenance.
 *
 * This grammar MUST NOT define:
 *
 *     SpecializationKey;
 *     MonomorphizationKey;
 *     BackendSpecialization;
 *     QuantumSpecializationIR;
 *     HardwareSpecializationIR.
 *
 * Those are implementation/semantic concepts.
 *
 * ============================================================================
 * 27. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the target;
 *     - determining whether the target is specializable;
 *     - resolving supplied arguments;
 *     - checking argument compatibility;
 *     - checking generic constraints;
 *     - checking compile-time evaluability;
 *     - checking specialization requirements;
 *     - checking resource requirements;
 *     - checking capability requirements;
 *     - checking portability;
 *     - determining whether specialization is legal;
 *     - determining whether specialization preserves semantics;
 *     - determining whether specialization is deterministic;
 *     - determining whether specialization can be reused;
 *     - determining whether specialization should be performed;
 *     - producing diagnostics.
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 * 28. SPECIALIZATION RESULT MODEL
 * ============================================================================
 *
 * Semantic analysis may produce one of several outcomes:
 *
 *     no specialization
 *     deferred specialization
 *     partial specialization
 *     complete specialization
 *     target-aware specialization
 *     rejected specialization
 *
 * These are semantic outcomes.
 *
 * They are deliberately NOT encoded as a finite grammar enumeration.
 *
 * ============================================================================
 * 29. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions remain symbolic.
 *
 * Valid:
 *
 *     requires qubits >= n
 *
 * Valid:
 *
 *     requires memory >= required_memory
 *
 * Valid:
 *
 *     requires capability("tensor.compute")
 *
 * Invalid as a language-level architecture:
 *
 *     MAX_QUBITS = ...
 *     MAX_GPUS = ...
 *     MAX_MEMORY = ...
 *
 * A program may contain numeric values as ordinary program semantics.
 *
 * What is prohibited is converting those values into universal compiler
 * ceilings.
 *
 * ============================================================================
 * 30. HARDWARE CONTRACT
 * ============================================================================
 *
 * Specialization may be influenced by hardware capabilities discovered
 * downstream.
 *
 * It MUST NOT encode hardware realization in the source grammar.
 *
 * In particular, this grammar does not define:
 *
 *     physical_qubit(...)
 *     cpu_core(...)
 *     gpu_device(...)
 *     fpga_slice(...)
 *     memory_bank(...)
 *     device_address(...)
 *
 * Target-specific realization belongs to:
 *
 *     hardware/
 *     resources/
 *     compile/target.g4
 *     compile/target-selection.g4
 *     execution/
 *     HAL
 *
 * ============================================================================
 * 31. QUANTUM CONTRACT
 * ============================================================================
 *
 * Specialization may apply to quantum algorithms and quantum abstractions.
 *
 * Examples of semantic specialization include:
 *
 *     specialization of a parameterized quantum algorithm;
 *     specialization for logical rather than physical qubits;
 *     specialization according to available quantum capabilities;
 *     specialization according to symbolic resource requirements.
 *
 * The grammar MUST NOT encode:
 *
 *     a fixed gate set;
 *     a fixed qubit count;
 *     a fixed physical topology;
 *     physical qubit IDs;
 *     QPU vendor assumptions;
 *     calibration data;
 *     QEC implementation;
 *     routing implementation.
 *
 * The downstream quantum path remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic quantum representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * 32. CLASSICAL CONTRACT
 * ============================================================================
 *
 * Classical specialization may cover:
 *
 *     scalar algorithms;
 *     vectorization;
 *     tensor operations;
 *     numeric algorithms;
 *     symbolic algorithms;
 *     data transformations;
 *     parallel algorithms;
 *     accelerator-independent implementations.
 *
 * The grammar does not enumerate processor instruction sets.
 *
 * ============================================================================
 * 33. HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Specialization may apply to parameterized hardware or HDL abstractions.
 *
 * It MUST NOT transform source syntax into a fixed chip description.
 *
 * No universal:
 *
 *     wire width;
 *     register width;
 *     memory size;
 *     pipeline depth;
 *     device count;
 *     clock frequency
 *
 * is encoded here.
 *
 * HDL and hardware semantics remain authoritative in:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * ============================================================================
 * 34. AI / DATA CONTRACT
 * ============================================================================
 *
 * The same specialization mechanism may apply to:
 *
 *     models;
 *     tensors;
 *     kernels;
 *     datasets;
 *     data pipelines;
 *     inference;
 *     training;
 *     distributed computation.
 *
 * Framework-specific specialization remains outside the universal grammar.
 *
 * ============================================================================
 * 35. DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Specialization may adapt a semantic program to available distributed
 * resources.
 *
 * The grammar MUST NOT enumerate:
 *
 *     node 0;
 *     node 1;
 *     node N;
 *     fixed cluster sizes.
 *
 * Node counts and placement remain resource/target/deployment semantics.
 *
 * ============================================================================
 * 36. PORTABILITY CONTRACT
 * ============================================================================
 *
 * A specialization must be portable unless the source explicitly requests a
 * non-portable semantic property.
 *
 * Compiler optimization or specialization must not silently introduce:
 *
 *     vendor dependence;
 *     device dependence;
 *     physical placement dependence;
 *     unavailable capability dependence.
 *
 * If a specialization cannot preserve the declared semantic contract, the
 * compiler must produce an explicit diagnostic or select another legal
 * realization according to the source policy.
 *
 * ============================================================================
 * 37. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Source ordering MUST be preserved.
 *
 * If the program requests deterministic specialization, the semantic/compiler
 * layers must ensure that specialization decisions do not depend on ambient
 * nondeterministic state.
 *
 * This grammar does not introduce randomness.
 *
 * ============================================================================
 * 38. REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Reproducibility is distinct from semantic determinism.
 *
 * Reproducible specialization may depend on:
 *
 *     compiler version;
 *     specialization policy;
 *     canonical inputs;
 *     explicit configuration;
 *     stable resource/capability descriptions;
 *     explicit seeds where appropriate.
 *
 * Such information belongs to compiler/provenance systems.
 *
 * This grammar only preserves source intent.
 *
 * ============================================================================
 * 39. SECURITY CONTRACT
 * ============================================================================
 *
 * Specialization syntax grants no implicit authority to:
 *
 *     filesystem;
 *     network;
 *     subprocesses;
 *     environment variables;
 *     devices;
 *     hardware;
 *     secrets;
 *     compiler-global mutable state.
 *
 * Compile-time evaluation and specialization must obey the existing:
 *
 *     effects/
 *     security/
 *     resources/
 *     interoperability/
 *
 * contracts.
 *
 * ============================================================================
 * 40. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every specialization AST node MUST preserve:
 *
 *     source/file identity;
 *     start position;
 *     end position.
 *
 * Where available, diagnostics should preserve:
 *
 *     target span;
 *     argument span;
 *     clause span;
 *     related constraint span;
 *     specialization provenance.
 *
 * ============================================================================
 * 41. ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors include:
 *
 *     missing target;
 *     malformed argument list;
 *     malformed clause;
 *     malformed block;
 *     malformed expression.
 *
 * Semantic errors include:
 *
 *     unknown target;
 *     target not specializable;
 *     wrong argument count;
 *     duplicate named argument;
 *     unknown specialization parameter;
 *     invalid type argument;
 *     invalid specialization value;
 *     unsatisfied generic constraint;
 *     unsatisfied resource requirement;
 *     unavailable capability;
 *     non-portable specialization;
 *     semantics-changing specialization;
 *     invalid specialization policy.
 *
 * These semantic errors MUST NOT be represented as arbitrary parser limits.
 *
 * ============================================================================
 * 42. DETERMINISTIC ORDERING
 * ============================================================================
 *
 * The following order is semantically observable to tooling and diagnostics:
 *
 *     target
 *     argument order
 *     clause order
 *     source spans
 *
 * The compiler may construct a canonical internal specialization key later,
 * but that canonicalization must not destroy source provenance.
 *
 * ============================================================================
 * 43. COMPILER INTEGRATION
 * ============================================================================
 *
 * Expected downstream flow:
 *
 *     compileSpecializationDeclaration
 *             |
 *             v
 *     frontend AST
 *             |
 *             v
 *     name resolution
 *             |
 *             v
 *     generic/type/value validation
 *             |
 *             v
 *     constraint analysis
 *             |
 *             v
 *     capability/resource analysis
 *             |
 *             v
 *     specialization planning
 *             |
 *       +-----+------+------+------+
 *       |            |             |
 *       v            v             v
 *     deferred    partial      complete
 *       |            |             |
 *       +------------+-------------+
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *                    v
 *              canonical IR
 *
 * Existing monomorphization infrastructure may consume the semantic result.
 *
 * The grammar MUST NOT depend on compiler implementation structs.
 *
 * ============================================================================
 * 44. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime does not parse specialization syntax.
 *
 * Runtime consumes the already-specialized or intentionally deferred semantic
 * representation produced by compilation.
 *
 * Runtime hardware discovery must not become a parser dependency.
 *
 * ============================================================================
 * 45. TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling should be able to:
 *
 *     - syntax-highlight specialization;
 *     - locate specialization targets;
 *     - locate specialization arguments;
 *     - inspect specialization policies;
 *     - navigate to specialization targets;
 *     - preserve source spans;
 *     - format specialization deterministically;
 *     - report unresolved targets;
 *     - report invalid policies;
 *     - display specialization provenance.
 *
 * Formatting must preserve semantic ordering.
 *
 * ============================================================================
 * 46. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same compilation-specialization contract applies to:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     AI;
 *     data;
 *     distributed;
 *     networking;
 *     security;
 *     accelerator;
 *     future computing domains.
 *
 * Domain-specific grammars remain responsible for their own syntax.
 *
 * This file remains domain-neutral.
 *
 * ============================================================================
 * 47. COMPATIBILITY
 * ============================================================================
 *
 * Existing explicit specialization syntax remains owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Existing compile-time specialization remains owned by:
 *
 *     grammar/compile/compile-time.g4
 *
 * This file MUST NOT silently replace those constructs.
 *
 * If the canonical language elects to unify the source spelling in a future
 * revision, that must be an explicit compatibility change with:
 *
 *     specification update;
 *     lexer update;
 *     parser composition update;
 *     AST mapping;
 *     semantic mapping;
 *     conformance tests;
 *     migration documentation.
 *
 * ============================================================================
 * 48. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     compile specialize compute;
 *
 *     compile specialize compute with (width = width);
 *
 *     compile specialize ns::compute with (precision = precision);
 *
 *     compile specialize quantum_algorithm
 *         with (qubits = required_qubits);
 *
 *     compile specialize tensor_kernel
 *         requires capability("tensor.compute");
 *
 *     compile specialize algorithm
 *         requires memory >= required_memory;
 *
 *     compile specialize algorithm
 *         prefers capability("gpu.compute");
 *
 *     compile specialize algorithm {
 *         requires capability("tensor.compute");
 *         prefer vectorization;
 *         hint cache_locality;
 *     }
 *
 * The exact concrete keyword spelling must remain synchronized with the
 * authoritative lexer.
 *
 * Negative syntax tests MUST include:
 *
 *     compile specialize;
 *
 *     compile specialize <...>;
 *
 *     compile specialize target with ();
 *
 *     compile specialize target with (a =);
 *
 *     compile specialize target requires;
 *
 *     compile specialize target { ;
 *
 * Semantic-negative tests MUST include:
 *
 *     unknown specialization target;
 *     non-specializable target;
 *     wrong argument arity;
 *     invalid type argument;
 *     invalid value argument;
 *     unsatisfied generic constraint;
 *     unavailable capability;
 *     unsatisfied resource requirement.
 *
 * Boundary tests MUST include:
 *
 *     deeply nested qualified names;
 *     deeply nested type expressions;
 *     deeply nested value expressions;
 *     large argument lists;
 *     large clause lists;
 *     many specialization declarations.
 *
 * Scalability tests MUST be parameterized.
 *
 * They MUST NOT define a maximum specialization count.
 *
 * Cross-domain tests MUST include:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     AI;
 *     distributed;
 *     accelerator;
 *
 * specialization scenarios.
 *
 * Determinism tests MUST verify identical source produces identical parse
 * structure and source ordering.
 *
 * ============================================================================
 * 49. HARD-CODING AUDIT
 * ============================================================================
 *
 * REQUIRED PASS CONDITIONS:
 *
 *     [x] No MAX_QUBITS.
 *     [x] No MAX_CPUS.
 *     [x] No MAX_GPUS.
 *     [x] No MAX_FPGAS.
 *     [x] No MAX_NODES.
 *     [x] No MAX_MEMORY.
 *     [x] No MAX_THREADS.
 *     [x] No MAX_TENSOR_RANK.
 *     [x] No MAX_REGISTER_WIDTH.
 *     [x] No MAX_NETWORK_SIZE.
 *     [x] No MAX_DEVICE_COUNT.
 *     [x] No physical-device enumeration.
 *     [x] No physical-qubit enumeration.
 *     [x] No vendor enumeration.
 *     [x] No fixed topology.
 *     [x] No fixed memory size.
 *     [x] No fixed register width.
 *     [x] No fixed tensor rank.
 *     [x] No fixed node count.
 *
 * Program values remain legal.
 *
 * For example:
 *
 *     let n = 1024;
 *
 * remains ordinary program semantics.
 *
 * The prohibited construct is a universal compiler rule such as:
 *
 *     compiler supports at most 1024 specialization resources
 *
 * ============================================================================
 * 50. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Grammar scalability is provided by:
 *
 *     repetition;
 *     symbolic references;
 *     compositional structures;
 *     canonical expressions;
 *     canonical types.
 *
 * The grammar does not assume infinite implementation resources.
 *
 * Large programs remain subject to:
 *
 *     compiler memory;
 *     compiler time;
 *     configured resource budgets;
 *     operating-system limits;
 *     available hardware.
 *
 * Those are implementation constraints, not language ceilings.
 *
 * Compiler implementations should prefer iterative processing and appropriate
 * worklists when processing very large specialization sets.
 *
 * ============================================================================
 * 51. RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no embedded Rust;
 *     - no unsafe;
 *     - no filesystem calls;
 *     - no network calls;
 *     - no subprocess execution.
 *
 * Generated parser/compiler integration MUST support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Compiler implementation MUST use safe Rust.
 *
 * ============================================================================
 * 52. INTEGRATION WITH EXISTING FILES
 * ============================================================================
 *
 * REQUIRED composition changes:
 *
 * 1. grammar/compile/compilation.g4
 *
 *    Import:
 *
 *        CompileSpecialization
 *
 *    and expose:
 *
 *        compileSpecializationDeclaration
 *
 *    through its compilation-element/plan composition.
 *
 * 2. grammar/compile/compile.g4
 *
 *    The compile composition grammar should expose the dedicated
 *    specialization entry point rather than duplicating these productions.
 *
 * 3. grammar/compile/compile-time.g4
 *
 *    `compileTimeSpecialization` remains the compile-time-control construct.
 *
 *    It should delegate to the specialization contract where appropriate
 *    rather than independently inventing another specialization model.
 *
 * 4. grammar/metaprogramming/specialization.g4
 *
 *    Remains authoritative for explicit generic specialization syntax.
 *
 *    It must NOT import this file merely to resolve ordinary generic
 *    specialization requests.
 *
 * 5. grammar/compile/target-selection.g4
 *
 *    May reference specialization policy semantically, but must not redefine
 *    specialization syntax.
 *
 * 6. grammar/compile/optimization.g4
 *
 *    May consume specialization results as downstream optimization context,
 *    but must not redefine specialization declarations.
 *
 * 7. grammar/Zamani.g4
 *
 *    Must expose compilation-specialization only through the canonical
 *    compilation composition boundary.
 *
 * 8. grammar/lexer/
 *
 *    `specialize` MUST have exactly one canonical lexical representation.
 *
 *    The repository currently contains historical references to both
 *    `K_SPECIALIZE` and `SPECIALIZE`.
 *
 *    These must be reconciled by the lexer authority.
 *
 *    This grammar intentionally does not declare a second lexer token.
 *
 * ============================================================================
 * 53. DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct direction:
 *
 *     lexer
 *       |
 *       v
 *     Core
 *       |
 *       +--> Types
 *       |
 *       +--> Expressions
 *       |
 *       v
 *     CompileSpecialization
 *       |
 *       v
 *     compile composition
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> generic resolution
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> specialization planning
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     canonical IR
 *
 * CompileSpecialization MUST NOT import:
 *
 *     Compilation
 *
 * because that would create a grammar cycle.
 *
 * ============================================================================
 * 54. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one owner for each concern:
 *
 *     generic declarations
 *         -> functions/types
 *
 *     generic type application
 *         -> types
 *
 *     explicit specialization request
 *         -> metaprogramming/specialization.g4
 *
 *     compile-time control
 *         -> compile/compile-time.g4
 *
 *     specialization compilation policy
 *         -> compile/specialization.g4
 *
 *     target intent
 *         -> compile/target.g4
 *
 *     target selection
 *         -> compile/target-selection.g4
 *
 *     optimization intent
 *         -> compile/optimization.g4
 *
 *     resource intent
 *         -> resources/
 *
 *     hardware capability
 *         -> hardware/
 *
 *     quantum semantic representation
 *         -> quantum::ir
 *
 * No duplicate grammar authority is permitted.
 *
 * ============================================================================
 * 55. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has one parser grammar declaration.
 *     [x] It uses the canonical lexer vocabulary.
 *     [x] It has no embedded Rust.
 *     [x] It requires no unsafe Rust.
 *     [x] It does not define an AST.
 *     [x] It does not define an IR.
 *     [x] It does not define a quantum IR.
 *     [x] It does not define hardware.
 *     [x] It does not define physical placement.
 *     [x] It does not define optimization algorithms.
 *     [x] It does not define generic declarations.
 *     [x] It does not redefine ordinary expressions.
 *     [x] It does not redefine ordinary types.
 *     [x] It does not encode hardware ceilings.
 *     [x] It supports open-ended clause cardinality.
 *     [x] It supports symbolic specialization targets.
 *     [x] It preserves source ordering.
 *     [x] It preserves source spans through the AST contract.
 *     [x] It separates requirement/constraint/preference/hint semantics.
 *     [x] It preserves POCO-REAF.
 *
 * Repository integration is complete when:
 *
 *     [ ] CompileSpecialization is imported exactly where required.
 *     [ ] No grammar imports it cyclically.
 *     [ ] `specialize` has one canonical lexer token.
 *     [ ] The frontend AST has a domain-neutral specialization-policy node.
 *     [ ] Semantic analysis validates specialization policy.
 *     [ ] Generic resolution remains outside this grammar.
 *     [ ] Monomorphization consumes semantic specialization results.
 *     [ ] Resource/capability analysis is downstream.
 *     [ ] Quantum specialization reaches `quantum::ir`.
 *     [ ] HDL/hardware specialization reaches its canonical semantic boundary.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Compatibility tests pass.
 *     [ ] Rust 1.97 / 1.97.1 integration passes.
 *     [ ] Repository-wide safe-Rust audit passes.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
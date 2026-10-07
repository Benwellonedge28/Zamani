/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/hdl/parallelism.g4
 *
 * Grammar:
 *     HardwareParallelism
 *
 * Status:
 *     CANONICAL PRODUCTION HDL PARALLELISM PARSER DELEGATE
 *
 * Baseline:
 *     Rust 1.97+
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust;
 *       - no semantic actions;
 *       - no semantic predicates;
 *       - no unsafe implementation;
 *       - no hardware discovery;
 *       - no filesystem access;
 *       - no network access;
 *       - no runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL HDL PARALLELISM STRUCTURE.
 *
 * HDL parallelism describes portable hardware/computational intent such as:
 *
 *     - independent hardware regions;
 *     - replicated logical computation;
 *     - parallel data paths;
 *     - parallel stages;
 *     - hardware task regions;
 *     - producer/consumer parallel structures;
 *     - parallel interfaces;
 *     - logical synchronization boundaries;
 *     - parallel resource requirements;
 *     - scalable symbolic multiplicity;
 *     - parallel implementation preferences;
 *     - parallel implementation constraints;
 *     - parallel capability requirements.
 *
 * This file deliberately does NOT decide:
 *
 *     - how many physical workers exist;
 *     - how many CPU cores exist;
 *     - how many GPU lanes exist;
 *     - how many FPGA resources exist;
 *     - how many ASIC units exist;
 *     - how many accelerators exist;
 *     - how many QPUs exist;
 *     - how many distributed nodes exist;
 *     - which target is selected;
 *     - how work is scheduled;
 *     - how hardware is placed;
 *     - how hardware is routed;
 *     - how synthesis is performed.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative direction:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/parallelism.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     canonical Zamani lexer/parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--------------------+---------------------+
 *          |                    |                     |
 *          v                    v                     v
 *        types             resources            capabilities
 *          |                    |                     |
 *          +--------------------+---------------------+
 *                               |
 *                               v
 *                       hardware semantics
 *                               |
 *                               v
 *                         canonical IR
 *                               |
 *                 +-------------+-------------+
 *                 |             |             |
 *                 v             v             v
 *             optimize      schedule       verify
 *                 |             |             |
 *                 +-------------+-------------+
 *                               |
 *                               v
 *                         placement/routing
 *                               |
 *                               v
 *                           synthesis
 *                               |
 *                               v
 *                         target lowering
 *                               |
 *                               v
 *                              HAL
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL parallelism declarations;
 *     - logical parallel regions;
 *     - parallel members;
 *     - logical replication intent;
 *     - symbolic multiplicity;
 *     - parallel data-path declarations;
 *     - parallel worker/branch references as logical names;
 *     - logical fork/join structure;
 *     - HDL synchronization boundaries;
 *     - parallel dependencies;
 *     - parallel resource intent;
 *     - parallel capability requirements;
 *     - parallel constraints;
 *     - parallel preferences;
 *     - parallel hints;
 *     - parallel properties;
 *     - parallel-local assertions;
 *     - parallel-local generation boundaries;
 *     - parallel-local expressions;
 *     - parallel source-level metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic parallel computation;
 *     - ordinary expressions;
 *     - ordinary statements;
 *     - ordinary types;
 *     - identifiers;
 *     - names;
 *     - generic type semantics;
 *     - task semantics;
 *     - actor semantics;
 *     - channel semantics;
 *     - generic synchronization semantics;
 *     - scheduling algorithms;
 *     - resource allocation;
 *     - capability negotiation;
 *     - placement;
 *     - routing;
 *     - synthesis;
 *     - physical replication;
 *     - hardware discovery;
 *     - runtime execution;
 *     - quantum operations;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * NON-OVERLAPPING OWNERSHIP
 * ============================================================================
 *
 * Generic computational parallelism remains owned by:
 *
 *     grammar/concurrency/parallel.g4
 *
 * Data parallelism remains owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * Task parallelism remains owned by:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * Task/spawn semantics remain owned by the existing concurrency/task system.
 *
 * Generic synchronization remains owned by:
 *
 *     grammar/concurrency/synchronization.g4
 *
 * HDL processes remain owned by:
 *
 *     grammar/hdl/processes.g4
 *
 * HDL pipelines remain owned by:
 *
 *     grammar/hdl/pipelines.g4
 *
 * HDL generation remains owned by:
 *
 *     grammar/hdl/generate.g4
 *
 * HDL state machines remain owned by:
 *
 *     grammar/hdl/state_machines.g4
 *
 * HDL timing remains owned by:
 *
 *     grammar/hdl/timing.g4
 *
 * Physical topology remains owned by:
 *
 *     grammar/hardware/topology.g4
 *
 * Physical placement remains owned by:
 *
 *     grammar/hardware/placement.g4
 *
 * Physical interconnect remains owned by:
 *
 *     grammar/hardware/interconnect.g4
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Parallelism is LOGICAL.
 *
 * A source program may describe:
 *
 *     parallel {
 *         ...
 *     }
 *
 * without requiring a particular physical amount of parallel hardware.
 *
 * Logical parallelism may be realized as:
 *
 *     - serial execution where semantics permit;
 *     - CPU parallel execution;
 *     - vector/SIMD execution;
 *     - GPU execution;
 *     - FPGA replication;
 *     - ASIC replication;
 *     - accelerator execution;
 *     - heterogeneous execution;
 *     - distributed execution;
 *     - quantum/classical orchestration;
 *     - future computational substrates.
 *
 * Therefore:
 *
 *     logical parallelism
 *              !=
 *     physical parallelism
 *
 * A source-level multiplicity describes semantic intent.
 *
 * The compiler may:
 *
 *     - preserve it;
 *     - specialize it;
 *     - replicate it;
 *     - serialize it;
 *     - distribute it;
 *     - vectorize it;
 *     - map it to another execution mechanism;
 *
 * provided that declared semantics and contracts are preserved.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This file MUST NOT impose universal limits on:
 *
 *     parallel regions
 *     branches
 *     workers
 *     lanes
 *     replicas
 *     instances
 *     paths
 *     channels
 *     processes
 *     stages
 *     nodes
 *     devices
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     memory
 *     bandwidth
 *     topology size
 *
 * It MUST NOT define:
 *
 *     MAX_PARALLEL_REGIONS
 *     MAX_BRANCHES
 *     MAX_WORKERS
 *     MAX_LANES
 *     MAX_REPLICAS
 *     MAX_INSTANCES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_CHANNELS
 *     MAX_PARALLELISM
 *
 * Numeric values occurring in source are program semantics.
 *
 * They are NOT universal language capacities.
 *
 * For example:
 *
 *     replicas = 8
 *
 * is a legitimate program value.
 *
 * It must never be interpreted as:
 *
 *     Zamani supports at most eight replicas.
 *
 * ============================================================================
 * RESOURCE EXHAUSTION
 * ============================================================================
 *
 * Parser and compiler implementations MAY impose operational resource
 * safeguards.
 *
 * Such safeguards belong to implementation/security policy and may include:
 *
 *     - memory limits;
 *     - parsing budgets;
 *     - elaboration budgets;
 *     - generation budgets;
 *     - verification budgets;
 *     - compilation timeouts;
 *     - backend resource limits.
 *
 * They MUST NOT become language semantics.
 *
 * The implementation must distinguish:
 *
 *     invalid source
 *
 * from:
 *
 *     implementation resource exhaustion
 *
 * and from:
 *
 *     target resource insufficiency.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     CPU0
 *     GPU0
 *     FPGA0
 *     QPU0
 *     CORE0
 *     physical_lane_0
 *     physical_worker_0
 *     physical_partition_0
 *     physical_pin_0
 *
 * Physical identity belongs downstream.
 *
 * Logical names such as:
 *
 *     lane
 *     worker
 *     branch
 *     replica
 *     path
 *
 * remain source-level semantic identifiers.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This parser consumes:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The following canonical tokens are relevant:
 *
 *     PARALLEL
 *     SPAWN
 *     SYNC
 *     BARRIER
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     RESOURCE
 *     CAPABILITY
 *     WHERE
 *     FOR
 *     IN
 *     CONNECT
 *     TO
 *     PROPERTY
 *
 * No HDL-specific lexer is permitted.
 *
 * No new lexical token is required by this grammar.
 *
 * ============================================================================
 * SHARED GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Types:
 *
 *     grammar/types/types.g4
 *
 * Expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Attributes:
 *
 *     grammar/core/attributes.g4
 *
 * Canonical expression blocks:
 *
 *     grammar/expressions/blocks.g4
 *
 * or the repository's current canonical block composition imported by the
 * expression subsystem.
 *
 * This file does NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     attribute
 *     blockExpression
 *
 * ============================================================================
 * GRAMMAR DESIGN PRINCIPLE
 * ============================================================================
 *
 * The grammar is intentionally generic.
 *
 * Future hardware parallelization strategies should normally be represented
 * by:
 *
 *     properties;
 *     parameters;
 *     capabilities;
 *     requirements;
 *     constraints;
 *     preferences;
 *     dialect metadata;
 *
 * rather than by adding a new keyword for every new architecture.
 *
 * This permits future:
 *
 *     vector engines;
 *     tensor engines;
 *     systolic arrays;
 *     spatial accelerators;
 *     reconfigurable fabrics;
 *     optical compute;
 *     neuromorphic hardware;
 *     quantum-classical systems;
 *     future substrates
 *
 * without changing the universal grammar merely because a new physical
 * realization appears.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The stable HDL composition rule is:
 *
 *     hdlParallelismDeclaration
 *
 * It intentionally does NOT consume EOF.
 *
 * The canonical root owns EOF.
 *
 * ============================================================================
 */

parser grammar HardwareParallelism;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    Attributes
;


/*
 * ============================================================================
 * 1. PUBLIC HDL PARALLELISM DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     parallel compute {
 *         ...
 *     }
 *
 * The declaration name is logical.
 *
 * It does not identify a physical processor, accelerator, FPGA region, or
 * other target resource.
 */

hdlParallelismDeclaration
    : attribute*
      PARALLEL
      identifier
      hdlParallelismParameterBlock?
      hdlParallelismHeaderClause*
      hdlParallelismBody
    ;


/*
 * ============================================================================
 * 2. PARAMETERS
 * ============================================================================
 *
 * Parameters may represent symbolic:
 *
 *     multiplicity;
 *     width;
 *     lane count;
 *     replica count;
 *     data dimensions;
 *     timing quantities;
 *     resource quantities;
 *     implementation preferences.
 *
 * Parameters remain ordinary source-level values.
 */

hdlParallelismParameterBlock
    : LPAREN
      hdlParallelismParameterList?
      RPAREN
    ;

hdlParallelismParameterList
    : hdlParallelismParameter
      (COMMA hdlParallelismParameter)*
      COMMA?
    ;

hdlParallelismParameter
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      hdlParallelismParameterConstraint*
    ;

hdlParallelismParameterConstraint
    : REQUIRES expression
    | WHERE expression
    ;


/*
 * ============================================================================
 * 3. DECLARATION HEADER
 * ============================================================================
 */

hdlParallelismHeaderClause
    : hdlParallelismRequirementClause
    | hdlParallelismConstraintClause
    | hdlParallelismPreferenceClause
    | hdlParallelismHintClause
    | hdlParallelismResourceClause
    | hdlParallelismCapabilityClause
    | hdlParallelismPropertyClause
    | hdlParallelismExpressionClause
    ;


/*
 * ============================================================================
 * 4. BODY
 * ============================================================================
 */

hdlParallelismBody
    : LBRACE
      hdlParallelismMember*
      RBRACE
    ;

hdlParallelismMember
    : attribute*
      (
          hdlParallelRegionDeclaration
        | hdlParallelReplicaDeclaration
        | hdlParallelDataPathDeclaration
        | hdlParallelForkDeclaration
        | hdlParallelJoinDeclaration
        | hdlParallelSynchronizationDeclaration
        | hdlParallelDependencyDeclaration
        | hdlParallelConnectionDeclaration
        | hdlParallelGenerateDeclaration
        | hdlParallelRequirementClause
        | hdlParallelismConstraintClause
        | hdlParallelismPreferenceClause
        | hdlParallelismHintClause
        | hdlParallelismResourceClause
        | hdlParallelismCapabilityClause
        | hdlParallelismPropertyClause
        | hdlParallelismAssertion
        | hdlParallelismExpressionClause
      )
    ;


/*
 * ============================================================================
 * 5. PARALLEL REGION
 * ============================================================================
 *
 * A region exposes logical parallelism.
 *
 * It does not mandate simultaneous physical execution.
 */

hdlParallelRegionDeclaration
    : PARALLEL
      identifier
      hdlParallelRegionParameterBlock?
      hdlParallelRegionHeaderClause*
      hdlParallelRegionBody
    ;

hdlParallelRegionParameterBlock
    : LPAREN
      hdlParallelRegionParameterList?
      RPAREN
    ;

hdlParallelRegionParameterList
    : hdlParallelRegionParameter
      (COMMA hdlParallelRegionParameter)*
      COMMA?
    ;

hdlParallelRegionParameter
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
    ;

hdlParallelRegionHeaderClause
    : hdlParallelismRequirementClause
    | hdlParallelismConstraintClause
    | hdlParallelismPreferenceClause
    | hdlParallelismHintClause
    | hdlParallelismResourceClause
    | hdlParallelismCapabilityClause
    | hdlParallelismPropertyClause
    ;

hdlParallelRegionBody
    : LBRACE
      hdlParallelRegionMember*
      RBRACE
    ;

hdlParallelRegionMember
    : attribute*
      (
          hdlParallelRegionDeclaration
        | hdlParallelReplicaDeclaration
        | hdlParallelDataPathDeclaration
        | hdlParallelForkDeclaration
        | hdlParallelJoinDeclaration
        | hdlParallelSynchronizationDeclaration
        | hdlParallelDependencyDeclaration
        | hdlParallelConnectionDeclaration
        | hdlParallelGenerateDeclaration
        | hdlParallelismRequirementClause
        | hdlParallelismConstraintClause
        | hdlParallelismPreferenceClause
        | hdlParallelismHintClause
        | hdlParallelismResourceClause
        | hdlParallelismCapabilityClause
        | hdlParallelismPropertyClause
        | hdlParallelismAssertion
        | hdlParallelismExpressionClause
      )
    ;


/*
 * ============================================================================
 * 6. LOGICAL REPLICATION
 * ============================================================================
 *
 * Replication is expressed through a symbolic expression.
 *
 * Examples of valid semantic forms:
 *
 *     replicas = count
 *     replicas = available
 *     replicas = desired_parallelism
 *
 * The expression is not interpreted by the parser.
 *
 * Physical replication is downstream.
 */

hdlParallelReplicaDeclaration
    : identifier
      LBRACKET
      expression
      RBRACKET
      hdlParallelReplicaTypeClause?
      hdlParallelReplicaHeaderClause*
      hdlParallelReplicaBody?
      SEMICOLON?
    ;

hdlParallelReplicaTypeClause
    : COLON
      qualifiedName
    ;

hdlParallelReplicaHeaderClause
    : hdlParallelismRequirementClause
    | hdlParallelismConstraintClause
    | hdlParallelismPreferenceClause
    | hdlParallelismHintClause
    | hdlParallelismResourceClause
    | hdlParallelismCapabilityClause
    | hdlParallelismPropertyClause
    ;

hdlParallelReplicaBody
    : LBRACE
      hdlParallelRegionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. PARALLEL DATA PATH
 * ============================================================================
 *
 * A data path represents logical computational flow.
 *
 * It does not represent a physical routing channel.
 */

hdlParallelDataPathDeclaration
    : identifier
      THIN_ARROW
      hdlParallelReferenceList
      hdlParallelDataPathPropertyBlock?
      SEMICOLON?
    ;

hdlParallelDataPathPropertyBlock
    : LBRACE
      hdlParallelismPropertyStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. FORK
 * ============================================================================
 *
 * A fork identifies a logical divergence of computation.
 *
 * Scheduling and physical realization are downstream.
 */

hdlParallelForkDeclaration
    : SPAWN
      identifier
      hdlParallelForkSourceClause?
      hdlParallelForkBody?
      SEMICOLON?
    ;

hdlParallelForkSourceClause
    : FROM
      hdlParallelReferenceList
    ;

hdlParallelForkBody
    : LBRACE
      hdlParallelRegionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. JOIN
 * ============================================================================
 *
 * JOIN is intentionally represented as a semantic property/reference form
 * rather than a new lexer token.
 *
 * The canonical lexical vocabulary does not need a JOIN token.
 *
 * The keyword-like operation is represented by the existing synchronization
 * vocabulary through:
 *
 *     sync(...)
 *
 * or by a generic property/qualified semantic name.
 *
 * This grammar therefore exposes a structured synchronization boundary
 * without modifying the lexer.
 */

hdlParallelJoinDeclaration
    : SYNC
      hdlParallelJoinTarget
      hdlParallelJoinPropertyBlock?
      SEMICOLON?
    ;

hdlParallelJoinTarget
    : LPAREN
      hdlParallelReferenceList?
      RPAREN
    | hdlParallelReferenceList
    ;

hdlParallelJoinPropertyBlock
    : LBRACE
      hdlParallelismPropertyStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. BARRIER / SYNCHRONIZATION
 * ============================================================================
 *
 * The BARRIER token already exists in the canonical lexer.
 *
 * Generic synchronization semantics remain owned by the concurrency
 * synchronization subsystem.
 *
 * This rule describes the HDL boundary at which synchronization participates
 * in hardware intent.
 */

hdlParallelSynchronizationDeclaration
    : BARRIER
      hdlParallelSynchronizationTarget?
      hdlParallelSynchronizationPropertyBlock?
      SEMICOLON?
    ;

hdlParallelSynchronizationTarget
    : LPAREN
      hdlParallelReferenceList?
      RPAREN
    | hdlParallelReferenceList
    ;

hdlParallelSynchronizationPropertyBlock
    : LBRACE
      hdlParallelismPropertyStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 11. DEPENDENCIES
 * ============================================================================
 *
 * Dependencies express logical ordering.
 *
 * They do not prescribe a scheduler.
 */

hdlParallelDependencyDeclaration
    : hdlParallelReferenceList
      DEPENDS
      hdlParallelReferenceList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. CONNECTIONS
 * ============================================================================
 *
 * Connections are logical.
 *
 * They do not describe:
 *
 *     physical wires;
 *     routing tracks;
 *     network cables;
 *     FPGA switch matrices;
 *     ASIC metal.
 *
 * Those belong downstream.
 */

hdlParallelConnectionDeclaration
    : CONNECT
      hdlParallelEndpoint
      TO
      hdlParallelEndpoint
      hdlParallelConnectionPropertyBlock?
      SEMICOLON?
    ;

hdlParallelEndpoint
    : qualifiedName
      hdlParallelIndexSuffix*
    ;

hdlParallelIndexSuffix
    : LBRACKET
      expression
      RBRACKET
    ;

hdlParallelConnectionPropertyBlock
    : LBRACE
      hdlParallelismPropertyStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. GENERATION BOUNDARY
 * ============================================================================
 *
 * HDL structural generation remains owned by grammar/hdl/generate.g4.
 *
 * This rule is an integration adapter only.
 *
 * If the existing generation grammar exposes a canonical public rule named
 * hdlGenerateDeclaration, it may be consumed here.
 */

hdlParallelGenerateDeclaration
    : GENERATE
      hdlParallelGenerateBody
    ;

hdlParallelGenerateBody
    : LBRACE
      hdlParallelRegionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 14. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are hard semantic conditions.
 *
 * They are not scheduling commands.
 */

hdlParallelismRequirementClause
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. CONSTRAINTS
 * ============================================================================
 */

hdlParallelismConstraintClause
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-binding implementation guidance.
 */

hdlParallelismPreferenceClause
    : PREFER
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. HINTS
 * ============================================================================
 *
 * Hints do not change source semantics.
 */

hdlParallelismHintClause
    : HINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 18. RESOURCE INTENT
 * ============================================================================
 *
 * Resource quantities remain expressions.
 *
 * Examples:
 *
 *     resource memory >= required_memory;
 *     resource bandwidth >= required_bandwidth;
 *
 * No physical resource is selected here.
 */

hdlParallelismResourceClause
    : RESOURCE
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. CAPABILITY INTENT
 * ============================================================================
 *
 * Capability names remain open-ended.
 *
 * The grammar does not enumerate:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     accelerator;
 *     future devices.
 *
 * Capability negotiation is downstream.
 */

hdlParallelismCapabilityClause
    : CAPABILITY
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 20. GENERIC PROPERTIES
 * ============================================================================
 *
 * Properties provide extensibility without keyword proliferation.
 *
 * Examples:
 *
 *     property strategy = "replicate";
 *     property ordering = "preserve";
 *     property determinism = true;
 *
 * The semantic model decides whether a property is valid.
 */

hdlParallelismPropertyClause
    : PROPERTY
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;

hdlParallelismPropertyStatement
    : qualifiedName
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PARALLEL EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Ordinary expressions remain owned by Expressions.
 *
 * This rule provides a stable HDL-local adapter.
 */

hdlParallelismExpressionClause
    : expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 22. ASSERTION INTEGRATION
 * ============================================================================
 *
 * Assertion syntax remains deliberately generic here.
 *
 * Detailed verification semantics belong to the HDL assertion/verification
 * subsystem.
 *
 * A canonical assertion declaration may be adapted through this rule once
 * the verification composition exposes its public entry point.
 *
 * The generic property form remains available without duplicating the
 * verification grammar.
 */

hdlParallelismAssertion
    : ASSERT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 23. REFERENCES
 * ============================================================================
 */

hdlParallelReferenceList
    : hdlParallelReference
      (
          COMMA
          hdlParallelReference
      )*
      COMMA?
    ;

hdlParallelReference
    : qualifiedName
      hdlParallelIndexSuffix*
    ;


/*
 * ============================================================================
 * 24. PUBLIC DOMAIN ADAPTERS
 * ============================================================================
 *
 * These adapters give the HDL composition root stable names without
 * duplicating generic concurrency syntax.
 */


/*
 * A logical parallel computation reference.
 */
hdlParallelComputation
    : hdlParallelismDeclaration
    ;


/*
 * A logical parallel region reference.
 */
hdlParallelRegion
    : hdlParallelRegionDeclaration
    ;


/*
 * A logical replica reference.
 */
hdlParallelReplica
    : hdlParallelReplicaDeclaration
    ;


/*
 * A logical data-path reference.
 */
hdlParallelDataPath
    : hdlParallelDataPathDeclaration
    ;


/*
 * A logical fork reference.
 */
hdlParallelFork
    : hdlParallelForkDeclaration
    ;


/*
 * A logical join/synchronization reference.
 */
hdlParallelJoin
    : hdlParallelJoinDeclaration
    ;


/*
 * ============================================================================
 * 25. SEMANTIC EXTENSION BOUNDARY
 * ============================================================================
 *
 * Parallel properties are intentionally open.
 *
 * Future semantic concepts should normally enter through:
 *
 *     parameter
 *     property
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     dialect
 *
 * rather than through a new universal keyword.
 */

hdlParallelismExtension
    : hdlParallelismPropertyClause
    | hdlParallelismRequirementClause
    | hdlParallelismConstraintClause
    | hdlParallelismPreferenceClause
    | hdlParallelismHintClause
    | hdlParallelismResourceClause
    | hdlParallelismCapabilityClause
    ;


/*
 * ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve, as applicable:
 *
 *     - construct kind;
 *     - identifier;
 *     - source span;
 *     - attributes;
 *     - parameters;
 *     - symbolic multiplicity;
 *     - nested region structure;
 *     - logical dependencies;
 *     - logical connections;
 *     - synchronization boundaries;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - capabilities;
 *     - resources;
 *     - properties;
 *     - source ordering.
 *
 * The AST MUST NOT manufacture:
 *
 *     worker IDs;
 *     thread IDs;
 *     core IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     ASIC IDs;
 *     QPU IDs;
 *     node IDs;
 *     physical lane IDs;
 *     physical addresses;
 *     physical routing resources.
 *
 * ============================================================================
 * 27. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether parallel regions are legal;
 *     - dependency correctness;
 *     - data/control dependence;
 *     - effect compatibility;
 *     - mutation conflicts;
 *     - aliasing constraints;
 *     - synchronization requirements;
 *     - determinism;
 *     - resource requirements;
 *     - capability requirements;
 *     - timing constraints;
 *     - memory requirements;
 *     - communication requirements;
 *     - whether serialization is semantically permissible;
 *     - whether replication is semantically permissible.
 *
 * This grammar performs none of those analyses.
 *
 * ============================================================================
 * 28. EFFECT INTEGRATION
 * ============================================================================
 *
 * Parallel HDL constructs participate in the universal effect system.
 *
 * Effects may include:
 *
 *     mutation
 *     io
 *     network
 *     distributed
 *     measurement
 *     quantum
 *     randomness
 *     learning
 *     adaptation
 *     simulation
 *     native
 *     foreign
 *
 * Effect legality belongs downstream.
 *
 * A parallel region MUST NOT silently remove or weaken an effect.
 *
 * ============================================================================
 * 29. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements flow through:
 *
 *     HDL parallelism
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     capability analysis
 *          |
 *          v
 *     target negotiation
 *          |
 *          v
 *     execution/synthesis planning
 *
 * Example semantic intent:
 *
 *     requires capability("parallel.compute");
 *
 *     requires memory >= required_memory;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 * The grammar does not determine where those resources come from.
 *
 * ============================================================================
 * 30. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability names remain extensible.
 *
 * Examples may include:
 *
 *     parallel.compute
 *     vector.compute
 *     tensor.compute
 *     hardware.pipeline
 *     hardware.replication
 *     distributed.compute
 *     quantum.control
 *
 * The grammar does not enumerate or hard-code a finite capability catalogue.
 *
 * ============================================================================
 * 31. SCHEDULING BOUNDARY
 * ============================================================================
 *
 * Scheduling consumes semantic parallelism.
 *
 * The flow is:
 *
 *     logical parallelism
 *          |
 *          v
 *     dependency analysis
 *          |
 *          v
 *     resource/capability analysis
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST NOT implement a scheduler.
 *
 * ============================================================================
 * 32. SERIALIZATION CONTRACT
 * ============================================================================
 *
 * If a target has insufficient physical parallel resources, the compiler may
 * serialize logically parallel work only when doing so preserves declared
 * semantics.
 *
 * This is a backend/execution decision.
 *
 * The source grammar remains unchanged.
 *
 * ============================================================================
 * 33. REPLICATION CONTRACT
 * ============================================================================
 *
 * Logical replication may be lowered to:
 *
 *     - duplicated hardware;
 *     - time multiplexing;
 *     - SIMD/vector execution;
 *     - GPU execution;
 *     - FPGA replication;
 *     - ASIC structures;
 *     - accelerator instances;
 *     - distributed execution;
 *     - another target-specific strategy.
 *
 * The source program does not need a different grammar for each target.
 *
 * ============================================================================
 * 34. PIPELINE INTEGRATION
 * ============================================================================
 *
 * Parallelism may contain or reference pipeline structures.
 *
 * Pipeline syntax remains owned by:
 *
 *     grammar/hdl/pipelines.g4
 *
 * Pipeline timing remains owned by the pipeline/timing subsystems.
 *
 * Parallelism must not duplicate pipeline-stage syntax.
 *
 * ============================================================================
 * 35. PROCESS INTEGRATION
 * ============================================================================
 *
 * Parallel regions may reference HDL processes.
 *
 * Process syntax remains owned by:
 *
 *     grammar/hdl/processes.g4
 *
 * This file does not redefine process declarations.
 *
 * ============================================================================
 * 36. GENERATION INTEGRATION
 * ============================================================================
 *
 * Large parallel structures may be generated.
 *
 * Structural generation remains owned by:
 *
 *     grammar/hdl/generate.g4
 *
 * The generation system determines elaboration.
 *
 * No generated cardinality is hard-coded here.
 *
 * ============================================================================
 * 37. CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Generic concurrency remains owned by:
 *
 *     grammar/concurrency/
 *
 * In particular:
 *
 *     parallel.g4
 *     data-parallel.g4
 *     task-parallel.g4
 *     synchronization.g4
 *
 * This file represents the HDL-facing semantic boundary.
 *
 * It must not become a second general concurrency language.
 *
 * ============================================================================
 * 38. ACTOR INTEGRATION
 * ============================================================================
 *
 * Actor semantics remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * An HDL parallel region may contain or reference actor-driven computation
 * only through the existing concurrency integration.
 *
 * This file does not define actors.
 *
 * ============================================================================
 * 39. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A logical parallel region may eventually be realized across:
 *
 *     one machine;
 *     multiple machines;
 *     a cluster;
 *     a distributed accelerator system;
 *     another distributed substrate.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *
 * Node counts are never encoded here as language limits.
 *
 * ============================================================================
 * 40. QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL parallelism may surround or coordinate quantum computation.
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * Quantum operations ultimately cross:
 *
 *     quantum::ir
 *
 * HDL parallelism MUST NOT:
 *
 *     - define quantum operations;
 *     - enumerate gates;
 *     - define qubit limits;
 *     - define a second quantum IR;
 *     - perform quantum routing;
 *     - implement QEC.
 *
 * ============================================================================
 * 41. HYBRID INTEGRATION
 * ============================================================================
 *
 * Valid semantic composition may include:
 *
 *     classical
 *         |
 *         v
 *     parallel HDL
 *         |
 *         v
 *     accelerator
 *         |
 *         v
 *     quantum
 *         |
 *         v
 *     classical
 *
 * The semantic model preserves the boundaries.
 *
 * ============================================================================
 * 42. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Parallel HDL may realize:
 *
 *     tensor computation;
 *     neural computation;
 *     dataflow;
 *     inference acceleration;
 *     training acceleration;
 *     symbolic computation.
 *
 * AI semantics remain owned by:
 *
 *     grammar/ai/
 *
 * Data semantics remain owned by:
 *
 *     grammar/data/
 *
 * Hardware parallelism describes realization intent rather than replacing
 * those semantic domains.
 *
 * ============================================================================
 * 43. TIMING INTEGRATION
 * ============================================================================
 *
 * Parallelism may carry expressions interpreted by the timing subsystem.
 *
 * Examples:
 *
 *     latency;
 *     throughput;
 *     initiation interval;
 *     synchronization interval;
 *     buffering.
 *
 * Timing semantics remain downstream.
 *
 * This grammar does not assign physical clock cycles.
 *
 * ============================================================================
 * 44. MEMORY INTEGRATION
 * ============================================================================
 *
 * Parallel execution may require:
 *
 *     shared memory;
 *     replicated memory;
 *     partitioned memory;
 *     streaming buffers;
 *     distributed memory;
 *     target-specific memory.
 *
 * Memory semantics remain owned by the memory/hardware subsystems.
 *
 * The grammar expresses requirements rather than physical bank allocation.
 *
 * ============================================================================
 * 45. VERIFICATION INTEGRATION
 * ============================================================================
 *
 * Parallel structures must remain observable by verification.
 *
 * Verification may check:
 *
 *     - safety;
 *     - liveness;
 *     - ordering;
 *     - synchronization;
 *     - race freedom;
 *     - deadlock;
 *     - determinism;
 *     - functional equivalence.
 *
 * The grammar does not perform verification.
 *
 * ============================================================================
 * 46. DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - source text;
 *     - canonical lexical vocabulary;
 *     - grammar version;
 *     - explicitly selected dialect/configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU count;
 *     GPU availability;
 *     FPGA availability;
 *     QPU availability;
 *     target topology;
 *     filesystem state;
 *     network state;
 *     environment state;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * Identical source under identical grammar/configuration must produce identical
 * parse structure and diagnostics.
 *
 * ============================================================================
 * 47. SOURCE ORDER
 * ============================================================================
 *
 * Source order must be preserved in the AST.
 *
 * The grammar MUST NOT assume that source order is an execution schedule.
 *
 * Parallel semantics are determined downstream.
 *
 * ============================================================================
 * 48. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics cover syntactic invalidity.
 *
 * Examples:
 *
 *     missing parallel name;
 *     malformed parameter list;
 *     malformed multiplicity expression;
 *     malformed region;
 *     malformed endpoint;
 *     malformed dependency;
 *     malformed synchronization construct;
 *     malformed property.
 *
 * Semantic diagnostics belong downstream:
 *
 *     cyclic dependency;
 *     conflicting effects;
 *     invalid synchronization;
 *     unsatisfied capability;
 *     insufficient resource;
 *     impossible timing;
 *     invalid hardware mapping;
 *     impossible target realization.
 *
 * ============================================================================
 * 49. AST / IR BOUNDARY
 * ============================================================================
 *
 * The path is:
 *
 *     parallel HDL syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     hardware semantic model
 *          |
 *          v
 *     canonical hardware/domain IR
 *          |
 *          +--> classical IR
 *          |
 *          +--> quantum::ir where applicable
 *          |
 *          +--> target-independent hardware representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     placement/routing
 *          |
 *          v
 *     synthesis/lowering
 *          |
 *          v
 *     HAL
 *
 * This grammar defines no IR.
 *
 * ============================================================================
 * 50. PHYSICAL REALIZATION BOUNDARY
 * ============================================================================
 *
 * Physical replication, placement, routing and resource allocation belong
 * downstream.
 *
 * The grammar does not know whether:
 *
 *     replica[workers]
 *
 * becomes:
 *
 *     one time-multiplexed unit;
 *
 *     many FPGA units;
 *
 *     many ASIC units;
 *
 *     GPU workgroups;
 *
 *     distributed workers;
 *
 *     another implementation.
 *
 * ============================================================================
 * 51. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * Zamani-owned Rust implementation associated with this grammar MUST:
 *
 *     - compile with Rust 1.97+;
 *     - use Rust 2021;
 *     - avoid unsafe blocks;
 *     - avoid unsafe functions;
 *     - avoid unsafe traits;
 *     - preserve structured diagnostics;
 *     - preserve source spans;
 *     - preserve deterministic parser behavior.
 *
 * Recommended crate-level policy:
 *
 *     #![deny(unsafe_code)]
 *
 * ============================================================================
 * 52. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Public source constructs include:
 *
 *     parallel declaration;
 *     parameter blocks;
 *     regions;
 *     replicas;
 *     data paths;
 *     synchronization;
 *     dependencies;
 *     connections;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     resources;
 *     capabilities;
 *     properties.
 *
 * Changes to their syntax require coordination with:
 *
 *     grammar/compatibility/
 *
 * Historical or experimental syntax remains non-canonical unless promoted.
 *
 * ============================================================================
 * 53. DIALECT CONTRACT
 * ============================================================================
 *
 * Vendor-specific or architecture-specific parallel behavior belongs in
 * explicit dialects.
 *
 * A dialect may extend semantics for:
 *
 *     SIMD;
 *     tensor engines;
 *     systolic arrays;
 *     FPGA fabrics;
 *     accelerator arrays;
 *     vendor scheduling;
 *     specialized interconnect;
 *     future architectures.
 *
 * Such dialects MUST NOT silently change the semantics of core parallelism.
 *
 * ============================================================================
 * 54. SECURITY CONTRACT
 * ============================================================================
 *
 * Parallelism does not grant permission to:
 *
 *     allocate arbitrary hardware;
 *     access another process;
 *     access another machine;
 *     access another device;
 *     bypass sandboxing;
 *     bypass authorization;
 *     bypass resource policy.
 *
 * Capability and security systems remain authoritative.
 *
 * ============================================================================
 * 55. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Parallel constructs must remain traceable through:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic parallel model
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     target realization
 *
 * Backend transformations should retain source provenance where the compiler
 * infrastructure supports it.
 *
 * ============================================================================
 * 56. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must pass an audit for:
 *
 *     MAX_*
 *     fixed worker counts;
 *     fixed lane counts;
 *     fixed branch counts;
 *     fixed device counts;
 *     fixed processor counts;
 *     fixed memory sizes;
 *     fixed pipeline sizes;
 *     vendor assumptions;
 *     physical resource assumptions.
 *
 * Legitimate program constants remain legal.
 *
 * Universal implementation ceilings do not.
 *
 * ============================================================================
 * 57. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS MUST INCLUDE:
 *
 *     - minimal parallel declaration;
 *     - parameterized parallel declaration;
 *     - symbolic multiplicity;
 *     - nested parallel regions;
 *     - multiple replicas;
 *     - symbolic replica counts;
 *     - data paths;
 *     - fork;
 *     - synchronization;
 *     - barriers;
 *     - dependencies;
 *     - connections;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - resource requirements;
 *     - capability requirements;
 *     - properties;
 *     - assertions;
 *     - generated structures;
 *     - classical parallel hardware;
 *     - accelerator parallelism;
 *     - heterogeneous parallelism;
 *     - distributed parallel intent;
 *     - quantum/classical parallel boundaries.
 *
 * NEGATIVE TESTS MUST INCLUDE:
 *
 *     - missing parallel name;
 *     - malformed parameter;
 *     - malformed multiplicity;
 *     - malformed region;
 *     - malformed replica;
 *     - malformed endpoint;
 *     - malformed dependency;
 *     - malformed synchronization;
 *     - malformed property;
 *     - malformed requirement.
 *
 * BOUNDARY TESTS MUST INCLUDE:
 *
 *     - smallest legal declaration;
 *     - empty region where legal;
 *     - one logical replica;
 *     - symbolic replica count;
 *     - very large symbolic expressions;
 *     - deeply nested regions;
 *     - many source-level members;
 *     - long qualified names;
 *     - mixed HDL/concurrency boundaries.
 *
 * SCALABILITY TESTS MUST VERIFY:
 *
 *     - no grammar-level parallelism ceiling;
 *     - no fixed branch ceiling;
 *     - no fixed replica ceiling;
 *     - no fixed lane ceiling;
 *     - no fixed worker ceiling;
 *     - symbolic scaling remains representable;
 *     - generated structures remain source-representable.
 *
 * DETERMINISM TESTS MUST VERIFY:
 *
 *     same source
 *         ->
 *     same tokens
 *         ->
 *     same parse structure
 *         ->
 *     same diagnostics.
 *
 * CROSS-DOMAIN TESTS MUST COVER:
 *
 *     classical + HDL parallelism;
 *     quantum + HDL parallelism;
 *     hybrid + HDL parallelism;
 *     AI + HDL parallelism;
 *     data + HDL parallelism;
 *     distributed + HDL parallelism;
 *     networking + HDL parallelism;
 *     security + HDL parallelism.
 *
 * ============================================================================
 * 58. INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/core/attributes.g4
 *
 * HDL COMPOSITION:
 *
 *     grammar/hdl/hdl.g4
 *
 * SIBLING HDL SYSTEMS:
 *
 *     grammar/hdl/processes.g4
 *     grammar/hdl/pipelines.g4
 *     grammar/hdl/generate.g4
 *     grammar/hdl/state_machines.g4
 *     grammar/hdl/timing.g4
 *     grammar/hdl/verification.g4
 *
 * CONCURRENCY SYSTEM:
 *
 *     grammar/concurrency/parallel.g4
 *     grammar/concurrency/data-parallel.g4
 *     grammar/concurrency/task-parallel.g4
 *     grammar/concurrency/synchronization.g4
 *     grammar/concurrency/actors.g4
 *
 * HARDWARE SYSTEM:
 *
 *     grammar/hardware/resources.g4
 *     grammar/hardware/capabilities.g4
 *     grammar/hardware/targets.g4
 *     grammar/hardware/topology.g4
 *     grammar/hardware/placement.g4
 *     grammar/hardware/interconnect.g4
 *
 * DOWNSTREAM:
 *
 *     domain-neutral AST
 *     structural validation
 *     semantic analysis
 *     resource analysis
 *     capability analysis
 *     hardware semantic model
 *     canonical IR
 *     optimization
 *     scheduling
 *     placement
 *     routing
 *     synthesis
 *     HAL
 *
 * QUANTUM:
 *
 *     quantum semantic layer
 *     quantum::ir
 *     QEC
 *     resilience
 *     ZQN
 *
 * ============================================================================
 * 59. REQUIRED hdl.g4 INTEGRATION
 * ============================================================================
 *
 * Because this file is a new HDL grammar delegate, grammar/hdl/hdl.g4 must
 * import this grammar:
 *
 *     HardwareParallelism
 *
 * and add the public declaration to its HDL declaration dispatch:
 *
 *     | hdlParallelismDeclaration
 *
 * The existing hdl.g4 file remains the single HDL composition root.
 *
 * It must NOT copy any rules from this file.
 *
 * ============================================================================
 * 60. REQUIRED Zamani.g4 / ZamaniParser INTEGRATION
 * ============================================================================
 *
 * No new universal root grammar is required.
 *
 * The integration direction remains:
 *
 *     grammar/hdl/parallelism.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *
 * If ZamaniParser currently imports HDL only through hdl.g4, no direct
 * parallelism import should be added to ZamaniParser.
 *
 * This prevents duplicate parser ownership.
 *
 * ============================================================================
 * 61. REQUIRED AST INTEGRATION
 * ============================================================================
 *
 * The AST implementation must provide a domain-neutral representation for:
 *
 *     HdlParallelism
 *     HdlParallelRegion
 *     HdlParallelReplica
 *     HdlParallelDataPath
 *     HdlParallelFork
 *     HdlParallelSynchronization
 *     HdlParallelDependency
 *     HdlParallelConnection
 *
 * Exact Rust names may follow the repository's established AST naming policy.
 *
 * The grammar must not force physical hardware node types into the AST.
 *
 * ============================================================================
 * 62. REQUIRED SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis must lower:
 *
 *     source parallelism
 *          |
 *          +--> dependency graph
 *          +--> effect requirements
 *          +--> resource requirements
 *          +--> capability requirements
 *          +--> timing requirements
 *          +--> synchronization semantics
 *          +--> determinism requirements
 *          |
 *          v
 *     hardware parallel semantic model
 *
 * ============================================================================
 * 63. REQUIRED IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not create IR.
 *
 * The semantic layer must map HDL parallel intent into the repository's
 * canonical IR/hardware representation.
 *
 * If classical computation is involved:
 *
 *     classical IR
 *
 * remains authoritative.
 *
 * If quantum computation is involved:
 *
 *     quantum::ir
 *
 * remains authoritative.
 *
 * No third parallelism-specific IR is introduced merely because this grammar
 * exists.
 *
 * ============================================================================
 * 64. REQUIRED SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling must consume:
 *
 *     parallel regions;
 *     dependencies;
 *     synchronization;
 *     resource requirements;
 *     capabilities;
 *     timing constraints;
 *     target information.
 *
 * Scheduling may choose:
 *
 *     serial;
 *     parallel;
 *     vector;
 *     replicated;
 *     distributed;
 *     heterogeneous;
 *
 * realization.
 *
 * ============================================================================
 * 65. REQUIRED TARGET INTEGRATION
 * ============================================================================
 *
 * The same source structure must remain representable for:
 *
 *     tiny systems;
 *     embedded systems;
 *     CPU systems;
 *     multicore systems;
 *     GPU systems;
 *     FPGA systems;
 *     ASIC systems;
 *     accelerators;
 *     QPU-connected systems;
 *     simulators;
 *     HPC;
 *     clusters;
 *     distributed systems;
 *     future computational targets.
 *
 * Target capability and available resources determine realization.
 *
 * ============================================================================
 * 66. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It exists as the sole HDL parallelism syntax owner.
 *
 *     [ ] It uses the canonical Zamani lexer.
 *
 *     [ ] It does not define lexer rules.
 *
 *     [ ] It does not duplicate generic concurrency parallelism.
 *
 *     [ ] It does not define task semantics.
 *
 *     [ ] It does not define actor semantics.
 *
 *     [ ] It does not define generic synchronization semantics.
 *
 *     [ ] It does not define pipeline syntax.
 *
 *     [ ] It does not define process syntax.
 *
 *     [ ] It does not define physical placement.
 *
 *     [ ] It does not define physical routing.
 *
 *     [ ] It does not define target discovery.
 *
 *     [ ] It does not define scheduling algorithms.
 *
 *     [ ] It does not define synthesis.
 *
 *     [ ] It does not define a new IR.
 *
 *     [ ] It does not define a second quantum IR.
 *
 *     [ ] It has no artificial hardware-size limits.
 *
 *     [ ] It has no MAX_* constants.
 *
 *     [ ] Multiplicity is expression-based.
 *
 *     [ ] Resource requirements are expression-based.
 *
 *     [ ] Capability requirements are extensible.
 *
 *     [ ] Properties are extensible.
 *
 *     [ ] Requirements, constraints, preferences and hints remain distinct.
 *
 *     [ ] Logical parallelism remains distinct from physical parallelism.
 *
 *     [ ] Parser diagnostics remain distinct from semantic diagnostics.
 *
 *     [ ] Resource exhaustion remains distinct from source invalidity.
 *
 *     [ ] Source provenance can be preserved.
 *
 *     [ ] Deterministic parsing is preserved.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] ANTLR generation succeeds.
 *
 *     [ ] Generated Rust remains compatible with Rust 1.97+.
 *
 *     [ ] Zamani-owned Rust remains safe Rust.
 *
 * ============================================================================
 * 67. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This grammar describes:
 *
 *     PORTABLE HDL PARALLEL INTENT
 *
 * It does NOT describe:
 *
 *     PHYSICAL PARALLEL HARDWARE.
 *
 * Therefore:
 *
 *     logical parallelism
 *          !=
 *     worker allocation
 *
 *     logical replication
 *          !=
 *     physical replication
 *
 *     logical connection
 *          !=
 *     physical routing
 *
 *     logical dependency
 *          !=
 *     scheduler implementation
 *
 *     capability requirement
 *          !=
 *     device selection
 *
 *     resource requirement
 *          !=
 *     resource allocation
 *
 * The complete path remains:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     resource/capability analysis
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     placement/routing
 *          |
 *          v
 *     synthesis/lowering
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * The source therefore remains scalable from:
 *
 *     tiny
 *       |
 *     embedded
 *       |
 *     CPU
 *       |
 *     GPU
 *       |
 *     FPGA
 *       |
 *     ASIC
 *       |
 *     accelerator
 *       |
 *     QPU-connected
 *       |
 *     heterogeneous
 *       |
 *     HPC
 *       |
 *     cluster
 *       |
 *     distributed
 *       |
 *     future computational substrate
 *
 * without encoding today's physical limits into the language.
 *
 * ============================================================================
 * END OF grammar/hdl/parallelism.g4
 * ============================================================================
 */
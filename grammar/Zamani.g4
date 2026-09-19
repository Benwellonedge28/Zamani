/**

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/Zamani.g4
* 
* STATUS
* ---
* CANONICAL ANTLR ROOT / UNIVERSAL GRAMMAR ORCHESTRATOR
* 
* PURPOSE
* ---
* This is the single combined ANTLR grammar used to compose the complete
* Zamani language.
* 
* This file intentionally contains only the root composition contract.
* 
* It does NOT duplicate:
* 
* - lexical rules;
* - parser rules;
* - expression rules;
* - type rules;
* - declaration rules;
* - statement rules;
* - classical rules;
* - quantum rules;
* - hybrid rules;
* - HDL rules;
* - hardware rules;
* - distributed rules;
* - AI rules;
* - data rules;
* - networking rules;
* - security rules;
* - resource rules;
* - compilation rules;
* - execution rules;
* - interoperability rules;
* - dialect rules;
* - macro rules;
* - metaprogramming rules.
* 
* Those responsibilities belong to the canonical parser/lexer composition
* grammars imported below.
* 
* ============================================================================
* ARCHITECTURE
* ============================================================================
* 
*                     grammar/Zamani.g4
*                              |
*               +--------------+--------------+
*               |                             |
*               v                             v
*      ZamaniLexer.g4                 ZamaniParser.g4
*               |                             |
*               |                  +----------+----------+
*               |                  |          |          |
*               |                core       types     expressions
*               |                  |          |          |
*               |                  +----------+----------+
*               |                             |
*               |                     declarations
*               |                     statements
*               |                     functions
*               |                     modules
*               |                     effects
*               |                     memory
*               |                     concurrency
*               |                             |
*               |             +---------------+----------------+
*               |             |       |       |       |        |
*               |           classical quantum  HDL   hybrid  hardware
*               |             |       |       |       |        |
*               |             +-------+-------+-------+--------+
*               |                     |
*               |              remaining domains
*               |                     |
*               +---------------------+
*                                     |
*                                     v
*                           Complete Zamani grammar
* 
* The root therefore orchestrates the complete language without knowing the
* implementation details of individual grammar domains.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Zamani follows:
* 
* Program Once
*      |
* Compile Once
*      |
* Run Everywhere
*      |
* Run Anywhere
*      |
* Forever
* 
* The grammar describes portable source-level computation.
* 
* It MUST NOT impose universal limits on:
* 
* qubits
* logical qubits
* physical resources
* CPUs
* cores
* threads
* GPUs
* FPGAs
* accelerators
* QPUs
* nodes
* processes
* tasks
* channels
* memory
* storage
* registers
* vector width
* tensor dimensions
* tensor rank
* devices
* links
* timelines
* modules
* declarations
* functions
* source size
* 
* Any resource limitation belongs to:
* 
* semantic analysis
* resource management
* target capabilities
* compilation
* scheduling
* routing
* runtime
* deployment
* 
* and is therefore outside this root grammar.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This root MUST NOT introduce language-wide constants such as:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ACCELERATORS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_STORAGE
* MAX_REGISTER_WIDTH
* MAX_TENSOR_RANK
* MAX_DEVICES
* MAX_TIMELINES
* 
* Numeric literals appearing in Zamani programs remain program semantics.
* 
* For example:
* 
* 1024
* 
* is a valid program value.
* 
* It MUST NOT be interpreted by this grammar as a universal implementation
* limit.
* 
* ============================================================================
* DOMAIN-NEUTRALITY
* ============================================================================
* 
* Zamani is one language.
* 
* Classical computing, quantum computing, HDL, AI, distributed computing,
* networking, data processing, security, hardware/software co-design and
* future domains are language domains rather than separate root languages.
* 
* The root does not privilege one domain over another.
* 
* Domain-specific syntax enters through ZamaniParser.g4 and its subordinate
* parser hierarchy.
* 
* ============================================================================
* QUANTUM INVARIANT
* ============================================================================
* 
* Quantum syntax is composed by ZamaniParser.g4 and its quantum grammar.
* 
* This root creates NO quantum IR.
* 
* The canonical quantum semantic path remains:
* 
* Zamani source
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      v
* quantum::ir
*      |
*      v
* optimization
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* QEC / resilience
*      |
*      v
* ZQN
*      |
*      v
* HAL
*      |
*      v
* target realization
* 
* The root therefore does not enumerate gates, physical qubits, topology,
* calibration data, routing decisions or hardware identifiers.
* 
* ============================================================================
* HDL / HARDWARE INVARIANT
* ============================================================================
* 
* HDL and hardware syntax are composed downstream through ZamaniParser.g4.
* 
* The root imposes no universal:
* 
* bus width
* register count
* memory capacity
* device count
* accelerator count
* topology size
* clock count
* pipeline depth
* 
* unless a value is explicitly part of the source program's semantics.
* 
* ============================================================================
* RESOURCE / CAPABILITY INVARIANT
* ============================================================================
* 
* Zamani distinguishes portable source intent from target realization.
* 
* Source programs may express:
* 
* requirements
* constraints
* capabilities
* preferences
* hints
* budgets
* negotiation
* 
* The parser does not discover hardware and does not decide whether a target
* satisfies a resource requirement.
* 
* Those decisions belong downstream.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing is determined only by the:
* 
* source text
* selected grammar version
* lexical rules
* parser rules
* explicitly supplied dialect configuration
* 
* Parsing MUST NOT depend on:
* 
* wall-clock time
* randomness
* hardware
* filesystem state
* network state
* environment variables
* runtime state
* target availability
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* This grammar contains no executable target-language actions.
* 
* It performs no:
* 
* filesystem access
* network access
* hardware discovery
* environment inspection
* secret access
* runtime execution
* random selection
* 
* The Rust implementation consuming the generated grammar is maintained for:
* 
* Rust 2021
* Rust 1.97 / 1.97.1
* 
* and must remain safe Rust with no unsafe implementation requirement.
* 
* ============================================================================
* AUTHORITY
* ============================================================================
* 
* Normative specification:
* 
* grammar/specification/
* 
* Canonical combined ANTLR root:
* 
* grammar/Zamani.g4
* 
* Canonical lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Canonical parser:
* 
* grammar/antlr/ZamaniParser.g4
* 
* Rust frontend/conformance:
* 
* src/lexer.rs
* src/parser.rs
* src/ast/
* 
* Canonical quantum semantic boundary:
* 
* quantum::ir
* 
* ============================================================================
* CRITICAL COMPOSITION RULE
* ============================================================================
* 
* This file imports EXACTLY TWO grammars:
* 
* ZamaniParser
* ZamaniLexer
* 
* It MUST NOT import the individual grammar/domain directories directly.
* 
* For example, this file must NOT contain imports such as:
* 
* Core
* Types
* Expressions
* Quantum
* HDL
* Hardware
* AI
* Distributed
* 
* Those imports belong to the parser/lexer composition hierarchy.
* 
* This produces one clean ownership chain:
* 
* Zamani.g4
*     |
*     +--> ZamaniParser.g4
*     |       |
*     |       +--> core
*     |       +--> types
*     |       +--> expressions
*     |       +--> declarations
*     |       +--> statements
*     |       +--> functions
*     |       +--> modules
*     |       +--> effects
*     |       +--> memory
*     |       +--> concurrency
*     |       +--> classical
*     |       +--> quantum
*     |       +--> hybrid
*     |       +--> hdl
*     |       +--> hardware
*     |       +--> distributed
*     |       +--> ai
*     |       +--> data
*     |       +--> networking
*     |       +--> security
*     |       +--> resources
*     |       +--> compile
*     |       +--> execution
*     |       +--> interoperability
*     |       +--> dialects
*     |       +--> macros
*     |       +--> metaprogramming
*     |
*     +--> ZamaniLexer.g4
*             |
*             +--> canonical lexical hierarchy
* 
* ============================================================================
* ROOT GRAMMAR
* ============================================================================
* 
* The imported parser grammar owns the actual parser rules.
* 
* The imported lexer grammar owns the actual lexer rules.
* 
* This root supplies the single complete-program entry point and therefore
* acts as the final ANTLR composition boundary.
* 
* ============================================================================
  */

grammar Zamani;

import ZamaniParser, ZamaniLexer;

/**

* ============================================================================
* COMPLETE ZAMANI PROGRAM
* ============================================================================
* 
* "program" is the single public entry point for parsing a complete Zamani
* source unit.
* 
* "sourceUnit" is inherited from ZamaniParser.g4.
* 
* EOF is enforced here so a successful parse represents the complete input,
* rather than an accepted prefix followed by unconsumed source.
* 
* ============================================================================
  */
  program
  : sourceUnit EOF
  ;
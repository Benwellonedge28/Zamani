/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/ai/facts.g4
* 
* Grammar:
* AIFacts
* 
* Status:
* CANONICAL AI/KNOWLEDGE FACT-SYNTAX COMPONENT
* 
* Language baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* 
* Implementation safety:
* Safe Rust only.
* No unsafe Rust.
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* ============================================================================
* FEATURE CONTRACT
* ============================================================================
* 
* PURPOSE
* ---
* 
* This file owns the reusable SOURCE-LEVEL FACT REPRESENTATION boundary used
* by Zamani's generic knowledge system.
* 
* A fact is a structured semantic claim or relation that may be consumed by:
* 
* - knowledge assertion;
* - knowledge retraction;
* - knowledge query;
* - reasoning;
* - learning;
* - adaptation;
* - explanation;
* - evidence;
* - provenance;
* - data processing;
* - configuration;
* - scientific computation;
* - hardware/resource knowledge;
* - security knowledge;
* - distributed knowledge;
* - application-defined knowledge.
* 
* The fact grammar is intentionally DOMAIN-NEUTRAL.
* 
* It does not assume that facts belong exclusively to AI.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* canonical parser
*      |
*      v
* AIFacts
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* structural validation
*      |
*      +-------------------+-------------------+
*      |                   |                   |
*      v                   v                   v
*    types              effects          provenance
*      |                   |                   |
*      +-------------------+-------------------+
*                          |
*                          v
*                   semantic knowledge model
*                          |
*      +-------------------+-------------------+
*      |                   |                   |
*      v                   v                   v
*   reasoning           learning          adaptation
*      |                   |                   |
*      +-------------------+-------------------+
*                          |
*                          v
*                  canonical semantic IR
*                          |
*         +----------------+----------------+
*         |                |                |
*         v                v                v
*     classical       quantum::ir       data/other
*                          |
*                          v
*                   downstream lowering
* 
* This grammar MUST NOT bypass the semantic layer.
* 
* ============================================================================
* OWNS
* ============================================================================
* 
* This file owns:
* 
* fact
* factTerm
* factPredicate
* factSubject
* factObject
* factRelation
* factPattern
* factField
* factFieldList
* factValue
* factReference
* factMetadataEntry
* factMetadata
* factQualifier
* factQualifierList
* factArgument
* factArgumentList
* 
* This file owns the STRUCTURE of a fact.
* 
* ============================================================================
* DOES NOT OWN
* ============================================================================
* 
* This file does NOT own:
* 
* - knowledge assertion operation;
* - knowledge retraction operation;
* - knowledge query operation;
* - knowledge lookup operation;
* - knowledge update operation;
* - general expression precedence;
* - general identifiers;
* - qualified names;
* - general patterns;
* - general guards;
* - general types;
* - contracts;
* - policies;
* - evidence semantics;
* - provenance semantics;
* - inference;
* - deduction;
* - induction;
* - abduction;
* - causality;
* - learning;
* - adaptation;
* - model execution;
* - database implementation;
* - graph database implementation;
* - storage;
* - networking;
* - scheduling;
* - hardware allocation;
* - quantum routing;
* - quantum scheduling;
* - QEC;
* - ZQN;
* - HAL;
* - runtime behavior;
* - IR construction.
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* This file is the canonical AI-domain owner for REUSABLE FACT STRUCTURE.
* 
* It must not become a second implementation of:
* 
* grammar/expressions/knowledge.g4
* 
* The existing expression-level knowledge grammar owns the operation boundary:
* 
* knowledgeExpression
* knowledgeAssertionExpression
* knowledgeRetractionExpression
* knowledgeQueryExpression
* knowledgeLookupExpression
* knowledgeUpdateExpression
* 
* This file supplies reusable fact structure to that subsystem.
* 
* Likewise, this file must not duplicate:
* 
* grammar/statements/assertions.g4
* 
* which owns the general source-level assertion statement.
* 
* The distinction is:
* 
* assertion statement
*     = program verification assertion
* 
* knowledge fact
*     = structured computational knowledge claim/relation
* 
* They may interact semantically but they are not the same grammar concept.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/core/names.g4
* grammar/expressions/expressions.g4
* 
* OPTIONAL SEMANTIC CONSUMERS:
* 
* grammar/expressions/knowledge.g4
* grammar/ai/knowledge.g4
* grammar/ai/assertions.g4
* grammar/ai/reasoning.g4
* grammar/ai/evidence.g4
* grammar/ai/provenance.g4
* grammar/ai/uncertainty.g4
* grammar/data/knowledge.g4
* 
* ============================================================================
* EXPORTS
* ============================================================================
* 
* Primary public rules:
* 
* fact
* factPattern
* factTerm
* factPredicate
* factSubject
* factObject
* factRelation
* factMetadata
* factQualifierList
* 
* These rules are intended to be imported by higher-level knowledge grammars.
* 
* ============================================================================
* CONSUMED_BY
* ============================================================================
* 
* Primary consumers:
* 
* grammar/expressions/knowledge.g4
* grammar/ai/knowledge.g4
* 
* Secondary consumers may include:
* 
* grammar/ai/reasoning.g4
* grammar/ai/evidence.g4
* grammar/ai/provenance.g4
* grammar/ai/uncertainty.g4
* grammar/data/knowledge.g4
* 
* A consumer must import these rules rather than reproduce them.
* 
* ============================================================================
* AST OWNER
* ============================================================================
* 
* The AST is owned by the existing frontend AST architecture:
* 
* src/frontend/ast/
* 
* This grammar does NOT define Rust AST structures.
* 
* The semantic representation should preserve the distinction between:
* 
* fact
* fact pattern
* fact term
* metadata
* qualifiers
* 
* without requiring a grammar-specific AST hierarchy.
* 
* Conceptual semantic representation:
* 
* KnowledgeFact {
*     subject,
*     relation,
*     object,
*     metadata,
*     qualifiers,
*     source
* }
* 
* The exact Rust type name and module remain owned by the frontend/semantic
* implementation.
* 
* ============================================================================
* SEMANTIC OWNER
* ============================================================================
* 
* Semantic ownership belongs to the knowledge/semantic layer.
* 
* This grammar does not decide:
* 
* - whether a fact is true;
* - whether a fact is false;
* - whether a fact is uncertain;
* - whether a fact is current;
* - whether a fact is retractable;
* - whether a relation is symmetric;
* - whether a relation is transitive;
* - whether a fact is causal;
* - whether a fact is probabilistic;
* - whether a fact is trusted;
* - whether a fact is authoritative;
* - whether a fact is executable.
* 
* Those are semantic properties.
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* Fact terms are based on the canonical expression system.
* 
* Therefore a fact may contain values represented by existing Zamani
* expressions, including where supported:
* 
* literals
* identifiers
* qualified references
* tuples
* records
* collections
* function results
* tensor values
* model values
* measurement results
* symbolic values
* distributed values
* hardware/resource values
* future domain values
* 
* This grammar does not introduce another value language.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* General expression syntax remains owned by:
* 
* grammar/expressions/expressions.g4
* 
* This file consumes:
* 
* expression
* 
* where a fact term is an ordinary computed value.
* 
* It MUST NOT redefine:
* 
* arithmetic;
* comparison;
* calls;
* indexing;
* member access;
* literals;
* lambdas;
* ranges;
* operators;
* comprehensions.
* 
* ============================================================================
* NAME CONTRACT
* ============================================================================
* 
* General symbolic names remain owned by:
* 
* grammar/core/names.g4
* 
* The fact grammar consumes:
* 
* identifier
* qualifiedName
* 
* through the canonical name architecture.
* 
* Fact predicates and relation names remain OPEN-WORLD.
* 
* The grammar MUST NOT enumerate:
* 
* person
* animal
* temperature
* location
* caused_by
* supports
* hardware
* quantum_state
* model
* robot
* vehicle
* 
* or any other application-specific predicate vocabulary.
* 
* Such concepts are represented by ordinary names and interpreted semantically.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* The canonical lexer is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Parser grammars consume:
* 
* tokenVocab = ZamaniLexer;
* 
* This file contains NO lexer rules.
* 
* It MUST NOT create tokens for:
* 
* fact;
* subject;
* relation;
* predicate;
* object;
* metadata;
* qualifier;
* evidence;
* provenance.
* 
* These are syntactic/semantic roles, not necessarily reserved lexical words.
* 
* ============================================================================
* KEYWORD POLICY
* ============================================================================
* 
* A fact does not require a dedicated global "fact" keyword.
* 
* This is deliberate.
* 
* The surrounding knowledge grammar supplies the contextual introducer.
* 
* For example, a higher-level knowledge construct may express:
* 
* knowledge assert(subject, relation, object);
* 
* while the semantic fact representation is constructed from:
* 
* subject
* relation
* object
* 
* The fact grammar therefore remains reusable by:
* 
* assertions;
* queries;
* reasoning;
* evidence;
* data;
* future knowledge providers.
* 
* No global keyword proliferation is required.
* 
* ============================================================================
* FACT MODEL
* ============================================================================
* 
* A basic fact is a relation:
* 
* subject relation object
* 
* represented conceptually as:
* 
* KnowledgeFact {
*     subject,
*     relation,
*     object
* }
* 
* A fact may also be represented as a structured predicate/value relation.
* 
* The grammar deliberately supports both without defining an ontology.
* 
* ============================================================================
* CANONICAL FACT FORM
* ============================================================================
* 
* The canonical structural form is:
* 
* fact
*     : factSubject
*       factRelation
*       factObject
*       factMetadata?
*       factQualifierList?
*     ;
* 
* The higher-level knowledge operation remains responsible for its surrounding
* operation syntax.
* 
* ============================================================================
* FACT SUBJECT
* ============================================================================
* 
* A subject identifies the entity, value, expression, or symbolic object to
* which the fact applies.
* 
* Examples of semantic subjects include:
* 
* object
* entity
* measurement
* dataset
* model
* computation
* resource
* device
* quantum state
* experiment
* 
* These are semantic categories, not grammar keywords.
* 
* ============================================================================
* FACT RELATION
* ============================================================================
* 
* A relation identifies the logical relationship being asserted.
* 
* It may be:
* 
* identifier
* qualifiedName
* expression-derived reference
* 
* The grammar does not prescribe an ontology.
* 
* A semantic registry may later establish:
* 
* relation arity;
* domain;
* range;
* symmetry;
* transitivity;
* temporal behavior;
* causal meaning;
* probabilistic meaning;
* authorization requirements.
* 
* None of these rules belong here.
* 
* ============================================================================
* FACT OBJECT
* ============================================================================
* 
* An object is the value associated with the relation.
* 
* It may be:
* 
* scalar
* symbolic
* structured
* collection
* tensor
* model output
* measurement result
* expression result
* future domain value.
* 
* The grammar does not impose a finite value-size limit.
* 
* ============================================================================
* PATTERN CONTRACT
* ============================================================================
* 
* Fact patterns are used for matching knowledge.
* 
* A pattern may contain:
* 
* exact terms
* symbolic terms
* wildcard patterns
* structured patterns
* expression-backed terms
* 
* General pattern syntax remains owned by:
* 
* grammar/expressions/patterns.g4
* 
* This grammar uses the canonical "pattern" rule where available.
* 
* It does not create:
* 
* FactPatternV2
* KnowledgePatternV2
* 
* or another competing pattern hierarchy.
* 
* ============================================================================
* METADATA CONTRACT
* ============================================================================
* 
* Metadata describes information ABOUT a fact.
* 
* It is not part of the relation's subject/relation/object identity unless the
* semantic layer explicitly says so.
* 
* Metadata may represent:
* 
* source;
* version;
* timestamp;
* confidence;
* evidence reference;
* provenance reference;
* policy reference;
* scope;
* validity;
* annotations;
* domain-specific metadata.
* 
* Metadata values are expressions.
* 
* This keeps the grammar extensible without adding a new keyword for every
* future metadata category.
* 
* ============================================================================
* QUALIFIER CONTRACT
* ============================================================================
* 
* Qualifiers refine the meaning or applicability of a fact.
* 
* Examples include semantic concepts such as:
* 
* temporal;
* spatial;
* contextual;
* probabilistic;
* causal;
* source;
* confidence;
* scope.
* 
* Their exact meaning is semantic.
* 
* Qualifier names remain open-world identifiers.
* 
* ============================================================================
* EVIDENCE INTEGRATION
* ============================================================================
* 
* Evidence is NOT defined as a special fact syntax in this file.
* 
* Evidence belongs to the evidence semantic subsystem.
* 
* A fact may carry evidence through metadata or qualifier structure.
* 
* Conceptually:
* 
* fact
*   |
*   +---- evidence reference
*   |
*   +---- provenance reference
* 
* The evidence subsystem may consume the resulting semantic fact.
* 
* ============================================================================
* PROVENANCE INTEGRATION
* ============================================================================
* 
* A fact may preserve provenance information such as:
* 
* source;
* derived_from;
* generated_by;
* transformed_by;
* verified_by;
* version;
* timestamp;
* derivation.
* 
* This grammar does not define the provenance model.
* 
* It only permits provenance-bearing metadata/qualifiers to be represented.
* 
* Canonical provenance semantics remain owned by:
* 
* grammar/ai/provenance.g4
* grammar/spec/provenance.md
* downstream provenance implementation.
* 
* ============================================================================
* UNCERTAINTY INTEGRATION
* ============================================================================
* 
* Uncertain facts are represented through ordinary fact values and metadata.
* 
* Examples of semantic values include:
* 
* probability;
* confidence;
* distribution;
* belief;
* 
* The grammar does not introduce a separate probabilistic fact language.
* 
* Uncertainty semantics belong to:
* 
* grammar/ai/uncertainty.g4
* 
* and the canonical type/semantic systems.
* 
* ============================================================================
* REASONING INTEGRATION
* ============================================================================
* 
* Facts are inputs to reasoning.
* 
* Conceptually:
* 
* facts
*   |
*   v
* premises
*   |
*   v
* inference
*   |
*   v
* conclusion
* 
* This file owns only the fact representation.
* 
* It does not define:
* 
* infer;
* deduce;
* reason;
* induction;
* abduction;
* theorem proving;
* inference algorithms.
* 
* ============================================================================
* LEARNING INTEGRATION
* ============================================================================
* 
* Facts may be used as:
* 
* training data;
* observations;
* labels;
* features;
* model metadata;
* learned knowledge;
* evaluation data.
* 
* Learning semantics remain outside this grammar.
* 
* The fact representation must therefore remain independent of:
* 
* optimizer;
* training algorithm;
* model architecture;
* accelerator;
* device;
* scheduler.
* 
* ============================================================================
* ADAPTATION INTEGRATION
* ============================================================================
* 
* Facts may provide observations to adaptive execution.
* 
* A fact does NOT grant permission to modify:
* 
* program state;
* model state;
* execution strategy;
* resources;
* policies.
* 
* Adaptation remains subject to:
* 
* policy;
* authorization;
* capabilities;
* effects;
* resources;
* validation;
* provenance.
* 
* ============================================================================
* RESOURCE/CAPABILITY INTEGRATION
* ============================================================================
* 
* Facts may describe resources or capabilities semantically.
* 
* For example, a knowledge provider may expose facts concerning:
* 
* capability;
* resource;
* topology;
* availability;
* performance;
* reliability.
* 
* This does NOT mean this grammar owns resource semantics.
* 
* Resource requirements remain owned by:
* 
* grammar/resources/
* 
* Capability semantics remain owned by:
* 
* grammar/core/capabilities.g4
* grammar/resources/capabilities.g4
* 
* as established by the repository's resource architecture.
* 
* ============================================================================
* CONTRACT INTEGRATION
* ============================================================================
* 
* A fact may be referenced by:
* 
* requires;
* ensures;
* invariant;
* assume;
* guarantee;
* property.
* 
* This grammar does not redefine those constructs.
* 
* Validation remains owned by:
* 
* grammar/validation/
* grammar/statements/
* 
* ============================================================================
* POLICY INTEGRATION
* ============================================================================
* 
* Knowledge access may be constrained by policies.
* 
* Examples of semantic policy concerns:
* 
* access;
* trust;
* retention;
* disclosure;
* modification;
* source authority;
* execution authority.
* 
* Policy syntax remains owned by:
* 
* grammar/policies/
* grammar/security/
* 
* This grammar only preserves syntactic metadata/qualifiers that can later be
* interpreted under those policies.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Fact structure itself does not inherently execute an effect.
* 
* The semantic operation containing a fact may produce effects such as:
* 
* mutation;
* IO;
* network;
* learning;
* adaptation;
* foreign;
* distributed.
* 
* Effect analysis belongs downstream.
* 
* The grammar MUST NOT classify an operation as authorized merely because a
* fact contains an effect-related name.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* - no actions;
* - no semantic predicates;
* - no external calls;
* - no filesystem access;
* - no network access;
* - no environment inspection;
* - no hardware inspection;
* - no randomness;
* - no time-dependent parsing;
* - no mutable global parser state.
* 
* Given identical:
* 
* source;
* lexer configuration;
* parser configuration;
* language version;
* 
* parsing must produce equivalent parse structure.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar imposes NO language-level finite limit on:
* 
* - fact count;
* - fact term size;
* - relation-name length;
* - identifier length;
* - metadata count;
* - qualifier count;
* - nesting depth;
* - expression complexity;
* - number of knowledge domains;
* - number of knowledge stores;
* - number of facts;
* - number of relations;
* - number of subjects;
* - number of objects.
* 
* Actual parser/compiler limits are implementation/resource concerns.
* 
* They MUST NOT be represented as language constants.
* 
* In particular, this file MUST NOT define:
* 
* MAX_FACTS
* MAX_RELATIONS
* MAX_KNOWLEDGE_ENTRIES
* MAX_FACT_FIELDS
* MAX_FACT_DEPTH
* MAX_METADATA
* MAX_FACT_SIZE
* MAX_KNOWLEDGE_SIZE
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* A fact's source representation must remain independent of the eventual
* computational target.
* 
* The same semantic fact may be consumed on:
* 
* tiny embedded systems;
* CPUs;
* multicore systems;
* GPUs;
* FPGAs;
* ASIC-oriented systems;
* accelerators;
* QPUs;
* simulators;
* HPC systems;
* clusters;
* distributed systems;
* cloud environments;
* future computational targets.
* 
* The fact grammar must not encode:
* 
* CPU identity;
* GPU identity;
* FPGA identity;
* QPU identity;
* physical qubit number;
* memory capacity;
* register width;
* node count;
* network size;
* accelerator count.
* 
* Physical realization belongs downstream.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* A fact may describe or reference quantum information.
* 
* Examples of semantic fact categories include:
* 
* quantum state;
* measurement result;
* operation result;
* capability;
* topology;
* experimental observation.
* 
* This grammar MUST NOT define:
* 
* qubits;
* physical qubit mappings;
* gate catalogs;
* coupling maps;
* calibration;
* routing;
* scheduling;
* QEC;
* ZQN.
* 
* If a fact participates in quantum computation, the semantic pipeline remains:
* 
* fact
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic quantum model
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* decomposition/routing/scheduling
*   |
*   v
* resilience/QEC
*   |
*   v
* ZQN/HAL
* 
* There is no fact-specific quantum IR.
* 
* ============================================================================
* HDL/HARDWARE INTEGRATION
* ============================================================================
* 
* Facts may represent hardware observations or design knowledge.
* 
* They do not define:
* 
* wires;
* registers;
* fixed bus widths;
* physical pins;
* device identifiers;
* placement;
* timing implementation.
* 
* HDL/hardware semantics remain owned by:
* 
* grammar/hdl/
* grammar/hardware/
* 
* ============================================================================
* DATA INTEGRATION
* ============================================================================
* 
* Facts can participate in data processing.
* 
* General data/query syntax remains owned by:
* 
* grammar/data/
* 
* SQL, JSON, XML and other external formats remain dialect/interoperability
* concerns.
* 
* This grammar MUST NOT become an SQL-like query language.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* A fact may be replicated, observed, exchanged, or derived in distributed
* computation.
* 
* This grammar does not define:
* 
* node placement;
* replication;
* consensus;
* network transport;
* partitioning;
* scheduling.
* 
* Those belong to:
* 
* grammar/distributed/
* grammar/networking/
* 
* ============================================================================
* SECURITY INTEGRATION
* ============================================================================
* 
* A fact may carry security-related metadata or provenance.
* 
* Parsing a fact MUST NOT:
* 
* grant access;
* grant capability;
* disclose data;
* invoke a provider;
* contact a network;
* execute code.
* 
* Security semantics remain downstream.
* 
* ============================================================================
* FOREIGN/INTEROPERABILITY INTEGRATION
* ============================================================================
* 
* Facts may refer to external values or systems.
* 
* This grammar does not execute or load foreign entities.
* 
* FFI/ABI semantics remain owned by:
* 
* grammar/interoperability/
* 
* ============================================================================
* METAPROGRAMMING INTEGRATION
* ============================================================================
* 
* Facts may be inspected by compile-time tooling where explicitly permitted.
* 
* This grammar does not execute reflection or generated code.
* 
* Metaprogramming remains owned by:
* 
* grammar/metaprogramming/
* grammar/macros/
* 
* ============================================================================
* SYNTAX
* ============================================================================
* 
* The rules below intentionally use canonical identifiers, qualified names,
* expressions and patterns rather than application-specific vocabularies.
* 
* ============================================================================
  */

parser grammar AIFacts;

options {
tokenVocab = ZamaniLexer;
}

import
Names,
Expressions,
Patterns
;

/*

* ============================================================================
* PUBLIC FACT RULE
* ============================================================================
* 
* Canonical reusable fact:
* 
* subject relation object
* 
* Metadata and qualifiers are optional.
* 
* The surrounding knowledge grammar owns the operation wrapper and any
* statement/expression terminator.
* 
* ============================================================================
  */

fact
: factSubject
factRelation
factObject
factMetadata?
factQualifierList?
;

/*

* ============================================================================
* FACT PATTERN
* ============================================================================
* 
* A fact pattern has the same structural shape as a fact, but its terms may
* contain pattern constructs.
* 
* Pattern matching semantics remain downstream.
* 
* ============================================================================
  */

factPattern
: factPatternSubject
factPatternRelation
factPatternObject
factMetadata?
factQualifierList?
;

/*

* ============================================================================
* SUBJECT
* ============================================================================
  */

factSubject
: factTerm
;

factPatternSubject
: factPatternTerm
;

/*

* ============================================================================
* RELATION
* ============================================================================
* 
* Relations are symbolic names rather than a closed keyword list.
* 
* Qualified names are preferred where namespace identity is significant.
* 
* ============================================================================
  */

factRelation
: qualifiedName
;

factPatternRelation
: qualifiedName
| factPatternVariable
;

/*

* ============================================================================
* OBJECT
* ============================================================================
  */

factObject
: factTerm
;

factPatternObject
: factPatternTerm
;

/*

* ============================================================================
* FACT TERM
* ============================================================================
* 
* A fact term is an ordinary Zamani expression.
* 
* This keeps fact syntax compatible with the universal expression system.
* 
* ============================================================================
  */

factTerm
: expression
;

/*

* ============================================================================
* PATTERN TERM
* ============================================================================
* 
* Prefer the canonical pattern grammar.
* 
* A pattern may also contain an ordinary expression when the semantic context
* permits expression-backed matching.
* 
* ============================================================================
  */

factPatternTerm
: pattern
| expression
;

factPatternVariable
: identifier
;

/*

* ============================================================================
* METADATA
* ============================================================================
* 
* Metadata is represented as a non-empty list of named values.
* 
* Example semantic shapes:
* 
* source = value
* confidence = value
* version = value
* 
* The exact metadata vocabulary remains open-world.
* 
* ============================================================================
  */

factMetadata
: factMetadataIntroducer
factMetadataEntries
;

factMetadataIntroducer
: WITH
;

factMetadataEntries
: factMetadataEntry
| factMetadataEntries COMMA factMetadataEntry
;

factMetadataEntry
: qualifiedName
factMetadataAssignment
expression
;

factMetadataAssignment
: ASSIGN
;

/*

* ============================================================================
* QUALIFIERS
* ============================================================================
* 
* Qualifiers are deliberately name/value based.
* 
* This permits future semantic categories without expanding the global lexer.
* 
* ============================================================================
  */

factQualifierList
: factQualifier
| factQualifierList factQualifier
;

factQualifier
: factQualifierIntroducer
qualifiedName
factQualifierValue?
;

factQualifierIntroducer
: AT
;

factQualifierValue
: LPAREN
argumentList?
RPAREN
;

/*

* ============================================================================
* OPTIONAL ARGUMENT
* ============================================================================
* 
* The canonical expression grammar owns argumentList.
* 
* This file merely consumes it.
* ============================================================================
  */

factArgument
: expression
;

factArgumentList
: argumentList
;

/*

* ============================================================================
* SEMANTIC NOTES
* ============================================================================
* 
* The following conceptual values are intentionally NOT grammar productions:
* 
* truth;
* falsehood;
* uncertainty;
* confidence;
* provenance;
* evidence;
* source;
* timestamp;
* validity;
* scope.
* 
* They are ordinary semantic values/metadata unless the corresponding
* language-wide subsystem establishes dedicated syntax.
* 
* This keeps the fact grammar stable as the semantic model expands.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parse tree must provide enough information for the canonical frontend AST
* to preserve:
* 
* - complete fact source span;
* - subject;
* - relation;
* - object;
* - metadata;
* - qualifiers;
* - nested expressions;
* - source ordering;
* - source spelling where required for diagnostics/formatting;
* - child source spans.
* 
* The AST must not encode target-specific realization.
* 
* It must not require:
* 
* CPU;
* GPU;
* FPGA;
* ASIC;
* QPU;
* node;
* device;
* memory;
* physical topology.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis must be able to determine:
* 
* - whether the subject is valid;
* - whether the relation resolves;
* - whether relation arity is valid;
* - whether object type is valid;
* - whether metadata is valid;
* - whether qualifiers are valid;
* - whether provenance is valid;
* - whether evidence is valid;
* - whether policy permits use;
* - whether a fact is compatible with the consuming operation.
* 
* None of these checks belong in this grammar.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This file does not own an IR.
* 
* Fact semantics first enter the canonical semantic representation.
* 
* Depending on use, they may subsequently participate in:
* 
* classical IR;
* quantum::ir;
* data representation;
* distributed representation;
* hardware/HDL representation;
* accelerator representation;
* future domain representations.
* 
* There must be no:
* 
* fact::ir
* ai::fact::ir
* knowledge::fact::ir
* 
* created solely by this grammar.
* 
* ============================================================================
* QUANTUM BOUNDARY
* ============================================================================
* 
* If a fact participates in a quantum computation:
* 
* fact
*   ->
* semantic quantum interpretation
*   ->
* quantum::ir
* 
* "quantum::ir" is the canonical quantum boundary.
* 
* This file must never encode physical qubit identity or routing.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Parsing a fact produces no runtime effect.
* 
* A semantic consumer may classify an enclosing operation as requiring:
* 
* mutation;
* IO;
* network;
* learning;
* adaptation;
* distributed;
* measurement;
* simulation;
* foreign;
* native.
* 
* The effect system owns that classification.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* A fact does not grant capabilities.
* 
* A semantic consumer may require:
* 
* knowledge.read;
* knowledge.write;
* knowledge.query;
* evidence.read;
* provenance.read;
* network.access;
* 
* or other capabilities defined by the repository's capability system.
* 
* The grammar merely preserves the fact structure.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* This grammar does not allocate resources.
* 
* A consuming semantic operation may produce resource requirements.
* 
* Resource analysis determines feasibility against actual available resources.
* 
* No resource ceiling is encoded here.
* 
* ============================================================================
* POLICY CONTRACT
* ============================================================================
* 
* A fact may be subject to:
* 
* access policy;
* retention policy;
* provenance policy;
* disclosure policy;
* mutation policy;
* execution policy.
* 
* Policies remain downstream.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* The source span and syntactic structure must remain available so that
* downstream provenance can record:
* 
* source;
* derived_from;
* generated_by;
* transformed_by;
* verified_by;
* version;
* reason;
* evidence;
* decision.
* 
* This grammar does not generate provenance records itself.
* 
* ============================================================================
* DIAGNOSTICS CONTRACT
* ============================================================================
* 
* Required syntax diagnostics include:
* 
* - missing subject;
* - missing relation;
* - missing object;
* - malformed qualified relation;
* - malformed metadata;
* - missing metadata value;
* - malformed qualifier;
* - malformed qualifier arguments;
* - malformed pattern.
* 
* Semantic diagnostics include:
* 
* - unresolved relation;
* - invalid subject type;
* - invalid object type;
* - invalid relation arity;
* - invalid metadata;
* - invalid qualifier;
* - policy violation;
* - unavailable capability;
* - unavailable resource.
* 
* Semantic diagnostics MUST remain outside this grammar.
* 
* ============================================================================
* POSITIVE TEST CONTRACT
* ============================================================================
* 
* The conformance suite should cover fact structures equivalent to:
* 
* subject relation object
* 
* entity relation value
* 
* namespace::subject namespace::relation value
* 
* subject relation expression()
* 
* subject relation (a, b)
* 
* subject relation tensor_value
* 
* subject relation measurement_result
* 
* subject relation model_output
* 
* subject relation object with source = source_value
* 
* subject relation object with confidence = confidence_value
* 
* subject relation object with source = source_value,
*     confidence = confidence_value
* 
* subject relation object @temporal
* 
* subject relation object @scope(scope_value)
* 
* subject relation object @evidence(evidence_value)
* 
* subject relation object @provenance(provenance_value)
* 
* Exact semantic validity is checked downstream.
* 
* ============================================================================
* PATTERN TEST CONTRACT
* ============================================================================
* 
* The suite should cover:
* 
* exact subject patterns;
* exact relation patterns;
* exact object patterns;
* wildcard patterns;
* identifier patterns;
* structured patterns;
* nested patterns;
* expression-backed patterns;
* qualified relation patterns.
* 
* ============================================================================
* NEGATIVE TEST CONTRACT
* ============================================================================
* 
* The grammar must reject malformed structures such as:
* 
* subject
* 
* subject relation
* 
* relation object
* 
* subject :: relation object
* 
* subject relation,
* 
* subject relation object with
* 
* subject relation object with key
* 
* subject relation object with = value
* 
* subject relation object @
* 
* subject relation object @scope(
* 
* subject relation object @scope(value
* 
* subject relation object @scope()
* 
* where the surrounding language requires a non-empty qualifier value.
* 
* Semantic-invalid examples must remain syntactically parseable when their
* structure is valid:
* 
* unknown_subject relation object
* 
* subject unknown_relation object
* 
* subject relation invalid_value
* 
* Such cases are semantic diagnostics, not grammar errors.
* 
* ============================================================================
* BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Test:
* 
* - one-character names;
* - long identifiers;
* - Unicode identifiers accepted by the lexer;
* - deeply qualified names;
* - nested expressions;
* - nested records;
* - nested tuples;
* - tensor expressions;
* - quantum-derived values;
* - model-derived values;
* - hardware/resource expressions;
* - distributed values;
* - provenance expressions;
* - evidence expressions;
* - policy-related values;
* - large metadata lists;
* - large qualifier lists.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* The conformance suite must progressively increase:
* 
* fact count;
* identifier size;
* qualification depth;
* expression size;
* nesting depth;
* metadata count;
* qualifier count;
* source size.
* 
* Any practical parser limit must be measured as an implementation/resource
* characteristic.
* 
* It must not become a language-level constant.
* 
* ============================================================================
* CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* The same fact grammar must support facts consumed by:
* 
* classical computation;
* quantum computation;
* hybrid computation;
* HDL;
* hardware;
* AI;
* reasoning;
* learning;
* adaptation;
* uncertainty;
* evidence;
* provenance;
* contracts;
* policies;
* concurrency;
* distributed computation;
* networking;
* data processing;
* tensor computation;
* accelerators;
* simulation;
* interoperability;
* metaprogramming.
* 
* No domain may require a parallel fact grammar.
* 
* ============================================================================
* DETERMINISM TEST CONTRACT
* ============================================================================
* 
* Parsing identical source under identical language/parser configuration must
* produce equivalent parse-tree structure.
* 
* The grammar must not depend on:
* 
* time;
* randomness;
* target hardware;
* available resources;
* network state;
* filesystem state;
* scheduler state.
* 
* ============================================================================
* PORTABILITY TEST CONTRACT
* ============================================================================
* 
* Fact syntax must remain source-compatible across:
* 
* embedded targets;
* CPUs;
* multicore systems;
* GPUs;
* FPGAs;
* ASIC-oriented targets;
* accelerators;
* QPUs;
* simulators;
* HPC systems;
* clusters;
* distributed systems;
* cloud systems;
* future targets.
* 
* Target feasibility is a downstream concern.
* 
* ============================================================================
* SECURITY TEST CONTRACT
* ============================================================================
* 
* Parsing a fact must never:
* 
* - execute a provider;
* - load a file;
* - access a network;
* - access credentials;
* - invoke hardware;
* - allocate a device;
* - mutate knowledge storage;
* - modify model state.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This grammar contains no Rust implementation.
* 
* Generated parser/frontend integration must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* and safe Rust.
* 
* No:
* 
* unsafe blocks;
* unsafe functions;
* unsafe traits;
* raw-pointer requirement;
* target-specific native implementation
* 
* is permitted by this grammar.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden:
* 
* machine capacities;
* CPU counts;
* GPU counts;
* FPGA counts;
* ASIC counts;
* QPU counts;
* qubit counts;
* node counts;
* device counts;
* memory capacities;
* register widths;
* tensor-rank ceilings;
* network-size ceilings;
* knowledge-entry ceilings;
* fact-count ceilings.
* 
* Allowed:
* 
* language-defined punctuation;
* canonical lexical tokens;
* recursive grammar structure;
* finite syntax vocabulary.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* This file must preserve the distinction between:
* 
* fact syntax
* 
* and:
* 
* knowledge operation syntax.
* 
* Existing knowledge consumers must migrate toward:
* 
* knowledge operation
*      ->
* fact/factPattern
* 
* rather than copying fact productions.
* 
* New knowledge providers must consume the same fact structure.
* 
* If future syntax introduces a new fact form, it must be added here only when
* it is genuinely a language-level structural concept rather than a provider-
* specific feature.
* 
* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* REQUIRED INTEGRATION 1
* ---
* 
* grammar/expressions/knowledge.g4
* 
* Its knowledge operation rules should consume:
* 
* fact
* factPattern
* 
* from AIFacts where the semantic operation is fact-based.
* 
* It must retain ownership of:
* 
* knowledgeExpression
* knowledgeAssertionExpression
* knowledgeRetractionExpression
* knowledgeQueryExpression
* knowledgeLookupExpression
* knowledgeUpdateExpression
* 
* It must NOT copy the fact rules.
* 
* ============================================================================
* REQUIRED INTEGRATION 2
* ---
* 
* grammar/ai/knowledge.g4
* 
* If this file is established as the AI knowledge composition grammar, it
* should import AIFacts and expose domain-level knowledge composition.
* 
* It must not redefine:
* 
* fact
* factPattern
* factTerm
* factRelation
* 
* ============================================================================
* REQUIRED INTEGRATION 3
* ---
* 
* grammar/ai/reasoning.g4
* 
* Reasoning consumes fact/factPattern values as premises and conclusions.
* 
* It does not redefine their syntax.
* 
* ============================================================================
* REQUIRED INTEGRATION 4
* ---
* 
* grammar/ai/evidence.g4
* 
* Evidence may reference or annotate facts.
* 
* Evidence syntax remains evidence-owned.
* 
* ============================================================================
* REQUIRED INTEGRATION 5
* ---
* 
* grammar/ai/provenance.g4
* 
* Provenance attaches semantic history to facts.
* 
* Provenance syntax remains provenance-owned.
* 
* ============================================================================
* REQUIRED INTEGRATION 6
* ---
* 
* grammar/data/knowledge.g4
* 
* If present/created, it must reuse AIFacts for logical fact structure rather
* than inventing a second representation.
* 
* ============================================================================
* REQUIRED INTEGRATION 7
* ---
* 
* grammar/expressions/expressions.g4
* 
* The canonical expression composition must integrate knowledge expressions
* through its existing expression architecture.
* 
* AIFacts must not modify primaryExpression itself.
* 
* ============================================================================
* REQUIRED INTEGRATION 8
* ---
* 
* grammar/Zamani.g4
* 
* The root grammar remains the composition root.
* 
* AIFacts must become reachable through the canonical parser dependency graph,
* not through an independent parser entry point that bypasses Zamani's normal
* AST construction.
* 
* ============================================================================
* REQUIRED INTEGRATION 9
* ---
* 
* grammar/antlr/ZamaniParser.g4
* 
* The parser composition must expose AI/knowledge constructs through its
* established grammar hierarchy.
* 
* AIFacts must remain a leaf component.
* 
* ============================================================================
* REQUIRED INTEGRATION 10
* ---
* 
* src/frontend/ast/
* 
* The frontend AST must receive fact structure without creating target-specific
* nodes.
* 
* The AST must preserve:
* 
* subject;
* relation;
* object;
* metadata;
* qualifiers;
* source span.
* 
* ============================================================================
* REQUIRED INTEGRATION 11
* ---
* 
* Semantic analysis
* 
* The semantic layer must transform the AST representation into the canonical
* knowledge semantic model.
* 
* It determines:
* 
* type validity;
* relation resolution;
* metadata semantics;
* provenance;
* evidence;
* uncertainty;
* policy;
* capabilities;
* resources;
* effects.
* 
* ============================================================================
* REQUIRED INTEGRATION 12
* ---
* 
* Canonical IR
* 
* Fact semantics must first enter the canonical semantic representation.
* 
* Only then may they participate in:
* 
* classical IR;
* quantum::ir;
* data;
* distributed;
* HDL/hardware;
* accelerator
* 
* lowering.
* 
* ============================================================================
* TOOLING CONTRACT
* ============================================================================
* 
* Formatter:
* 
* must preserve fact semantic structure.
* 
* LSP:
* 
* must be able to resolve subject/relation/object symbols through semantic
* analysis rather than grammar-specific symbol tables.
* 
* Diagnostics:
* 
* must preserve source spans for each fact component.
* 
* Syntax highlighting:
* 
* may identify fact structure without assigning application semantics.
* 
* Refactoring:
* 
* relation renaming must use semantic symbol resolution.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when all of the following are true:
* 
* [ ] File is grammar/ai/facts.g4.
* 
* [ ] Grammar is named AIFacts.
* 
* [ ] It is a parser grammar.
* 
* [ ] tokenVocab is ZamaniLexer.
* 
* [ ] It imports canonical Names.
* 
* [ ] It imports canonical Expressions.
* 
* [ ] It imports canonical Patterns.
* 
* [ ] It defines fact structure exactly once.
* 
* [ ] It defines factPattern structure exactly once.
* 
* [ ] It does not define knowledge operations.
* 
* [ ] It does not redefine assertion statements.
* 
* [ ] It does not define lexer tokens.
* 
* [ ] It does not define application-specific predicates.
* 
* [ ] It does not define database syntax.
* 
* [ ] It does not define SQL syntax.
* 
* [ ] It does not define graph-query syntax.
* 
* [ ] It does not define reasoning algorithms.
* 
* [ ] It does not define learning algorithms.
* 
* [ ] It does not define adaptation semantics.
* 
* [ ] It does not define provenance semantics.
* 
* [ ] It does not define evidence semantics.
* 
* [ ] It does not define probability semantics.
* 
* [ ] It does not define hardware semantics.
* 
* [ ] It does not define quantum physical mapping.
* 
* [ ] It does not define quantum gates.
* 
* [ ] It does not define QEC.
* 
* [ ] It does not define ZQN.
* 
* [ ] It does not define HAL.
* 
* [ ] It does not define runtime behavior.
* 
* [ ] It does not construct IR.
* 
* [ ] It contains no embedded Rust.
* 
* [ ] It contains no unsafe implementation requirement.
* 
* [ ] It contains no hardware-capacity constants.
* 
* [ ] It contains no fact-count constants.
* 
* [ ] It contains no fixed metadata ceiling.
* 
* [ ] It contains no fixed qualifier ceiling.
* 
* [ ] It supports recursively extensible symbolic structures.
* 
* [ ] It preserves source structure for AST construction.
* 
* [ ] It is deterministic.
* 
* [ ] It is target independent.
* 
* [ ] It is suitable for POCO-REAF.
* 
* [ ] Positive tests exist.
* 
* [ ] Negative tests exist.
* 
* [ ] Boundary tests exist.
* 
* [ ] Scalability tests exist.
* 
* [ ] Cross-domain tests exist.
* 
* [ ] Determinism tests exist.
* 
* [ ] Portability tests exist.
* 
* [ ] Security tests exist.
* 
* [ ] Formatter/round-trip tests exist where formatter support exists.
* 
* [ ] Knowledge grammar consumes this file rather than duplicating it.
* 
* [ ] Reasoning consumes this file rather than duplicating it.
* 
* [ ] Evidence/provenance consume this structure through semantic
*     integration.
* 
* [ ] quantum::ir remains the canonical quantum IR boundary.
* 
* [ ] Canonical AST ownership remains in src/frontend/ast/.
* 
* [ ] Rust 1.97 / 1.97.1 integration is verified.
* 
* ============================================================================
* FINAL ARCHITECTURAL GUARANTEE
* ============================================================================
* 
* A fact is SOURCE-LEVEL COMPUTATIONAL KNOWLEDGE.
* 
* It is not:
* 
* a database row;
* a graph database node;
* a physical device;
* a CPU;
* a GPU;
* an FPGA;
* an ASIC;
* a QPU;
* a qubit;
* a network node;
* a memory allocation;
* a runtime object.
* 
* The grammar therefore preserves the universal architecture:
* 
* source
*   ->
* canonical AST
*   ->
* semantic fact
*   ->
* knowledge/reasoning/learning/etc.
*   ->
* canonical semantic representation
*   ->
* domain lowering
*   ->
* target realization
* 
* This separation allows one source representation to remain stable while the
* implementation realization changes with available capabilities and
* resources.
* 
* ============================================================================
  */

/*

* ============================================================================
* GRAMMAR IMPLEMENTATION
* ============================================================================
  */

/*

* A fact is intentionally expressed without a statement terminator.
* 
* The enclosing knowledge operation owns its own delimiters and terminator.
  */
  fact
  : factSubject
  factRelation
  factObject
  factMetadata?
  factQualifierList?
  ;

/*

* Pattern form used by knowledge matching.
  */
  factPattern
  : factPatternSubject
  factPatternRelation
  factPatternObject
  factMetadata?
  factQualifierList?
  ;

/*

* ---
* SUBJECT
* ---

*/

factSubject
: factTerm
;

factPatternSubject
: factPatternTerm
;

/*

* ---
* RELATION
* ---
* 
* Relations are names, not a fixed ontology.
  */
  factRelation
  : qualifiedName
  ;

factPatternRelation
: qualifiedName
| factPatternVariable
;

/*

* ---
* OBJECT
* ---

*/

factObject
: factTerm
;

factPatternObject
: factPatternTerm
;

/*

* ---
* TERM
* ---
* 
* Reuse the canonical expression system.
  */
  factTerm
  : expression
  ;

factPatternTerm
: pattern
| expression
;

factPatternVariable
: identifier
;

/*

* ---
* METADATA
* ---
* 
* Metadata syntax is deliberately generic:
* 
* with key = value
* with key = value, other = value
* 
* The semantic layer determines recognized metadata keys.
  */
  factMetadata
  : WITH
  factMetadataEntries
  ;

factMetadataEntries
: factMetadataEntry
| factMetadataEntries COMMA factMetadataEntry
;

factMetadataEntry
: qualifiedName
ASSIGN
expression
;

/*

* ---
* QUALIFIERS
* ---
* 
* Generic qualifier syntax:
* 
* @name
* @name(value)
* @namespace::name(value)
* 
* The semantic layer determines qualifier meaning.
  */
  factQualifierList
  : factQualifier+
  ;

factQualifier
: AT
qualifiedName
factQualifierArguments?
;

factQualifierArguments
: LPAREN
argumentList?
RPAREN
;
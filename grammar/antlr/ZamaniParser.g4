/**

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* File:
* grammar/antlr/ZamaniParser.g4
* 
* Status:
* CANONICAL PRODUCTION PARSER ORCHESTRATOR
* 
* Role:
* Single ANTLR4 parser composition root for Zamani.
* 
* ---
* ARCHITECTURE
* ---
* 
*                     Zamani source
*                          |
*                          v
*                 ZamaniLexer.g4
*                          |
*                          v
*                ZamaniParser.g4
*                          |
*          +---------------+----------------+
*          |               |                |
*          v               v                v
*      Core syntax     Domain syntax     Extensions
*          |               |                |
*          +---------------+----------------+
*                          |
*                          v
*                domain-neutral AST
*                          |
*                          v
*             structural validation
*                          |
*                          v
*             semantic analysis
*                          |
*          +---------------+----------------+
*          |               |                |
*          v               v                v
*    classical IR     quantum::ir       HDL/hardware IR
*                          |
*                          v
*               optimization/lowering
*                          |
*          +---------------+----------------+
*          |               |                |
*          v               v                v
*       routing       scheduling       resilience
*                          |
*                          v
*                         ZQN
*                          |
*                          v
*                         HAL
*                          |
*                          v
*                 target realization
* 
* ---
* AUTHORITATIVE COMPONENTS
* ---
* 
* The parser consumes the single production lexical vocabulary:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* This grammar composes the reusable parser components:
* 
* grammar/antlr/Core.g4
* grammar/antlr/Types.g4
* grammar/antlr/Quantum.g4
* grammar/antlr/Concurrency.g4
* grammar/antlr/Effects.g4
* grammar/antlr/Modules.g4
* grammar/antlr/Mathematics.g4
* grammar/antlr/Meta.g4
* 
* The wider grammar/ tree remains organized by domain and feature.
* Those files must converge on these canonical parser contracts rather than
* creating another root parser.
* 
* ---
* RUST BASELINE
* ---
* 
* Rust implementation:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* safe Rust only
* 
* This grammar contains no Rust actions.
* 
* The compiler implementation MUST NOT require Rust "unsafe".
* 
* ---
* POCO-REAF
* ---
* 
* This parser describes portable source-level computation.
* 
* It MUST NOT impose universal limits on:
* 
* CPUs
* cores
* threads
* GPUs
* FPGAs
* ASICs
* QPUs
* qubits
* registers
* memory
* storage
* nodes
* devices
* accelerators
* tensor dimensions
* tensor rank
* vector width
* timelines
* processes
* channels
* tasks
* program size
* 
* The language may contain program-defined constants and requirements.
* Those are semantic source values, not implementation limits.
* 
* For example:
* 
* requires qubits >= n;
* 
* is valid source intent.
* 
* A parser rule equivalent to:
* 
* MAX_QUBITS = 1024
* 
* is prohibited.
* 
* ---
* SEMANTIC BOUNDARY
* ---
* 
* This parser answers:
* 
* "What syntactic structure did the programmer write?"
* 
* It does NOT answer:
* 
* "Which machine executes it?"
* "Which physical qubit is used?"
* "Which CPU core is used?"
* "Which GPU is selected?"
* "Which FPGA is selected?"
* "Which network node is selected?"
* "How is the program scheduled?"
* "How is quantum routing performed?"
* "How is QEC performed?"
* "What noise model applies?"
* "What backend is selected?"
* 
* Those decisions belong downstream.
* 
* ---
* QUANTUM BOUNDARY
* ---
* 
* Quantum syntax is parsed here and lowered downstream to:
* 
* quantum::ir
* 
* There is no parser-owned second quantum IR.
* 
* The parser MUST NOT:
* 
* - enumerate a finite gate catalogue;
* - assign physical qubit IDs;
* - assign QPU topology;
* - choose native gates;
* - perform routing;
* - perform scheduling;
* - perform calibration;
* - perform QEC;
* - implement ZQN;
* - access HAL;
* - access hardware.
* 
* A quantum operation is represented structurally and remains extensible.
* 
* ---
* HARDWARE / RESOURCE BOUNDARY
* ---
* 
* Hardware, resources and capabilities describe source-level intent.
* 
* Examples:
* 
* requires capability("quantum.measurement");
* requires resource(qubits >= n);
* prefers capability("tensor.compute");
* 
* These are not hardware-discovery operations.
* 
* Resource realization belongs to:
* 
* semantic analysis
* resource planning
* compiler
* routing
* scheduling
* HAL
* runtime
* 
* ---
* SECURITY / DETERMINISM
* ---
* 
* This parser:
* 
* - performs no I/O;
* - performs no filesystem access;
* - performs no networking;
* - performs no environment inspection;
* - performs no hardware discovery;
* - performs no randomness;
* - performs no wall-clock decisions;
* - performs no backend selection;
* - performs no source execution;
* - performs no compiler-global mutation.
* 
* Identical source + identical lexer vocabulary + identical language version
* MUST produce structurally equivalent parse trees.
* 
* ============================================================================
  */

parser grammar ZamaniParser;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* COMPONENT COMPOSITION
* ============================================================================
* 
* These are the canonical reusable ANTLR parser components.
* 
* Core is intentionally imported here as the source of the broad universal
* declaration vocabulary. The root grammar owns the final dispatch points
* where domain components need to be composed or reconciled.
* 
* The grammar directories outside grammar/antlr are feature/domain ownership
* surfaces. They must ultimately lower into these parser contracts rather than
* becoming competing root grammars.
* ============================================================================
  */

import
Core,
Types,
Modules,
Effects,
Quantum,
Concurrency,
Mathematics,
Meta
;

/*

* ============================================================================
* 1. CANONICAL ENTRY POINT
* ============================================================================
* 
* Exactly one public source-program entry point exists.
* 
* No domain grammar may create another top-level program entry point.
* ============================================================================
  */

program
: sourceElement* EOF
;

sourceElement
: attribute
| item
| statement
| domainItem
;

/*

* ============================================================================
* 2. UNIVERSAL SOURCE ITEM DISPATCH
* ============================================================================
* 
* This is the principal orchestration boundary.
* 
* Every domain remains syntactically composable while the root parser retains
* ownership of the complete source-unit structure.
* ============================================================================
  */

item
: declaration
| domainDeclaration
;

declaration
: moduleDeclaration
| importDeclaration
| exportDeclaration
| useDeclaration
| packageDeclaration
| functionDeclaration
| structDeclaration
| recordDeclaration
| enumDeclaration
| traitDeclaration
| implDeclaration
| classDeclaration
| interfaceDeclaration
| typeAliasDeclaration
| constantDeclaration
| effectDeclaration
| languageDeclaration
| macroDeclaration
| externDeclaration
| quantumDeclaration
| concurrencyDeclaration
| mathDeclaration
| metaDeclaration
;

/*

* ============================================================================
* 3. DOMAIN DECLARATIONS
* ============================================================================
* 
* Domain declarations are intentionally open-ended.
* 
* The parser recognizes semantic domains through their structural grammar,
* not through a finite hardware/vendor catalogue.
* ============================================================================
  */

domainDeclaration
: quantumDeclaration
| concurrencyDeclaration
| mathDeclaration
| metaDeclaration
| domainDeclarationExtension
;

domainDeclarationExtension
: identifier
domainDeclarationPayload
;

domainDeclarationPayload
: blockExpression
| SEMI
;

/*

* ============================================================================
* 4. UNIVERSAL STATEMENT DISPATCH
* ============================================================================
  */

statement
: bindingStatement
| controlStatement
| flowStatement
| exceptionStatement
| effectStatement
| concurrencyStatement
| quantumStatement
| mathematicalStatement
| metaStatement
| expressionStatement
;

/*

* ---
* Bindings
* ---

*/

bindingStatement
: letStatement
| varStatement
| constStatement
;

letStatement
: LET MUT? pattern
(COLON typeExpression)?
ASSIGN expression
SEMI?
;

varStatement
: VAR pattern
(COLON typeExpression)?
ASSIGN expression
SEMI?
;

constStatement
: CONST pattern
(COLON typeExpression)?
ASSIGN expression
SEMI?
;

/*

* ---
* Control flow
* ---

*/

controlStatement
: ifStatement
| whileStatement
| forStatement
| loopStatement
| matchStatement
;

ifStatement
: IF expression blockExpression
(ELSE IF expression blockExpression)*
(ELSE blockExpression)?
;

whileStatement
: WHILE expression blockExpression
;

forStatement
: FOR pattern IN expression blockExpression
;

loopStatement
: LOOP blockExpression
;

matchStatement
: MATCH expression
LBRACE
matchArm*
RBRACE
;

matchArm
: pattern
(IF expression)?
FAT_ARROW
(blockExpression | expression)
COMMA?
;

/*

* ---
* Flow statements
* ---

*/

flowStatement
: returnStatement
| breakStatement
| continueStatement
;

returnStatement
: RETURN expression? SEMI?
;

breakStatement
: BREAK identifier? SEMI?
;

continueStatement
: CONTINUE identifier? SEMI?
;

/*

* ---
* Exceptions / failure
* ---

*/

exceptionStatement
: tryStatement
| throwStatement
;

tryStatement
: TRY blockExpression
catchClause*
finallyClause?
;

catchClause
: CATCH
(
LPAREN pattern RPAREN
| pattern
)?
blockExpression
;

finallyClause
: FINALLY blockExpression
;

throwStatement
: THROW expression SEMI?
;

/*

* ============================================================================
* 5. EFFECT ORCHESTRATION
* ============================================================================
  */

effectStatement
: performStatement
| handleStatement
;

performStatement
: PERFORM
effectInvocation
SEMI?
;

effectInvocation
: effectName
(
LPAREN argumentList? RPAREN
)?
;

/*

* The Effects component owns effect declaration/handler semantics. The root
* parser only determines where effect syntax can appear in a source program.
  */

effectName
: identifier
| qualifiedName
;

effectDeclaration
: EFFECT effectDeclarationSignature
(
SEMI
| blockExpression
)
;

effectDeclarationSignature
: identifier
genericParameters?
effectParameterClause?
returnType?
;

effectParameterClause
: LPAREN parameterList? RPAREN
;

/*

* ============================================================================
* 6. CONCURRENCY ORCHESTRATION
* ============================================================================
  */

concurrencyStatement
: concurrencyStatementCore
;

concurrencyStatementCore
: spawnStatement
| awaitStatement
| joinStatement
| sendStatement
| channelReceiveStatement
| synchronizeStatement
| criticalStatement
| atomicStatement
| cancelStatement
;

concurrencyDeclaration
: concurrencyDeclarationCore
;

concurrencyDeclarationCore
: actorDeclaration
| channelDeclaration
| concurrencyRegion
;

/*

* ============================================================================
* 7. QUANTUM ORCHESTRATION
* ============================================================================
* 
* Quantum.g4 owns detailed quantum syntax.
* 
* The root parser determines where it participates in the universal source
* language. The quantum component determines the structure inside it.
* ============================================================================
  */

quantumDeclaration
: QUANTUM quantumDeclarationBody
;

quantumStatement
: quantumStatementCore
;

quantumStatementCore
: quantumOperationStatement
| quantumMeasurementStatement
| quantumResetStatement
| quantumBarrierStatement
| quantumSynchronizeStatement
| quantumAllocateStatement
| quantumReleaseStatement
| quantumEntangleStatement
| quantumChannelStatement
| quantumComposeStatement
;

/*

* ============================================================================
* 8. MATHEMATICS ORCHESTRATION
* ============================================================================
  */

mathematicalStatement
: mathStatement
;

mathDeclaration
: mathItem
;

/*

* Mathematical algorithms remain semantic operations rather than a finite
* parser-level keyword catalogue.
* 
* Examples such as FFT, SVD, optimization, calculus, statistics, tensor
* operations and linear algebra remain extensible through identifiers,
* expressions, types and calls.
  */

/*

* ============================================================================
* 9. META / MACRO ORCHESTRATION
* ============================================================================
  */

metaStatement
: metaStatementCore
;

metaStatementCore
: macroInvocation
| compileTimeExpression
;

metaDeclaration
: metaDeclarationCore
;

metaDeclarationCore
: macroDeclaration
| languageDeclaration
| externDeclaration
;

/*

* ============================================================================
* 10. EXPRESSIONS
* ============================================================================
* 
* The root owns the precedence hierarchy.
* 
* Domain grammars provide specialized primary expressions and semantic
* constructs but do not create independent expression languages.
* ============================================================================
  */

expression
: assignmentExpression
;

assignmentExpression
: conditionalExpression
(
assignmentOperator
assignmentExpression
)?
;

assignmentOperator
: ASSIGN
| PLUS_ASSIGN
| MINUS_ASSIGN
| STAR_ASSIGN
| SLASH_ASSIGN
| MODULO_ASSIGN
| BIT_AND_ASSIGN
| BIT_OR_ASSIGN
| XOR_ASSIGN
| LEFT_SHIFT_ASSIGN
| RIGHT_SHIFT_ASSIGN
;

conditionalExpression
: rangeExpression
(
QUESTION_MARK
expression
COLON
expression
)?
;

rangeExpression
: logicalOrExpression
(
rangeOperator
logicalOrExpression?
)?
;

rangeOperator
: DOT_DOT
| DOT_DOT_EQ
;

logicalOrExpression
: logicalAndExpression
(
LOGICAL_OR
logicalAndExpression
)*
;

logicalAndExpression
: bitwiseOrExpression
(
LOGICAL_AND
bitwiseOrExpression
)*
;

bitwiseOrExpression
: bitwiseXorExpression
(
BIT_OR
bitwiseXorExpression
)*
;

bitwiseXorExpression
: bitwiseAndExpression
(
CARET
bitwiseAndExpression
)*
;

bitwiseAndExpression
: equalityExpression
(
BIT_AND
equalityExpression
)*
;

equalityExpression
: comparisonExpression
(
EQUALS
| NOT_EQUALS
)
comparisonExpression
*
;

comparisonExpression
: shiftExpression
(
(
LESS_THAN
| LESS_THAN_EQUAL
| GREATER_THAN
| GREATER_THAN_EQUAL
)
shiftExpression
)*
;

shiftExpression
: additiveExpression
(
(
LEFT_SHIFT
| RIGHT_SHIFT
)
additiveExpression
)*
;

additiveExpression
: multiplicativeExpression
(
(
PLUS
| MINUS
)
multiplicativeExpression
)*
;

multiplicativeExpression
: prefixExpression
(
(
STAR
| SLASH
| MODULO
)
prefixExpression
)*
;

prefixExpression
: prefixOperator prefixExpression
| postfixExpression
;

prefixOperator
: PLUS
| MINUS
| NOT_OPERATOR
| TILDE
| AMPERSAND
| STAR
;

postfixExpression
: primaryExpression postfixPart*
;

postfixPart
: callSuffix
| indexSuffix
| memberSuffix
| questionSuffix
;

callSuffix
: LPAREN argumentList? RPAREN
;

indexSuffix
: LBRACKET expression RBRACKET
;

memberSuffix
: DOT identifier
| DOUBLE_COLON identifier
;

questionSuffix
: QUESTION_MARK
;

argumentList
: argument
(
COMMA
argument
)*
COMMA?
;

argument
: expression
;

/*

* ============================================================================
* 11. PRIMARY EXPRESSIONS
* ============================================================================
* 
* Generic operation names are identifiers.
* 
* This is deliberate.
* 
* It prevents the language from becoming a finite catalogue such as:
* 
* H | X | Y | Z | CNOT | ...
* 
* Quantum operation semantics are determined downstream.
* ============================================================================
  */

primaryExpression
: literal
| identifierExpression
| qualifiedIdentifierExpression
| parenthesizedExpression
| tupleExpression
| arrayExpression
| structExpression
| blockExpression
| ifExpression
| matchExpression
| loopExpression
| closureExpression
| quantumPrimaryExpression
| concurrencyPrimaryExpression
| mathPrimaryExpression
| metaPrimaryExpression
;

identifierExpression
: identifier
;

qualifiedIdentifierExpression
: qualifiedName
;

qualifiedName
: identifier
(
DOUBLE_COLON
identifier
)*
;

parenthesizedExpression
: LPAREN expression RPAREN
;

tupleExpression
: LPAREN
expression
COMMA
expression
(
COMMA
expression
)*
COMMA?
RPAREN
;

arrayExpression
: LBRACKET
(
expression
(
COMMA
expression
)*
COMMA?
)?
RBRACKET
;

structExpression
: identifier
LBRACE
structExpressionField*
RBRACE
;

structExpressionField
: identifier
COLON
expression
COMMA?
;

ifExpression
: IF expression blockExpression
(
ELSE IF expression blockExpression
)*
(
ELSE blockExpression
)?
;

matchExpression
: MATCH expression
LBRACE
matchArm*
RBRACE
;

loopExpression
: LOOP blockExpression
;

/*

* ============================================================================
* 12. CLOSURES
* ============================================================================
  */

closureExpression
: PIPE closureParameterList? PIPE
returnType?
(
expression
| blockExpression
)
;

closureParameterList
: closureParameter
(
COMMA
closureParameter
)*
;

closureParameter
: MUT? pattern
(
COLON
typeExpression
)?
;

/*

* ============================================================================
* 13. DOMAIN PRIMARY EXPRESSION ADAPTERS
* ============================================================================
* 
* These adapters deliberately do not introduce finite operation inventories.
* ============================================================================
  */

quantumPrimaryExpression
: quantumOperationExpression
| quantumStateExpression
| quantumComposition
| quantumConditional
| quantumFeedback
;

concurrencyPrimaryExpression
: awaitExpression
| spawnExpression
| parallelExpression
| joinExpression
| sendExpression
| channelExpression
| selectExpression
| remoteExpression
;

mathPrimaryExpression
: mathExpression
| symbolicExpression
| calculusExpression
| linearAlgebraExpression
| tensorExpression
| probabilityExpression
| numericalExpression
| mathematicalObjectExpression
| optimizationExpression
;

metaPrimaryExpression
: macroInvocation
| macroExpression
| metaQuoteExpression
| metaSpliceExpression
| compileTimeExpression
;

/*

* ============================================================================
* 14. BLOCKS
* ============================================================================
  */

blockExpression
: LBRACE
blockElement*
RBRACE
;

blockElement
: attribute
| item
| statement
;

/*

* ============================================================================
* 15. TYPES
* ============================================================================
* 
* Types.g4 owns the detailed type language.
* 
* The root parser only establishes its universal participation points.
* ============================================================================
  */

typeExpression
: typeQualifier*
typeCore
typePostfix*
;

typeQualifier
: LINEAR
| AFFINE
;

typeCore
: primitiveType
| unitType
| neverType
| namedType
| genericType
| tupleType
| arrayType
| sliceType
| functionType
| referenceType
| pointerType
| optionalType
| resultType
| quantumType
| temporalType
| dependentType
| parenthesizedType
;

typePostfix
: optionalPostfix
;

/*

* ============================================================================
* 16. GENERICS
* ============================================================================
  */

genericParameters
: LESS_THAN
genericParameterList
GREATER_THAN
;

genericParameterList
: genericParameter
(
COMMA
genericParameter
)*
COMMA?
;

genericParameter
: identifier
genericBounds?
;

genericBounds
: COLON
typeBoundList
;

typeBoundList
: typeBound
(
PLUS
typeBound
)*
;

typeBound
: typeExpression
;

/*

* ============================================================================
* 17. PARAMETERS
* ============================================================================
  */

parameterList
: parameter
(
COMMA
parameter
)*
COMMA?
;

parameter
: parameterPattern
(
COLON
typeExpression
)?
(
ASSIGN
expression
)?
;

parameterPattern
: pattern
;

returnType
: THIN_ARROW
typeExpression
;

/*

* ============================================================================
* 18. WHERE / CONTRACT / EFFECT CLAUSES
* ============================================================================
  */

whereClause
: WHERE
wherePredicate
(
COMMA
wherePredicate
)*
;

wherePredicate
: identifier
COLON
typeExpression
;

contractClause
: REQUIRES
expression
;

effectClause
: EFFECTS
effectReferenceList
;

effectReferenceList
: effectReference
(
COMMA
effectReference
)*
;

effectReference
: effectName
;

/*

* ============================================================================
* 19. ATTRIBUTES
* ============================================================================
* 
* Attribute syntax is source metadata.
* 
* It does not select hardware or execute compiler operations by itself.
* ============================================================================
  */

attribute
: HASH
LBRACKET
attributeBody
RBRACKET
| AT
identifier
(
LPAREN
argumentList?
RPAREN
)?
;

attributeBody
: qualifiedName
(
LPAREN
argumentList?
RPAREN
)?
;

/*

* ============================================================================
* 20. PATTERNS
* ============================================================================
  */

pattern
: wildcardPattern
| identifierPattern
| literalPattern
| tuplePattern
| arrayPattern
| structPattern
| enumPattern
| referencePattern
| orPattern
| rangePattern
;

wildcardPattern
: UNDERSCORE
;

identifierPattern
: MUT?
identifier
;

literalPattern
: literal
;

tuplePattern
: LPAREN
pattern
COMMA
pattern
(
COMMA
pattern
)*
COMMA?
RPAREN
;

arrayPattern
: LBRACKET
(
pattern
(
COMMA
pattern
)*
COMMA?
)?
RBRACKET
;

structPattern
: identifier
LBRACE
structPatternField*
RBRACE
;

structPatternField
: identifier
(
COLON
pattern
)?
COMMA?
;

enumPattern
: qualifiedIdentifierExpression
(
LPAREN
patternList?
RPAREN
| LBRACE
structPatternField*
RBRACE
)?
;

referencePattern
: AMPERSAND
MUT?
pattern
;

orPattern
: pattern
(
BIT_OR
pattern
)+
;

rangePattern
: literalPattern
rangeOperator
literalPattern
;

patternList
: pattern
(
COMMA
pattern
)*
COMMA?
;

/*

* ============================================================================
* 21. LITERALS
* ============================================================================
* 
* Lexical ownership remains entirely in ZamaniLexer.
* 
* The parser does not impose machine-size limits on literals.
* ============================================================================
  */

literal
: INTEGER
| FLOAT
| STRING
| CHAR
| TRUE
| FALSE
| NIL
| NULL
| quantumLiteral
;

quantumLiteral
: PIPE
quantumBasis
KET_CLOSE
;

quantumBasis
: ZERO
| ONE
| PLUS
| MINUS
;

/*

* ============================================================================
* 22. IDENTIFIERS
* ============================================================================
* 
* IDENTIFIER is lexically owned by ZamaniLexer.
* 
* The parser does not introduce domain-specific identifier categories.
* ============================================================================
  */

identifier
: IDENTIFIER
;

/*

* ============================================================================
* 23. FUNCTION DECLARATIONS
* ============================================================================
  */

functionDeclaration
: visibility?
modifiers?
ASYNC?
FN
identifier
genericParameters?
LPAREN
parameterList?
RPAREN
returnType?
whereClause?
contractClause*
effectClause?
blockExpression
;

functionSignature
: ASYNC?
FN
identifier
genericParameters?
LPAREN
parameterList?
RPAREN
returnType?
whereClause?
SEMI?
;

/*

* ============================================================================
* 24. DECLARATION ADAPTERS
* ============================================================================
  */

constantDeclaration
: visibility?
CONST
identifier
(
COLON
typeExpression
)?
ASSIGN
expression
SEMI?
;

typeAliasDeclaration
: visibility?
TYPE
identifier
genericParameters?
(
ASSIGN
typeExpression
)?
whereClause?
SEMI?
;

/*

* ============================================================================
* 25. VISIBILITY / MODIFIERS
* ============================================================================
  */

visibility
: PUBLIC
| PUB
| PRIVATE
| PROTECTED
;

modifier
: STATIC
| VIRTUAL
| OVERRIDE
| ABSTRACT
| FINAL
| INLINE
| EXTERN
| CONST
;

modifiers
: modifier+
;

/*

* ============================================================================
* 26. MODULE PATH COMPATIBILITY
* ============================================================================
  */

modulePath
: qualifiedName
;

/*

* ============================================================================
* 27. RESOURCE / CAPABILITY / HARDWARE INTENT
* ============================================================================
* 
* Resource and hardware grammar components in grammar/resources/,
* grammar/hardware/, grammar/distributed/, grammar/networking/, etc. are
* intentionally represented through open syntactic contracts here.
* 
* The parser does not resolve physical resources.
* ============================================================================
  */

resourceRequirement
: REQUIRES
resourcePredicate
(
COMMA
resourcePredicate
)*
;

resourcePredicate
: capabilityRequirement
| resourceExpression
;

capabilityRequirement
: CAPABILITY
LPAREN
expression
RPAREN
;

resourceExpression
: identifier
(
LPAREN
argumentList?
RPAREN
)?
;

/*

* ============================================================================
* 28. HARDWARE / TARGET INTENT
* ============================================================================
* 
* These constructs describe intent, not physical topology.
* ============================================================================
  */

targetSpecification
: TARGET
(
identifier
| stringLiteral
| expression
)
;

stringLiteral
: STRING
;

/*

* ============================================================================
* 29. UNIVERSAL DOMAIN EXTENSION
* ============================================================================
* 
* Future domains can participate without modifying the lexical vocabulary.
* 
* A future domain is introduced through ordinary identifiers and structured
* declarations, while its semantic implementation is registered downstream.
* 
* This permits:
* 
* classical
* quantum
* hybrid
* HDL
* embedded
* systems
* distributed
* HPC
* AI/ML
* data
* accelerators
* networking
* cryptography
* scientific computing
* edge/cloud
* future computational substrates
* 
* without making the parser a finite catalogue of technologies.
* ============================================================================
  */

futureDomainDeclaration
: identifier
genericParameters?
blockExpression
;

/*

* ============================================================================
* 30. SOURCE-COMPATIBILITY ADAPTERS
* ============================================================================
* 
* These rules retain structural compatibility with existing component
* grammars while ensuring that the root parser remains authoritative.
* ============================================================================
  */

domainItem
: domainDeclaration
| futureDomainDeclaration
;

/*

* ============================================================================
* 31. EMPTY / UNIT SOURCE
* ============================================================================
  */

unit
: LPAREN RPAREN
;

/*

* ============================================================================
* 32. COMPLETION CONTRACT
* ============================================================================
* 
* This parser is considered integrated only when:
* 
* [ ] ZamaniLexer.g4 generates successfully.
* [ ] This parser generates successfully with ZamaniLexer.
* [ ] Every imported component compiles with the same token vocabulary.
* [ ] Every universal rule has one authoritative owner.
* [ ] No domain creates a competing program rule.
* [ ] No domain creates a competing lexer.
* [ ] No domain creates a competing universal expression grammar.
* [ ] No domain creates a competing type grammar.
* [ ] No domain creates a competing AST authority.
* [ ] Quantum syntax lowers to the existing domain-neutral AST.
* [ ] Quantum semantic lowering terminates at quantum::ir.
* [ ] Classical syntax lowers to the existing classical semantic pipeline.
* [ ] HDL syntax lowers to the hardware/HDL semantic pipeline.
* [ ] Resource syntax remains target-independent.
* [ ] Concurrency syntax remains independent of machine thread counts.
* [ ] Distributed syntax remains independent of node counts.
* [ ] AI/data syntax remains independent of accelerator counts.
* [ ] No fixed qubit count exists in this parser.
* [ ] No fixed CPU/GPU/FPGA/QPU count exists in this parser.
* [ ] No fixed memory capacity exists in this parser.
* [ ] No fixed topology exists in this parser.
* [ ] No vendor backend is selected here.
* [ ] No Rust unsafe is required.
* [ ] No Rust action appears in this grammar.
* [ ] Parser behavior is deterministic.
* [ ] Positive tests exist for every public rule.
* [ ] Negative tests exist for malformed constructs.
* [ ] Boundary tests exist for nesting and composition.
* [ ] Scalability tests do not establish artificial language limits.
* [ ] Compatibility tests cover existing accepted Zamani syntax.
* 
* ============================================================================
* FINAL RULE
* ============================================================================
* 
* The parser describes WHAT the program means structurally.
* 
* It does not prescribe WHERE the program executes.
* 
* Therefore:
* 
* Program Once
*      ->
* Compile Once
*      ->
* Run Everywhere
*      ->
* Run Anywhere
*      ->
* Run Forever
* 
* is achieved by keeping this parser independent of physical realization and
* allowing downstream semantic, resource, optimization, routing, scheduling,
* resilience, ZQN, HAL and backend layers to realize the same source program
* against whatever resources are actually available.
* 
* ============================================================================
  */
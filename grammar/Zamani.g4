/*
 * Zamani.g4
 * ============================================================================
 * Canonical Zamani language grammar.
 *
 * Language goals
 * --------------
 * - Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
 * - Scale from the smallest supported computation to arbitrarily large
 *   computations subject only to program semantics and available resources.
 * - One language for classical, quantum, hybrid, HDL, hardware/software
 *   co-design, AI/ML, data, distributed, networking, security, scientific,
 *   embedded, accelerator and future computational domains.
 *
 * Architectural boundary
 * ----------------------
 * This grammar defines SOURCE SYNTAX.
 *
 * It does NOT define:
 *   - machine limits
 *   - physical topology
 *   - device identifiers
 *   - physical qubit mappings
 *   - scheduling algorithms
 *   - routing algorithms
 *   - QEC implementation
 *   - ZQN fault/noise implementation
 *   - HAL implementation
 *   - calibration implementation
 *   - optimizer implementation
 *   - compiler backend implementation
 *   - runtime implementation
 *
 * Canonical downstream pipeline:
 *
 *   Zamani source
 *       |
 *       v
 *   Zamani.g4
 *       |
 *       v
 *   Lexer / Parser
 *       |
 *       v
 *   Domain-neutral AST
 *       |
 *       v
 *   Structural + semantic analysis
 *       |
 *       v
 *   Canonical semantic model
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   Classical IR          quantum::ir          HDL/Hardware IR
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                  optimization / lowering
 *                              |
 *                  routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * IMPORTANT:
 *   quantum::ir remains the canonical quantum semantic boundary.
 *   This grammar does not create a second quantum IR.
 *
 * Rust implementation policy:
 *   - Rust 2021
 *   - Rust 1.97.1 baseline
 *   - safe Rust only
 *   - no unsafe language construct
 *
 * Hard-coding policy:
 *   The grammar MUST NOT establish universal limits for:
 *     qubits, CPUs, cores, threads, GPUs, FPGAs, nodes, memory,
 *     tensor dimensions, vector widths, registers, accelerators,
 *     timelines, processes, devices, network links, or similar resources.
 *
 * Literal program values are allowed. Universal implementation limits are not.
 *
 * This file is the canonical ANTLR composition/root grammar.
 * Domain-specific documentation and contracts live under grammar/.
 */

grammar Zamani;


/* ============================================================================
 * 1. COMPILATION UNIT
 * ========================================================================== */

program
    : documentation* attribute* compilationUnitItem* EOF
    ;

compilationUnitItem
    : packageDeclaration
    | moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | usingDeclaration
    | languageDeclaration
    | dialectDeclaration
    | declaration
    | statement
    ;


/* ============================================================================
 * 2. DOCUMENTATION / ATTRIBUTES
 * ========================================================================== */

documentation
    : DOC_COMMENT
    ;

attribute
    : '@' qualifiedName
    | '@' qualifiedName '(' argumentList? ')'
    | '@' qualifiedName '{' attributeEntry* '}'
    ;

attributeEntry
    : identifier ':' expression ';'?
    | identifier '=' expression ';'?
    ;

languageDeclaration
    : 'language' qualifiedName versionClause? block
    ;

versionClause
    : 'version' versionLiteral
    ;

versionLiteral
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | STRING_LITERAL
    ;


/* ============================================================================
 * 3. PACKAGES / MODULES / IMPORTS
 * ========================================================================== */

packageDeclaration
    : 'package' qualifiedName versionClause? packageBody?
    ;

packageBody
    : '{' packageItem* '}'
    ;

packageItem
    : packageProperty
    | dependencyDeclaration
    | moduleDeclaration
    | declaration
    ;

packageProperty
    : identifier ':' expression ';'
    ;

dependencyDeclaration
    : 'depends' qualifiedName versionConstraint? dependencyOptions? ';'
    ;

dependencyOptions
    : '{' attributeEntry* '}'
    ;

versionConstraint
    : STRING_LITERAL
    | versionLiteral
    | comparisonVersion+
    ;

comparisonVersion
    : comparisonOperator versionLiteral
    ;

moduleDeclaration
    : visibility? 'module' qualifiedName genericParameterList? moduleBody?
    ;

moduleBody
    : '{' compilationUnitItem* '}'
    ;

importDeclaration
    : 'import' importPath importAlias? ';'
    | 'import' '{' importSpecifierList '}' 'from' importPath ';'
    | 'import' '*' 'as' identifier 'from' importPath ';'
    ;

importPath
    : qualifiedName
    | STRING_LITERAL
    ;

importAlias
    : 'as' identifier
    ;

importSpecifierList
    : importSpecifier (',' importSpecifier)*
    ;

importSpecifier
    : identifier importAlias?
    ;

exportDeclaration
    : 'export' exportSpecifier ';'
    | 'export' '{' exportSpecifierList '}' ';'
    | 'export' '*' 'from' importPath ';'
    ;

exportSpecifier
    : qualifiedName exportAlias?
    ;

exportAlias
    : 'as' identifier
    ;

exportSpecifierList
    : exportSpecifier (',' exportSpecifier)*
    ;

usingDeclaration
    : 'use' qualifiedName ('as' identifier)? ';'
    ;


/* ============================================================================
 * 4. VISIBILITY / MODIFIERS
 * ========================================================================== */

visibility
    : 'pub'
    | 'public'
    | 'private'
    | 'protected'
    | 'internal'
    ;

modifier
    : 'static'
    | 'const'
    | 'async'
    | 'inline'
    | 'override'
    | 'final'
    | 'abstract'
    | 'mut'
    | 'sealed'
    | 'partial'
    | 'extern'
    | 'volatile'
    | 'pure'
    | 'immutable'
    | 'parallel'
    | 'vectorized'
    | 'simd'
    ;

modifierList
    : modifier+
    ;


/* ============================================================================
 * 5. DECLARATIONS
 * ========================================================================== */

declaration
    : annotatedDeclaration
    | functionDeclaration
    | structDeclaration
    | recordDeclaration
    | enumDeclaration
    | classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implementationDeclaration
    | typeAliasDeclaration
    | typeDeclaration
    | constantDeclaration
    | resourceDeclaration
    | capabilityDeclaration
    | effectDeclaration
    | macroDeclaration
    | foreignDeclaration
    | domainDeclaration
    | hardwareDeclaration
    | modelDeclaration
    | dataDeclaration
    | hdlDeclaration
    | quantumDeclaration
    | dialectDeclaration
    ;

annotatedDeclaration
    : attribute+ declaration
    ;


/* ============================================================================
 * 6. FUNCTIONS
 * ========================================================================== */

functionDeclaration
    : visibility?
      modifierList?
      'fn'
      identifier
      genericParameterList?
      '(' parameterList? ')'
      returnTypeClause?
      whereClause?
      effectClause?
      contractClause*
      block
    ;

returnTypeClause
    : '->' typeExpression
    ;

parameterList
    : parameter (',' parameter)* ','?
    ;

parameter
    : modifierList?
      'self'
    | modifierList?
      identifier
      (':' typeExpression)?
      defaultValue?
    ;

defaultValue
    : '=' expression
    ;

genericParameterList
    : '<' genericParameter (',' genericParameter)* ','? '>'
    ;

genericParameter
    : identifier genericBoundList?
    | 'const' identifier ':' typeExpression
    | 'type' identifier genericBoundList?
    ;

genericBoundList
    : ':' typeBound ('+' typeBound)*
    ;

typeBound
    : qualifiedName
    | lifetime
    ;

whereClause
    : 'where' wherePredicate (',' wherePredicate)*
    ;

wherePredicate
    : typeExpression ':' typeBound ('+' typeBound)*
    ;

effectClause
    : 'with' 'effects' effectSet
    ;

effectSet
    : '{' effectNameList? '}'
    ;

effectNameList
    : effectName (',' effectName)*
    ;

effectName
    : qualifiedName
    ;

contractClause
    : 'requires' expression ';'
    | 'ensures' expression ';'
    | 'invariant' expression ';'
    ;


/* ============================================================================
 * 7. STRUCTS / RECORDS / ENUMS / CLASSES / TRAITS
 * ========================================================================== */

structDeclaration
    : visibility?
      'struct'
      identifier
      genericParameterList?
      '{'
      structField*
      '}'
    ;

structField
    : visibility? identifier ':' typeExpression ','?
    ;

recordDeclaration
    : visibility?
      'record'
      identifier
      genericParameterList?
      '{'
      structField*
      '}'
    ;

enumDeclaration
    : visibility?
      'enum'
      identifier
      genericParameterList?
      '{'
      enumVariant*
      '}'
    ;

enumVariant
    : identifier
    | identifier '(' typeList? ')'
    | identifier '{' structField* '}'
    ;

classDeclaration
    : visibility?
      modifierList?
      'class'
      identifier
      genericParameterList?
      inheritanceClause?
      interfaceClause?
      '{'
      classMember*
      '}'
    ;

classMember
    : fieldDeclaration
    | functionDeclaration
    | constantDeclaration
    | typeAliasDeclaration
    | declaration
    ;

fieldDeclaration
    : visibility? modifierList? identifier ':' typeExpression ('=' expression)? ';'
    ;

inheritanceClause
    : 'extends' typeExpression (',' typeExpression)*
    ;

interfaceClause
    : 'implements' typeExpression (',' typeExpression)*
    ;

interfaceDeclaration
    : visibility?
      'interface'
      identifier
      genericParameterList?
      interfaceInheritanceClause?
      '{'
      interfaceMember*
      '}'
    ;

interfaceInheritanceClause
    : ':' typeExpression (',' typeExpression)*
    ;

interfaceMember
    : functionSignature
    | associatedTypeDeclaration
    | constantDeclaration
    ;

functionSignature
    : visibility?
      'fn'
      identifier
      genericParameterList?
      '(' parameterList? ')'
      returnTypeClause?
      effectClause?
      ';'
    ;

associatedTypeDeclaration
    : 'type' identifier genericParameterList? typeBoundClause? ';'
    ;

typeBoundClause
    : ':' typeBound ('+' typeBound)*
    ;

traitDeclaration
    : visibility?
      'trait'
      identifier
      genericParameterList?
      interfaceInheritanceClause?
      '{'
      traitMember*
      '}'
    ;

traitMember
    : functionSignature
    | associatedTypeDeclaration
    | constantDeclaration
    ;

implementationDeclaration
    : 'impl'
      genericParameterList?
      implementationTarget
      implementationFor?
      whereClause?
      '{'
      implementationMember*
      '}'
    ;

implementationTarget
    : typeExpression
    ;

implementationFor
    : 'for' typeExpression
    ;

implementationMember
    : functionDeclaration
    | constantDeclaration
    | typeAliasDeclaration
    ;


/* ============================================================================
 * 8. TYPE DECLARATIONS
 * ========================================================================== */

typeAliasDeclaration
    : visibility? 'type' identifier genericParameterList? '=' typeExpression ';'
    ;

typeDeclaration
    : visibility? 'type' identifier genericParameterList? typeBoundClause? ';'
    ;

constantDeclaration
    : visibility?
      'const'
      identifier
      (':' typeExpression)?
      '=' expression
      ';'
    ;


/* ============================================================================
 * 9. TYPES
 *
 * Types describe semantics. They do not encode implementation limits.
 * ========================================================================== */

typeExpression
    : functionType
    | referenceType
    | pointerType
    | optionalType
    | resultType
    | arrayType
    | sliceType
    | tupleType
    | genericType
    | quantumType
    | tensorType
    | resourceType
    | capabilityType
    | primitiveType
    | namedType
    | neverType
    ;

functionType
    : '(' typeList? ')' '->' typeExpression
    ;

referenceType
    : '&' lifetime? mutableMarker? typeExpression
    ;

mutableMarker
    : 'mut'
    ;

pointerType
    : '*' mutableMarker? typeExpression
    ;

optionalType
    : '?' typeExpression
    ;

resultType
    : 'Result' '<' typeExpression ',' typeExpression '>'
    ;

arrayType
    : '[' typeExpression ';' expression ']'
    ;

sliceType
    : '[' typeExpression ']'
    ;

tupleType
    : '(' typeList? ')'
    ;

genericType
    : qualifiedName '<' typeArgumentList '>'
    ;

typeArgumentList
    : typeArgument (',' typeArgument)* ','?
    ;

typeArgument
    : typeExpression
    | expression
    ;

quantumType
    : 'Qubit'
    | 'Qubit' '<' expression '>'
    | 'QRegister' '<' expression '>'
    | 'QState' '<' expression '>'
    | 'QuantumRegister' '<' expression '>'
    | 'LogicalQubit'
    | 'LogicalRegister' '<' expression '>'
    ;

tensorType
    : 'Tensor' '<' typeExpression tensorShape? '>'
    ;

tensorShape
    : ',' shapeExpressionList
    ;

shapeExpressionList
    : shapeExpression (',' shapeExpression)*
    ;

shapeExpression
    : expression
    | '_'
    ;

resourceType
    : 'Resource' '<' typeExpression '>'
    ;

capabilityType
    : 'Capability' '<' qualifiedName '>'
    ;

primitiveType
    : 'bool'
    | 'char'
    | 'int'
    | 'uint'
    | 'float'
    | 'f16'
    | 'f32'
    | 'f64'
    | 'f128'
    | 'str'
    | 'String'
    | 'byte'
    | 'unit'
    | 'void'
    ;

neverType
    : 'never'
    ;

namedType
    : qualifiedName
    ;

typeList
    : typeExpression (',' typeExpression)*
    ;


/* ============================================================================
 * 10. STATEMENTS
 * ========================================================================== */

statement
    : variableDeclaration
    | expressionStatement
    | ifStatement
    | whileStatement
    | doWhileStatement
    | forStatement
    | quantifiedLoopStatement
    | parallelLoopStatement
    | reductionStatement
    | matchStatement
    | returnStatement
    | breakStatement
    | continueStatement
    | throwStatement
    | tryStatement
    | blockStatement
    | spawnStatement
    | awaitStatement
    | resourceStatement
    | capabilityStatement
    | requirementStatement
    | constraintStatement
    | preferenceStatement
    | hintStatement
    | effectStatement
    | handleStatement
    | performStatement
    | quantumStatement
    | hybridStatement
    | hdlStatement
    | hardwareStatement
    | distributedStatement
    | dataStatement
    | aiStatement
    | networkingStatement
    | securityStatement
    | compileStatement
    | executionStatement
    | memoryStatement
    | timelineStatement
    | rememberStatement
    | recallStatement
    | learnStatement
    ;

variableDeclaration
    : modifierList?
      ('let' | 'var')
      identifier
      (':' typeExpression)?
      ('=' expression)?
      ';'
    ;

expressionStatement
    : expression ';'
    ;

ifStatement
    : 'if' expression block ('else' 'if' expression block)* ('else' block)?
    ;

whileStatement
    : 'while' expression block
    ;

doWhileStatement
    : 'do' block 'while' expression ';'
    ;

forStatement
    : 'for' identifier 'in' expression block
    | 'for' '(' variableDeclaration expression ';' expression? ')' block
    ;

quantifiedLoopStatement
    : 'forall' identifier 'in' expression ('when' expression)? block
    ;

parallelLoopStatement
    : 'foreach' identifier 'in' expression 'parallel' block
    ;

reductionStatement
    : 'reduce' identifier 'in' expression 'with' expression block
    ;

matchStatement
    : 'match' expression '{' matchArm* '}'
    ;

matchArm
    : 'case' pattern ('when' expression)? '=>' statementOrExpression
    ;

statementOrExpression
    : statement
    | expression
    ;

returnStatement
    : 'return' expression? ';'
    ;

breakStatement
    : 'break' ';'
    ;

continueStatement
    : 'continue' ';'
    ;

throwStatement
    : 'throw' expression ';'
    ;

tryStatement
    : 'try' block catchClause* finallyClause?
    ;

catchClause
    : 'catch' '(' identifier (':' typeExpression)? ')' block
    ;

finallyClause
    : 'finally' block
    ;

blockStatement
    : block
    ;

block
    : '{' blockItem* '}'
    ;

blockItem
    : declaration
    | statement
    ;

spawnStatement
    : 'spawn' expression ';'
    ;

awaitStatement
    : 'await' expression ';'
    ;


/* ============================================================================
 * 11. PATTERNS
 * ========================================================================== */

pattern
    : '_'
    | identifier
    | literal
    | tuplePattern
    | arrayPatternPattern
    | rangePattern
    | qualifiedPattern
    | orPattern
    ;

tuplePattern
    : '(' pattern (',' pattern)+ ')'
    ;

arrayPatternPattern
    : '[' pattern (',' pattern)* ']'
    ;

rangePattern
    : pattern ('..' | '..=') pattern
    ;

qualifiedPattern
    : qualifiedName '(' patternList? ')'
    ;

orPattern
    : pattern ('|' pattern)+
    ;

patternList
    : pattern (',' pattern)*
    ;


/* ============================================================================
 * 12. EXPRESSIONS
 *
 * The grammar provides generic expression machinery. Domain operations are
 * represented as ordinary calls/operations rather than forcing every library
 * algorithm into the language keyword set.
 * ========================================================================== */

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

conditionalExpression
    : logicalOrExpression
      ('?' expression ':' expression)?
    ;

logicalOrExpression
    : logicalAndExpression ('||' logicalAndExpression)*
    ;

logicalAndExpression
    : bitwiseOrExpression ('&&' bitwiseOrExpression)*
    ;

bitwiseOrExpression
    : bitwiseXorExpression ('|' bitwiseXorExpression)*
    ;

bitwiseXorExpression
    : bitwiseAndExpression ('^' bitwiseAndExpression)*
    ;

bitwiseAndExpression
    : equalityExpression ('&' equalityExpression)*
    ;

equalityExpression
    : comparisonExpression (('==' | '!=') comparisonExpression)*
    ;

comparisonExpression
    : shiftExpression
      (
          ('<' | '<=' | '>' | '>=' | 'in' | 'is')
          shiftExpression
      )*
    ;

shiftExpression
    : additiveExpression (('<<' | '>>') additiveExpression)*
    ;

additiveExpression
    : multiplicativeExpression (('+' | '-') multiplicativeExpression)*
    ;

multiplicativeExpression
    : powerExpression (('*' | '/' | '%') powerExpression)*
    ;

powerExpression
    : unaryExpression ('**' powerExpression)?
    ;

unaryExpression
    : ('+' | '-' | '!' | '~' | 'not' | 'await') unaryExpression
    | postfixExpression
    ;

postfixExpression
    : primaryExpression postfixOperation*
    ;

postfixOperation
    : '(' argumentList? ')'
    | '[' expression ']'
    | '[' expression '..' expression ']'
    | '[' expression '..=' expression ']'
    | '.' identifier
    | '::' identifier
    | '?' 
    ;

primaryExpression
    : literal
    | identifier
    | qualifiedName
    | lambdaExpression
    | ifExpression
    | matchExpression
    | blockExpression
    | arrayExpression
    | tupleExpression
    | mapExpression
    | structExpression
    | quantumOperationExpression
    | tensorExpression
    | resourceExpression
    | capabilityExpression
    | macroInvocation
    | '(' expression ')'
    ;

lambdaExpression
    : '|' parameterList? '|' ('->' typeExpression)? expressionOrBlock
    ;

expressionOrBlock
    : expression
    | block
    ;

ifExpression
    : 'if' expression block ('else' 'if' expression block)* 'else' block
    ;

matchExpression
    : 'match' expression '{' matchArm* '}'
    ;

blockExpression
    : block
    ;

arrayExpression
    : '[' argumentList? ']'
    ;

tupleExpression
    : '(' expression (',' expression)+ ','? ')'
    ;

mapExpression
    : '{' mapEntryList? '}'
    ;

mapEntryList
    : mapEntry (',' mapEntry)* ','?
    ;

mapEntry
    : expression ':' expression
    ;

structExpression
    : qualifiedName '{' structInitializerList? '}'
    ;

structInitializerList
    : structInitializer (',' structInitializer)* ','?
    ;

structInitializer
    : identifier ':' expression
    ;

argumentList
    : argument (',' argument)* ','?
    ;

argument
    : expression
    | namedArgument
    ;

namedArgument
    : identifier '=' expression
    ;

quantumOperationExpression
    : 'apply'
      qualifiedName
      quantumArgumentList?
      quantumModifierList?
    ;

quantumArgumentList
    : 'to' quantumTargetList
    | '(' argumentList? ')' 'to' quantumTargetList
    ;

quantumTargetList
    : quantumTarget (',' quantumTarget)*
    ;

quantumTarget
    : expression
    | quantumTargetRange
    ;

quantumTargetRange
    : expression '..' expression
    ;

quantumModifierList
    : quantumModifier+
    ;

quantumModifier
    : 'controlled'
    | 'adjoint'
    | 'inverse'
    | 'power' '(' expression ')'
    | 'with' '(' argumentList? ')'
    ;

tensorExpression
    : 'tensor' '<' typeExpression '>' '(' argumentList? ')'
    ;

resourceExpression
    : 'resource' '(' argumentList? ')'
    ;

capabilityExpression
    : 'capability' '(' argumentList? ')'
    ;

macroInvocation
    : identifier '!' '(' argumentList? ')'
    ;

assignmentOperator
    : '='
    | '+='
    | '-='
    | '*='
    | '/='
    | '%='
    ;


/* ============================================================================
 * 13. LITERALS
 *
 * Numeric magnitude is intentionally unrestricted by this grammar.
 * Semantic/backend representability is decided later.
 * ========================================================================== */

literal
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | BOOLEAN_LITERAL
    | NIL_LITERAL
    | QUANTUM_LITERAL
    | COMPLEX_LITERAL
    | DURATION_LITERAL
    ;

complexLiteral
    : DECIMAL_LITERAL ('+' | '-') DECIMAL_LITERAL 'i'
    ;

durationLiteral
    : INTEGER_LITERAL identifier
    ;

BOOLEAN_LITERAL
    : 'true'
    | 'false'
    ;

NIL_LITERAL
    : 'nil'
    | 'null'
    ;


/* ============================================================================
 * 14. QUANTUM COMPUTING
 *
 * No universal gate enumeration.
 *
 * The operation identifier is semantic data and is resolved after parsing.
 * Therefore the grammar supports:
 *
 *   apply H to q
 *   apply X to q
 *   apply vendor.operation to q
 *   apply custom_gate(theta) to q0, q1
 *
 * without modifying the grammar for every new gate, vendor operation,
 * logical operation, or future quantum primitive.
 * ========================================================================== */

quantumDeclaration
    : quantumRegisterDeclaration
    | quantumCircuitDeclaration
    | quantumKernelDeclaration
    | quantumOperationDeclaration
    | quantumObservableDeclaration
    | quantumChannelDeclaration
    | quantumErrorModelDeclaration
    ;

quantumRegisterDeclaration
    : visibility?
      ('qubit' | 'qregister' | 'quantum')
      identifier
      (':' typeExpression)?
      ('=' expression)?
      ';'
    ;

quantumCircuitDeclaration
    : visibility?
      'circuit'
      identifier
      genericParameterList?
      '(' parameterList? ')'
      block
    ;

quantumKernelDeclaration
    : visibility?
      'kernel'
      identifier
      genericParameterList?
      '(' parameterList? ')'
      block
    ;

quantumOperationDeclaration
    : visibility?
      'operation'
      qualifiedName
      genericParameterList?
      '(' parameterList? ')'
      returnTypeClause?
      block
    ;

quantumObservableDeclaration
    : visibility?
      'observable'
      identifier
      ':' typeExpression
      ('=' expression)?
      ';'
    ;

quantumChannelDeclaration
    : visibility?
      'channel'
      identifier
      ':' typeExpression
      ('=' expression)?
      ';'
    ;

quantumErrorModelDeclaration
    : visibility?
      'noise'
      identifier
      ('{' quantumErrorEntry* '}')?
    ;

quantumErrorEntry
    : identifier ':' expression ';'
    ;

quantumStatement
    : quantumApplyStatement
    | quantumMeasureStatement
    | quantumResetStatement
    | quantumBarrierStatement
    | quantumAllocateStatement
    | quantumReleaseStatement
    | quantumControlStatement
    | quantumClassicalFeedForwardStatement
    ;

quantumApplyStatement
    : 'apply'
      qualifiedName
      quantumInvocationArguments?
      'to'
      quantumTargetList
      ';'
    ;

quantumInvocationArguments
    : '(' argumentList? ')'
    ;

quantumMeasureStatement
    : 'measure'
      quantumTargetList
      ('into' expression)?
      ';'
    ;

quantumResetStatement
    : 'reset' quantumTargetList ';'
    ;

quantumBarrierStatement
    : 'barrier' quantumTargetList? ';'
    ;

quantumAllocateStatement
    : 'allocate'
      ('qubits' | 'qbits' | 'quantum')
      '[' expression ']'
      ('as' identifier)?
      ';'
    ;

quantumReleaseStatement
    : 'release'
      ('qubits' | 'qbits' | 'quantum')
      quantumTargetList
      ';'
    ;

quantumControlStatement
    : 'control'
      '(' quantumTargetList ')'
      quantumOperationExpression
      ';'
    ;

quantumClassicalFeedForwardStatement
    : 'if' expression quantumStatement
    ;

hybridStatement
    : 'quantum' block
    | 'classical' block
    | 'host' block
    | 'device' block
    | 'feedforward' block
    | 'synchronize' block
    ;


/* ============================================================================
 * 15. RESOURCES / CAPABILITIES / PORTABILITY
 *
 * Requirement != constraint != capability != preference != hint != decision.
 *
 * These constructs express portable intent. They do not select a physical
 * machine during parsing.
 * ========================================================================== */

resourceDeclaration
    : visibility?
      'resource'
      identifier
      resourceTypeClause?
      resourcePropertyBlock?
      ';'?
    ;

resourceTypeClause
    : ':' typeExpression
    ;

resourcePropertyBlock
    : '{' resourceProperty* '}'
    ;

resourceProperty
    : identifier ':' expression ';'
    ;

capabilityDeclaration
    : visibility?
      'capability'
      qualifiedName
      capabilityPropertyBlock?
      ';'?
    ;

capabilityPropertyBlock
    : '{' capabilityProperty* '}'
    ;

capabilityProperty
    : identifier ':' expression ';'
    ;

requirementStatement
    : 'requires'
      capabilityRequirementList?
      resourceRequirementList?
      requirementPropertyBlock?
      ';'
    ;

capabilityRequirementList
    : 'capability' '(' qualifiedName ')'
      (',' 'capability' '(' qualifiedName ')')*
    ;

resourceRequirementList
    : 'resource' '(' expression ')'
      (',' 'resource' '(' expression ')')*
    ;

requirementPropertyBlock
    : '{' requirementProperty* '}'
    ;

requirementProperty
    : identifier ':' expression ';'
    ;

constraintStatement
    : 'constrain'
      expression
      ';'
    ;

preferenceStatement
    : 'prefer'
      preferenceTarget
      ';'
    ;

preferenceTarget
    : qualifiedName
    | expression
    ;

hintStatement
    : 'hint'
      identifier
      (':' expression | '=' expression)
      ';'
    ;

capabilityStatement
    : 'requires' 'capability' '(' qualifiedName ')' ';'
    ;


/* ============================================================================
 * 16. HARDWARE / TARGET-NEUTRAL HARDWARE INTENT
 * ========================================================================== */

hardwareDeclaration
    : visibility?
      'hardware'
      identifier
      hardwarePropertyBlock?
      ';'?
    ;

hardwarePropertyBlock
    : '{' hardwareProperty* '}'
    ;

hardwareProperty
    : hardwareCapabilityProperty
    | hardwareResourceProperty
    | hardwareTopologyProperty
    | hardwareTimingProperty
    | hardwareReliabilityProperty
    | hardwareDeploymentProperty
    | propertyEntry
    ;

hardwareCapabilityProperty
    : 'capability' ':' expression ';'
    ;

hardwareResourceProperty
    : 'resource' ':' expression ';'
    ;

hardwareTopologyProperty
    : 'topology' ':' expression ';'
    ;

hardwareTimingProperty
    : 'timing' ':' expression ';'
    ;

hardwareReliabilityProperty
    : 'reliability' ':' expression ';'
    ;

hardwareDeploymentProperty
    : 'deployment' ':' expression ';'
    ;

hardwareStatement
    : 'target' qualifiedName hardwareBlock?
    | 'requires' 'hardware' expression ';'
    ;

hardwareBlock
    : '{' hardwareProperty* '}'
    ;


/* ============================================================================
 * 17. HDL / HARDWARE-SOFTWARE CO-DESIGN
 * ========================================================================== */

hdlDeclaration
    : visibility?
      'hdl'
      identifier
      hdlParameterList?
      hdlBlock
    ;

hdlParameterList
    : '<' hdlParameter (',' hdlParameter)* '>'
    ;

hdlParameter
    : identifier ':' typeExpression
    | identifier '=' expression
    ;

hdlBlock
    : '{' hdlItem* '}'
    ;

hdlItem
    : hdlPortDeclaration
    | hdlSignalDeclaration
    | hdlRegisterDeclaration
    | hdlMemoryDeclaration
    | hdlClockDeclaration
    | hdlResetDeclaration
    | hdlCombinationalBlock
    | hdlSequentialBlock
    | hdlGenerateBlock
    | hdlStateMachine
    | hdlPipeline
    | hdlAssertion
    | declaration
    | statement
    ;

hdlPortDeclaration
    : 'port'
      identifier
      ':' typeExpression
      hdlDirection?
      ';'
    ;

hdlDirection
    : 'in'
    | 'out'
    | 'inout'
    ;

hdlSignalDeclaration
    : 'signal' identifier ':' typeExpression ';'
    ;

hdlRegisterDeclaration
    : 'register' identifier ':' typeExpression ('=' expression)? ';'
    ;

hdlMemoryDeclaration
    : 'memory'
      identifier
      ':'
      typeExpression
      ('[' expression ']')?
      ';'
    ;

hdlClockDeclaration
    : 'clock' identifier
      (':' typeExpression)?
      ('=' expression)?
      ';'
    ;

hdlResetDeclaration
    : 'reset' identifier
      ('=' expression)?
      ';'
    ;

hdlCombinationalBlock
    : 'combinational' block
    ;

hdlSequentialBlock
    : 'sequential' block
    ;

hdlGenerateBlock
    : 'generate'
      '(' expression ')'
      block
    ;

hdlStateMachine
    : 'state' identifier
      '{'
      hdlState*
      '}'
    ;

hdlState
    : identifier
      ('when' expression)?
      block
    ;

hdlPipeline
    : 'pipeline'
      identifier?
      ('stages' expression)?
      block
    ;

hdlAssertion
    : 'assert'
      expression
      ';'
    ;

hdlStatement
    : 'drive' expression 'with' expression ';'
    | 'sample' expression ';'
    | 'clock' expression ';'
    | 'synthesize' expression ';'
    | 'simulate' expression ';'
    | 'verify' expression ';'
    ;


/* ============================================================================
 * 18. CLASSICAL COMPUTING
 *
 * Mathematical algorithms remain available without making every algorithm a
 * reserved keyword. The language supports generic calls such as:
 *
 *   fft(signal)
 *   svd(matrix)
 *   gradient(f, x)
 *   optimize(problem)
 *
 * while domain libraries/intrinsics provide semantic resolution.
 * ========================================================================== */

domainDeclaration
    : visibility?
      'domain'
      qualifiedName
      domainBody?
    ;

domainBody
    : '{' domainMember* '}'
    ;

domainMember
    : capabilityDeclaration
    | resourceDeclaration
    | typeAliasDeclaration
    | declaration
    ;

classicalOperation
    : qualifiedName '(' argumentList? ')'
    ;

tensorExpressionStatement
    : tensorExpression ';'
    ;

memoryStatement
    : 'allocate' expression ';'
    | 'deallocate' expression ';'
    | 'move' expression ';'
    | 'copy' expression ';'
    | 'share' expression ';'
    | 'borrow' expression ';'
    ;


/* ============================================================================
 * 19. EFFECTS
 * ========================================================================== */

effectDeclaration
    : visibility?
      'effect'
      qualifiedName
      effectBody?
    ;

effectBody
    : '{'
      effectOperation*
      '}'
    ;

effectOperation
    : 'perform' identifier
      '(' parameterList? ')'
      returnTypeClause?
      ';'
    ;

effectStatement
    : 'effect' qualifiedName ';'
    ;

handleStatement
    : 'handle'
      qualifiedName
      block
      ('with' block)?
    ;

performStatement
    : 'perform'
      qualifiedName
      '(' argumentList? ')'
      ';'
    ;


/* ============================================================================
 * 20. DISTRIBUTED / PARALLEL COMPUTING
 * ========================================================================== */

distributedDeclaration
    : 'distributed'
      qualifiedName?
      distributedBlock
    ;

distributedBlock
    : '{' distributedItem* '}'
    ;

distributedItem
    : nodeDeclaration
    | serviceDeclaration
    | processDeclaration
    | channelDeclaration
    | placementDeclaration
    | replicationDeclaration
    | consistencyDeclaration
    | communicationDeclaration
    | declaration
    | statement
    ;

nodeDeclaration
    : 'node' identifier
      ('requires' expression)?
      ';'
    ;

serviceDeclaration
    : 'service' identifier
      serviceBody?
    ;

serviceBody
    : '{' declaration* '}'
    ;

processDeclaration
    : 'process' identifier block
    ;

channelDeclaration
    : 'channel' identifier
      (':' typeExpression)?
      ';'
    ;

placementDeclaration
    : 'placement' identifier
      'where'
      expression
      ';'
    ;

replicationDeclaration
    : 'replicate' expression
      ('factor' expression)?
      ';'
    ;

consistencyDeclaration
    : 'consistency' expression ';'
    ;

communicationDeclaration
    : 'communicate' expression ';'
    ;


/* ============================================================================
 * 21. CONCURRENCY
 * ========================================================================== */

concurrencyStatement
    : 'spawn' expression ';'
    | 'await' expression ';'
    | 'parallel' block
    | 'task' block
    | 'actor' identifier block
    | 'channel' identifier ';'
    | 'synchronize' block
    ;

parallelismExpression
    : 'parallel' '(' expression ')'
    ;


/* ============================================================================
 * 22. AI / ML
 *
 * Framework-neutral syntax. No framework or accelerator is part of the
 * canonical grammar.
 * ========================================================================== */

modelDeclaration
    : visibility?
      'model'
      identifier
      genericParameterList?
      modelBody
    ;

modelBody
    : '{'
      modelItem*
      '}'
    ;

modelItem
    : modelParameter
    | modelInput
    | modelOutput
    | modelArchitecture
    | modelTraining
    | modelInference
    | declaration
    ;

modelParameter
    : 'parameter' identifier ':' typeExpression ('=' expression)? ';'
    ;

modelInput
    : 'input' identifier ':' typeExpression ';'
    ;

modelOutput
    : 'output' identifier ':' typeExpression ';'
    ;

modelArchitecture
    : 'architecture' expression ';'
    ;

modelTraining
    : 'training' block
    ;

modelInference
    : 'inference' block
    ;

aiStatement
    : 'train' expression ';'
    | 'infer' expression ';'
    | 'learn' expression ';'
    | 'optimize' expression ';'
    | 'predict' expression ';'
    | 'agent' identifier block
    ;


/* ============================================================================
 * 23. DATA
 * ========================================================================== */

dataDeclaration
    : visibility?
      'data'
      identifier
      dataSchema?
      dataBody?
    ;

dataSchema
    : ':' typeExpression
    ;

dataBody
    : '{' dataItem* '}'
    ;

dataItem
    : 'field' identifier ':' typeExpression ';'
    | 'schema' expression ';'
    | 'source' expression ';'
    | 'transform' expression ';'
    | 'pipeline' block
    | declaration
    ;

dataStatement
    : 'load' expression ';'
    | 'save' expression ';'
    | 'stream' expression ';'
    | 'query' expression ';'
    | 'transform' expression ';'
    | 'serialize' expression ';'
    | 'deserialize' expression ';'
    ;


/* ============================================================================
 * 24. NETWORKING
 * ========================================================================== */

networkingStatement
    : 'connect' expression ';'
    | 'listen' expression ';'
    | 'send' expression ';'
    | 'receive' expression ';'
    | 'request' expression ';'
    | 'respond' expression ';'
    | 'stream' expression ';'
    ;


/* ============================================================================
 * 25. SECURITY / CRYPTOGRAPHY
 * ========================================================================== */

securityStatement
    : 'authorize' expression ';'
    | 'authenticate' expression ';'
    | 'encrypt' expression ';'
    | 'decrypt' expression ';'
    | 'sign' expression ';'
    | 'verify' expression ';'
    | 'hash' expression ';'
    | 'prove' expression ';'
    | 'protect' expression ';'
    ;


/* ============================================================================
 * 26. COMPILATION / DEPLOYMENT / EXECUTION
 *
 * These describe intent and policies. They do not force a particular target.
 * ========================================================================== */

compileStatement
    : 'compile' compileTarget? compileOptions? ';'
    ;

compileTarget
    : expression
    ;

compileOptions
    : '{' compileOption* '}'
    ;

compileOption
    : identifier ':' expression ';'
    ;

executionStatement
    : 'execute' expression ';'
    | 'run' expression ';'
    | 'deploy' expression ';'
    | 'schedule' expression ';'
    | 'checkpoint' expression ';'
    | 'recover' expression ';'
    | 'observe' expression ';'
    ;


/* ============================================================================
 * 27. RESILIENCE / QEC / FAULT SEMANTICS
 *
 * Syntax carries intent only.
 *
 * QEC implementation remains in the QEC subsystem.
 * ZQN remains the canonical fault/noise semantic layer.
 * HAL remains responsible for device capabilities/state.
 * ========================================================================== */

resilienceStatement
    : 'requires' 'reliability' expression ';'
    | 'requires' 'fault_tolerance' expression ';'
    | 'requires' 'error_correction' expression ';'
    | 'requires' 'noise_budget' expression ';'
    ;

timelineStatement
    : 'timeline' identifier timelineBody
    ;

timelineBody
    : '{' timelineItem* '}'
    ;

timelineItem
    : 'fork' expression ';'
    | 'merge' expression ';'
    | 'observe' expression ';'
    | 'rewind' expression ';'
    | 'speculate' expression ';'
    | declaration
    | statement
    ;


/* ============================================================================
 * 28. SANKOFA / LONG-LIVED COMPUTATION / PROVENANCE
 * ========================================================================== */

rememberStatement
    : 'remember' expression ('as' identifier)? ';'
    ;

recallStatement
    : 'recall' expression ';'
    ;

learnStatement
    : 'learn' expression ';'
    ;


/* ============================================================================
 * 29. FOREIGN INTEROPERABILITY
 * ========================================================================== */

foreignDeclaration
    : visibility?
      'extern'
      foreignLanguage?
      'fn'
      identifier
      '(' parameterList? ')'
      returnTypeClause?
      ';'
    | visibility?
      'foreign'
      qualifiedName
      foreignBlock?
    ;

foreignLanguage
    : 'c'
    | 'cpp'
    | 'python'
    | 'rust'
    | 'wasm'
    | 'qasm'
    | 'qir'
    | 'hdl'
    | identifier
    ;

foreignBlock
    : '{' foreignItem* '}'
    ;

foreignItem
    : declaration
    | statement
    ;


/* ============================================================================
 * 30. DIALECTS
 *
 * Dialects extend syntax/semantics explicitly. They are not separate
 * languages and must eventually map back into the canonical AST/semantic model.
 * ========================================================================== */

dialectDeclaration
    : visibility?
      'dialect'
      qualifiedName
      dialectVersion?
      dialectBody?
    ;

dialectVersion
    : '@' versionLiteral
    ;

dialectBody
    : '{'
      dialectProperty*
      '}'
    ;

dialectProperty
    : 'syntax' ':' expression ';'
    | 'semantics' ':' expression ';'
    | 'capabilities' ':' expression ';'
    | 'compatibility' ':' expression ';'
    | 'feature' ':' expression ';'
    ;


/* ============================================================================
 * 31. MACROS / METAPROGRAMMING
 *
 * Macro expansion must still pass the normal AST, semantic and IR validation
 * pipeline.
 * ========================================================================== */

macroDeclaration
    : visibility?
      'macro'
      identifier
      genericParameterList?
      '(' macroParameterList? ')'
      macroBody
    ;

macroParameterList
    : macroParameter (',' macroParameter)*
    ;

macroParameter
    : identifier
    | '$' identifier
    ;

macroBody
    : block
    | '=>' expression ';'
    ;

metaprogrammingExpression
    : 'quote' block
    | 'quote' expression
    | 'unquote' expression
    | 'reflect' expression
    | 'generate' expression
    ;


/* ============================================================================
 * 32. SOURCE-LEVEL SECURITY / NO-UNSAFE POLICY
 *
 * There is intentionally NO `unsafe` production.
 *
 * Zamani source cannot bypass the semantic safety boundary through a grammar
 * construct. Low-level implementation mechanisms, if ever required, belong
 * outside the safe Zamani language and must not become a portable language
 * escape hatch.
 * ========================================================================== */


/* ============================================================================
 * 33. QUALIFIED NAMES
 * ========================================================================== */

qualifiedName
    : identifier ('::' identifier)*
    ;

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 34. PROPERTY / GENERIC HELPERS
 * ========================================================================== */

propertyEntry
    : identifier ':' expression ';'
    ;


/* ============================================================================
 * 35. LIFETIMES
 * ========================================================================== */

lifetime
    : '\'' identifier
    ;


/* ============================================================================
 * 36. LEXER
 *
 * This lexical layer is deliberately conservative:
 * - identifiers may contain ASCII and broad Unicode code points;
 * - numeric literals are lexically unbounded;
 * - semantic validation decides representability;
 * - no machine-size constants occur here.
 *
 * The Rust lexer must maintain lexical conformance with this vocabulary.
 * ========================================================================== */

DOC_COMMENT
    : '///' ~[\r\n]*
    | '/**' .*? '*/'
    ;

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;

QUANTUM_LITERAL
    : '|' [01+\-] '⟩'
    | '|' ~[|⟩\r\n]+ '⟩'
    ;

STRING_LITERAL
    : '"' (ESCAPE_SEQUENCE | ~["\\\r\n])* '"'
    | '\'' (ESCAPE_SEQUENCE | ~['\\\r\n])* '\''
    ;

CHAR_LITERAL
    : '\'' (ESCAPE_SEQUENCE | ~['\\\r\n]) '\''
    ;

fragment ESCAPE_SEQUENCE
    : '\\' [btnfr"\\'/]
    | '\\' 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
    | '\\' 'x' HEX_DIGIT HEX_DIGIT
    ;

COMPLEX_LITERAL
    : DECIMAL_LITERAL [+-] DECIMAL_LITERAL 'i'
    ;

FLOAT_LITERAL
    : DIGIT_SEQUENCE '.' DIGIT_SEQUENCE EXPONENT?
    | DIGIT_SEQUENCE EXPONENT
    ;

DECIMAL_LITERAL
    : DIGIT_SEQUENCE
    ;

INTEGER_LITERAL
    : '0'
    | [1-9] DIGIT*
    | '0' [xX] HEX_DIGIT+
    | '0' [bB] [01]+
    | '0' [oO] [0-7]+
    ;

fragment DIGIT_SEQUENCE
    : DIGIT+
    ;

fragment DIGIT
    : [0-9]
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment EXPONENT
    : [eE] [+-]? DIGIT+
    ;

DURATION_LITERAL
    : DIGIT_SEQUENCE [a-zA-Z]+
    ;

IDENTIFIER
    : ID_START ID_CONTINUE*
    ;

fragment ID_START
    : [a-zA-Z_]
    | [\u0080-\uFFFF]
    ;

fragment ID_CONTINUE
    : [a-zA-Z0-9_]
    | [\u0080-\uFFFF]
    ;

WS
    : [ \t\r\n\u000B\u000C]+ -> channel(HIDDEN)
    ;


/* ============================================================================
 * 37. RESERVED / KEYWORD VOCABULARY
 *
 * ANTLR implicit literal tokens are used by parser rules above.
 * Keeping the semantic vocabulary in parser rules prevents a second keyword
 * registry from silently diverging from the language grammar.
 *
 * The Rust lexer must map the corresponding spellings to its TokenType values.
 * Unknown/future domain names remain identifiers where semantics permit them.
 * ========================================================================== */
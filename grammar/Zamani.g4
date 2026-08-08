/*
 * Zamani.g4 — ANTLR4 Universal Trinity Edition Grammar
 * 
 * This is the comprehensive, unified grammar for the Zamani language,
 * incorporating all features from the core language, NIMBUS legacy rules,
 * Quantum computing, Nano-agent swarms, Sankofa memory, Algebraic effects,
 * Advanced OOP, Meta-programming, HDL, Distributed systems, Globalization,
 * and all common/advanced modern programming language features (Null-safety,
 * FFI, Extension blocks, Complex types, and more).
 */

grammar Zamani;

// ===========================================================================
// Parser Rules
// ===========================================================================

program
    : (pragma | declaration)* EOF
    ;

pragma
    : '#' '!' '[' (identLike | STRING | INTEGER | '=' | ',' | '(' | ')')* ']'
    ;

declaration
    : docComment? attribute? declarationInner
    ;

docComment
    : DOC_COMMENT+
    ;

attribute
    : '#' '[' (identLike | STRING | INTEGER | '=' | ',' | '(' | ')')* ']'
    ;

declarationInner
    : moduleDecl
    | importStmt
    | exportDecl
    | functionDecl
    | structDecl
    | enumDecl
    | traitDecl
    | implBlock
    | classDecl
    | interfaceDecl
    | recordDecl
    | typeAliasDecl
    | constStmt
    | quantumCircuitDecl
    | nanoAgentDecl
    | languageDecl
    | effectDecl
    | hdlModuleDecl
    | cloudDecl
    | agentDecl
    | cognitiveBlock
    | metaBlock
    | sankofaDecl
    | actorDefinition
    | typeClassDefinition
    | domainSpecificLanguageDecl
    | aspectDecl
    | typeProviderDecl
    | extensionDecl
    | externBlock
    | testDecl
    | benchDecl
    | statement
    ;

// --- Module System ---
moduleDecl
    : 'module' identLike (('::' | '.') identLike)* (blockExpr | ';')
    ;

importStmt
    : 'import' modulePath ('as' identLike)? ';'?
    ;

exportDecl
    : 'export' identLike ('to' identLike)? ';'?
    ;

modulePath
    : identLike (('::' | '.') identLike)*
    ;

// --- Functions ---
functionDecl
    : modifiers? 'fn' identLike typeParams? '(' params? ')' returnType? ('with' effectList)? blockExpr
    ;

returnType
    : '->' typeExpr
    ;

params
    : param (',' param)*
    ;

param
    : 'mut'? '...'? typeExpr? identLike ('=' expression)?
    ;

modifiers
    : ( 'public' | 'private' | 'protected' | 'static' | 'const' | 'async' 
      | 'unsafe' | 'inline' | 'override' | 'final' | 'abstract' | 'virtual'
      | 'sealed' | 'partial' | 'file' | 'required' | 'init' | 'mut' | 'extern'
      )*
    ;

effectList
    : '[' identLike (',' identLike)* ']'
    ;

// --- Statements ---
statement
    : letStmt
    | constStmt
    | returnStmt
    | ifStmt
    | whileStmt
    | forStmt
    | loopStmt
    | matchStmt
    | breakStmt
    | continueStmt
    | deferStmt
    | yieldStmt
    | selectStmt
    | expressionStmt
    | blockExpr
    | unsafeBlock
    | throwStmt
    | tryCatchStmt
    | quantumStmt
    | nanoStmt
    | mtsStmt
    | sankofaStmt
    | cognitiveStmt
    | effectHandleStmt
    | langStmt
    | transitionStmt
    | hdlAssignment
    | invokeStmt
    | transcodeStmt
    | pluginStmt
    | dataStmt
    | databaseOp
    | webService
    | assertStmt
    | retractStmt
    | adaptStmt
    | inferStmt
    | deduceStmt
    | selfAdjustStmt
    ;

letStmt
    : ('let' | 'var') 'mut'? pattern (':' typeExpr)? '=' expression ';'?
    ;

constStmt
    : 'const' pattern (':' typeExpr)? '=' expression ';'?
    ;

returnStmt
    : 'return' expression? ';'?
    ;

ifStmt
    : 'if' ('let' pattern '=')? expression blockExpr ('else' (ifStmt | blockExpr))?
    ;

whileStmt
    : label? 'while' ('let' pattern '=')? expression blockExpr
    ;

forStmt
    : label? 'for' pattern 'in' expression blockExpr
    ;

loopStmt
    : label? 'loop' blockExpr
    ;

matchStmt
    : 'match' expression '{' matchCase* '}'
    ;

matchCase
    : ('case' | pattern) ('when' expression)? ('=>' | '->') (blockExpr | expression) ','?
    | 'default' ('=>' | '->') (blockExpr | expression) ','?
    ;

breakStmt
    : 'break' identLike? ';'?
    ;

continueStmt
    : 'continue' identLike? ';'?
    ;

deferStmt
    : 'defer' (blockExpr | statement)
    ;

yieldStmt
    : 'yield' expression? ';'?
    ;

selectStmt
    : 'select' '{' selectCase* '}'
    ;

selectCase
    : ('case' | pattern) ('=>' | '->') statement
    | 'default' ('=>' | '->') statement
    ;

expressionStmt
    : expression ';'?
    ;

unsafeBlock
    : 'unsafe' ('!' '(' 'evas' ':' expression ')')? blockExpr
    ;

throwStmt
    : 'throw' expression ';'?
    ;

tryCatchStmt
    : 'try' blockExpr catchClause* finallyClause?
    ;

catchClause
    : 'catch' ('(' param ')')? blockExpr
    ;

finallyClause
    : 'finally' blockExpr
    ;

label
    : IDENT ':'
    ;

// --- Expressions ---
expression
    : assignmentExpr
    ;

assignmentExpr
    : nullCoalescingExpr (assignOp assignmentExpr)?
    ;

assignOp
    : '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>='
    | '||=' | '&&=' | '??='
    ;

nullCoalescingExpr
    : logicalOrExpr ('??' logicalOrExpr)*
    ;

logicalOrExpr
    : logicalAndExpr (('||' | 'or') logicalAndExpr)*
    ;

logicalAndExpr
    : equalityExpr (('&&' | 'and') equalityExpr)*
    ;

equalityExpr
    : relationalExpr (('==' | '!=' | '===' | '!==') relationalExpr)*
    ;

relationalExpr
    : bitwiseOrExpr (('<' | '>' | '<=' | '>=' | 'instanceof' | 'is' | 'has') bitwiseOrExpr)*
    ;

bitwiseOrExpr
    : bitwiseXorExpr ('|' bitwiseXorExpr)*
    ;

bitwiseXorExpr
    : bitwiseAndExpr ('^' bitwiseAndExpr)*
    ;

bitwiseAndExpr
    : shiftExpr ('&' shiftExpr)*
    ;

shiftExpr
    : additiveExpr (('<<' | '>>' | '>>>') additiveExpr)*
    ;

additiveExpr
    : multiplicativeExpr (('+' | '-') multiplicativeExpr)*
    ;

multiplicativeExpr
    : castExpr (('*' | '/' | '%' | '**') castExpr)*
    ;

castExpr
    : unaryExpr (('as' | ':') typeExpr)*
    ;

unaryExpr
    : ('!' | '-' | '+' | '~' | '&' 'mut'? | '*' | '++' | '--') unaryExpr
    | postfixExpr
    ;

postfixExpr
    : primaryExpr (postfixOp)*
    ;

postfixOp
    : '.' identLike ('(' args? ')')?
    | '?.' identLike ('(' args? ')')?
    | '[' expression ']'
    | '?[' expression ']'
    | '(' args? ')'
    | '?'
    | '!'
    ;

primaryExpr
    : literal
    | identLike structLiteralTail?
    | '(' expression ')'
    | arrayLit
    | tupleLit
    | structLit
    | mapLit
    | lambdaExpr
    | quantumExpr
    | nanoExpr
    | mtsExpr
    | recallExpr
    | consensusExpr
    | invokeExpr
    | newExpr
    | thisExpr
    | superExpr
    | interpolatedString
    | rawStringLit
    | blockExpr
    | ifStmt
    | matchStmt
    | yieldExpr
    | ioExpr
    | typeofExpr
    | 'async' expression
    | 'await' expression
    | 'spawn' expression
    ;

typeofExpr
    : 'typeof' expression
    ;

args
    : expression (',' expression)*
    ;

arrayLit
    : '[' (arrayElement (',' arrayElement)*)? ']'
    ;

arrayElement
    : '...'? expression
    ;

tupleLit
    : '(' expression (',' expression)+ ')'
    ;

structLit
    : identLike '{' (structFieldLit (',' structFieldLit)*)? '}'
    ;

structFieldLit
    : identLike (':' expression)?
    | '...' expression
    ;

structLiteralTail
    : '{' (structFieldLit (',' structFieldLit)*)? '}'
    ;

mapLit
    : 'map'? '{' (expression ('=>' | ':') expression (',' expression ('=>' | ':') expression)*)? '}'
    ;

lambdaExpr
    : '|' params? '|' (typeExpr)? (blockExpr | ('->' expression))
    ;

yieldExpr
    : 'yield' expression?
    ;

ioExpr
    : 'io' '(' expression ')'
    ;

// --- Patterns ---
pattern
    : identPattern
    | tuplePattern
    | structPattern
    | enumPattern
    | wildcardPattern
    | literalPattern
    | unionPattern
    | rangePattern
    ;

identPattern : 'mut'? identLike ;
tuplePattern : '(' (pattern (',' pattern)*)? ')' ;
structPattern : identLike '{' (fieldPattern (',' fieldPattern)*)? (',' '..')? '}' ;
fieldPattern : identLike (':' pattern)? ;
enumPattern : identLike ('(' (pattern (',' pattern)*)? ')' | '{' (fieldPattern (',' fieldPattern)*)? '}')? ;
wildcardPattern : '_' ;
literalPattern : literal ;
unionPattern : pattern '|' pattern ;
rangePattern : literal ('..' | '..=') literal ;

// --- Literals ---
literal
    : INTEGER
    | FLOAT
    | STRING
    | CHAR
    | BOOLEAN
    | 'null'
    | 'nil'
    | quantumLit
    | nanoLit
    | mtsLit
    | QUANTUM_LITERAL
    | NANO_ANNOTATION
    | MTS_LITERAL
    ;

interpolatedString
    : '$' '"' (interpolationContent | interpolationPart)* '"'
    ;

interpolationContent
    : ~[\\"$]
    ;

interpolationPart
    : '${' expression '}'
    ;

rawStringLit
    : 'r' '#'* STRING
    ;

// --- Types ---
typeExpr
    : baseType
    | genericType
    | arrayType
    | tupleType
    | functionType
    | dependentType
    | linearType
    | affineType
    | effectfulType
    | universeType
    | quantumType
    | nanoType
    | mtsType
    | sankofaType
    | cognitiveType
    | nullableType
    | refType
    | boxedType
    | unionType
    | intersectionType
    | typeofType
    | conditionalType
    | mappedType
    | '(' typeExpr ')'
    ;

baseType
    : 'void' | 'int' | 'float' | 'bool' | 'string' | 'String' | 'str' | 'char' | 'bytes' 
    | 'i8' | 'i16' | 'i32' | 'i64' | 'i128' | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
    | 'f32' | 'f64' | 'usize' | 'isize' | 'Self' | 'self' | 'Never' | identLike
    ;

genericType
    : baseType '<' typeExpr (',' typeExpr)* '>'
    ;

arrayType
    : typeExpr '[' expression? ']'
    | '[' typeExpr (';' expression)? ']'
    ;

tupleType
    : '(' typeExpr (',' typeExpr)+ ')'
    ;

functionType
    : '(' (typeExpr (',' typeExpr)*)? ')' '->' typeExpr
    | 'fn' '(' (typeExpr (',' typeExpr)*)? ')' ('->' typeExpr)?
    ;

nullableType
    : typeExpr '?'
    ;

refType
    : 'ref' typeExpr | '&' 'mut'? typeExpr
    ;

boxedType
    : 'Box' '<' typeExpr '>'
    ;

unionType
    : typeExpr '|' typeExpr
    ;

intersectionType
    : typeExpr '&' typeExpr
    ;

typeofType
    : 'typeof' expression
    ;

conditionalType
    : typeExpr 'extends' typeExpr '?' typeExpr ':' typeExpr
    ;

mappedType
    : '{' '[' identLike 'in' typeExpr ']' ':' typeExpr '}'
    ;

typeParams
    : '<' typeParam (',' typeParam)* '>'
    ;

typeParam
    : identLike (':' typeConstraint)?
    ;

typeConstraint
    : typeExpr ('+' typeExpr)*
    ;

// --- Quantum Computing ---
quantumCircuitDecl
    : 'quantum' 'circuit' identLike '(' params? ')' blockExpr
    | ('circuit' | 'quantum') identLike (blockExpr | expression)
    ;

quantumStmt
    : quantumGate
    | quantumMeasure
    | quantumReset
    | quantumBarrier
    ;

quantumGate
    : ('Hadamard' | 'CNOT' | 'PauliX' | 'PauliY' | 'PauliZ' | 'T' | 'S' | 'Swap' | 'Toffoli' | 'Fredkin') '(' identLike (',' identLike)* ')'
    ;

quantumMeasure
    : 'measure' identLike ('->' identLike)?
    ;

quantumReset
    : 'reset' identLike
    ;

quantumBarrier
    : 'barrier' (identLike (',' identLike)*)?
    ;

quantumLit
    : '|' ( '0' | '1' | '+' | '-' | identLike ) '⟩'
    ;

quantumExpr
    : quantumLit
    | 'superpose' '(' expression (',' expression)* ')' 
    | 'entangle' '(' identLike ',' identLike ')'
    | 'quantum' identLike ('(' args? ')')?
    ;

quantumType
    : 'Qubit' | 'QReg' '[' expression ']' | 'Superposition' '<' typeExpr '>' 
    | 'Entangled' '<' typeExpr ',' typeExpr '>' | 'QMeasured' '<' typeExpr '>'
    | 'QArray' '<' typeExpr ',' expression '>'
    ;

// --- Nano-Agent Computing ---
nanoAgentDecl
    : 'nano' 'agent' identLike '{' nanoAgentBody '}'
    | ('agent' | 'nano') identLike (blockExpr | expression)
    ;

nanoAgentBody
    : (nanoCapability | nanoBehavior | nanoProtocol | agentGoal | agentConstraint)*
    ;

nanoCapability
    : 'capability' identLike '(' params? ')' blockExpr
    ;

nanoBehavior
    : 'behavior' identLike '(' params? ')' blockExpr
    ;

nanoProtocol
    : 'protocol' identLike '{' protocolRule* '}'
    ;

protocolRule
    : 'on' identLike '->' blockExpr
    ;

nanoLit
    : '@atom' '(' identLike ':' identLike ')' 
    | '@molecule' '(' identLike (DIGIT* identLike)* ')'
    ;

nanoExpr
    : nanoLit 
    | 'assemble' '(' expression ')' 
    | 'deploy' '(' expression ')'
    ;

nanoType
    : 'Atom' '<' typeExpr '>' | 'Molecule' '<' typeExpr '>' 
    | 'NanoAgent' '<' typeExpr '>' | 'Archaeve' '<' typeExpr '>'
    ;

// --- Multi-Timeline System (MTS) ---
mtsStmt
    : 'mts' 'timeline' identLike '{' mtsBody '}'
    ;

mtsBody
    : (mtsSlice | mtsOperation)*
    ;

mtsSlice
    : 'slice' identLike '[' INTEGER ']' blockExpr
    ;

mtsOperation
    : 'fork' '(' identLike ')' 
    | 'merge' '(' identLike ')' 
    | 'observe' '(' identLike ')' 
    | 'rewind' '(' INTEGER ')'
    ;

mtsLit
    : 'mts' '[' INTEGER ']'
    ;

mtsExpr
    : mtsLit 
    | 'parallel' '(' blockExpr ')' 
    | 'speculative' '(' blockExpr ')'
    | 'counterfactual' '(' expression ',' blockExpr ')'
    ;

mtsType
    : 'MtsSlice' '<' expression '>'
    ;

// --- Sankofa Memory System ---
sankofaDecl
    : memoryDecl | wisdomDecl | historyDecl | consensusDecl | interMemoryDecl
    ;

memoryDecl
    : 'remember' identLike (':' typeExpr)? '=' expression ';'?
    ;

wisdomDecl
    : 'wisdom' identLike ('=' expression)? ';'?
    | 'wisdom' identLike '{' wisdomBody '}'
    ;

wisdomBody
    : (premiseDecl | inferenceRule | wisdomStmtInner)*
    ;

premiseDecl
    : 'premise' identLike ':' typeExpr '=' expression ';'?
    ;

inferenceRule
    : 'rule' identLike '(' params? ')' '->' blockExpr
    ;

wisdomStmtInner
    : 'conclude' expression ';'?
    ;

sankofaStmt
    : learnStmt | rememberStmt | recallStmt | wisdomStmt | ancestorCall | consensusStmt | historyDecl | consensusDecl | interMemoryDecl
    ;

learnStmt
    : 'learn' ('from')? expression ('with' 'weight' expression)? ';'?
    | 'infer' expression 'from' expression ';'?
    ;

rememberStmt
    : 'remember' identLike (':' typeExpr)? '=' expression ';'?
    ;

recallStmt
    : 'recall' ('(' expression ')' | expression)
    | 'recall' '(' expression ',' expression ')' ';'?
    ;

ancestorCall
    : 'ancestral' identLike '(' args? ')' ';'?
    ;

consensusStmt
    : consensusExpr ';'?
    ;

consensusExpr
    : 'consensus' '[' args ']' 'vote' expression
    ;

zamaniBlock
    : 'zamani' (blockExpr | expression)
    ;

sasaBlock
    : 'sasa' (blockExpr | expression)
    ;

sankofaType
    : 'History' '<' typeExpr ',' expression '>'
    | 'ConsensusTrue' '<' typeExpr '>'
    | 'InterMemory' '<' STRING ',' typeExpr '>'
    ;

historyDecl
    : 'history' identLike '{' statement* '}'
    ;

consensusDecl
    : 'consensus' identLike '{' statement* '}'
    ;

interMemoryDecl
    : 'inter_memory' identLike '{' statement* '}'
    ;

// --- Advanced Type System ---
dependentType
    : ('Pi' | 'Π') '(' identLike ':' typeExpr ')' typeExpr
    | ('Sigma' | 'Σ') '(' identLike ':' typeExpr ')' typeExpr
    | 'Id' '(' typeExpr ',' expression ',' expression ')'
    ;

universeType
    : 'Type_0' | 'Type_1' | 'Type_2' | 'Type_N' 
    | 'Kind' | 'Sort' | 'Prop'
    ;

linearType
    : 'linear' typeExpr
    ;

affineType
    : 'affine' typeExpr
    ;

effectfulType
    : typeExpr 'with' 'effects'? '{' identLike (',' identLike)* '}'
    ;

sessionType
    : 'session' '{' sessionOp* '}'
    ;

sessionOp
    : 'send' typeExpr 
    | 'recv' typeExpr 
    | 'offer' '{' sessionBranch* '}' 
    | 'choice' '{' sessionBranch* '}' 
    | 'close'
    ;

sessionBranch
    : identLike '->' sessionType
    ;

cognitiveType
    : 'CognitiveState' '<' typeExpr '>' 
    | 'Consciousness' '<' typeExpr '>'
    | 'Biological' '<' typeExpr '>' 
    | 'Neural' '<' typeExpr '>'
    | 'MemoryBank' '<' typeExpr '>' 
    | 'AgentType'
    ;

agentType
    : 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI'
    ;

// --- Algebraic Effects ---
effectDecl
    : 'effect' identLike typeParams? ('(' params? ')')? returnType? (blockExpr | ';')
    ;

effectHandleStmt
    : 'handle' expression (blockExpr | ('{' effectHandler* '}')) ('with' blockExpr)?
    ;

effectHandler
    : 'case' identLike ('(' params? ')')? '->' blockExpr
    ;

// --- Meta-Programming ---
metaBlock
    : 'meta' '{' statement* '}'
    ;

mopExpr
    : 'reflect' '(' expression ')' 
    | 'introspect' '(' identLike ')'
    | 'meta_eval' '(' expression ')' 
    | 'quote' '{' statement* '}'
    | 'unquote' '(' expression ')' 
    | 'splice' '(' expression ')'
    ;

macroDecl
    : 'macro' identLike '(' params? ')' blockExpr
    ;

macroCall
    : identLike '!' '(' args? ')'
    ;

domainSpecificLanguageDecl
    : 'domain' identLike (':' typeExpr)? ('module' identLike)? ('type' typeExpr)? ('plugin' identLike)? ('provider' identLike)? '{' statement* '}'
    ;

aspectDecl
    : 'aspect' identLike ('extends' identLike)? ('depends' identLike)? ('compose' identLike)? ('weave' identLike)? ('domain' identLike)? '{' statement* '}'
    ;

typeProviderDecl
    : 'provider' identLike '(' typeParams? ')' '{' statement* '}' ('cache' identLike)? ('inject' identLike)? ('metadata' typeExpr)? ('aspect' identLike)?
    ;

// --- OOP Features ---
extensionDecl
    : 'extension' typeParams? typeExpr ('for' typeExpr)? '{' classMember* '}'
    ;

classDecl
    : modifiers? 'class' identLike typeParams? extendsClause? implementsClause? permitsClause? '{' classBody '}'
    ;

extendsClause
    : 'extends' identLike (',' identLike)*
    ;

implementsClause
    : 'implements' identLike (',' identLike)*
    ;

permitsClause
    : 'permits' identLike (',' identLike)*
    ;

classBody
    : classMember*
    ;

classMember
    : modifiers? (functionDecl | constructorDecl | fieldDecl | propertyDef | indexerDef | operatorOverload | delegateDef | eventDef)
    ;

constructorDecl
    : 'new' '(' params? ')' blockExpr
    | 'fn' identLike '(' params? ')' blockExpr
    ;

fieldDecl
    : identLike (':' typeExpr)? ('=' expression)? (',' | ';')
    ;

propertyDef
    : typeExpr identLike ('=' expression)? ';'
    | 'property' typeExpr identLike '{' propertyBody '}'
    ;

propertyBody
    : 'get' blockExpr 'set' blockExpr
    | 'get' blockExpr
    | 'set' blockExpr
    ;

indexerDef
    : 'this' '[' params ']' '{' 'get' blockExpr ('set' blockExpr)? '}'
    ;

operatorOverload
    : ('explicit' | 'implicit')? 'operator' (assignOp | additiveOp | multiplicativeOp | typeExpr) '(' params ')' blockExpr
    ;

delegateDef
    : 'delegate' returnType? identLike '(' params? ')' ';'
    ;

eventDef
    : 'event' typeExpr identLike ';'
    ;

interfaceDecl
    : modifiers? 'interface' identLike typeParams? extendsClause? '{' interfaceMember* '}'
    ;

interfaceMember
    : modifiers? (functionDecl | 'default' functionDecl)
    ;

recordDecl
    : 'record' ('struct')? identLike typeParams? '(' params? ')' extendsClause? implementsClause? ('{' classBody '}')?
    ;

structDecl
    : 'struct' identLike typeParams? '{' structField* '}'
    ;

structField
    : modifiers? identLike (':' typeExpr)? (',' | ';')?
    ;

enumDecl
    : 'enum' identLike typeParams? (':' typeExpr)? '{' enumVariant* '}'
    ;

enumVariant
    : identLike enumVariantKind? ','?
    ;

enumVariantKind
    : '(' typeExpr (',' typeExpr)* )
    | '{' structField* '}'
    ;

traitDecl
    : 'trait' identLike typeParams? (':' typeConstraint)? '{' traitItem* '}'
    ;

traitItem
    : functionDecl
    | 'type' identLike (':' typeConstraint)? ';'
    | 'const' identLike ':' typeExpr '=' expression ';'
    ;

implBlock
    : 'impl' typeParams? typeExpr ('for' typeExpr)? '{' implItem* '}'
    ;

implItem
    : functionDecl
    | 'type' identLike (':' typeConstraint)? ';'
    | 'const' identLike ':' typeExpr '=' expression ';'
    ;

typeAliasDecl
    : 'type' identLike typeParams? '=' typeExpr ';'?
    ;

// --- AI / Cognitive ---
cognitiveBlock
    : 'cognitive_architecture' identLike '{' statement* '}'
    ;

cognitiveStmt
    : adaptStmt | selfAdjustStmt | deduceStmt | retractStmt | assertStmt
    ;

adaptStmt
    : 'adapt' expression ('to' expression)? ';'
    ;

selfAdjustStmt
    : 'self_adjust' '(' expression ')' blockExpr
    ;

deduceStmt
    : 'deduce' expression ('via' expression)? ';'
    ;

retractStmt
    : 'retract' expression ';'
    ;

assertStmt
    : 'assert' expression ';'
    ;

agentDecl
    : 'agent' identLike (':' agentType)? '{' agentBody '}'
    ;

agentGoal
    : 'goal' expression ';'
    ;

agentConstraint
    : 'constraint' expression ';'
    ;

// --- Actor ---
actorDefinition
    : 'actor' identLike '{' actorBody '}'
    ;

actorBody
    : actorMember*
    ;

actorMember
    : messageHandlerDefinition | propertyDefinition | methodDefinition
    ;

messageHandlerDefinition
    : 'handler' identLike '(' params? ')' blockExpr
    ;

propertyDefinition
    : modifiers? typeExpr identLike ('=' expression)? ';'
    ;

methodDefinition
    : modifiers? 'fn' identLike typeParams? '(' params? ')' returnType? blockExpr
    ;

// --- HDL (Hardware Description Language) ---
hdlModuleDecl
    : 'hdl' 'module' identLike '{' hdlBody '}'
    ;

hdlBody
    : (hdlPort | hdlComponent | hdlLogicGate | hdlSignal | hdlAssignment)*
    ;

hdlPort
    : 'port' hdlPortDirection identLike ':' hdlPortType
    ;

hdlPortDirection
    : 'input' | 'output' | 'inout'
    ;

hdlPortType
    : 'wire' | 'reg' | 'logic' | 'quantum' | 'nano' | typeExpr
    ;

hdlComponent
    : 'component' identLike '{' hdlPort* '}'
    ;

hdlLogicGate
    : 'gate' hdlLogicGateType '(' identLike (',' identLike)* ')'
    ;

hdlLogicGateType
    : 'AND' | 'OR' | 'NOT' | 'XOR' | 'NAND' | 'NOR' | 'MUX' | 'DEMUX'
    | 'Hadamard' | 'CNOT' | 'Toffoli' | 'Fredkin' | 'PauliX' | 'PauliY' | 'PauliZ'
    ;

hdlSignal
    : 'signal' identLike ':' hdlPortType
    ;

hdlAssignment
    : 'assign' identLike '=' hdlExpression ';'
    ;

hdlExpression
    : identLike 
    | hdlLogicGate 
    | '(' hdlExpression ')' 
    | hdlExpression ('&' | '|' | '^' | '~') hdlExpression
    ;

// --- Distributed / Cloud ---
cloudDecl
    : 'cloud' identLike '{' cloudBody '}'
    ;

cloudBody
    : (cloudPlatform | container | devOpsTool)*
    ;

cloudPlatform
    : ('AWS' | 'Azure' | 'GCP') identLike '{' statement* '}'
    ;

container
    : 'Docker' identLike '{' statement* '}'
    ;

devOpsTool
    : ('Jenkins' | 'Git' | 'Ansible') identLike '{' statement* '}'
    ;

invokeStmt
    : 'invoke' modulePath '(' args? ')' ';'
    ;

transcodeStmt
    : 'transcode' identLike '::' STRING 'to' identLike ';'
    ;

pluginStmt
    : 'plugin' identLike '{' (('language' | 'transpiler') identLike ';')* '}'
    ;

dataStmt
    : 'data' identLike '{' statement* '}'
    ;

databaseOp
    : 'Database' identLike '.' identLike '(' args? ')' ';'
    ;

webService
    : 'HTTP' identLike '.' identLike '(' args? ')' ';'
    ;

transitionStmt
    : 'transition' 'from' identLike 'to' identLike 'on' expression ';'
    ;

// --- Extern / FFI ---
externBlock
    : 'extern' STRING? '{' declaration* '}'
    | 'extern' functionDecl
    ;

// --- Testing ---
testDecl
    : 'test' STRING? blockExpr
    ;

benchDecl
    : 'bench' STRING? blockExpr
    ;

// --- Helper Rules ---
identLike
    : IDENT
    | THIS | SELF_LOWER | SELF_UPPER | SUPER
    | INT_KW | FLOAT_KW | BOOL_KW | STR_KW | STRING_KW | CHAR_KW | VOID
    | QUANTUM | NANO | AGENT | CIRCUIT | EFFECT | HANDLE | REMEMBER | RECALL
    | LEARN | INFER | WISDOM | ZAMANI | SASA | ANCESTOR | LINEAR | AFFINE
    | LANGUAGE | MTS_KW | LEN | PRINT | PRINTLN | ASSERT | PANIC
    | PI_KW | SIGMA_KW | AS | IS | AND_KW | OR_KW | NOT_KW
    | LOOP | DEFER | YIELD | TEST | BENCH | EXTERN
    ;

additiveOp
    : '+' | '-'
    ;

multiplicativeOp
    : '*' | '/' | '%' | '**'
    ;

blockExpr
    : '{' statement* '}'
    ;

// ===========================================================================
// Lexer Rules
// ===========================================================================

// --- Keywords ---
LET: 'let'; VAR: 'var'; CONST: 'const'; MUT: 'mut'; FN: 'fn'; RETURN: 'return';
IF: 'if'; ELSE: 'else'; WHILE: 'while'; FOR: 'for'; IN: 'in'; LOOP: 'loop';
BREAK: 'break'; CONTINUE: 'continue'; MATCH: 'match'; WITH: 'with';
TRUE: 'true'; FALSE: 'false'; NIL_KW: 'nil'; NULL_KW: 'null';
DEFER: 'defer'; YIELD: 'yield'; TEST: 'test'; BENCH: 'bench'; EXTERN: 'extern';

STRUCT: 'struct'; ENUM: 'enum'; TRAIT: 'trait'; IMPL: 'impl'; CLASS: 'class';
INTERFACE: 'interface'; EXTENDS: 'extends'; IMPLEMENTS: 'implements'; PERMITS: 'permits';
PUBLIC: 'public'; PRIVATE: 'private'; PROTECTED: 'protected'; STATIC: 'static';
VIRTUAL: 'virtual'; OVERRIDE: 'override'; ABSTRACT: 'abstract'; FINAL: 'final';
SEALED: 'sealed'; PARTIAL: 'partial'; FILE_SCOPED: 'file'; REQUIRED: 'required';
INIT: 'init'; NEW: 'new'; THIS: 'this'; SUPER: 'super'; TYPE: 'type';
SELF_LOWER: 'self'; SELF_UPPER: 'Self'; VOID: 'void'; INT_KW: 'int';
FLOAT_KW: 'float'; BOOL_KW: 'bool'; STR_KW: 'str'; STRING_KW: 'String'; CHAR_KW: 'char';

MODULE: 'module'; IMPORT: 'import'; EXPORT: 'export'; USE: 'use'; FROM: 'from';
AS: 'as'; IS: 'is'; WHERE: 'where'; GLOBAL: 'global';

EFFECT: 'effect'; PERFORM: 'perform'; HANDLE: 'handle'; UNSAFE: 'unsafe';
TRY: 'try'; CATCH: 'catch'; THROW: 'throw'; FINALLY: 'finally';

ASYNC: 'async'; AWAIT: 'await'; SPAWN: 'spawn';

QUANTUM: 'quantum'; CIRCUIT: 'circuit'; NANO: 'nano'; AGENT: 'agent';
REMEMBER: 'remember'; RECALL: 'recall'; LEARN: 'learn'; INFER: 'infer';
WISDOM: 'wisdom'; ZAMANI: 'zamani'; SASA: 'sasa'; ANCESTOR: 'ancestor';
LINEAR: 'linear'; AFFINE: 'affine'; LANGUAGE: 'language';
AND_KW: 'and'; OR_KW: 'or'; NOT_KW: 'not'; SIZEOF: 'sizeof'; LEN: 'len';
PRINT: 'print'; PRINTLN: 'println'; ASSERT: 'assert'; PANIC: 'panic';
SWITCH: 'switch'; CASE: 'case'; THEN: 'then'; MTS_KW: 'mts';

PI_KW: 'Pi' | '\u03A0';
SIGMA_KW: 'Sigma' | '\u03A3';

// --- Domain Keywords ---
HADAMARD: 'Hadamard'; CNOT: 'CNOT'; PAULIX: 'PauliX'; PAULIY: 'PauliY'; PAULIZ: 'PauliZ';
TOFFOLI: 'Toffoli'; FREDKIN: 'Fredkin';
ATOM: 'atom'; MOLECULE: 'molecule'; ASSEMBLE: 'assemble'; DEPLOY: 'deploy';
SUPERPOSE: 'superpose'; ENTANGLE: 'entangle'; MEASURE: 'measure'; RESET: 'reset';
BARRIER: 'barrier'; FORK: 'fork'; MERGE: 'merge'; OBSERVE: 'observe'; REWIND: 'rewind';
PARALLEL: 'parallel'; SPECULATIVE: 'speculative'; COUNTERFACTUAL: 'counterfactual';
CONSENSUS: 'consensus'; VOTE: 'vote'; ANCESTRAL: 'ancestral';

// --- AI / Cognitive Keywords ---
COGNITIVESTATE: 'CognitiveState'; CONSCIOUSNESS: 'Consciousness'; BIOLOGICAL: 'Biological';
NEURAL: 'Neural'; MEMORYBANK: 'MemoryBank'; AGENTTYPE: 'AgentType';
NARROWAI: 'NarrowAI'; AGI_KW: 'AGI'; ASI_KW: 'ASI'; AESI_KW: 'AESI'; ASESI_KW: 'ASESI';
META: 'meta'; INVOKE: 'invoke'; TRANSCODE: 'transcode'; PLUGIN: 'plugin'; TRANSPILER: 'transpiler';
MACRO: 'macro'; REFLECT: 'reflect'; INTROSPECT: 'introspect'; META_EVAL: 'meta_eval';
QUOTE: 'quote'; UNQUOTE: 'unquote'; SPLICE: 'splice'; DOMAIN: 'domain'; ASPECT: 'aspect';
PROVIDER: 'provider'; CACHE: 'cache'; INJECT: 'inject'; METADATA: 'metadata';
COMPOSE: 'compose'; WEAVE: 'weave'; DEPENDS: 'depends';

// --- HDL / Cloud / Distributed ---
HDL: 'hdl'; PORT: 'port'; INPUT: 'input'; OUTPUT: 'output'; INOUT: 'inout';
WIRE: 'wire'; REG: 'reg'; LOGIC: 'logic'; GATE: 'gate'; AND: 'AND'; OR: 'OR'; NOT: 'NOT';
XOR: 'XOR'; NAND: 'NAND'; NOR: 'NOR'; MUX: 'MUX'; DEMUX: 'DEMUX'; SIGNAL: 'signal'; ASSIGN: 'assign';
DISTRIBUTED: 'distributed'; NODE: 'node'; SERVICE: 'service'; AT: 'at'; REMOTE: 'remote';
TELEPORT: 'teleport'; MIGRATE: 'migrate'; DSM: 'dsm'; DMTS: 'dmts';
AWS: 'AWS'; AZURE: 'Azure'; GCP: 'GCP'; DOCKER: 'Docker'; JENKINS: 'Jenkins'; GIT: 'Git'; ANSIBLE: 'Ansible';
DATABASE: 'Database'; HTTP: 'HTTP'; SERIALIZE: 'serialize'; DESERIALIZE: 'deserialize';
JSON: 'json'; XML: 'xml'; MESSAGEPACK: 'messagepack'; PROTOBUF: 'protobuf'; CBOR: 'cbor';
STREAM: 'stream'; PIPE: 'pipe'; FOREIGN: 'foreign';

// --- Crypto ---
ENCRYPT: 'encrypt'; DECRYPT: 'decrypt'; ENCRYPT_HOMOMORPHIC: 'encrypt_homomorphic';
DECRYPT_HOMOMORPHIC: 'decrypt_homomorphic'; ENCRYPT_LAYERED: 'encrypt_layered';
ENCRYPT_QUANTUM_SAFE: 'encrypt_quantum_safe'; AES256: 'AES256'; CHACHA20: 'ChaCha20';
RSA4096: 'RSA4096'; KYBER: 'Kyber'; DILITHIUM: 'Dilithium'; SPHINCS: 'SPHINCS+';
GENERATE_ZK_PROOF: 'generate_zk_proof'; VERIFY_ZK_PROOF: 'verify_zk_proof';
SECURE_MULTI_PARTY_COMPUTE: 'secure_multi_party_compute'; REQUEST_KEY: 'request_key';
ROTATE_KEY: 'rotate_key'; REVOKE_KEY: 'revoke_key'; GENERATE_KEY: 'generate_key';

// --- On-device / Edge ---
ON_DEVICE: 'on_device'; ON_DEVICE_AGENT: 'on_device_agent'; REQUIRES: 'requires';
MEMORY: 'memory'; POWER: 'power'; STORAGE: 'storage'; PROCESSOR: 'processor';
SELF_PRESERVE: 'self_preserve'; SELF_PRESERVATION: 'self_preservation'; OFFLINE: 'offline';
LOCAL_RESOURCES: 'local_resources'; LOCAL_MTS: 'local_mts'; LIFECYCLE: 'lifecycle';
MONITOR: 'monitor'; UPDATE: 'update'; MANAGE: 'manage';

// --- Self-evolution ---
SELF_EVOLVE: 'self_evolve'; OPTIMIZE: 'optimize'; PATCH: 'patch'; VERIFY: 'verify';
AUTONOMOUSLY_GENERATE: 'autonomously_generate'; AUTONOMOUSLY_OPTIMIZE: 'autonomously_optimize';
AUTONOMOUSLY_VERIFY: 'autonomously_verify';

// --- Optimization / Target ---
OPTIMIZATION: 'optimization'; PASS: 'pass'; TARGET: 'target'; STRATEGY: 'strategy';
CLASSICAL: 'classical'; NEUROMORPHIC: 'neuromorphic'; GPU: 'GPU'; FPGA_KW: 'FPGA';
SIMD: 'SIMD'; WASM: 'WASM'; USSD: 'USSD'; EDGE: 'edge'; STELLAR: 'stellar';
DCE: 'DCE'; CSE: 'CSE'; INLINING: 'inlining'; LOOP_UNROLL: 'loop_unroll';
CONSTANT_FOLD: 'constant_fold'; QUANTUM_GATE_OPT: 'quantum_gate_opt';
NANO_ATP_EFFICIENCY: 'nano_atp_efficiency'; AI_QUANTIZATION: 'ai_quantization';
THERMAL_THROTTLE_PREVENTION: 'thermal_throttle_prevention'; SIMD_VECTORIZATION: 'simd_vectorization';
GPU_OFFLOADING: 'gpu_offloading'; QUANTUM_ERROR_CORRECTION: 'quantum_error_correction';
ARM64: 'ARM64'; RISCV: 'RISC-V'; LLVM_IR: 'LLVM_IR'; BARE_METAL: 'bare_metal';
ANDROID: 'Android'; IOS: 'iOS'; CLOUD: 'cloud'; IOT: 'IoT';
TARIRO_RUNTIME: 'Tariro_Runtime'; Z_MMP: 'Z_MMP';

// --- Runtime ---
RUNTIME: 'runtime'; CONFIGURE: 'configure'; GC: 'gc'; ENABLED: 'enabled';
DISABLED: 'disabled'; HYBRID: 'hybrid'; SELF_HEAL: 'self_heal'; QUANTUM_SIM: 'quantum_sim';
NANO_ORCHESTRATION: 'nano_orchestration'; EFFECT_DISPATCH: 'effect_dispatch';
EAGER: 'eager'; LAZY: 'lazy'; BATCHED: 'batched'; SCHEDULER: 'scheduler';
PREEMPTIVE: 'preemptive'; COOPERATIVE: 'cooperative'; CHANNEL: 'channel'; SELECT: 'select';

// --- Actor ---
ACTOR: 'actor'; MESSAGE: 'message'; BEHAVIOR: 'behavior'; STATE_KW: 'state';
CONCURRENT: 'concurrent';

// --- AI / Cognitive Domains ---
COGNITIVE_ARCHITECTURE: 'cognitive_architecture'; MACHINE_LEARNING: 'machine_learning';
DEEP_LEARNING: 'deep_learning'; NEURAL_NETWORK: 'neural_network';
GRAPH_BASED_DEEP_LEARNING: 'graph_based_deep_learning'; CAUSAL_DISCOVERY: 'causal_discovery';
CAUSAL_REASONING: 'causal_reasoning'; PROBABILISTIC_MODELING: 'probabilistic_modeling';
ADVANCED_COMPUTER_VISION: 'advanced_computer_vision'; PREDICTIVE_ANALYTICS: 'predictive_analytics';
PRESCRIPTIVE_ANALYTICS: 'prescriptive_analytics'; DATA_VISUALIZATION: 'data_visualization';
HUMAN_AI_COLLABORATION: 'human_ai_collaboration'; ADVANCED_HUMAN_AI_COLLABORATION: 'advanced_human_ai_collaboration';
EXPLAINABILITY_AND_TRANSPARENCY: 'explainability_and_transparency'; SAFETY_AND_SECURITY: 'safety_and_security';
ETHICS_AND_GOVERNANCE: 'ethics_and_governance'; QUANTUM_MACHINE_LEARNING: 'quantum_machine_learning';
COGNITIVE_ARCHITECTURES: 'cognitive_architectures'; HUMAN_COMPUTER_INTERACTION: 'human_computer_interaction';
DATA_ANALYTICS: 'data_analytics'; TIME_SERIES_ML: 'time_series_ml';
TIME_SERIES_FORECASTING: 'time_series_forecasting'; TIME_SERIES_ANALYSIS: 'time_series_analysis';
TEXT_GENERATION: 'text_generation'; SENTIMENT_ANALYSIS: 'sentiment_analysis';
LANGUAGE_TRANSLATION: 'language_translation'; OBJECT_DETECTION: 'object_detection';
OBJECT_RECOGNITION: 'object_recognition'; IMAGE_CLASSIFICATION: 'image_classification';
AUTONOMOUS_NAVIGATION: 'autonomous_navigation'; AUTONOMOUS_ROBOTICS: 'autonomous_robotics';
AUTONOMOUS_SYSTEMS: 'autonomous_systems'; MODEL_EXPLAINABILITY: 'model_explainability';
AI_SAFETY_AND_SECURITY: 'ai_safety_and_security'; AI_ETHICS_AND_GOVERNANCE: 'ai_ethics_and_governance';
QUANTUM_OPTIMIZATION: 'quantum_optimization'; RECOMMENDATION_SYSTEMS: 'recommendation_systems';
QUANTUM_COMPUTING: 'quantum_computing'; BLOCKCHAIN_TECHNOLOGY: 'blockchain_technology';
IOT_KW: 'iot'; CYBERSECURITY: 'cybersecurity'; DATA_SCIENCE: 'data_science';
NATURAL_LANGUAGE_GENERATION: 'natural_language_generation'; NL_GENERATION: 'nl_generation';
ADVANCED_ROBOTICS: 'advanced_robotics'; TRANSFER_LEARNING: 'transfer_learning';
EXPLAINABLE_REINFORCEMENT_LEARNING: 'explainable_reinforcement_learning';
GRAPH_NEURAL_NETWORK: 'graph_neural_network';
ADVANCED_NATURAL_LANGUAGE_PROCESSING: 'advanced_natural_language_processing';
ADVANCED_NATURAL_LANGUAGE_GENERATION: 'advanced_natural_language_generation';
ADVANCED_COMPUTER_VISION_FOR_OBJECT_RECOGNITION: 'advanced_computer_vision_for_object_recognition';
ADVANCED_COMPUTER_VISION_FOR_OBJECT_DETECTION: 'advanced_computer_vision_for_object_detection';
ADVANCED_COMPUTER_VISION_FOR_IMAGE_CLASSIFICATION: 'advanced_computer_vision_for_image_classification';
ADVANCED_ROBOTICS_FOR_AUTONOMOUS_SYSTEMS: 'advanced_robotics_for_autonomous_systems';
ADVANCED_ROBOTICS_FOR_AUTONOMOUS_NAVIGATION: 'advanced_robotics_for_autonomous_navigation';
ADVANCED_EXPLAINABILITY_AND_TRANSPARENCY_FOR_AI_MODELS: 'advanced_explainability_and_transparency_for_ai_models';
ADVANCED_SAFETY_AND_SECURITY_FOR_AI_SYSTEMS: 'advanced_safety_and_security_for_ai_systems';
ADVANCED_ETHICS_AND_GOVERNANCE_FOR_AI_DEVELOPMENT: 'advanced_ethics_and_governance_for_ai_development';
ADVANCED_QUANTUM_COMPUTING_FOR_OPTIMIZATION_PROBLEMS: 'advanced_quantum_computing_for_optimization_problems';
ADVANCED_QUANTUM_COMPUTING_FOR_MACHINE_LEARNING: 'advanced_quantum_computing_for_machine_learning';
ADVANCED_CAUSAL_REASONING_FOR_DECISION_MAKING: 'advanced_causal_reasoning_for_decision_making';
ADVANCED_COGNITIVE_ARCHITECTURES_FOR_HUMAN_AI_COLLABORATION: 'advanced_cognitive_architectures_for_human_ai_collaboration';
ADVANCED_MACHINE_LEARNING_FOR_RECOMMENDATION_SYSTEMS: 'advanced_machine_learning_for_recommendation_systems';
ADVANCED_NLP_FOR_SENTIMENT_ANALYSIS: 'advanced_nlp_for_sentiment_analysis';
ADVANCED_NLP_FOR_TEXT_GENERATION: 'advanced_nlp_for_text_generation';
ADVANCED_NLP_FOR_LANGUAGE_TRANSLATION: 'advanced_nlp_for_language_translation';
ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_DATA: 'advanced_machine_learning_for_time_series_data';
ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_FORECASTING: 'advanced_machine_learning_for_time_series_forecasting';
ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_ANALYSIS: 'advanced_machine_learning_for_time_series_analysis';

// --- Punctuation ---
LPAREN: '('; RPAREN: ')'; LBRACE: '{'; RBRACE: '}'; LBRACK: '['; RBRACK: ']';
COMMA: ','; DOT: '.'; SEMI: ';'; COLON: ':'; ARROW: '->'; FAT_ARROW: '=>';
DCOLON: '::'; TILDE: '~'; HASH: '#'; AT: '@'; QMARK: '?'; BANG: '!';

// --- Operators ---
PLUS: '+'; MINUS: '-'; STAR: '*'; SLASH: '/'; PERCENT: '%'; EQ: '=';
EQEQ: '=='; NEQ: '!='; LT: '<'; GT: '>'; LTE: '<='; GTE: '>=';
AND: '&&'; OR: '||'; AMP: '&'; PIPE: '|'; CARET: '^';
LSHIFT: '<<'; RSHIFT: '>>'; URSHIFT: '>>>';
PLUS_EQ: '+='; MINUS_EQ: '-='; STAR_EQ: '*='; SLASH_EQ: '/='; PERCENT_EQ: '%=';
AMP_EQ: '&='; PIPE_EQ: '|='; CARET_EQ: '^='; LSHIFT_EQ: '<<='; RSHIFT_EQ: '>>=';
DOTDOT: '..'; DOTDOTEQ: '..=';
INC: '++'; DEC: '--';
POWER: '**';
NULL_COALESCE: '??';
SAFE_NAV: '?.';
SAFE_INDEX: '?[';
OR_ASSIGN: '||=';
AND_ASSIGN: '&&=';
NULL_ASSIGN: '??=';

// --- Basic Lexical Units ---
INTEGER: [0-9]+ ('_' [0-9]+)*;
FLOAT: [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)?;
STRING: '"' (ESC | ~["\\])* '"';
CHAR: '\'' (ESC | ~['\\]) '\'';
fragment ESC: '\\' [btnfr"'\\] | '\\' 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT;
fragment HEX_DIGIT: [0-9a-fA-F];
fragment DIGIT: [0-9];

IDENT: [a-zA-Z_] [a-zA-Z0-9_]*;

// --- Dirac Notation ---
QUANTUM_LITERAL: '|' ('0' | '1' | '+' | '-' | IDENT) '⟩';

// --- Annotations ---
NANO_ANNOTATION: '@' IDENT ('(' (IDENT ':' IDENT | IDENT) ')');

// --- MTS ---
MTS_LITERAL: 'mts' '[' INTEGER ']';

// --- Globalization / CJK ---
fragment CJK_CHARACTER: [\u4E00-\u9FFF\u3040-\u309F\u30A0-\u30FF];
GLOBAL_IDENT: (IDENT | CJK_CHARACTER) (IDENT | DIGIT | CJK_CHARACTER)*;

WS: [ \t\r\n]+ -> skip;
LINE_COMMENT: '//' ~[\n]* '\n' -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
DOC_COMMENT: '///' ~[\n]*;

// Zamani.g4 — Comprehensive ANTLR4 combined grammar for the Zamani language.
//
// This grammar unifies all documented features from:
//   - GRAMMAR.md (reference implementation)
//   - ZAMANI_GRAMMAR.md (NIMBUS v3.0 Universal Trinity + UBUNTU + OOP + Omniversal)
//   - Zenith.g4 (ANTLR4 baseline)
//
// It covers: core language, OOP, quantum, nano, MTS, Sankofa, algebraic effects,
// dependent types, session types, meta-programming, HDL, distributed/cloud computing,
// cryptography, on-device AI, self-evolution, actor model, AI/AGI governance,
// omniversal simulation, green computing, developer relations, and higher-kinded types.
//
// Build:
//   antlr4 -Dlanguage=Python3 -visitor -listener -o generated grammar/Zamani.g4
//   antlr4 -Dlanguage=Java   -visitor -listener -o generated grammar/Zamani.g4
//
// NOTE: Many advanced features (omniversal, AGI governance, self-evolution, etc.)
// are syntactically modeled here as documented but may not yet have runtime
// semantics in the reference compiler.

grammar Zamani;


// ═══════════════════════════════════════════════════════════════════════════════
// Parser Rules
// ═══════════════════════════════════════════════════════════════════════════════

program
    : declaration* EOF
    ;

declaration
    : docComment? attributeDecl? ( moduleDecl
    | importDecl
    | exportDecl
    | functionDecl
    | structDecl
    | enumDecl
    | traitDecl
    | implDecl
    | typeAliasDecl
    | constDecl
    | classDecl
    | interfaceDecl
    | recordDecl
    | quantumCircuitDecl
    | nanoAgentDecl
    | languageDecl
    | effectDecl
    | mtsDecl
    | sankofaDecl
    | agentDecl
    | cognitiveBlock
    | metaBlock
    | hdlModuleDecl
    | cloudDecl
    | distributedDecl
    | onDeviceAgentDecl
    | selfEvolveDecl
    | optPassDecl
    | targetPlatform
    | runtimeDecl
    | actorDecl
    | aiSystemDecl
    | agiSystemDecl
    | asiSystemDecl
    | aesiSystemDecl
    | asesiSystemDecl
    | adminInterfaceDecl
    | paymentGatewayDecl
    | userFeedbackDecl
    | copyrightNoticeDecl
    | tailorMadeFeatureDecl
    | programOnceDecl
    | maliciousIdeaDetection
    | userBlockingDecl
    | legalActionDecl
    | sandboxDecl
    | omniversalSimulationDecl
    | omniversalCodeSynthDecl
    | omniversalDeployDecl
    | omniversalAlignmentDecl
    | omniversalContainmentDecl
    | omniversalTrustDecl
    | omniversalKnowledgeDecl
    | omniversalGenerativeDecl
    | omniversalSovereigntyDecl
    | omniversalGoalDecl
    | omniversalBioNanoDecl
    | omniversalRealityDecl
    | omniversalNlpDecl
    | chatArchitectDecl
    | greenComputingAttr
    | thermalOptDecl
    | resourceConserveDecl
    | selfDiscoverDecl
    | developerAnalyticsDecl
    | licenseTrackingDecl
    | deploymentDecl
    | versionReleaseDecl
    | lspServerDecl
    | typeClassDecl
    | typeClassInstance
    | higherKindedTypeDecl
    | selfAdjustDecl
    | selfVersioningDecl
    | extensionMethodDecl
    | extensionPropertyDecl
    | extensionIndexerDecl
    | extensionOperatorDecl
    | macroDecl
    | domainSpecificLanguageDecl
    | aspectDecl
    | typeProviderDecl
    | dataParallelismDecl
    | concurrentDataStructureDecl
    | messageHandlerDecl
    | musicDecl
    | roboticsDecl
    | deepLearningDecl
    | graphicsDecl
    | videoDecl
    | tensorDecl
    | matrixDecl
    | vectorDecl
    | mlModelDecl
    | quantumMlBlock
    | explainableRlBlock
    | explainableDeepLearningBlock
    | knowledgeGraphBlock
    | probabilisticGraphicalModelBlock
    | transferLearningBlock
    | multiAgentBlock
    | autonomousSystemBlock
    | graphModelingBlock
    | advancedNlpBlock
    | cognitiveArchitectureBlock
    | aiForBusinessBlock
    | vrArInteractionBlock
    | imageVideoAnalysisBlock
    | fileScopedType
    | attributeDecl
    | statement
    );

moduleDecl
    : 'module' ident ('::' ident)* blockExpr
    | 'module' ident ('::' ident)* ';'
    ;

importDecl
    : 'import' modulePath ('as' ident)? ';'?
    ;

exportDecl
    : 'export' ident ('to' ident)? ';'?
    ;

modulePath
    : ident ('::' ident | '.' ident)*
    ;

useStmt
    : 'use' usePath ';'?
    ;

usePath
    : segment ('::' segment)* ('::' '*')?
    | segment ('::' segment)* '::' '{' ident (',' ident)* '}'
    ;

segment
    : ident
    ;

globalUsing
    : 'global' 'using' ident ';'
    ;

functionDecl
    : modifiers? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('with' effectList)? blockExpr
    ;

params
    : param (',' param)*
    ;

param
    : 'mut'? ident (':' typeExpr)? ('=' expression)?
    | '...' typeExpr ident
    ;

modifiers
    : modifier+
    ;

modifier
    : 'pub' | 'private' | 'protected' | 'static' | 'const' | 'async'
    | 'unsafe' | 'inline' | 'override' | 'final' | 'abstract' | 'virtual'
    | 'sealed' | 'partial' | 'file' | 'required' | 'init'
    ;

returnStmt
    : 'return' expression? ';

breakStmt
    : 'break' ';

continueStmt
    : 'continue' ';

whileStmt
    : 'while' expression blockExpr
    ;

forStmt
    : 'for' ident 'in' expression blockExpr
    ;

loopExpr
    : 'loop' blockExpr
    ;

ifExpr
    : 'if' expression blockExpr ('else' (ifExpr | blockExpr))?
    ;

matchStmt
    : matchExpr
    ;

matchExpr
    : 'match' expression '{' matchCase* '}'
    ;

matchCase
    : 'case' pattern ('when' expression)? '=>' expression ','?
    | pattern ('when' expression)? '=>' expression ','?
    ;

pattern
    : ident
    | literal
    | '_'
    | '(' pattern (',' pattern)* ')'
    | '[' pattern (',' pattern)* ']'
    | '[' pattern (',' pattern)* '...' pattern ']'
    | pattern '|' pattern
    | ident ':' typeExpr
    ;

unsafeBlock
    : 'unsafe' ident? blockExpr
    | 'unsafe' '!' '(' 'evas' ':' expression ')' blockExpr
    ;

throwStmt
    : 'throw' expression ';

tryCatchStmt
    : 'try' blockExpr catchClause* finallyClause?
    ;

catchClause
    : 'catch' ('(' param ')')? blockExpr
    ;

finallyClause
    : 'finally' blockExpr
    ;

blockExpr
    : '{' statement* '}'
    ;

letStmt
    : ('let' | 'var') 'mut'? ident (':' typeExpr)? '=' expression ';

constStmt
    : 'const' ident (':' typeExpr)? '=' expression ';

constDecl
    : 'const' ident (':' typeExpr)? '=' expression ';

expression
    : assignmentExpr
    ;

assignmentExpr
    : rangeExpr (assignOp assignmentExpr)?
    ;

assignOp
    : '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>='
    ;

rangeExpr
    : logicalOrExpr (('..' | '..=') logicalOrExpr)?
    ;

logicalOrExpr
    : logicalAndExpr (('||' | 'or') logicalAndExpr)*
    ;

logicalAndExpr
    : bitOrExpr (('&&' | 'and') bitOrExpr)*
    ;

bitOrExpr
    : bitXorExpr ('|' bitXorExpr)*
    ;

bitXorExpr
    : bitAndExpr ('^' bitAndExpr)*
    ;

bitAndExpr
    : equalityExpr ('&' equalityExpr)*
    ;

equalityExpr
    : comparisonExpr (('==' | '!=' | '===' | '!==') comparisonExpr)*
    ;

comparisonExpr
    : shiftExpr (('<' | '<=' | '>' | '>=' | 'instanceof' | 'is' | 'has') shiftExpr)*
    ;

shiftExpr
    : sumExpr (('<<' | '>>' | '>>>') sumExpr)*
    ;

sumExpr
    : productExpr (('+' | '-') productExpr)*
    ;

productExpr
    : castExpr (('*' | '/' | '%') castExpr)*
    ;

castExpr
    : prefixExpr (('as' | ':') typeExpr)*
    ;

prefixExpr
    : ('-' | '!' | '~' | '&' 'mut'? | '*' | '++' | '--') prefixExpr
    | postfixExpr
    ;

postfixExpr
    : primaryExpr postfixOp*
    ;

postfixOp
    : '(' args? ')'                    # callOp
    | '[' expression ']'               # indexOp
    | '.' ident ('(' args? ')')?       # memberOp
    | '?'                              # tryPropagateOp
    | '++'                             # postIncOp
    | '--'                             # postDecOp
    ;

args
    : expression (',' expression)*
    | namedArgument (',' namedArgument)*
    ;

namedArgument
    : ident '=' expression
    ;

primaryExpr
    : ident structLiteralTail?         # identExpr
    | literal                          # literalExpr
    | '(' expression ')'               # parenExpr
    | '(' expression (',' expression)+ ')' # tupleExpr
    | '[' (expression (',' expression)*)? ']' # arrayExpr
    | 'map' '{' (expression '=>' expression (',' expression '=>' expression)*)? '}' # mapExpr
    | blockExpr                        # blockValExpr
    | '|' params? '|' (blockExpr | expression) # lambdaExpr
    | 'fn' '(' params? ')' ('->' typeExpr)? blockExpr # anonFnExpr
    | ifExpr                           # ifValExpr
    | matchExpr                        # matchValExpr
    | loopExpr                         # loopValExpr
    | 'async' expression               # asyncExpr
    | 'await' expression               # awaitExpr
    | 'spawn' expression               # spawnExpr
    | 'new' ident typeArgs? '(' args? ')' # newExpr
    | 'try' expression catchClause*    # tryCatchExpr
    | 'yield' expression?              # yieldExpr
    | recallExpr                       # recallValExpr
    | learnExpr                        # learnValExpr
    | performExpr                      # performValExpr
    | zamaniExpr                       # zamaniValExpr
    | sasaExpr                         # sasaValExpr
    | quantumOpExpr                    # quantumOpValExpr
    | nanoExpr                         # nanoValExpr
    | mtsExpr                          # mtsValExpr
    | consensusExpr                    # consensusValExpr
    | ancestorCall                     # ancestorValExpr
    | withExpr                         # withValExpr
    | mopExpr                          # mopValExpr
    | macroCall                        # macroCallExpr
    | 'super' ('.' ident | '(' args? ')')? # superExpr
    | 'this'                           # thisExpr
    | 'self'                           # selfExpr
    | interpolatedString               # interpStringExpr
    ;

structLiteralTail
    : '{' (ident ':' expression ','?)* '}'
    ;

recallExpr
    : 'recall' ('(' expression ')' | expression)
    ;

learnExpr
    : ('learn' | 'infer') 'from'? expression ('with' 'weight' expression)?
    ;

performExpr
    : 'perform' expression
    ;

zamaniExpr
    : 'zamani' (blockExpr | expression)
    ;

sasaExpr
    : 'sasa' (blockExpr | expression)
    ;

quantumOpExpr
    : 'quantum' ident ('(' args? ')')?
    | 'superpose' '(' expression (',' expression)* ')'
    | 'entangle' '(' ident ',' ident ')'
    ;

nanoExpr
    : nanoLit
    | 'assemble' '(' expression ')'
    | 'deploy' '(' expression ')'
    ;

mtsExpr
    : mtsLit
    | 'parallel' '(' blockExpr ')'
    | 'speculative' '(' blockExpr ')'
    | 'counterfactual' '(' expression ',' blockExpr ')'
    ;

consensusExpr
    : 'consensus' '[' exprList ']' 'vote' expression
    ;

ancestorCall
    : 'ancestral' ident '(' args? ')' ';

withExpr
    : expression 'with' '[' effectList ']'
    | expression 'with' '{' (ident ':' expression ';

mopExpr
    : 'reflect' '(' expression ')'
    | 'introspect' '(' ident ')'
    | 'meta_eval' '(' expression ')'
    | 'quote' '{' statement* '}'
    | 'unquote' '(' expression ')'
    | 'splice' '(' expression ')'
    ;

macroCall
    : ident '!' '(' args? ')'
    ;

exprList
    : expression (',' expression)*
    ;

expressionStmt
    : expression ';

literal
    : INTEGER
    | FLOAT
    | STRING
    | CHAR
    | BOOLEAN
    | NIL
    | quantumLit
    | nanoLit
    | mtsLit
    | rawStringLit
    | utf8StringLit
    ;

quantumLit
    : '|' ('0' | '1' | '+' | '-' | ident) '\\u27E9'
    ;

nanoLit
    : '@atom' '(' ident ':' ORBITAL ')'
    | '@molecule' '(' FORMULA ')'
    ;

mtsLit
    : 'mts' '[' INTEGER ']'
    ;

interpolatedString
    : '$' STRING
    ;

rawStringLit
    : 'r' '#'* STRING
    ;

utf8StringLit
    : 'u8' STRING
    ;

typeExpr
    : baseType ('<' typeExpr (',' typeExpr)* '>')?   # namedType
    | '(' ')'                                         # unitType
    | '(' typeExpr ')'                                # parenType
    | '(' typeExpr (',' typeExpr)+ ')' ('->' typeExpr)? # tupleOrFnType
    | 'fn' '(' (typeExpr (',' typeExpr)*)? ')' ('->' typeExpr)? # fnType
    | '&' 'mut'? '[' typeExpr ']'                     # sliceRefType
    | '&' 'mut'? typeExpr                             # refType
    | '*' 'mut'? typeExpr                             # rawPtrType
    | '[' typeExpr (';

baseType
    : 'void' | 'int' | 'float' | 'bool' | 'string' | 'char' | 'bytes'
    | 'i8' | 'i16' | 'i32' | 'i64' | 'i128'
    | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
    | 'f32' | 'f64' | 'usize' | 'isize'
    | ident
    ;

typeParams
    : '<' typeParam (',' typeParam)* '>'
    ;

typeParam
    : ('out' | 'in' | '*')? ident (':' typeConstraint)?
    ;

typeConstraint
    : typeExpr ('+' typeExpr)*
    ;

typeArgs
    : '<' typeExpr (',' typeExpr)* '>'
    ;

piType
    : ('\\u03A0' | 'Pi') '(' ident ':' typeExpr ')' typeExpr
    ;

sigmaType
    : ('\\u03A3' | 'Sigma') '(' ident ':' typeExpr ')' typeExpr
    ;

identityType
    : 'Id' '(' typeExpr ',' expression ',' expression ')'
    ;

sessionOp
    : 'send' typeExpr
    | 'recv' typeExpr
    | 'offer' '{' sessionBranch* '}'
    | 'choice' '{' sessionBranch* '}'
    | 'close'
    ;

sessionBranch
    : ident '->' typeExpr
    ;

quantumType
    : 'Qubit'
    | 'QReg' '[' expression ']'
    | 'Superposition' '<' typeExpr '>'
    | 'Entangled' '<' typeExpr ',' typeExpr '>'
    | 'QMeasured' '<' typeExpr '>'
    | 'QArray' '<' typeExpr ',' expression '>'
    ;

nanoType
    : 'Atom' '<' typeExpr '>'
    | 'Molecule' '<' typeExpr '>'
    | 'NanoAgent' '<' typeExpr '>'
    | 'Archaeve' '<' typeExpr '>'
    ;

mtsType
    : 'MtsSlice' '<' expression '>'
    ;

sankofaType
    : 'History' '<' typeExpr ',' expression '>'
    | 'ConsensusTrue' '<' typeExpr '>'
    | 'InterMemory' '<' STRING ',' typeExpr '>'
    ;

cognitiveType
    : 'CognitiveState' '<' typeExpr '>'
    | 'Consciousness' '<' typeExpr '>'
    | 'Biological' '<' typeExpr '>'
    | 'Neural' '<' typeExpr '>'
    | 'MemoryBank' '<' typeExpr '>'
    | 'AgentType'
    | 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI'
    ;

structDecl
    : 'struct' ident typeParams? '{' structField* '}'
    ;

structField
    : modifiers? typeExpr ident ('=' expression)? ','?
    ;

enumDecl
    : 'enum' ident typeParams? (':' typeExpr)? '{' enumVariant* '}'
    ;

enumVariant
    : ident enumVariantKind? ','?
    ;

enumVariantKind
    : '(' typeExpr (',' typeExpr)* ')'    # tupleVariant
    | '{' structField* '}'                # structVariant
    ;

traitDecl
    : 'trait' ident typeParams? (':' typeExpr (',' typeExpr)*)? '{' traitItem* '}'
    ;

traitItem
    : traitMethod
    | traitAssocType
    | constantDef
    ;

traitMethod
    : 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? (blockExpr | ';

traitAssocType
    : 'type' ident (':' typeExpr)? ';

constantDef
    : 'const' ident ':' typeExpr '=' expression ';

implDecl
    : 'impl' typeParams? typeExpr ('for' typeExpr)? '{' implItem* '}'
    ;

implItem
    : functionDecl
    | traitAssocType
    | constantDef
    ;

typeAliasDecl
    : 'type' ident typeParams? '=' typeExpr ';

classDecl
    : modifiers? 'class' ident typeParams?
      extendsClause? implementsClause? permitsClause?
      '{' classMember* '}'
    | 'abstract' 'class' ident '{' classMember* '}'
    | 'final' 'class' ident extendsClause? '{' classMember* '}'
    | 'sealed' 'class' ident extendsClause? permitsClause? '{' classMember* '}'
    | 'static' 'class' ident '{' classMember* '}'
    | 'partial' 'class' ident '{' classMember* '}'
    | 'file' 'class' ident '{' classMember* '}'
    ;

extendsClause
    : ('extends' | 'implements') ident (',' ident)*
    ;

implementsClause
    : 'implements' ident (',' ident)*
    ;

permitsClause
    : 'permits' ident (',' ident)*
    ;

classMember
    : modifiers? typeExpr ident ('=' expression)? ';

getter
    : 'get' blockExpr
    ;

setter
    : 'set' blockExpr
    ;

interfaceDecl
    : 'interface' ident typeParams? extendsClause? '{' interfaceMember* '}'
    | 'sealed' 'interface' ident '{' interfaceMember* '}'
    ;

interfaceMember
    : 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? (blockExpr | ';

recordDecl
    : 'record' ident typeParams? '(' params? ')' extendsClause? implementsClause? '{' recordBody? '}'
    | 'record' 'struct' ident '(' params? ')' '{' recordBody? '}'
    ;

recordBody
    : classMember*
    ;

extensionMethodDecl
    : 'extension' 'fn' ident '(' params? ')' ('->' typeExpr)? ('where' typeConstraint)? blockExpr
    ;

extensionPropertyDecl
    : 'extension' typeExpr ident '{' getter ';

extensionIndexerDecl
    : 'extension' typeExpr 'this' '[' params? ']' '{' getter ';

extensionOperatorDecl
    : 'extension' 'operator' OPERATOR '(' params? ')' ('where' typeConstraint)? blockExpr
    ;

quantumCircuitDecl
    : ('quantum' 'circuit' | 'circuit' | 'quantum') ident '(' params? ')' blockExpr
    ;

quantumStmt
    : quantumGate
    | quantumMeasure
    | quantumReset
    | quantumBarrier
    ;

quantumGate
    : ('Hadamard' | 'CNOT' | 'PauliX' | 'PauliY' | 'PauliZ' | 'T' | 'S' | 'Swap') '(' ident (',' ident)* ')'
    | ('hadamard' | 'cnot' | 'paulix' | 'pauliy' | 'pauliz' | 't' | 's' | 'swap') '(' ident (',' ident)* ')'
    ;

quantumMeasure
    : 'measure' ident ('->' ident)?
    ;

quantumReset
    : 'reset' ident
    ;

quantumBarrier
    : 'barrier' (ident (',' ident)*)?
    ;

nanoAgentDecl
    : ('nano' 'agent' | 'agent' | 'nano') ident '{' nanoAgentBody '}'
    ;

nanoAgentBody
    : nanoCapability* nanoBehavior* nanoProtocol*
    ;

nanoCapability
    : 'capability' ident '(' params? ')' blockExpr
    ;

nanoBehavior
    : 'behavior' ident '(' params? ')' blockExpr
    ;

nanoProtocol
    : 'protocol' ident '{' protocolRule* '}'
    ;

protocolRule
    : 'on' ident '->' blockExpr
    ;

mtsDecl
    : 'mts' 'timeline' ident '{' mtsBody '}'
    ;

mtsBody
    : mtsSlice* mtsOperation*
    ;

mtsSlice
    : 'slice' ident '[' INTEGER ']' blockExpr
    ;

mtsOperation
    : 'fork' '(' ident ')'
    | 'merge' '(' ident ')'
    | 'observe' '(' ident ')'
    | 'rewind' '(' INTEGER ')'
    ;

sankofaDecl
    : memoryDecl
    | wisdomDecl
    | historyDecl
    | consensusDecl
    | interMemoryDecl
    ;

memoryDecl
    : 'remember' ident ':' typeExpr '=' expression ';

wisdomDecl
    : 'wisdom' ident '{' wisdomBody '}'
    ;

wisdomBody
    : (premiseDecl | inferenceRule | wisdomStmt)*
    ;

premiseDecl
    : 'premise' ident ':' typeExpr '=' expression ';

inferenceRule
    : 'rule' ident '(' params? ')' '->' blockExpr
    ;

wisdomStmt
    : 'conclude' expression ';

historyDecl
    : 'history' ident '{' statement* '}'
    ;

consensusDecl
    : 'consensus' ident '{' statement* '}'
    ;

interMemoryDecl
    : 'intermemory' ident '{' statement* '}'
    ;

sankofaRememberStmt
    : 'remember' ident (':' typeExpr)? '=' expression ';

learnStmt
    : 'learn' 'from' expression ('with' 'weight' expression)? ';

effectDecl
    : 'effect' ident typeParams? '(' params? ')' ('->' typeExpr)? ';

effectList
    : '[' effectName (',' effectName)* ']'
    ;

effectName
    : ident
    ;

handleStmt
    : 'handle' expression '{' effectHandler* '}' ('with' blockExpr)?
    ;

effectHandler
    : 'case' effectName '(' params? ')' '->' blockExpr
    ;

languageDecl
    : 'language' ident ('=' STRING)? ('{' grammarRule* '}')? ';

grammarRule
    : ident ':' STRING ';

metaBlock
    : 'meta' '{' statement* '}'
    ;

invokeStmt
    : 'invoke' modulePath '(' args? ')' ';

transcodeStmt
    : 'transcode' ident '::' STRING 'to' ident ';

overrideStmt
    : 'override' ident '::' ident '(' params? ')' blockExpr
    ;

langStmt
    : 'lang' ident blockExpr
    ;

pluginStmt
    : 'plugin' ident '{' pluginDefinition* '}'
    ;

pluginDefinition
    : ('language' | 'transpiler') ident ';

macroDecl
    : 'macro' ident '(' params? ')' blockExpr
    ;

domainSpecificLanguageDecl
    : 'domain' ident (':' typeExpr)? ('module' ident)? ('type' typeExpr)? ('plugin' ident)? ('provider' ident)? '{' statement* '}'
    ;

aspectDecl
    : 'aspect' ident ('extends' ident)? ('depends' ident)? ('compose' ident)? ('weave' ident)? ('domain' ident)? '{' statement* '}'
    ;

typeProviderDecl
    : 'provider' ident '(' typeParams? ')' '{' statement* '}' ('cache' ident)? ('inject' ident)? ('metadata' typeExpr)? ('aspect' ident)?
    ;

hdlModuleDecl
    : 'hdl' 'module' ident '{' hdlBody '}'
    ;

hdlBody
    : hdlPort* hdlComponent* hdlLogicGate* hdlSignal* hdlAssignment*
    ;

hdlPort
    : 'port' hdlPortDirection ident ':' hdlPortType
    ;

hdlPortDirection
    : 'input' | 'output' | 'inout'
    ;

hdlPortType
    : 'wire' | 'reg' | 'logic' | 'quantum' | 'nano' | typeExpr
    ;

hdlComponent
    : 'component' ident '{' hdlPort* '}'
    ;

hdlLogicGate
    : 'gate' hdlLogicGateType '(' ident (',' ident)* ')'
    | 'gate' quantumGateType '(' ident (',' ident)* ')'
    ;

hdlLogicGateType
    : 'AND' | 'OR' | 'NOT' | 'XOR' | 'NAND' | 'NOR' | 'MUX' | 'DEMUX'
    ;

quantumGateType
    : 'Hadamard' | 'CNOT' | 'Toffoli' | 'Fredkin' | 'PauliX' | 'PauliY' | 'PauliZ'
    ;

hdlSignal
    : 'signal' ident ':' hdlPortType
    ;

hdlAssignment
    : 'assign' ident '=' hdlExpression ';

hdlExpression
    : ident
    | hdlLogicGate
    | '(' hdlExpression ')'
    | hdlExpression ('&' | '|' | '^' | '~') hdlExpression
    ;

externalHdlImport
    : 'import_hdl' '(' STRING ',' ident ')'
    ;

externalHdlLink
    : 'link_hdl' '(' ident ',' ident ')'
    ;

distributedDecl
    : 'distributed' 'node' ident '{' distributedBody '}'
    ;

distributedBody
    : (serviceHandle | remoteCall | distributedOperation)*
    ;

serviceHandle
    : 'service' ident 'at' STRING
    ;

remoteCall
    : 'remote' ident '::' ident '(' args? ')' ';

distributedOperation
    : 'teleport' '(' ident ',' STRING ')'
    | 'migrate' '(' ident ',' STRING ')'
    | 'dsm' '(' ident ',' STRING ')'
    | 'dmts' '(' ident ',' STRING ')'
    ;

cloudDecl
    : cloudPlatform
    | container
    | devOpsTool
    | databaseOp
    | webService
    ;

cloudPlatform
    : ('AWS' | 'Azure' | 'GCP') '::' ident '(' args? ')' ';

container
    : 'Docker' '::' ident '(' args? ')' ';

devOpsTool
    : 'Jenkins' '::' ident '(' args? ')' ';

databaseOp
    : 'Database' '::' ident '(' args? ')' ';

webService
    : 'HTTP' '::' ident '(' args? ')' ';

dataStmt
    : ('serialize' | 'deserialize') expression ('to' | 'from') dataFormat ';

dataFormat
    : 'json' | 'xml' | 'messagepack' | 'protobuf' | 'cbor'
    ;

streamingData
    : 'stream' expression 'pipe' expression ';

foreignFunctionCall
    : 'foreign' ident '::' ident '(' args? ')' ';

cryptoExpr
    : 'encrypt' '(' expression ',' cryptoAlgo ')'
    | 'decrypt' '(' expression ',' cryptoAlgo ')'
    | 'encrypt_homomorphic' '(' expression ',' cryptoKey ')'
    | 'decrypt_homomorphic' '(' expression ',' cryptoKey ')'
    | 'encrypt_layered' '(' expression ',' cryptoAlgoList ')'
    | 'encrypt_quantum_safe' '(' expression ',' cryptoKey ')'
    ;

cryptoAlgo
    : 'AES256' | 'ChaCha20' | 'RSA4096' | 'Kyber' | 'Dilithium' | 'SPHINCS+'
    ;

cryptoAlgoList
    : '[' cryptoAlgo (',' cryptoAlgo)* ']'
    ;

cryptoKey
    : 'key' ident
    | 'generate_key' '(' typeExpr ')'
    ;

zkExpr
    : 'generate_zk_proof' '(' expression ',' expression ')'
    | 'verify_zk_proof' '(' expression ',' expression ')'
    ;

smcExpr
    : 'secure_multi_party_compute' '(' exprList ')'
    ;

keyManagement
    : 'request_key' '(' typeExpr ')'
    | 'rotate_key' '(' ident ')'
    | 'revoke_key' '(' ident ')'
    ;

onDeviceAgentDecl
    : 'on_device' 'agent' ident '{' onDeviceAgentBody '}'
    | 'on_device_agent' ident '{' onDeviceAgentBody '}'
    ;

onDeviceAgentBody
    : (agentCapability | agentBehavior | deviceConstraint | selfPreservationProtocol | offlineProtocol)*
    ;

deviceConstraint
    : 'requires' deviceSpec (',' deviceSpec)* ';

deviceSpec
    : 'memory' '>=' expression
    | 'power' '>=' expression
    | 'storage' '>=' expression
    | 'processor' '=' typeExpr
    ;

selfPreservationProtocol
    : 'self_preserve' blockExpr
    | 'self_preservation' '{' preservationProtocol* '}'
    ;

preservationProtocol
    : 'protocol' STRING
    ;

offlineProtocol
    : 'offline' blockExpr
    ;

offlineAgent
    : 'offline' 'agent' ident '{' offlineBody '}'
    ;

offlineBody
    : (localResourceSpec | mtsLocalSpec | lifecycleSpec)*
    ;

localResourceSpec
    : 'local_resources' '=' '[' identList ']'
    ;

mtsLocalSpec
    : 'local_mts' '=' 'true'
    ;

lifecycleSpec
    : 'lifecycle' '{' lifecycleAction* '}'
    ;

lifecycleAction
    : ('monitor' | 'update' | 'manage') ident
    ;

selfEvolveDecl
    : 'self_evolve' '{' evolveBody '}'
    ;

evolveBody
    : (monitorRule | optimizeRule | patchRule | verifyRule)*
    ;

monitorRule
    : 'monitor' expression '->' blockExpr
    ;

optimizeRule
    : 'optimize' expression '->' blockExpr
    ;

patchRule
    : 'patch' expression '->' blockExpr
    ;

verifyRule
    : 'verify' expression '->' blockExpr
    ;

autoCodeGen
    : 'autonomously_generate' '(' expression ')' '->' blockExpr
    ;

autoOptimize
    : 'autonomously_optimize' '(' expression ')' '->' blockExpr
    ;

autoVerify
    : 'autonomously_verify' '(' expression ')' '->' blockExpr
    ;

optPassDecl
    : 'optimization' 'pass' ident '{' optPassBody '}'
    ;

optPassBody
    : ('target' optTarget ';

optTarget
    : 'classical' | 'quantum' | 'nano' | 'neuromorphic' | 'GPU' | 'FPGA'
    | 'SIMD' | 'WASM' | 'USSD' | 'edge' | 'stellar'
    ;

optStrategy
    : 'DCE' | 'CSE' | 'inlining' | 'loop_unroll' | 'constant_fold'
    | 'quantum_gate_opt' | 'nano_atp_efficiency' | 'ai_quantization'
    | 'thermal_throttle_prevention' | 'simd_vectorization'
    | 'gpu_offloading' | 'quantum_error_correction'
    ;

targetPlatform
    : 'target' platformSpec
    ;

platformSpec
    : 'x86_64' | 'ARM64' | 'RISC-V' | 'WASM' | 'LLVM_IR' | 'bare_metal'
    | 'Android' | 'iOS' | 'cloud' | 'IoT' | 'USSD' | 'FPGA'
    | 'quantum' | 'nano' | 'neuromorphic' | 'stellar'
    | 'Tariro_Runtime' | 'Z_MMP'
    ;

runtimeDecl
    : 'runtime' 'configure' '{' runtimeConfig* '}'
    ;

runtimeConfig
    : 'gc' '=' ('enabled' | 'disabled' | 'hybrid') ';

spawnExpr
    : 'spawn' blockExpr
    ;

channelExpr
    : 'channel' '<' typeExpr '>'
    ;

selectStmt
    : 'select' '{' selectCase* '}'
    ;

selectCase
    : 'case' expression '->' blockExpr
    ;

actorDecl
    : 'actor' ident '{' actorBody '}'
    ;

actorBody
    : (actorMessage | actorBehavior | actorState | concurrentExpr)*
    ;

actorMessage
    : 'message' ident '(' params? ')' blockExpr
    ;

actorBehavior
    : 'behavior' ident blockExpr
    ;

actorState
    : 'state' ident ':' typeExpr '=' expression ';

concurrentExpr
    : 'concurrent' expression blockExpr
    ;

cognitiveBlock
    : 'cognitive_architecture' ident '{' statement* '}'
    ;

aiDomainBlock
    : aiDomain ident '{' statement* '}'
    ;

aiDomain
    : 'machine_learning' | 'deep_learning' | 'neural_network'
    | 'graph_based_deep_learning' | 'causal_discovery' | 'causal_reasoning'
    | 'probabilistic_modeling' | 'advanced_computer_vision'
    | 'predictive_analytics' | 'prescriptive_analytics' | 'data_visualization'
    | 'human_ai_collaboration' | 'advanced_human_ai_collaboration'
    | 'explainability_and_transparency' | 'safety_and_security'
    | 'ethics_and_governance' | 'quantum_machine_learning'
    | 'cognitive_architectures' | 'human_computer_interaction'
    | 'data_analytics' | 'time_series_ml' | 'time_series_forecasting'
    | 'time_series_analysis' | 'text_generation' | 'sentiment_analysis'
    | 'language_translation' | 'object_detection' | 'object_recognition'
    | 'image_classification' | 'autonomous_navigation' | 'autonomous_robotics'
    | 'autonomous_systems' | 'model_explainability' | 'ai_safety_and_security'
    | 'ai_ethics_and_governance' | 'quantum_optimization'
    | 'recommendation_systems' | 'quantum_computing' | 'blockchain_technology'
    | 'iot' | 'cybersecurity' | 'data_science' | 'natural_language_generation'
    | 'nl_generation' | 'advanced_robotics' | 'transfer_learning'
    | 'explainable_reinforcement_learning' | 'graph_neural_network'
    | 'advanced_natural_language_processing' | 'advanced_natural_language_generation'
    | 'advanced_computer_vision_for_object_recognition'
    | 'advanced_computer_vision_for_object_detection'
    | 'advanced_computer_vision_for_image_classification'
    | 'advanced_robotics_for_autonomous_systems'
    | 'advanced_robotics_for_autonomous_navigation'
    | 'advanced_explainability_and_transparency_for_ai_models'
    | 'advanced_safety_and_security_for_ai_systems'
    | 'advanced_ethics_and_governance_for_ai_development'
    | 'advanced_quantum_computing_for_optimization_problems'
    | 'advanced_quantum_computing_for_machine_learning'
    | 'advanced_causal_reasoning_for_decision_making'
    | 'advanced_cognitive_architectures_for_human_ai_collaboration'
    | 'advanced_machine_learning_for_recommendation_systems'
    | 'advanced_nlp_for_sentiment_analysis' | 'advanced_nlp_for_text_generation'
    | 'advanced_nlp_for_language_translation'
    | 'advanced_machine_learning_for_time_series_data'
    | 'advanced_machine_learning_for_time_series_forecasting'
    | 'advanced_machine_learning_for_time_series_analysis'
    ;

agentDecl
    : 'agent' ident (':' agentType)? '{' agentBody '}'
    ;

agentBody
    : (agentCapability | agentBehavior | agentGoal | agentConstraint)*
    ;

agentGoal
    : 'goal' expression ';

agentConstraint
    : 'constraint' expression ';

agentType
    : 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI'
    ;

humanAgiCollaboration
    : 'human_ai_collaboration' ident '{' statement* '}'
    ;

aiSystemDecl
    : 'ai' ident '{' aiSystemBody '}'
    ;

aiSystemBody
    : (aiTypeDef | aiCapabilityDef | explainStmt | transparentStmt | selfVersioningDecl)*
    ;

aiTypeDef
    : 'type' ident '=' ('narrow' | 'general' | 'super') ';

aiCapabilityDef
    : 'capability' ident '{' statement* '}'
    ;

agiSystemDecl
    : 'agi' ident '{' agiSystemBody '}'
    ;

agiSystemBody
    : (agiCapabilityDef | agiLearningDef)*
    ;

agiCapabilityDef
    : 'capability' ident '{' statement* '}'
    ;

agiLearningDef
    : 'learning' ident '{' statement* '}'
    ;

asiSystemDecl
    : 'asi' ident '{' asiSystemBody '}'
    ;

asiSystemBody
    : (asiCapabilityDef | asiSelfImprovementDef)*
    ;

asiSelfImprovementDef
    : 'self_improvement' ident '{' statement* '}'
    ;

aesiSystemDecl
    : 'aesi' ident '{' aesiSystemBody '}'
    ;

aesiSystemBody
    : (aesiCapabilityDef | aesiTransformationDef)*
    ;

aesiTransformationDef
    : 'transformation' ident '{' statement* '}'
    ;

asesiSystemDecl
    : 'asesi' ident '{' asesiSystemBody '}'
    ;

asesiSystemBody
    : (asesiCapabilityDef | asesiOmnipotenceDef)*
    ;

asesiOmnipotenceDef
    : 'omnipotence' ident '{' statement* '}'
    ;

evasCert
    : 'unsafe' '!' '(' 'evas' ':' expression ')' blockExpr
    ;

safetyAttr
    : '#safety' '(' attrArgs ')'
    ;

securityAttr
    : '#security' '(' attrArgs ')'
    ;

ethicsAttr
    : '#ethics' '(' attrArgs ')'
    ;

governanceAttr
    : '#governance' '(' attrArgs ')'
    ;

complianceAttr
    : '#compliance' '(' 'standard' '=' STRING ',' 'certified_by' '=' STRING ')'
    ;

greenComputingAttr
    : '#green' '(' greenArgs ')'
    ;

attrArgs
    : attrArg (',' attrArg)*
    ;

attrArg
    : ident '=' expression
    ;

greenArgs
    : 'minimize_power' '=' expression
    | 'minimize_heat' '=' expression
    | 'minimize_water' '=' expression
    | 'maximize_efficiency' '=' expression
    ;

attributeDecl
    : '@' ident '(' annotationArgs? ')'
    | '#' '[' expression* ']'
    ;

annotationArgs
    : annotationArg (',' annotationArg)*
    ;

annotationArg
    : ident '=' expression
    ;

adminInterfaceDecl
    : 'admin' ident '{' adminInterfaceBody '}'
    ;

adminInterfaceBody
    : (changeLogDisplay | suggestionInput | hybridDef | interfaceDef)*
    ;

changeLogDisplay
    : 'display' 'changes' '{' changeLogBody '}'
    ;

changeLogBody
    : changeLogEntry*
    ;

changeLogEntry
    : 'change' ident 'made' 'by' ident ';

suggestionInput
    : 'input' 'suggestions' '{' suggestionBody '}'
    ;

suggestionBody
    : suggestionEntry*
    ;

suggestionEntry
    : 'suggestion' ident 'from' ident ';

paymentGatewayDecl
    : 'payment' ident '{' paymentGatewayBody '}'
    ;

paymentGatewayBody
    : (paymentMethodDef | paymentVerificationDef)*
    ;

paymentMethodDef
    : 'method' ident '{' statement* '}'
    ;

paymentVerificationDef
    : 'verify' ident '{' statement* '}'
    ;

userFeedbackDecl
    : 'feedback' ident '{' userFeedbackBody '}'
    ;

userFeedbackBody
    : (feedbackInputDef | feedbackValidationDef)*
    ;

feedbackInputDef
    : 'input' 'feedback' '{' statement* '}'
    ;

feedbackValidationDef
    : 'validate' 'feedback' '{' statement* '}'
    ;

copyrightNoticeDecl
    : 'copyright' ident '{' copyrightNoticeBody '}'
    ;

copyrightNoticeBody
    : copyrightNoticeStatement*
    ;

copyrightNoticeStatement
    : 'copyright' ident 'owned' 'by' ident ';

tailorMadeFeatureDecl
    : 'feature' ident '{' tailorMadeFeatureBody '}'
    ;

tailorMadeFeatureBody
    : (featureDef | featureCustomizationDef)*
    ;

featureDef
    : 'define' 'feature' ident '{' statement* '}'
    ;

featureCustomizationDef
    : 'customize' 'feature' ident '{' statement* '}'
    ;

programOnceDecl
    : 'program_once' ident '{' programOnceBody '}'
    ;

programOnceBody
    : (systemConfigDef | systemLogicDef)*
    ;

systemConfigDef
    : 'config' ident '{' statement* '}'
    ;

systemLogicDef
    : 'logic' ident '{' statement* '}'
    ;

maliciousIdeaDetection
    : 'malicious' 'idea' 'detection' '{' maliciousIdeaBody '}'
    ;

maliciousIdeaBody
    : (ideaAnalysisDef | ideaBlockingDef)*
    ;

ideaAnalysisDef
    : 'analyze' 'idea' ident '{' statement* '}'
    ;

ideaBlockingDef
    : 'block' 'idea' ident '{' statement* '}'
    ;

userBlockingDecl
    : 'block' 'user' ident '{' userBlockingBody '}'
    ;

userBlockingBody
    : (userIdentificationDef | userBlockingActionDef)*
    ;

userIdentificationDef
    : 'identify' 'user' ident '{' statement* '}'
    ;

userBlockingActionDef
    : 'block' 'user' ident '{' statement* '}'
    ;

legalActionDecl
    : 'legal' 'action' ident '{' legalActionBody '}'
    ;

legalActionBody
    : (legalProceedingDef | legalNoticeDef)*
    ;

legalProceedingDef
    : 'proceeding' 'legal' ident '{' statement* '}'
    ;

legalNoticeDef
    : 'notice' 'legal' ident '{' statement* '}'
    ;

sandboxDecl
    : 'sandbox' ident '{' sandboxBody '}'
    ;

sandboxBody
    : (simulationDef | testingDef)*
    ;

simulationDef
    : 'simulate' ident '{' statement* '}'
    ;

testingDef
    : 'test' ident '{' statement* '}'
    ;

// ═══════════════════════════════════════════════════════════════════════════════
// Lexer Rules
// ═══════════════════════════════════════════════════════════════════════════════

ADVANCED_NLP_FOR_LANGUAGE_TRANSLATION: 'advanced_nlp_for_language_translation';

ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_DATA: 'advanced_machine_learning_for_time_series_data';

ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_FORECASTING: 'advanced_machine_learning_for_time_series_forecasting';

ADVANCED_MACHINE_LEARNING_FOR_TIME_SERIES_ANALYSIS: 'advanced_machine_learning_for_time_series_analysis';

GOAL_KW: 'goal';

EXPLAIN: 'explain';

LOG: 'log';

SELF_IMPROVEMENT: 'self_improvement';

EVAS: 'evas';

COMPLIANCE: 'compliance';

GREEN: 'green';

MINIMIZE_WATER: 'minimize_water';

ADMIN: 'admin';

SUGGESTION: 'suggestion';

VALIDATE: 'validate';

CUSTOMIZE: 'customize';

MALICIOUS: 'malicious';

USER: 'user';

NOTICE: 'notice';

OMNIVERSAL_SIMULATE: 'omniversal_simulate';

OMNIVERSAL_DEPLOY: 'omniversal_deploy';

OMNIVERSAL_CONTAINMENT: 'omniversal_containment';

OMNIVERSAL_KNOWLEDGE: 'omniversal_knowledge';

OMNIVERSAL_SOVEREIGNTY: 'omniversal_sovereignty';

OMNIVERSAL_BIONANO: 'omniversal_bionano';

OMNIVERSAL_NLP: 'omniversal_nlp';

UNIVERSE: 'universe';

CHAT_AGENT: 'chat_agent';

NL_GENERATE: 'nl_generate';

DOCUMENT: 'document';

JOURNAL: 'journal';

MULTIMODAL: 'multimodal';

THERMAL_OPTIMIZE: 'thermal_optimize';

MAX_TEMP: 'max_temp';

CARBON_FOOTPRINT: 'carbon_footprint';

AOT: 'AOT';

DVFS: 'dvfs';

RENEWABLE: 'renewable';

QPU: 'QPU';

SELF_DISCOVER: 'self_discover';

AUTO_SUGGEST: 'auto_suggest';

DETECT_IDE: 'detect_ide';

INTRODUCE: 'introduce';

DEVELOPER_ANALYTICS: 'developer_analytics';

TRACK_COMPANIES: 'track_companies';

TRACK_USAGE: 'track_usage';

LICENSES: 'licenses';

CHANGELOG: 'changelog';

GO_TO_DEF: 'go_to_def';

TYPECLASS: 'typeclass';

SELF_ADJUST: 'self_adjust';

CREATED: 'created';

TENSOR: 'tensor';

MODEL: 'model';

MUSIC: 'music';

ROBOT: 'robot';

DL: 'dl';

GRAPHICS: 'graphics';

RESOLUTION: 'resolution';

DIM: 'dim';

INFINITY: 'infinity';

GENERATION: 'generation';

PERFORM: 'perform';

ADD: 'add';

INSERT: 'insert';

VAR: 'var';

POINT: 'point';

FILE_SCOPED: 'file';

FINAL: 'final';

GET: 'get';

FILE: 'file';

TYPEOF: 'typeof';

SIZE: 'size';

LINEARIZATION: 'linearization';

PATH_DEPENDENT: 'path';

TYPE_FAMILY: 'type_family';

BOUNDED_RECURSION: 'bounded_recursion';

CONTEXT_DEPENDENT: 'context_dependent';

TYPE_PROVIDER: 'type_provider';

SQL: 'sql';

DATA_STRUCTURE: 'data_structure';

MESSAGE_HANDLER: 'message_handler';

DATA_INTERCHANGE: 'data_interchange';

RAW_PTR: 'raw_ptr';

OWNED: 'owned';

FUTURE: 'Future';

FROM: 'from';

CLONE: 'clone';

CONST_KW: 'const';

SUPER: 'super';

IF: 'if';

BREAK: 'break';

TRY: 'try';

UNSAFE: 'unsafe';

PROC: 'proc';

STRUCT: 'struct';

LET: 'let';

DYN: 'dyn';

OVERRIDE: 'override';

UNSIZED: 'unsized';

TRAIT2: 'trait';

LET2: 'let';

DYN2: 'dyn';

OVERRIDE2: 'override';

UNSIZED2: 'unsized';

BOOLEAN: TRUE | FALSE;

NIL: NIL_KW | NULL_KW;

INTEGER: DIGIT+ | '0x' HEX_DIGIT+ | '0b' BIN_DIGIT+ | '0o' OCT_DIGIT+;

FLOAT: DIGIT+ '.' DIGIT+ ('e' ('+' | '-')? DIGIT+)?;

STRING: '"' (ESC | ~["\\])* '"';

CHAR: '\'' (ESC | ~['\\]) '\'';

QUANTUM_LITERAL: '|' ('0' | '1' | '+' | '-') '\\u27E9';

NANO_ANNOTATION: '@' IDENT ('(' ~[)]* ')')?;

MTS_LITERAL: 'mts' '[' ~[\\]]* ']';

IDENT: ALPHA (ALPHA | DIGIT)*;

LPAREN: '(';

COMMA: ',';

COLONCOLON: '::';

PLUS: '+';

EQ: '==';

ANDAND: '&&';

SHL: '<<';

PLUSEQ: '+=';

DOTDOT: '..';

QUESTION: '?';

LINE_COMMENT: '//' ~[\r\n]* -> skip;

BLOCK_COMMENT: '/*' .*? '*/' -> skip;

DOC_COMMENT: '///' ~[\r\n]*;

WS: [ \t\r\n]+ -> skip;


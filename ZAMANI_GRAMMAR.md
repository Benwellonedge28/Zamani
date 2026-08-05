# Grammar File: NIMBUS Grammar v3.0 — UNIVERSAL TRINITY EDITION — ANTLR4 format
# Inherits all 847 original NIMBUS rules PLUS absorbs ALL Zamani features PLUS adds Sankofa memory features PLUS integrates AI/Cognitive, OOP, Meta-Programming, HDL, Cloud, and Cybersecurity grammar rules.

===============================================================================
SECTION 1: CORE LANGUAGE FOUNDATION (NIMBUS Original Rules)
===============================================================================

// --- Program Structure ---
program: declaration* EOF;
declaration: moduleDecl | importDecl | exportDecl | functionDecl | structDecl
          | enumDecl | traitDecl | implDecl | typeAlias | constDecl
          | quantumCircuitDecl | nanoAgentDecl | languageDecl | effectDecl
          | classDecl | interfaceDecl | recordDecl | moduleDecl
          | globalUsing | attributeDecl | hdlModuleDecl | cloudDecl
          | agentDecl | cognitiveBlock | metaBlock | sankofaDecl;

// --- Module System ---
moduleDecl: 'module' IDENTIFIER ('::' IDENTIFIER)* '{' declaration* '}';
importDecl: 'import' modulePath ('as' IDENTIFIER)? ';';
exportDecl: 'export' IDENTIFIER ('to' IDENTIFIER)? ';';
modulePath: IDENTIFIER ('::' IDENTIFIER)*;
globalUsing: 'global' 'using' IDENTIFIER ';';
usingDirective: 'using' IDENTIFIER ';';

// --- Functions ---
functionDecl: modifiers? 'fn' IDENTIFIER typeParams? '(' parameterList? ')' 
             returnType? ('with' effectList)? block;
returnType: '->' typeExpr;
parameterList: parameter (',' parameter)*;
parameter: typeExpr IDENTIFIER ('=' defaultExpr)? | '...' typeExpr IDENTIFIER;
defaultExpr: expression;
modifiers: ('pub' | 'private' | 'protected' | 'static' | 'const' | 'async' 
           | 'unsafe' | 'inline' | 'override' | 'final' | 'abstract')*;
effectList: '[' effectName (',' effectName)* ']';
effectName: IDENTIFIER;
block: '{' statement* '}';

// --- Control Flow ---
statement: letStmt | constStmt | returnStmt | ifStmt | whileStmt 
         | forStmt | matchStmt | breakStmt | continueStmt 
         | exprStmt | block | unsafeBlock | throwStmt | tryCatchStmt
         | quantumStmt | nanoStmt | mtsStmt | sankofaStmt
         | learnStmt | rememberStmt | recallStmt | wisdomStmt
         | ancestorCall | consensusStmt | zamaniBlock | sasaBlock
         | invokeStmt | transcodeStmt | overrideStmt | pluginStmt
         | foreignFunctionCall | dataStmt | databaseOp | webService
         | cloudPlatform | container | devOpsTool | cognitiveStmt
         | assertStmt | retractStmt | adaptStmt | inferStmt | deduceStmt
         | selfAdjustStmt | effectHandleStmt | langStmt;

letStmt: 'let' 'mut'? IDENTIFIER (':' typeExpr)? '=' expression ';';
constStmt: 'const' IDENTIFIER ':' typeExpr '=' expression ';';
returnStmt: 'return' expression? ';';
ifStmt: 'if' expression block ('else' (ifStmt | block))?;
whileStmt: 'while' expression block;
forStmt: 'for' IDENTIFIER 'in' expression block;
matchStmt: 'match' expression '{' matchCase* '}';
matchCase: 'case' pattern ('when' expression)? '->' block;
breakStmt: 'break' ';';
continueStmt: 'continue' ';';
exprStmt: expression ';';
unsafeBlock: 'unsafe' '!'? '(' 'evas' ':' expression ')' block
          | 'unsafe' block;
throwStmt: 'throw' expression ';';
tryCatchStmt: 'try' block catchClause* finallyClause?;
catchClause: 'catch' '(' parameter ')' block;
finallyClause: 'finally' block;

// --- Expressions ---
expression: assignmentExpr;
assignmentExpr: logicalOrExpr (('=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>=') expression)?;
logicalOrExpr: logicalAndExpr ('||' logicalAndExpr)*;
logicalAndExpr: equalityExpr ('&&' equalityExpr)*;
equalityExpr: relationalExpr (('==' | '!=' | '===' | '!==') relationalExpr)*;
relationalExpr: bitwiseOrExpr (('<' | '>' | '<=' | '>=' | 'instanceof' | 'is' | 'has') bitwiseOrExpr)*;
bitwiseOrExpr: bitwiseXorExpr ('|' bitwiseXorExpr)*;
bitwiseXorExpr: bitwiseAndExpr ('^' bitwiseAndExpr)*;
bitwiseAndExpr: shiftExpr ('&' shiftExpr)*;
shiftExpr: additiveExpr (('<<' | '>>' | '>>>') additiveExpr)*;
additiveExpr: multiplicativeExpr (('+' | '-') multiplicativeExpr)*;
multiplicativeExpr: unaryExpr (('*' | '/' | '%') unaryExpr)*;
unaryExpr: ('!' | '-' | '+' | '~' | '&' | '*' | '++' | '--') unaryExpr
         | postfixExpr;
postfixExpr: primaryExpr ('.' IDENTIFIER | '[' expression ']' | '(' argumentList? ')')*;
primaryExpr: literal | IDENTIFIER | '(' expression ')' | arrayLit | tupleLit
           | structLit | mapLit | lambdaExpr | quantumExpr | nanoExpr
           | mtsExpr | recallExpr | consensusExpr | invokeExpr
           | newExpr | thisExpr | superExpr | memberAccess | indexExpr
           | interpolatedString | rawStringLit | withExpr;

argumentList: expression (',' expression)*;
arrayLit: '[' (expression (',' expression)*)? ']';
tupleLit: '(' (expression (',' expression)+) ')';
structLit: IDENTIFIER '{' (IDENTIFIER ':' expression (',' IDENTIFIER ':' expression)*)? '}';
mapLit: 'map' '{' (expression '=>' expression (',' expression '=>' expression)*)? '}';
lambdaExpr: '|' parameterList? '|' (typeExpr)? block
          | '|' parameterList? '|' '->' expression;

// --- Literals ---
literal: INTEGER | DECIMAL | STRING | CHAR | BOOLEAN | 'null'
       | quantumLit | nanoLit | mtsLit | interpolatedString | rawStringLit
       | utf8StringLit;
BOOLEAN: 'true' | 'false';
INTEGER: DIGIT+ | '0x' HEX_DIGIT+ | '0b' BIN_DIGIT+ | '0o' OCT_DIGIT+;
DECIMAL: DIGIT+ '.' DIGIT+ ('e' ('+' | '-')? DIGIT+)?;
STRING: '"' (ESC | ~["\])* '"';
CHAR: ''' (ESC | ~['\]) ''';
interpolatedString: '$' STRING;
rawStringLit: 'r' '#'* STRING;
utf8StringLit: 'u8' STRING;

// --- Types ---
typeExpr: baseType | genericType | arrayType | tupleType | functionType
        | dependentType | linearType | affineType | effectfulType
        | universeType | quantumType | nanoType | mtsType | sankofaType
        | cognitiveType | nullableType | refType | boxedType;
baseType: 'void' | 'int' | 'float' | 'bool' | 'string' | 'char' | 'bytes' 
        | 'i8' | 'i16' | 'i32' | 'i64' | 'i128' | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
        | 'f32' | 'f64' | 'usize' | 'isize' | IDENTIFIER;
genericType: baseType '<' typeExpr (',' typeExpr)* '>';
arrayType: typeExpr '[' expression? ']';
tupleType: '(' typeExpr (',' typeExpr)+ ')';
functionType: '(' typeExpr (',' typeExpr)* ')' '->' typeExpr;
nullableType: typeExpr '?';
refType: 'ref' typeExpr | '&' 'mut'? typeExpr;
boxedType: 'Box' '<' typeExpr '>';
typeParams: '<' typeParam (',' typeParam)* '>';
typeParam: IDENTIFIER (':' typeConstraint)?;
typeConstraint: typeExpr ('+' typeExpr)*;
typeAlias: 'type' IDENTIFIER typeParams? '=' typeExpr ';';
typeConstraintClause: 'where' IDENTIFIER ':' typeConstraint;

===============================================================================
SECTION 2: QUANTUM COMPUTING RULES
===============================================================================

// --- Quantum Declarations ---
quantumCircuitDecl: 'quantum' 'circuit' IDENTIFIER '(' parameterList? ')' block;
quantumStmt: quantumGate | quantumMeasure | quantumReset | quantumBarrier;
quantumGate: ('Hadamard' | 'CNOT' | 'PauliX' | 'PauliY' | 'PauliZ' | 'T' | 'S' | 'Swap') '(' IDENTIFIER (',' IDENTIFIER)* ')';
quantumMeasure: 'measure' IDENTIFIER ('->' IDENTIFIER)?;
quantumReset: 'reset' IDENTIFIER;
quantumBarrier: 'barrier' (IDENTIFIER (',' IDENTIFIER)*)?;

// --- Quantum Literals ---
quantumLit: '|' QUBIT_STATE 'rangle';  // Dirac notation: |0>, |1>, |+>, |->, |psi>
QUBIT_STATE: ('0' | '1' | '+' | '-' | IDENTIFIER);
quantumExpr: quantumLit | 'superpose' '(' expression (',' expression)* ')' 
           | 'entangle' '(' IDENTIFIER ',' IDENTIFIER ')';

// --- Quantum Types ---
quantumType: 'Qubit' | 'QReg' '[' expression ']' | 'Superposition' '<' typeExpr '>' 
          | 'Entangled' '<' typeExpr ',' typeExpr '>' | 'QMeasured' '<' typeExpr '>'
          | 'QArray' '<' typeExpr ',' expression '>';

===============================================================================
SECTION 3: NANO-AGENT COMPUTING RULES
===============================================================================

// --- Nano-Agent Declarations ---
nanoAgentDecl: 'nano' 'agent' IDENTIFIER '{' nanoAgentBody '}';
nanoAgentBody: nanoCapability* nanoBehavior* nanoProtocol*;
nanoCapability: 'capability' IDENTIFIER '(' parameterList? ')' block;
nanoBehavior: 'behavior' IDENTIFIER '(' parameterList? ')' block;
nanoProtocol: 'protocol' IDENTIFIER '{' protocolRule* '}';
protocolRule: 'on' IDENTIFIER '->' block;

// --- Nano Literals ---
nanoLit: '@atom' '(' ELEMENT ':' ORBITAL ')' | '@molecule' '(' FORMULA ')';
ELEMENT: IDENTIFIER;  // H, He, Li, etc.
ORBITAL: ('1s' | '2s' | '2p' | '3s' | '3p' | '3d' | '4s' | '4p' | '4d' | '4f');
FORMULA: IDENTIFIER (DIGIT* IDENTIFIER)*;
nanoExpr: nanoLit | 'assemble' '(' expression ')' | 'deploy' '(' expression ')';

// --- Nano Types ---
nanoType: 'Atom' '<' typeExpr '>' | 'Molecule' '<' typeExpr '>' 
        | 'NanoAgent' '<' typeExpr '>' | 'Archaeve' '<' typeExpr '>';

===============================================================================
SECTION 4: MULTI-TIMELINE SYSTEM (MTS) RULES
===============================================================================

// --- MTS Declarations ---
mtsDecl: 'mts' 'timeline' IDENTIFIER '{' mtsBody '}';
mtsBody: mtsSlice* mtsOperation*;
mtsSlice: 'slice' IDENTIFIER '[' INTEGER ']' block;
mtsOperation: 'fork' '(' IDENTIFIER ')' | 'merge' '(' IDENTIFIER ')' 
            | 'observe' '(' IDENTIFIER ')' | 'rewind' '(' INTEGER ')';

// --- MTS Literals ---
mtsLit: 'mts' '[' INTEGER ']';
mtsExpr: mtsLit | 'parallel' '(' block ')' | 'speculative' '(' block ')'
       | 'counterfactual' '(' expression ',' block ')';

// --- MTS Types ---
mtsType: 'MtsSlice' '<' expression '>';

===============================================================================
SECTION 5: SANKOFA MEMORY SYSTEM RULES
===============================================================================

// --- Sankofa Declarations ---
sankofaDecl: memoryDecl | wisdomDecl | historyDecl | consensusDecl | interMemoryDecl;
memoryDecl: 'remember' IDENTIFIER ':' typeExpr '=' expression ';';
wisdomDecl: 'wisdom' IDENTIFIER '{' wisdomBody '}';
wisdomBody: (premiseDecl | inferenceRule | wisdomStmt)*;
premiseDecl: 'premise' IDENTIFIER ':' typeExpr '=' expression ';';
inferenceRule: 'rule' IDENTIFIER '(' parameterList? ')' '->' block;
wisdomStmt: 'conclude' expression ';';

// --- Sankofa Statements ---
learnStmt: 'learn' 'from' expression ('with' 'weight' expression)? ';';
recallStmt: 'recall' '(' domainExpr ',' contextExpr ')' ';';
ancestorCall: 'ancestral' IDENTIFIER '(' argumentList? ')' ';';
consensusStmt: consensusExpr ';';
consensusExpr: 'consensus' '[' exprList ']' 'vote' expression;
zamaniBlock: 'zamani' '{' statement* '}';
sasaBlock: 'sasa' '{' statement* '}';
domainExpr: expression;
contextExpr: expression;
exprList: expression (',' expression)*;

// --- Sankofa Annotations ---
sankofaObserve: '@observe' '(' scope ')';
sankofaLivingDoc: '@living_doc' '(' updatePolicy ')';
sankofaTemporalLearn: '@temporal_learn' '(' span ')';
scope: IDENTIFIER ('.' IDENTIFIER)*;
updatePolicy: 'continuous' | 'manual' | 'versioned';
span: 'span' '(' expression ',' expression ')';

// --- Sankofa Types ---
historyType: 'History' '<' typeExpr ',' yearsExpr '>';
consensusType: 'ConsensusTrue' '<' typeExpr '>';
interMemoryType: 'InterMemory' '<' langId ',' typeExpr '>';
yearsExpr: expression;
langId: STRING;
sankofaType: historyType | consensusType | interMemoryType;

===============================================================================
SECTION 6: ADVANCED TYPE SYSTEM RULES
===============================================================================

// --- Dependent Types ---
dependentType: piType | sigmaType | identityType;
piType: (PI_SYMBOL | 'Pi') '(' IDENTIFIER ':' typeExpr ')' typeExpr;
sigmaType: (SIGMA_SYMBOL | 'Sigma') '(' IDENTIFIER ':' typeExpr ')' typeExpr;
identityType: 'Id' '(' typeExpr ',' expression ',' expression ')';
PI_SYMBOL: '\u03A0';   // Pi
SIGMA_SYMBOL: '\u03A3'; // Sigma

// --- Universe Hierarchy ---
universeType: 'Type_0' | 'Type_1' | 'Type_2' | 'Type_N' 
           | 'Kind' | 'Sort' | 'Prop';

// --- Linear / Affine Types ---
linearType: 'linear' typeExpr;
affineType: 'affine' typeExpr;

// --- Effectful Types ---
effectfulType: typeExpr 'with' 'effects' '{' effectName (',' effectName)* '}';

// --- Session Types ---
sessionType: 'session' '{' sessionOp* '}';
sessionOp: 'send' typeExpr | 'recv' typeExpr | 'offer' '{' sessionBranch* '}' 
        | 'choice' '{' sessionBranch* '}' | 'close';
sessionBranch: IDENTIFIER '->' sessionType;

// --- Cognitive and Biological Types ---
cognitiveType: 'CognitiveState' '<' typeExpr '>' | 'Consciousness' '<' typeExpr '>'
            | 'Biological' '<' typeExpr '>' | 'Neural' '<' typeExpr '>'
            | 'MemoryBank' '<' typeExpr '>' | 'AgentType';

// --- Agent Types ---
agentType: 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI';

===============================================================================
SECTION 7: ALGEBRAIC EFFECTS RULES
===============================================================================

// --- Effect Declarations ---
effectDecl: 'effect' IDENTIFIER typeParams? '(' parameterList? ')' returnType? ';';
effectHandleStmt: 'handle' expression '{' effectHandler* '}';
effectHandler: 'case' effectName '(' parameterList? ')' '->' block;

// --- Effect Expressions ---
withExpr: expression 'with' '[' effectList ']';

===============================================================================
SECTION 8: META-COMPILATION AND LANGUAGE DEFINITION RULES
===============================================================================

// --- Language Declaration ---
languageDecl: 'language' IDENTIFIER '=' STRING ('{' langBody '}')?;
langBody: grammarRule*;
grammarRule: IDENTIFIER ':' STRING ';';

// --- Meta-Programming Statements ---
invokeStmt: 'invoke' modulePath '(' argumentList? ')' ';';
transcodeStmt: 'transcode' IDENTIFIER '::' STRING 'to' IDENTIFIER ';';
overrideStmt: 'override' IDENTIFIER '::' IDENTIFIER '(' parameterList? ')' block;
langStmt: 'lang' IDENTIFIER block;
pluginStmt: 'plugin' IDENTIFIER '{' pluginDefinition* '}';
pluginDefinition: ('language' | 'transpiler') IDENTIFIER ';';
metaBlock: 'meta' '{' statement* '}';

// --- Meta-Object Protocol (MOP) ---
mopExpr: 'reflect' '(' expression ')' | 'introspect' '(' IDENTIFIER ')'
       | 'meta_eval' '(' expression ')' | 'quote' '{' statement* '}'
       | 'unquote' '(' expression ')' | 'splice' '(' expression ')';

// --- Macros ---
macroDecl: 'macro' IDENTIFIER '(' parameterList? ')' block;
macroCall: IDENTIFIER '!' '(' argumentList? ')';

===============================================================================
SECTION 9: OOP FEATURES (COMPREHENSIVE)
===============================================================================

// --- Class Definitions ---
classDecl: modifiers? 'class' IDENTIFIER typeParams? extendsClause? implementsClause? 
           permitsClause? '{' classBody '}';
extendsClause: 'extends' IDENTIFIER (',' IDENTIFIER)*;
implementsClause: 'implements' IDENTIFIER (',' IDENTIFIER)*;
permitsClause: 'permits' IDENTIFIER (',' IDENTIFIER)*;
classBody: classMember*;
classMember: propertyDef | methodDef | constructorDef | staticPropertyDef 
           | staticMethodDef | innerClassDef | eventDef | indexerDef
           | operatorOverload | delegateDef;

// --- Properties and Methods ---
propertyDef: modifiers? typeExpr IDENTIFIER ('=' expression)? ';';
methodDef: modifiers? 'fn' IDENTIFIER typeParams? '(' parameterList? ')' 
          returnType? ('with' effectList)? block;
constructorDef: 'fn' IDENTIFIER '(' parameterList? ')' block;
staticPropertyDef: 'static' modifiers? typeExpr IDENTIFIER ('=' expression)? ';';
staticMethodDef: 'static' modifiers? 'fn' IDENTIFIER '(' parameterList? ')' 
                returnType? block;

// --- Abstraction ---
abstractClassDecl: 'abstract' 'class' IDENTIFIER '{' classBody '}';
abstractMethodDef: 'abstract' 'fn' IDENTIFIER '(' parameterList? ')' returnType? ';';
abstractPropertyDef: 'abstract' modifiers? typeExpr IDENTIFIER ';';

// --- Inheritance ---
finalClassDecl: 'final' 'class' IDENTIFIER extendsClause? '{' classBody '}';
finalMethodDef: 'final' modifiers? 'fn' IDENTIFIER '(' parameterList? ')' 
               returnType? block;
sealedClassDecl: 'sealed' 'class' IDENTIFIER extendsClause? permitsClause? '{' classBody '}';
staticClassDecl: 'static' 'class' IDENTIFIER '{' classBody '}';
innerClassDecl: 'class' IDENTIFIER '{' classBody '}';
anonymousClassDecl: 'new' IDENTIFIER '(' argumentList? ')' '{' classBody '}';
partialClassDecl: 'partial' 'class' IDENTIFIER '{' classBody '}';

// --- Interfaces ---
interfaceDecl: 'interface' IDENTIFIER typeParams? extendsClause? '{' interfaceBody '}';
interfaceBody: interfaceMember*;
interfaceMember: methodDef | defaultInterfaceMethod | staticInterfaceMethod 
              | privateInterfaceMethod | asyncInterfaceMethod;
defaultInterfaceMethod: 'default' 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;
staticInterfaceMethod: 'static' 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;
privateInterfaceMethod: 'private' 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;
asyncInterfaceMethod: 'async' 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;
sealedInterfaceDecl: 'sealed' 'interface' IDENTIFIER '{' interfaceBody '}';
nestedInterfaceDecl: 'interface' IDENTIFIER '{' interfaceBody '}';

// --- Enums ---
enumDecl: 'enum' IDENTIFIER typeParams? (':' typeExpr)? '{' enumBody '}';
enumBody: enumMember*;
enumMember: IDENTIFIER ('(' argumentList? ')')?;

// --- Records ---
recordDecl: 'record' IDENTIFIER typeParams? '(' parameterList? ')' 
           extendsClause? implementsClause? '{' recordBody? '}';
recordBody: classMember*;
recordStructDecl: 'record' 'struct' IDENTIFIER '(' parameterList? ')' '{' recordBody? '}';

// --- Structs ---
structDecl: 'struct' IDENTIFIER typeParams? '{' structBody '}';
structBody: (modifiers? typeExpr IDENTIFIER ';')*;

// --- Traits ---
traitDecl: 'trait' IDENTIFIER typeParams? '{' traitBody '}';
traitBody: (methodDef | associatedType | constantDef)*;
associatedType: 'type' IDENTIFIER (':' typeConstraint)? ';';
constantDef: 'const' IDENTIFIER ':' typeExpr '=' expression ';';
implDecl: 'impl' typeParams? IDENTIFIER ('for' typeExpr)? '{' implBody* '}';
implBody: methodDef | constantDef | associatedType;

// --- OOP Expressions ---
newExpr: 'new' IDENTIFIER typeArgs? '(' argumentList? ')';
thisExpr: 'this';
superExpr: 'super' ('.' IDENTIFIER | '(' argumentList? ')')?;
memberAccess: expression '.' IDENTIFIER;
indexExpr: expression '[' expression ']';
typeCheck: expression 'instanceof' typeExpr;
typeCast: '(' typeExpr ')' expression;
withExprObj: expression 'with' '{' (IDENTIFIER ':' expression ';')* '}';

// --- Operator Overloading ---
operatorOverload: 'operator' OPERATOR '(' parameterList? ')' block;
explicitOperatorOverload: 'explicit' 'operator' typeExpr '(' parameterList? ')' block;
implicitOperatorOverload: 'implicit' 'operator' typeExpr '(' parameterList? ')' block;
extensionOperatorDef: 'extension' 'operator' OPERATOR '(' parameterList? ')' block;
userDefinedConversion: 'operator' typeExpr '(' parameterList? ')' block;

// --- Indexers ---
indexerDef: 'this' '[' parameterList? ']' '{' getter ';' setter? ';'}';
getter: 'get' block;
setter: 'set' block;
staticIndexerDef: 'static' 'this' '[' parameterList? ']' '{' getter ';' setter? ';'}';
asyncIndexerDef: 'async' 'this' '[' parameterList? ']' '{' getter ';' setter? ';'}';

// --- Events ---
eventDef: 'event' typeExpr IDENTIFIER ';';
extensionEventDef: 'extension' typeExpr IDENTIFIER '{' eventDef '}';

// --- Delegates ---
delegateDef: 'delegate' returnType? IDENTIFIER '(' parameterList? ')' ';';

// --- Extension Methods ---
extensionMethodDef: 'extension' 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;
extensionPropertyDef: 'extension' typeExpr IDENTIFIER '{' getter ';' setter? ';'}';
extensionIndexerDef: 'extension' typeExpr 'this' '[' parameterList? ']' '{' getter ';' setter? ';'}';

// --- Async Members ---
asyncMethodDef: 'async' modifiers? 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;
asyncPropertyDef: 'async' modifiers? typeExpr IDENTIFIER ';';
asyncMainMethod: 'async' 'fn' 'main' '(' parameterList? ')' returnType? block;
asyncStream: 'async' 'stream' typeExpr ';';
asyncIterator: 'async' 'iterator' typeExpr ';';
asyncDispose: 'async' 'dispose' block;

// --- Pattern Matching ---
pattern: IDENTIFIER | literal | tuplePattern | arrayPattern | listPattern 
       | slicePattern | wildcardPattern | orPattern | typePattern;
tuplePattern: '(' pattern (',' pattern)* ')';
arrayPattern: '[' pattern (',' pattern)* ']';
listPattern: '[' pattern (',' pattern)* ']' ('...' pattern)?;
slicePattern: '[' pattern (',' pattern)* '...' pattern ']';
wildcardPattern: '_';
orPattern: pattern '|' pattern;
typePattern: IDENTIFIER ':' typeExpr;
patternGuard: 'when' expression;

// --- Named/Optional Arguments ---
namedArgument: IDENTIFIER '=' expression;
optionalParameter: typeExpr IDENTIFIER '=' defaultExpr;

// --- Expression-Bodied Members ---
exprBodiedMember: '=>' expression;

// --- Tuples with Named Elements ---
namedTupleElement: IDENTIFIER ':' typeExpr;

// --- Deconstruction ---
deconstructionStmt: '(' variableList ')' '=' expression ';';
variableList: IDENTIFIER (',' IDENTIFIER)*;

// --- Init-Only Properties ---
initOnlyProperty: 'init' modifiers? typeExpr IDENTIFIER ';';
initOnlySetter: 'init' '{' setter '}';

// --- Required Properties ---
requiredProperty: 'required' modifiers? typeExpr IDENTIFIER ';';

// --- File-Scoped Types ---
fileScopedType: 'file' 'class' IDENTIFIER '{' classBody '}';

// --- Member Hiding ---
memberHiding: 'new' modifiers? typeExpr IDENTIFIER ';';

// --- Ref Returns ---
refReturn: 'ref' returnType;

// --- Tuple Equality ---
tupleEquality: tupleLit '==' tupleLit;
tupleInequality: tupleLit '!=' tupleLit;

// --- Global Alias ---
globalAlias: 'global' 'alias' IDENTIFIER '=' typeExpr ';';

// --- Multidimensional and Jagged Arrays ---
multidimArray: typeExpr '[' ','+ ']';
jaggedArray: typeExpr '[' ']' '[' ']';

// --- Covariant / Contravariant ---
covariantTypeParam: 'out' typeParam;
contravariantTypeParam: 'in' typeParam;

// --- Local Static Variables ---
localStaticVar: 'static' typeExpr IDENTIFIER '=' expression ';';

// --- Local Functions ---
localFunctionDef: 'fn' IDENTIFIER '(' parameterList? ')' returnType? block;

// --- Varargs ---
varargParameter: '...' typeExpr IDENTIFIER;

// --- Type-Safe Enums ---
typeSafeEnum: 'enum' IDENTIFIER ':' typeExpr '{' enumBody '}';

// --- Primary Constructors ---
primaryConstructor: '(' parameterList ')';

===============================================================================
SECTION 10: AI / COGNITIVE GRAMMAR RULES
===============================================================================

// --- AI Cognition Keywords ---
inferStmt: 'infer' expression 'from' exprList ';';
deduceStmt: 'deduce' expression ('via' expression)? ';';
assertStmt: 'assert' expression ';';
retractStmt: 'retract' expression ';';
adaptStmt: 'adapt' expression ('to' expression)? ';';
selfAdjustStmt: 'self_adjust' '(' expression ')' block;

// --- Cognitive Architecture Blocks ---
cognitiveBlock: cognitiveDecl '{' statement* '}';
cognitiveDecl: 'cognitive_architecture' IDENTIFIER;

// --- AI/AGI Domain Blocks ---
aiDomainBlock: aiDomain IDENTIFIER '{' statement* '}';
aiDomain: 'machine_learning' | 'deep_learning' | 'neural_network' 
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
        | 'advanced_machine_learning_for_time_series_analysis';

// --- Agent System ---
agentDecl: 'agent' IDENTIFIER (':' agentType)? '{' agentBody '}';
agentBody: (agentCapability | agentBehavior | agentGoal | agentConstraint)*;
agentCapability: 'capability' IDENTIFIER '(' parameterList? ')' block;
agentBehavior: 'behavior' IDENTIFIER '(' parameterList? ')' block;
agentGoal: 'goal' expression ';';
agentConstraint: 'constraint' expression ';';

// --- Human-AGI Teaming ---
humanAgiCollaboration: 'human_ai_collaboration' IDENTIFIER '{' statement* '}';

===============================================================================
SECTION 11: SAFETY / SECURITY / ETHICS ATTRIBUTES
===============================================================================

// --- EVAS Certification ---
evasCert: 'unsafe' '!' '(' 'evas' ':' expression ')' block;

// --- Safety Attributes ---
safetyAttr: '#safety' '(' attrArgs ')';
securityAttr: '#security' '(' attrArgs ')';
ethicsAttr: '#ethics' '(' attrArgs ')';
governanceAttr: '#governance' '(' attrArgs ')';
attrArgs: attrArg (',' attrArg)*;
attrArg: IDENTIFIER '=' expression;

// --- Attribute Declaration ---
attributeDecl: '@' IDENTIFIER '(' annotationArgs? ')';
annotationArgs: annotationArg (',' annotationArg)*;
annotationArg: IDENTIFIER '=' expression;
genericAttribute: '[' typeExpr ']';

// --- Compliance Attributes ---
complianceAttr: '#compliance' '(' 'standard' '=' STRING ',' 'certified_by' '=' STRING ')';

===============================================================================
SECTION 12: HDL (HARDWARE DESCRIPTION LANGUAGE) RULES
===============================================================================

// --- HDL Module Declaration ---
hdlModuleDecl: 'hdl' 'module' IDENTIFIER '{' hdlBody '}';
hdlBody: hdlPort* hdlComponent* hdlLogicGate* hdlSignal* hdlAssignment*;
hdlPort: 'port' hdlPortDirection IDENTIFIER ':' hdlPortType;
hdlPortDirection: 'input' | 'output' | 'inout';
hdlPortType: 'wire' | 'reg' | 'logic' | 'quantum' | 'nano' | typeExpr;
hdlComponent: 'component' IDENTIFIER '{' hdlPort* '}';
hdlLogicGate: 'gate' hdlLogicGateType '(' IDENTIFIER (',' IDENTIFIER)* ')';
hdlLogicGateType: 'AND' | 'OR' | 'NOT' | 'XOR' | 'NAND' | 'NOR' | 'MUX' | 'DEMUX';
quantumGateType: 'Hadamard' | 'CNOT' | 'Toffoli' | 'Fredkin' | 'PauliX' | 'PauliY' | 'PauliZ';
hdlSignal: 'signal' IDENTIFIER ':' hdlPortType;
hdlAssignment: 'assign' IDENTIFIER '=' hdlExpression ';';
hdlExpression: IDENTIFIER | hdlLogicGate | '(' hdlExpression ')' 
            | hdlExpression ('&' | '|' | '^' | '~') hdlExpression;

// --- External HDL Import ---
externalHdlImport: 'import_hdl' '(' STRING ',' IDENTIFIER ')';
externalHdlLink: 'link_hdl' '(' IDENTIFIER ',' IDENTIFIER ')';

===============================================================================
SECTION 13: DISTRIBUTED COMPUTING RULES
===============================================================================

// --- Distributed Declarations ---
distributedDecl: 'distributed' 'node' IDENTIFIER '{' distributedBody '}';
distributedBody: (serviceHandle | remoteCall | distributedOperation)*;
serviceHandle: 'service' IDENTIFIER 'at' STRING;
remoteCall: 'remote' IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';';
distributedOperation: 'teleport' '(' IDENTIFIER ',' STRING ')'   // Quantum teleportation
                   | 'migrate' '(' IDENTIFIER ',' STRING ')'     // Nano-agent swarm migration
                   | 'dsm' '(' IDENTIFIER ',' STRING ')'          // Distributed shared memory
                   | 'dmts' '(' IDENTIFIER ',' STRING ')';       // Distributed MTS

===============================================================================
SECTION 14: CLOUD / NETWORK COMPUTING RULES
===============================================================================

// --- Cloud Platform Integration ---
cloudDecl: cloudPlatform;
cloudPlatform: 'AWS' '::' IDENTIFIER '(' argumentList? ')' ';'
            | 'Azure' '::' IDENTIFIER '(' argumentList? ')' ';'
            | 'GCP' '::' IDENTIFIER '(' argumentList? ')' ';';

// --- Container and DevOps ---
container: 'Docker' '::' IDENTIFIER '(' argumentList? ')' ';';
devOpsTool: 'Jenkins' '::' IDENTIFIER '(' argumentList? ')' ';';

// --- Database Operations ---
databaseOp: 'Database' '::' IDENTIFIER '(' argumentList? ')' ';';

// --- Web Services ---
webService: 'HTTP' '::' IDENTIFIER '(' argumentList? ')' ';';

// --- Data Serialization ---
dataStmt: ('serialize' | 'deserialize') expression ('to' | 'from') dataFormat ';';
dataFormat: 'json' | 'xml' | 'messagepack' | 'protobuf' | 'cbor';

// --- Streaming Data ---
streamingData: 'stream' expression 'pipe' expression ';';

// --- Foreign Function Interface ---
foreignFunctionCall: 'foreign' IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';';

===============================================================================
SECTION 15: ADVANCED CRYPTOGRAPHY RULES
===============================================================================

// --- Encryption Expressions ---
cryptoExpr: 'encrypt' '(' expression ',' cryptoAlgo ')'
         | 'decrypt' '(' expression ',' cryptoAlgo ')'
         | 'encrypt_homomorphic' '(' expression ',' cryptoKey ')'
         | 'decrypt_homomorphic' '(' expression ',' cryptoKey ')'
         | 'encrypt_layered' '(' expression ',' cryptoAlgoList ')'
         | 'encrypt_quantum_safe' '(' expression ',' cryptoKey ')';
cryptoAlgo: 'AES256' | 'ChaCha20' | 'RSA4096' | 'Kyber' | 'Dilithium' | 'SPHINCS+';
cryptoAlgoList: '[' cryptoAlgo (',' cryptoAlgo)* ']';
cryptoKey: 'key' IDENTIFIER | 'generate_key' '(' typeExpr ')';

// --- Zero-Knowledge Proofs ---
zkExpr: 'generate_zk_proof' '(' expression ',' expression ')'
      | 'verify_zk_proof' '(' expression ',' expression ')';

// --- Secure Multi-Party Computation ---
smcExpr: 'secure_multi_party_compute' '(' exprList ')';

// --- Key Management ---
keyManagement: 'request_key' '(' typeExpr ')' | 'rotate_key' '(' IDENTIFIER ')'
            | 'revoke_key' '(' IDENTIFIER ')';

===============================================================================
SECTION 16: ON-DEVICE AI / EDGE COMPUTING RULES
===============================================================================

// --- On-Device Agent Declaration ---
onDeviceAgentDecl: 'on_device' 'agent' IDENTIFIER '{' onDeviceAgentBody '}';
onDeviceAgentBody: (agentCapability | agentBehavior | deviceConstraint 
                  | selfPreservationProtocol | offlineProtocol)*;
deviceConstraint: 'requires' (deviceSpec (',' deviceSpec)*) ';';
deviceSpec: 'memory' '>=' expression | 'power' '>=' expression 
         | 'storage' '>=' expression | 'processor' '=' typeExpr;
selfPreservationProtocol: 'self_preserve' block;
offlineProtocol: 'offline' block;

===============================================================================
SECTION 17: SELF-EVOLVING / AUTONOMOUS FEATURES
===============================================================================

// --- Self-Evolution ---
selfEvolveDecl: 'self_evolve' '{' evolveBody '}';
evolveBody: (monitorRule | optimizeRule | patchRule | verifyRule)*;
monitorRule: 'monitor' expression '->' block;
optimizeRule: 'optimize' expression '->' block;
patchRule: 'patch' expression '->' block;
verifyRule: 'verify' expression '->' block;

// --- Autonomous Code Generation ---
autoCodeGen: 'autonomously_generate' '(' expression ')' '->' block;
autoOptimize: 'autonomously_optimize' '(' expression ')' '->' block;
autoVerify: 'autonomously_verify' '(' expression ')' '->' block;

===============================================================================
SECTION 18: OPTIMIZATION STRATEGY RULES
===============================================================================

// --- Optimization Passes ---
optPassDecl: 'optimization' 'pass' IDENTIFIER '{' optPassBody '}';
optPassBody: ('target' optTarget ';')* ('strategy' optStrategy ';')* block;
optTarget: 'classical' | 'quantum' | 'nano' | 'neuromorphic' | 'GPU' | 'FPGA' 
        | 'SIMD' | 'WASM' | 'USSD' | 'edge' | 'stellar';
optStrategy: 'DCE' | 'CSE' | 'inlining' | 'loop_unroll' | 'constant_fold'
           | 'quantum_gate_opt' | 'nano_atp_efficiency' | 'ai_quantization'
           | 'thermal_throttle_prevention' | 'simd_vectorization'
           | 'gpu_offloading' | 'quantum_error_correction';

===============================================================================
SECTION 19: COMPILATION TARGET RULES
===============================================================================

// --- Target Platform Specification ---
targetPlatform: 'target' platformSpec;
platformSpec: 'x86_64' | 'ARM64' | 'RISC-V' | 'WASM' | 'LLVM_IR' | 'bare_metal'
            | 'Android' | 'iOS' | 'cloud' | 'IoT' | 'USSD' | 'FPGA' 
            | 'quantum' | 'nano' | 'neuromorphic' | 'stellar'
            | 'Tariro_Runtime' | 'Z_MMP';

===============================================================================
SECTION 20: RUNTIME FEATURES (POCO-REAF)
===============================================================================

// --- Runtime Declarations ---
runtimeDecl: 'runtime' 'configure' '{' runtimeConfig* '}';
runtimeConfig: 'gc' '=' ('enabled' | 'disabled' | 'hybrid') ';'
            | 'self_heal' '=' BOOLEAN ';'
            | 'quantum_sim' '=' BOOLEAN ';'
            | 'nano_orchestration' '=' BOOLEAN ';'
            | 'effect_dispatch' '=' ('eager' | 'lazy' | 'batched') ';'
            | 'scheduler' '=' ('preemptive' | 'cooperative' | 'quantum') ';';

// --- Spawn / Channel / Select ---
spawnExpr: 'spawn' block;
channelExpr: 'channel' '<' typeExpr '>';
selectStmt: 'select' '{' selectCase* '}';
selectCase: 'case' expression '->' block;

===============================================================================
SECTION 21: KEYWORDS (140 TOTAL)
===============================================================================

// --- Core Keywords (95 NIMBUS Original) ---
// fn, let, const, if, else, return, while, for, in, break, continue,
// match, case, struct, enum, trait, impl, pub, use, mod, type,
// true, false, null, unsafe, async, await, channel, spawn, select,
// import, export, module, this, super, new, operator, event,
// delegate, static, const, abstract, final, override, virtual,
// extends, implements, permits, sealed, partial, record,
// get, set, init, required, file, global, where, as, is, has,
// ref, mut, move, box, external, throw, try, catch, finally,
// typeof, sizeof, alignof, offsetof, instanceof, when,
// private, protected, public, inline, out, in, default,
// void, int, float, bool, string, char, bytes,
// i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, usize, isize

// --- Zamani-Specific Keywords (25) ---
// quantum, quantum_circuit, nano, nano_agent, mts,
// effect, handle, language, type (universe), kind, sort, prop,
// linear, affine, with, invoke, transcode, override,
// plugin, lang, meta, macro, reflect, introspect, assert

// --- Sankofa-Specific Keywords (20) ---
// remember, recall, learn, wisdom, ancestral, consensus, observe,
// zamani, sasa, living_doc, temporal_learn, History, ConsensusTrue,
// InterMemory, Superposition, Entangled, QMeasured, Archaeve,
// infer, deduce, adapt, retract

===============================================================================
SECTION 22: OPERATORS AND SPECIAL SYMBOLS
===============================================================================

// --- Operators ---
OPERATOR: '+' | '-' | '*' | '/' | '%' | '**'
       | '==' | '!=' | '===' | '!=='
       | '<' | '>' | '<=' | '>=' | '<<' | '>>' | '>>>'
       | '&' | '|' | '^' | '~' | '&&' | '||' | '!'
       | '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^='
       | '<<=' | '>>=' | '=>' | '->' | '..' | '...'
       | '?' | '|' | 'rangle' | '@' | '#';

// --- Special Symbols ---
LBRACE: '{'; RBRACE: '}';
LBRACKET: '['; RBRACKET: ']';
LPAREN: '('; RPAREN: ')';
SEMICOLON: ';'; COLON: ':'; COMMA: ',';
DOT: '.'; DOUBLE_COLON: '::'; ARROW: '->'; FAT_ARROW: '=>';
PIPE: '|'; BANG: '!'; AT: '@'; HASH: '#';

===============================================================================
SECTION 23: COMMENTS AND WHITESPACE
===============================================================================

// --- Comments ---
LINE_COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
DOC_COMMENT: '///' ~[\r\n]* -> skip;

// --- Whitespace ---
WS: [ \t\r\n\u000C]+ -> skip;

// --- Unicode Support ---
IDENTIFIER: XID_START XID_CONTINUE*;
XID_START: [\p{XID_Start}];
XID_CONTINUE: [\p{XID_Continue}];

===============================================================================
TOTAL RULES: ~1,400 (NIMBUS v3.0 Universal Trinity Edition)
KEYWORDS: 140 total (95 original + 25 Zamani + 20 Sankofa)
ALL PARADIGMS: 80+ (original 71 + 9 Sankofa + additional AI/Cognitive/Edge paradigms)
TARGET PLATFORMS: x86_64, ARM64, RISC-V, WASM, LLVM IR, bare metal, Android, iOS,
                 cloud, IoT, USSD, FPGA, quantum, nano, neuromorphic, stellar, Z-MMP,
                 Tariro Runtime
COMPILER: ZUTC (Zamani Unified Toolchain Compiler) - preserved
FILE EXTENSION: .zn (preserved)
RUNTIME: POCO-REAF (Persistent, Omni-Cognitive, Reactive, Event-driven, Adaptive, Self-healing)
===============================================================================

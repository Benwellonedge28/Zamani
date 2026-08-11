// Generated from /home/ubuntu/Zamani/grammar/Zamani.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class ZamaniParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		T__0=1, T__1=2, T__2=3, T__3=4, T__4=5, T__5=6, T__6=7, T__7=8, T__8=9, 
		T__9=10, T__10=11, T__11=12, T__12=13, T__13=14, T__14=15, T__15=16, T__16=17, 
		T__17=18, T__18=19, T__19=20, T__20=21, T__21=22, T__22=23, T__23=24, 
		T__24=25, T__25=26, T__26=27, T__27=28, T__28=29, T__29=30, T__30=31, 
		T__31=32, T__32=33, T__33=34, T__34=35, T__35=36, T__36=37, T__37=38, 
		T__38=39, T__39=40, T__40=41, T__41=42, T__42=43, T__43=44, T__44=45, 
		T__45=46, T__46=47, T__47=48, T__48=49, T__49=50, T__50=51, T__51=52, 
		T__52=53, T__53=54, T__54=55, T__55=56, T__56=57, T__57=58, T__58=59, 
		T__59=60, T__60=61, T__61=62, T__62=63, T__63=64, T__64=65, T__65=66, 
		T__66=67, T__67=68, T__68=69, T__69=70, T__70=71, T__71=72, T__72=73, 
		T__73=74, T__74=75, T__75=76, T__76=77, T__77=78, T__78=79, T__79=80, 
		T__80=81, T__81=82, T__82=83, T__83=84, T__84=85, T__85=86, T__86=87, 
		T__87=88, T__88=89, T__89=90, T__90=91, T__91=92, T__92=93, T__93=94, 
		T__94=95, T__95=96, T__96=97, T__97=98, T__98=99, T__99=100, T__100=101, 
		T__101=102, T__102=103, T__103=104, T__104=105, T__105=106, T__106=107, 
		T__107=108, T__108=109, T__109=110, T__110=111, T__111=112, T__112=113, 
		T__113=114, T__114=115, T__115=116, T__116=117, T__117=118, T__118=119, 
		T__119=120, T__120=121, T__121=122, T__122=123, T__123=124, T__124=125, 
		T__125=126, T__126=127, T__127=128, T__128=129, T__129=130, T__130=131, 
		T__131=132, T__132=133, T__133=134, T__134=135, T__135=136, T__136=137, 
		T__137=138, T__138=139, T__139=140, T__140=141, T__141=142, T__142=143, 
		T__143=144, T__144=145, T__145=146, T__146=147, T__147=148, T__148=149, 
		T__149=150, T__150=151, T__151=152, T__152=153, T__153=154, T__154=155, 
		T__155=156, T__156=157, T__157=158, T__158=159, T__159=160, T__160=161, 
		T__161=162, T__162=163, T__163=164, T__164=165, T__165=166, T__166=167, 
		T__167=168, T__168=169, T__169=170, T__170=171, T__171=172, T__172=173, 
		T__173=174, T__174=175, T__175=176, T__176=177, T__177=178, T__178=179, 
		T__179=180, T__180=181, T__181=182, T__182=183, T__183=184, T__184=185, 
		T__185=186, T__186=187, T__187=188, T__188=189, T__189=190, T__190=191, 
		T__191=192, T__192=193, T__193=194, T__194=195, T__195=196, T__196=197, 
		T__197=198, T__198=199, T__199=200, T__200=201, T__201=202, T__202=203, 
		T__203=204, T__204=205, T__205=206, T__206=207, T__207=208, T__208=209, 
		T__209=210, T__210=211, T__211=212, T__212=213, T__213=214, T__214=215, 
		T__215=216, T__216=217, T__217=218, T__218=219, T__219=220, T__220=221, 
		T__221=222, T__222=223, ORBITAL=224, FORMULA=225, OPERATOR=226, MODULE=227, 
		IMPORT=228, EXPORT=229, AS=230, USE=231, GLOBAL=232, USING=233, FN=234, 
		PUB=235, PRIVATE=236, PROTECTED=237, STATIC=238, CONST=239, ASYNC=240, 
		UNSAFE=241, INLINE=242, OVERRIDE=243, FINAL=244, ABSTRACT=245, VIRTUAL=246, 
		SEALED=247, PARTIAL=248, FILE=249, REQUIRED=250, INIT=251, RETURN=252, 
		BREAK=253, CONTINUE=254, WHILE=255, FOR=256, IN=257, LOOP=258, IF=259, 
		ELSE=260, MATCH=261, CASE=262, WHEN=263, THROW=264, TRY=265, CATCH=266, 
		FINALLY=267, LET=268, VAR=269, TYPE=270, STRUCT=271, ENUM=272, TRAIT=273, 
		IMPL=274, CLASS=275, INTERFACE=276, RECORD=277, QUANTUM=278, CIRCUIT=279, 
		MEASURE=280, RESET=281, BARRIER=282, THIS=283, SELF_LOWER=284, SELF_UPPER=285, 
		INT_KW=286, FLOAT_KW=287, BOOL_KW=288, STR_KW=289, STRING_KW=290, CHAR_KW=291, 
		VOID=292, NANO=293, AGENT=294, EFFECT=295, HANDLE=296, REMEMBER=297, RECALL=298, 
		LEARN=299, INFER=300, WISDOM=301, ZAMANI=302, SASA=303, ANCESTOR=304, 
		LINEAR=305, AFFINE=306, LANGUAGE=307, MTS_KW=308, LEN=309, PRINT=310, 
		PRINTLN=311, ASSERT=312, PANIC=313, TRUE=314, FALSE=315, NIL_KW=316, NULL_KW=317, 
		BOOLEAN=318, NIL=319, INTEGER=320, FLOAT=321, STRING=322, CHAR=323, QUANTUM_LITERAL=324, 
		NANO_ANNOTATION=325, MTS_LITERAL=326, IDENT=327, LPAREN=328, RPAREN=329, 
		LBRACE=330, RBRACE=331, LBRACK=332, RBRACK=333, COMMA=334, DOT=335, SEMI=336, 
		COLON=337, ARROW=338, FATARROW=339, COLONCOLON=340, TILDE=341, HASH=342, 
		AT=343, BANG=344, PLUS=345, MINUS=346, STAR=347, SLASH=348, PERCENT=349, 
		ASSIGN=350, EQ=351, NEQ=352, LT=353, GT=354, LE=355, GE=356, ANDAND=357, 
		OROR=358, AMP=359, PIPE=360, CARET=361, SHL=362, SHR=363, SHRU=364, PLUSEQ=365, 
		MINUSEQ=366, STAREQ=367, SLASHEQ=368, DOTDOT=369, DOTDOTEQ=370, QUESTION=371, 
		DOLLAR=372, LINE_COMMENT=373, BLOCK_COMMENT=374, DOC_COMMENT=375, WS=376;
	public static final int
		RULE_program = 0, RULE_declaration = 1, RULE_moduleDecl = 2, RULE_importDecl = 3, 
		RULE_exportDecl = 4, RULE_modulePath = 5, RULE_useStmt = 6, RULE_usePath = 7, 
		RULE_segment = 8, RULE_globalUsing = 9, RULE_functionDecl = 10, RULE_params = 11, 
		RULE_param = 12, RULE_modifiers = 13, RULE_modifier = 14, RULE_returnStmt = 15, 
		RULE_breakStmt = 16, RULE_continueStmt = 17, RULE_whileStmt = 18, RULE_forStmt = 19, 
		RULE_loopExpr = 20, RULE_ifExpr = 21, RULE_matchStmt = 22, RULE_matchExpr = 23, 
		RULE_matchCase = 24, RULE_pattern = 25, RULE_unsafeBlock = 26, RULE_throwStmt = 27, 
		RULE_tryCatchStmt = 28, RULE_catchClause = 29, RULE_finallyClause = 30, 
		RULE_blockExpr = 31, RULE_letStmt = 32, RULE_constStmt = 33, RULE_expression = 34, 
		RULE_assignmentExpr = 35, RULE_assignOp = 36, RULE_rangeExpr = 37, RULE_logicalOrExpr = 38, 
		RULE_logicalAndExpr = 39, RULE_bitOrExpr = 40, RULE_bitXorExpr = 41, RULE_bitAndExpr = 42, 
		RULE_equalityExpr = 43, RULE_comparisonExpr = 44, RULE_shiftExpr = 45, 
		RULE_sumExpr = 46, RULE_productExpr = 47, RULE_castExpr = 48, RULE_prefixExpr = 49, 
		RULE_postfixExpr = 50, RULE_postfixOp = 51, RULE_args = 52, RULE_namedArgument = 53, 
		RULE_primaryExpr = 54, RULE_structLiteralTail = 55, RULE_recallExpr = 56, 
		RULE_learnExpr = 57, RULE_performExpr = 58, RULE_zamaniExpr = 59, RULE_sasaExpr = 60, 
		RULE_quantumOpExpr = 61, RULE_nanoExpr = 62, RULE_mtsExpr = 63, RULE_consensusExpr = 64, 
		RULE_ancestorCall = 65, RULE_mopExpr = 66, RULE_macroCall = 67, RULE_exprList = 68, 
		RULE_expressionStmt = 69, RULE_typeExpr = 70, RULE_baseType = 71, RULE_typeParams = 72, 
		RULE_typeParam = 73, RULE_typeArgs = 74, RULE_structDecl = 75, RULE_structField = 76, 
		RULE_enumDecl = 77, RULE_enumVariant = 78, RULE_traitDecl = 79, RULE_traitItem = 80, 
		RULE_implDecl = 81, RULE_implItem = 82, RULE_typeAliasDecl = 83, RULE_constDecl = 84, 
		RULE_classDecl = 85, RULE_classItem = 86, RULE_constructorDecl = 87, RULE_destructorDecl = 88, 
		RULE_interfaceDecl = 89, RULE_recordDecl = 90, RULE_quantumCircuitDecl = 91, 
		RULE_nanoAgentDecl = 92, RULE_languageDecl = 93, RULE_effectDecl = 94, 
		RULE_effectOp = 95, RULE_effectList = 96, RULE_effectName = 97, RULE_mtsDecl = 98, 
		RULE_sankofaDecl = 99, RULE_agentDecl = 100, RULE_cognitiveBlock = 101, 
		RULE_metaBlock = 102, RULE_hdlModuleDecl = 103, RULE_cloudDecl = 104, 
		RULE_distributedDecl = 105, RULE_onDeviceAgentDecl = 106, RULE_selfEvolveDecl = 107, 
		RULE_optPassDecl = 108, RULE_targetPlatform = 109, RULE_runtimeDecl = 110, 
		RULE_actorDecl = 111, RULE_aiSystemDecl = 112, RULE_agiSystemDecl = 113, 
		RULE_asiSystemDecl = 114, RULE_aesiSystemDecl = 115, RULE_asesiSystemDecl = 116, 
		RULE_adminInterfaceDecl = 117, RULE_paymentGatewayDecl = 118, RULE_userFeedbackDecl = 119, 
		RULE_copyrightNoticeDecl = 120, RULE_tailorMadeFeatureDecl = 121, RULE_programOnceDecl = 122, 
		RULE_maliciousIdeaDetection = 123, RULE_userBlockingDecl = 124, RULE_legalActionDecl = 125, 
		RULE_sandboxDecl = 126, RULE_omniversalSimulationDecl = 127, RULE_omniversalCodeSynthDecl = 128, 
		RULE_omniversalDeployDecl = 129, RULE_omniversalAlignmentDecl = 130, RULE_omniversalContainmentDecl = 131, 
		RULE_omniversalTrustDecl = 132, RULE_omniversalKnowledgeDecl = 133, RULE_omniversalGenerativeDecl = 134, 
		RULE_omniversalSovereigntyDecl = 135, RULE_omniversalGoalDecl = 136, RULE_omniversalBioNanoDecl = 137, 
		RULE_omniversalRealityDecl = 138, RULE_omniversalNlpDecl = 139, RULE_chatArchitectDecl = 140, 
		RULE_greenComputingAttr = 141, RULE_thermalOptDecl = 142, RULE_resourceConserveDecl = 143, 
		RULE_selfDiscoverDecl = 144, RULE_developerAnalyticsDecl = 145, RULE_licenseTrackingDecl = 146, 
		RULE_deploymentDecl = 147, RULE_versionReleaseDecl = 148, RULE_lspServerDecl = 149, 
		RULE_typeClassDecl = 150, RULE_typeClassInstance = 151, RULE_higherKindedTypeDecl = 152, 
		RULE_selfAdjustDecl = 153, RULE_selfVersioningDecl = 154, RULE_extensionMethodDecl = 155, 
		RULE_extensionPropertyDecl = 156, RULE_extensionIndexerDecl = 157, RULE_extensionOperatorDecl = 158, 
		RULE_macroDecl = 159, RULE_domainSpecificLanguageDecl = 160, RULE_aspectDecl = 161, 
		RULE_typeProviderDecl = 162, RULE_dataParallelismDecl = 163, RULE_concurrentDataStructureDecl = 164, 
		RULE_messageHandlerDecl = 165, RULE_musicDecl = 166, RULE_roboticsDecl = 167, 
		RULE_deepLearningDecl = 168, RULE_graphicsDecl = 169, RULE_videoDecl = 170, 
		RULE_tensorDecl = 171, RULE_matrixDecl = 172, RULE_vectorDecl = 173, RULE_mlModelDecl = 174, 
		RULE_quantumMlBlock = 175, RULE_explainableRlBlock = 176, RULE_explainableDeepLearningBlock = 177, 
		RULE_knowledgeGraphBlock = 178, RULE_probabilisticGraphicalModelBlock = 179, 
		RULE_transferLearningBlock = 180, RULE_multiAgentBlock = 181, RULE_autonomousSystemBlock = 182, 
		RULE_graphModelingBlock = 183, RULE_advancedNlpBlock = 184, RULE_cognitiveArchitectureBlock = 185, 
		RULE_aiForBusinessBlock = 186, RULE_vrArInteractionBlock = 187, RULE_imageVideoAnalysisBlock = 188, 
		RULE_fileScopedType = 189, RULE_hybridDef = 190, RULE_interfaceDef = 191, 
		RULE_agentCapability = 192, RULE_agentBehavior = 193, RULE_explainStmt = 194, 
		RULE_transparentStmt = 195, RULE_asiCapabilityDef = 196, RULE_aesiCapabilityDef = 197, 
		RULE_asesiCapabilityDef = 198, RULE_docComment = 199, RULE_attributeDecl = 200, 
		RULE_ident = 201, RULE_statement = 202, RULE_identList = 203, RULE_whereClause = 204, 
		RULE_literal = 205, RULE_quantumLit = 206, RULE_nanoLit = 207, RULE_mtsLit = 208, 
		RULE_rawStringLit = 209, RULE_utf8StringLit = 210, RULE_interpolatedString = 211, 
		RULE_piType = 212, RULE_sigmaType = 213, RULE_identityType = 214, RULE_sessionOp = 215, 
		RULE_sessionBranch = 216, RULE_quantumType = 217, RULE_nanoType = 218, 
		RULE_mtsType = 219, RULE_sankofaType = 220, RULE_cognitiveType = 221;
	private static String[] makeRuleNames() {
		return new String[] {
			"program", "declaration", "moduleDecl", "importDecl", "exportDecl", "modulePath", 
			"useStmt", "usePath", "segment", "globalUsing", "functionDecl", "params", 
			"param", "modifiers", "modifier", "returnStmt", "breakStmt", "continueStmt", 
			"whileStmt", "forStmt", "loopExpr", "ifExpr", "matchStmt", "matchExpr", 
			"matchCase", "pattern", "unsafeBlock", "throwStmt", "tryCatchStmt", "catchClause", 
			"finallyClause", "blockExpr", "letStmt", "constStmt", "expression", "assignmentExpr", 
			"assignOp", "rangeExpr", "logicalOrExpr", "logicalAndExpr", "bitOrExpr", 
			"bitXorExpr", "bitAndExpr", "equalityExpr", "comparisonExpr", "shiftExpr", 
			"sumExpr", "productExpr", "castExpr", "prefixExpr", "postfixExpr", "postfixOp", 
			"args", "namedArgument", "primaryExpr", "structLiteralTail", "recallExpr", 
			"learnExpr", "performExpr", "zamaniExpr", "sasaExpr", "quantumOpExpr", 
			"nanoExpr", "mtsExpr", "consensusExpr", "ancestorCall", "mopExpr", "macroCall", 
			"exprList", "expressionStmt", "typeExpr", "baseType", "typeParams", "typeParam", 
			"typeArgs", "structDecl", "structField", "enumDecl", "enumVariant", "traitDecl", 
			"traitItem", "implDecl", "implItem", "typeAliasDecl", "constDecl", "classDecl", 
			"classItem", "constructorDecl", "destructorDecl", "interfaceDecl", "recordDecl", 
			"quantumCircuitDecl", "nanoAgentDecl", "languageDecl", "effectDecl", 
			"effectOp", "effectList", "effectName", "mtsDecl", "sankofaDecl", "agentDecl", 
			"cognitiveBlock", "metaBlock", "hdlModuleDecl", "cloudDecl", "distributedDecl", 
			"onDeviceAgentDecl", "selfEvolveDecl", "optPassDecl", "targetPlatform", 
			"runtimeDecl", "actorDecl", "aiSystemDecl", "agiSystemDecl", "asiSystemDecl", 
			"aesiSystemDecl", "asesiSystemDecl", "adminInterfaceDecl", "paymentGatewayDecl", 
			"userFeedbackDecl", "copyrightNoticeDecl", "tailorMadeFeatureDecl", "programOnceDecl", 
			"maliciousIdeaDetection", "userBlockingDecl", "legalActionDecl", "sandboxDecl", 
			"omniversalSimulationDecl", "omniversalCodeSynthDecl", "omniversalDeployDecl", 
			"omniversalAlignmentDecl", "omniversalContainmentDecl", "omniversalTrustDecl", 
			"omniversalKnowledgeDecl", "omniversalGenerativeDecl", "omniversalSovereigntyDecl", 
			"omniversalGoalDecl", "omniversalBioNanoDecl", "omniversalRealityDecl", 
			"omniversalNlpDecl", "chatArchitectDecl", "greenComputingAttr", "thermalOptDecl", 
			"resourceConserveDecl", "selfDiscoverDecl", "developerAnalyticsDecl", 
			"licenseTrackingDecl", "deploymentDecl", "versionReleaseDecl", "lspServerDecl", 
			"typeClassDecl", "typeClassInstance", "higherKindedTypeDecl", "selfAdjustDecl", 
			"selfVersioningDecl", "extensionMethodDecl", "extensionPropertyDecl", 
			"extensionIndexerDecl", "extensionOperatorDecl", "macroDecl", "domainSpecificLanguageDecl", 
			"aspectDecl", "typeProviderDecl", "dataParallelismDecl", "concurrentDataStructureDecl", 
			"messageHandlerDecl", "musicDecl", "roboticsDecl", "deepLearningDecl", 
			"graphicsDecl", "videoDecl", "tensorDecl", "matrixDecl", "vectorDecl", 
			"mlModelDecl", "quantumMlBlock", "explainableRlBlock", "explainableDeepLearningBlock", 
			"knowledgeGraphBlock", "probabilisticGraphicalModelBlock", "transferLearningBlock", 
			"multiAgentBlock", "autonomousSystemBlock", "graphModelingBlock", "advancedNlpBlock", 
			"cognitiveArchitectureBlock", "aiForBusinessBlock", "vrArInteractionBlock", 
			"imageVideoAnalysisBlock", "fileScopedType", "hybridDef", "interfaceDef", 
			"agentCapability", "agentBehavior", "explainStmt", "transparentStmt", 
			"asiCapabilityDef", "aesiCapabilityDef", "asesiCapabilityDef", "docComment", 
			"attributeDecl", "ident", "statement", "identList", "whereClause", "literal", 
			"quantumLit", "nanoLit", "mtsLit", "rawStringLit", "utf8StringLit", "interpolatedString", 
			"piType", "sigmaType", "identityType", "sessionOp", "sessionBranch", 
			"quantumType", "nanoType", "mtsType", "sankofaType", "cognitiveType"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'to'", "'with'", "'mut'", "'...'", "'_'", "'evas'", "'%='", "'&='", 
			"'|='", "'^='", "'<<='", "'>>='", "'or'", "'and'", "'==='", "'!=='", 
			"'instanceof'", "'is'", "'has'", "'++'", "'--'", "'map'", "'await'", 
			"'spawn'", "'new'", "'yield'", "'super'", "'from'", "'weight'", "'perform'", 
			"'superpose'", "'entangle'", "'assemble'", "'deploy'", "'parallel'", 
			"'speculative'", "'counterfactual'", "'consensus'", "'vote'", "'reflect'", 
			"'introspect'", "'meta_eval'", "'quote'", "'unquote'", "'splice'", "'Box'", 
			"'session'", "'Type_0'", "'Type_1'", "'Type_2'", "'Type_N'", "'Kind'", 
			"'Sort'", "'Prop'", "'effects'", "'hkt'", "'exists'", "'singleton'", 
			"'bytes'", "'i8'", "'i16'", "'i32'", "'i64'", "'i128'", "'u8'", "'u16'", 
			"'u32'", "'u64'", "'u128'", "'f32'", "'f64'", "'usize'", "'isize'", "'where'", 
			"'extends'", "'implements'", "'deinit'", "'op'", "'sankofa'", "'cognitive'", 
			"'meta'", "'hdl'", "'cloud'", "'distributed'", "'on_device'", "'self_evolve'", 
			"'opt_pass'", "'target_platform'", "'runtime'", "'actor'", "'ai'", "'system'", 
			"'agi'", "'asi'", "'aesi'", "'asesi'", "'admin'", "'payment'", "'gateway'", 
			"'user'", "'feedback'", "'copyright'", "'notice'", "'feature'", "'program_once'", 
			"'malicious'", "'idea'", "'detection'", "'block'", "'legal'", "'action'", 
			"'sandbox'", "'omniversal'", "'simulate'", "'synthesize'", "'alignment'", 
			"'containment'", "'trust'", "'knowledge'", "'generate'", "'sovereignty'", 
			"'goal'", "'bionano'", "'reality'", "'nlp'", "'chat'", "'architect'", 
			"'green'", "'thermal'", "'optimize'", "'resource'", "'conserve'", "'discover'", 
			"'developer'", "'analytics'", "'license'", "'tracking'", "'deployment'", 
			"'version'", "'release'", "'lsp'", "'server'", "'typeclass'", "'instance'", 
			"'self_adjust'", "'self_versioning'", "'extension'", "'method'", "'property'", 
			"'indexer'", "'operator'", "'macro'", "'dsl'", "'aspect'", "'type_provider'", 
			"'data'", "'parallelism'", "'concurrent'", "'structure'", "'message'", 
			"'handler'", "'music'", "'robotics'", "'deep_learning'", "'graphics'", 
			"'video'", "'tensor'", "'matrix'", "'vector'", "'ml'", "'model'", "'quantum_ml'", 
			"'explainable_rl'", "'explainable_deep_learning'", "'knowledge_graph'", 
			"'probabilistic_graphical_model'", "'transfer_learning'", "'multi_agent'", 
			"'autonomous_system'", "'graph_modeling'", "'advanced_nlp'", "'cognitive_architecture'", 
			"'ai_for_business'", "'vr_ar_interaction'", "'image_video_analysis'", 
			"'scoped'", "'hybrid'", "'capability'", "'behavior'", "'explain'", "'transparent'", 
			"'#['", "'0'", "'1'", "'\\u27E9'", "'@atom'", "'@molecule'", "'r'", "'f'", 
			"'Pi'", "'Sigma'", "'Id'", "'send'", "'recv'", "'offer'", "'choice'", 
			"'close'", "'Qubit'", "'QReg'", "'Superposition'", "'Entangled'", "'Atom'", 
			"'Molecule'", "'NanoAgent'", "'Mts'", "'Parallel'", "'Speculative'", 
			"'Past'", "'Present'", "'Future'", "'CognitiveState'", "'Consciousness'", 
			"'Neural'", null, null, null, "'module'", "'import'", "'export'", "'as'", 
			"'use'", "'global'", "'using'", "'fn'", "'pub'", "'private'", "'protected'", 
			"'static'", "'const'", "'async'", "'unsafe'", "'inline'", "'override'", 
			"'final'", "'abstract'", "'virtual'", "'sealed'", "'partial'", "'file'", 
			"'required'", "'init'", "'return'", "'break'", "'continue'", "'while'", 
			"'for'", "'in'", "'loop'", "'if'", "'else'", "'match'", "'case'", "'when'", 
			"'throw'", "'try'", "'catch'", "'finally'", "'let'", "'var'", "'type'", 
			"'struct'", "'enum'", "'trait'", "'impl'", "'class'", "'interface'", 
			"'record'", "'quantum'", "'circuit'", "'measure'", "'reset'", "'barrier'", 
			"'this'", "'self'", "'Self'", "'int'", "'float'", "'bool'", "'string'", 
			"'String'", "'char'", "'void'", "'nano'", "'agent'", "'effect'", "'handle'", 
			"'remember'", "'recall'", "'learn'", "'infer'", "'wisdom'", "'zamani'", 
			"'sasa'", "'ancestral'", "'linear'", "'affine'", "'language'", "'mts'", 
			"'len'", "'print'", "'println'", "'assert'", "'panic'", "'true'", "'false'", 
			"'nil'", "'null'", null, null, null, null, null, null, null, null, null, 
			null, "'('", "')'", "'{'", "'}'", "'['", "']'", "','", "'.'", "';'", 
			"':'", "'->'", "'=>'", "'::'", "'~'", "'#'", "'@'", "'!'", "'+'", "'-'", 
			"'*'", "'/'", "'%'", "'='", "'=='", "'!='", "'<'", "'>'", "'<='", "'>='", 
			"'&&'", "'||'", "'&'", "'|'", "'^'", "'<<'", "'>>'", "'>>>'", "'+='", 
			"'-='", "'*='", "'/='", "'..'", "'..='", "'?'", "'$'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, "ORBITAL", "FORMULA", 
			"OPERATOR", "MODULE", "IMPORT", "EXPORT", "AS", "USE", "GLOBAL", "USING", 
			"FN", "PUB", "PRIVATE", "PROTECTED", "STATIC", "CONST", "ASYNC", "UNSAFE", 
			"INLINE", "OVERRIDE", "FINAL", "ABSTRACT", "VIRTUAL", "SEALED", "PARTIAL", 
			"FILE", "REQUIRED", "INIT", "RETURN", "BREAK", "CONTINUE", "WHILE", "FOR", 
			"IN", "LOOP", "IF", "ELSE", "MATCH", "CASE", "WHEN", "THROW", "TRY", 
			"CATCH", "FINALLY", "LET", "VAR", "TYPE", "STRUCT", "ENUM", "TRAIT", 
			"IMPL", "CLASS", "INTERFACE", "RECORD", "QUANTUM", "CIRCUIT", "MEASURE", 
			"RESET", "BARRIER", "THIS", "SELF_LOWER", "SELF_UPPER", "INT_KW", "FLOAT_KW", 
			"BOOL_KW", "STR_KW", "STRING_KW", "CHAR_KW", "VOID", "NANO", "AGENT", 
			"EFFECT", "HANDLE", "REMEMBER", "RECALL", "LEARN", "INFER", "WISDOM", 
			"ZAMANI", "SASA", "ANCESTOR", "LINEAR", "AFFINE", "LANGUAGE", "MTS_KW", 
			"LEN", "PRINT", "PRINTLN", "ASSERT", "PANIC", "TRUE", "FALSE", "NIL_KW", 
			"NULL_KW", "BOOLEAN", "NIL", "INTEGER", "FLOAT", "STRING", "CHAR", "QUANTUM_LITERAL", 
			"NANO_ANNOTATION", "MTS_LITERAL", "IDENT", "LPAREN", "RPAREN", "LBRACE", 
			"RBRACE", "LBRACK", "RBRACK", "COMMA", "DOT", "SEMI", "COLON", "ARROW", 
			"FATARROW", "COLONCOLON", "TILDE", "HASH", "AT", "BANG", "PLUS", "MINUS", 
			"STAR", "SLASH", "PERCENT", "ASSIGN", "EQ", "NEQ", "LT", "GT", "LE", 
			"GE", "ANDAND", "OROR", "AMP", "PIPE", "CARET", "SHL", "SHR", "SHRU", 
			"PLUSEQ", "MINUSEQ", "STAREQ", "SLASHEQ", "DOTDOT", "DOTDOTEQ", "QUESTION", 
			"DOLLAR", "LINE_COMMENT", "BLOCK_COMMENT", "DOC_COMMENT", "WS"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "Zamani.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public ZamaniParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProgramContext extends ParserRuleContext {
		public TerminalNode EOF() { return getToken(ZamaniParser.EOF, 0); }
		public List<DeclarationContext> declaration() {
			return getRuleContexts(DeclarationContext.class);
		}
		public DeclarationContext declaration(int i) {
			return getRuleContext(DeclarationContext.class,i);
		}
		public ProgramContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_program; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterProgram(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitProgram(this);
		}
	}

	public final ProgramContext program() throws RecognitionException {
		ProgramContext _localctx = new ProgramContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_program);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(447);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 72127412219936768L) != 0) || ((((_la - 65)) & ~0x3f) == 0 && ((1L << (_la - 65)) & -6917050001482858495L) != 0) || ((((_la - 129)) & ~0x3f) == 0 && ((1L << (_la - 129)) & -8791030876318738779L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2305843269059215345L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029990809L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 281487863332907L) != 0)) {
				{
				{
				setState(444);
				declaration();
				}
				}
				setState(449);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(450);
			match(EOF);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DeclarationContext extends ParserRuleContext {
		public ModuleDeclContext moduleDecl() {
			return getRuleContext(ModuleDeclContext.class,0);
		}
		public ImportDeclContext importDecl() {
			return getRuleContext(ImportDeclContext.class,0);
		}
		public ExportDeclContext exportDecl() {
			return getRuleContext(ExportDeclContext.class,0);
		}
		public FunctionDeclContext functionDecl() {
			return getRuleContext(FunctionDeclContext.class,0);
		}
		public StructDeclContext structDecl() {
			return getRuleContext(StructDeclContext.class,0);
		}
		public EnumDeclContext enumDecl() {
			return getRuleContext(EnumDeclContext.class,0);
		}
		public TraitDeclContext traitDecl() {
			return getRuleContext(TraitDeclContext.class,0);
		}
		public ImplDeclContext implDecl() {
			return getRuleContext(ImplDeclContext.class,0);
		}
		public TypeAliasDeclContext typeAliasDecl() {
			return getRuleContext(TypeAliasDeclContext.class,0);
		}
		public ConstStmtContext constStmt() {
			return getRuleContext(ConstStmtContext.class,0);
		}
		public ClassDeclContext classDecl() {
			return getRuleContext(ClassDeclContext.class,0);
		}
		public InterfaceDeclContext interfaceDecl() {
			return getRuleContext(InterfaceDeclContext.class,0);
		}
		public RecordDeclContext recordDecl() {
			return getRuleContext(RecordDeclContext.class,0);
		}
		public QuantumCircuitDeclContext quantumCircuitDecl() {
			return getRuleContext(QuantumCircuitDeclContext.class,0);
		}
		public NanoAgentDeclContext nanoAgentDecl() {
			return getRuleContext(NanoAgentDeclContext.class,0);
		}
		public LanguageDeclContext languageDecl() {
			return getRuleContext(LanguageDeclContext.class,0);
		}
		public EffectDeclContext effectDecl() {
			return getRuleContext(EffectDeclContext.class,0);
		}
		public MtsDeclContext mtsDecl() {
			return getRuleContext(MtsDeclContext.class,0);
		}
		public SankofaDeclContext sankofaDecl() {
			return getRuleContext(SankofaDeclContext.class,0);
		}
		public AgentDeclContext agentDecl() {
			return getRuleContext(AgentDeclContext.class,0);
		}
		public CognitiveBlockContext cognitiveBlock() {
			return getRuleContext(CognitiveBlockContext.class,0);
		}
		public MetaBlockContext metaBlock() {
			return getRuleContext(MetaBlockContext.class,0);
		}
		public HdlModuleDeclContext hdlModuleDecl() {
			return getRuleContext(HdlModuleDeclContext.class,0);
		}
		public CloudDeclContext cloudDecl() {
			return getRuleContext(CloudDeclContext.class,0);
		}
		public DistributedDeclContext distributedDecl() {
			return getRuleContext(DistributedDeclContext.class,0);
		}
		public OnDeviceAgentDeclContext onDeviceAgentDecl() {
			return getRuleContext(OnDeviceAgentDeclContext.class,0);
		}
		public SelfEvolveDeclContext selfEvolveDecl() {
			return getRuleContext(SelfEvolveDeclContext.class,0);
		}
		public OptPassDeclContext optPassDecl() {
			return getRuleContext(OptPassDeclContext.class,0);
		}
		public TargetPlatformContext targetPlatform() {
			return getRuleContext(TargetPlatformContext.class,0);
		}
		public RuntimeDeclContext runtimeDecl() {
			return getRuleContext(RuntimeDeclContext.class,0);
		}
		public ActorDeclContext actorDecl() {
			return getRuleContext(ActorDeclContext.class,0);
		}
		public AiSystemDeclContext aiSystemDecl() {
			return getRuleContext(AiSystemDeclContext.class,0);
		}
		public AgiSystemDeclContext agiSystemDecl() {
			return getRuleContext(AgiSystemDeclContext.class,0);
		}
		public AsiSystemDeclContext asiSystemDecl() {
			return getRuleContext(AsiSystemDeclContext.class,0);
		}
		public AesiSystemDeclContext aesiSystemDecl() {
			return getRuleContext(AesiSystemDeclContext.class,0);
		}
		public AsesiSystemDeclContext asesiSystemDecl() {
			return getRuleContext(AsesiSystemDeclContext.class,0);
		}
		public AdminInterfaceDeclContext adminInterfaceDecl() {
			return getRuleContext(AdminInterfaceDeclContext.class,0);
		}
		public PaymentGatewayDeclContext paymentGatewayDecl() {
			return getRuleContext(PaymentGatewayDeclContext.class,0);
		}
		public UserFeedbackDeclContext userFeedbackDecl() {
			return getRuleContext(UserFeedbackDeclContext.class,0);
		}
		public CopyrightNoticeDeclContext copyrightNoticeDecl() {
			return getRuleContext(CopyrightNoticeDeclContext.class,0);
		}
		public TailorMadeFeatureDeclContext tailorMadeFeatureDecl() {
			return getRuleContext(TailorMadeFeatureDeclContext.class,0);
		}
		public ProgramOnceDeclContext programOnceDecl() {
			return getRuleContext(ProgramOnceDeclContext.class,0);
		}
		public MaliciousIdeaDetectionContext maliciousIdeaDetection() {
			return getRuleContext(MaliciousIdeaDetectionContext.class,0);
		}
		public UserBlockingDeclContext userBlockingDecl() {
			return getRuleContext(UserBlockingDeclContext.class,0);
		}
		public LegalActionDeclContext legalActionDecl() {
			return getRuleContext(LegalActionDeclContext.class,0);
		}
		public SandboxDeclContext sandboxDecl() {
			return getRuleContext(SandboxDeclContext.class,0);
		}
		public OmniversalSimulationDeclContext omniversalSimulationDecl() {
			return getRuleContext(OmniversalSimulationDeclContext.class,0);
		}
		public OmniversalCodeSynthDeclContext omniversalCodeSynthDecl() {
			return getRuleContext(OmniversalCodeSynthDeclContext.class,0);
		}
		public OmniversalDeployDeclContext omniversalDeployDecl() {
			return getRuleContext(OmniversalDeployDeclContext.class,0);
		}
		public OmniversalAlignmentDeclContext omniversalAlignmentDecl() {
			return getRuleContext(OmniversalAlignmentDeclContext.class,0);
		}
		public OmniversalContainmentDeclContext omniversalContainmentDecl() {
			return getRuleContext(OmniversalContainmentDeclContext.class,0);
		}
		public OmniversalTrustDeclContext omniversalTrustDecl() {
			return getRuleContext(OmniversalTrustDeclContext.class,0);
		}
		public OmniversalKnowledgeDeclContext omniversalKnowledgeDecl() {
			return getRuleContext(OmniversalKnowledgeDeclContext.class,0);
		}
		public OmniversalGenerativeDeclContext omniversalGenerativeDecl() {
			return getRuleContext(OmniversalGenerativeDeclContext.class,0);
		}
		public OmniversalSovereigntyDeclContext omniversalSovereigntyDecl() {
			return getRuleContext(OmniversalSovereigntyDeclContext.class,0);
		}
		public OmniversalGoalDeclContext omniversalGoalDecl() {
			return getRuleContext(OmniversalGoalDeclContext.class,0);
		}
		public OmniversalBioNanoDeclContext omniversalBioNanoDecl() {
			return getRuleContext(OmniversalBioNanoDeclContext.class,0);
		}
		public OmniversalRealityDeclContext omniversalRealityDecl() {
			return getRuleContext(OmniversalRealityDeclContext.class,0);
		}
		public OmniversalNlpDeclContext omniversalNlpDecl() {
			return getRuleContext(OmniversalNlpDeclContext.class,0);
		}
		public ChatArchitectDeclContext chatArchitectDecl() {
			return getRuleContext(ChatArchitectDeclContext.class,0);
		}
		public GreenComputingAttrContext greenComputingAttr() {
			return getRuleContext(GreenComputingAttrContext.class,0);
		}
		public ThermalOptDeclContext thermalOptDecl() {
			return getRuleContext(ThermalOptDeclContext.class,0);
		}
		public ResourceConserveDeclContext resourceConserveDecl() {
			return getRuleContext(ResourceConserveDeclContext.class,0);
		}
		public SelfDiscoverDeclContext selfDiscoverDecl() {
			return getRuleContext(SelfDiscoverDeclContext.class,0);
		}
		public DeveloperAnalyticsDeclContext developerAnalyticsDecl() {
			return getRuleContext(DeveloperAnalyticsDeclContext.class,0);
		}
		public LicenseTrackingDeclContext licenseTrackingDecl() {
			return getRuleContext(LicenseTrackingDeclContext.class,0);
		}
		public DeploymentDeclContext deploymentDecl() {
			return getRuleContext(DeploymentDeclContext.class,0);
		}
		public VersionReleaseDeclContext versionReleaseDecl() {
			return getRuleContext(VersionReleaseDeclContext.class,0);
		}
		public LspServerDeclContext lspServerDecl() {
			return getRuleContext(LspServerDeclContext.class,0);
		}
		public TypeClassDeclContext typeClassDecl() {
			return getRuleContext(TypeClassDeclContext.class,0);
		}
		public TypeClassInstanceContext typeClassInstance() {
			return getRuleContext(TypeClassInstanceContext.class,0);
		}
		public HigherKindedTypeDeclContext higherKindedTypeDecl() {
			return getRuleContext(HigherKindedTypeDeclContext.class,0);
		}
		public SelfAdjustDeclContext selfAdjustDecl() {
			return getRuleContext(SelfAdjustDeclContext.class,0);
		}
		public SelfVersioningDeclContext selfVersioningDecl() {
			return getRuleContext(SelfVersioningDeclContext.class,0);
		}
		public ExtensionMethodDeclContext extensionMethodDecl() {
			return getRuleContext(ExtensionMethodDeclContext.class,0);
		}
		public ExtensionPropertyDeclContext extensionPropertyDecl() {
			return getRuleContext(ExtensionPropertyDeclContext.class,0);
		}
		public ExtensionIndexerDeclContext extensionIndexerDecl() {
			return getRuleContext(ExtensionIndexerDeclContext.class,0);
		}
		public ExtensionOperatorDeclContext extensionOperatorDecl() {
			return getRuleContext(ExtensionOperatorDeclContext.class,0);
		}
		public MacroDeclContext macroDecl() {
			return getRuleContext(MacroDeclContext.class,0);
		}
		public DomainSpecificLanguageDeclContext domainSpecificLanguageDecl() {
			return getRuleContext(DomainSpecificLanguageDeclContext.class,0);
		}
		public AspectDeclContext aspectDecl() {
			return getRuleContext(AspectDeclContext.class,0);
		}
		public TypeProviderDeclContext typeProviderDecl() {
			return getRuleContext(TypeProviderDeclContext.class,0);
		}
		public DataParallelismDeclContext dataParallelismDecl() {
			return getRuleContext(DataParallelismDeclContext.class,0);
		}
		public ConcurrentDataStructureDeclContext concurrentDataStructureDecl() {
			return getRuleContext(ConcurrentDataStructureDeclContext.class,0);
		}
		public MessageHandlerDeclContext messageHandlerDecl() {
			return getRuleContext(MessageHandlerDeclContext.class,0);
		}
		public MusicDeclContext musicDecl() {
			return getRuleContext(MusicDeclContext.class,0);
		}
		public RoboticsDeclContext roboticsDecl() {
			return getRuleContext(RoboticsDeclContext.class,0);
		}
		public DeepLearningDeclContext deepLearningDecl() {
			return getRuleContext(DeepLearningDeclContext.class,0);
		}
		public GraphicsDeclContext graphicsDecl() {
			return getRuleContext(GraphicsDeclContext.class,0);
		}
		public VideoDeclContext videoDecl() {
			return getRuleContext(VideoDeclContext.class,0);
		}
		public TensorDeclContext tensorDecl() {
			return getRuleContext(TensorDeclContext.class,0);
		}
		public MatrixDeclContext matrixDecl() {
			return getRuleContext(MatrixDeclContext.class,0);
		}
		public VectorDeclContext vectorDecl() {
			return getRuleContext(VectorDeclContext.class,0);
		}
		public MlModelDeclContext mlModelDecl() {
			return getRuleContext(MlModelDeclContext.class,0);
		}
		public QuantumMlBlockContext quantumMlBlock() {
			return getRuleContext(QuantumMlBlockContext.class,0);
		}
		public ExplainableRlBlockContext explainableRlBlock() {
			return getRuleContext(ExplainableRlBlockContext.class,0);
		}
		public ExplainableDeepLearningBlockContext explainableDeepLearningBlock() {
			return getRuleContext(ExplainableDeepLearningBlockContext.class,0);
		}
		public KnowledgeGraphBlockContext knowledgeGraphBlock() {
			return getRuleContext(KnowledgeGraphBlockContext.class,0);
		}
		public ProbabilisticGraphicalModelBlockContext probabilisticGraphicalModelBlock() {
			return getRuleContext(ProbabilisticGraphicalModelBlockContext.class,0);
		}
		public TransferLearningBlockContext transferLearningBlock() {
			return getRuleContext(TransferLearningBlockContext.class,0);
		}
		public MultiAgentBlockContext multiAgentBlock() {
			return getRuleContext(MultiAgentBlockContext.class,0);
		}
		public AutonomousSystemBlockContext autonomousSystemBlock() {
			return getRuleContext(AutonomousSystemBlockContext.class,0);
		}
		public GraphModelingBlockContext graphModelingBlock() {
			return getRuleContext(GraphModelingBlockContext.class,0);
		}
		public AdvancedNlpBlockContext advancedNlpBlock() {
			return getRuleContext(AdvancedNlpBlockContext.class,0);
		}
		public CognitiveArchitectureBlockContext cognitiveArchitectureBlock() {
			return getRuleContext(CognitiveArchitectureBlockContext.class,0);
		}
		public AiForBusinessBlockContext aiForBusinessBlock() {
			return getRuleContext(AiForBusinessBlockContext.class,0);
		}
		public VrArInteractionBlockContext vrArInteractionBlock() {
			return getRuleContext(VrArInteractionBlockContext.class,0);
		}
		public ImageVideoAnalysisBlockContext imageVideoAnalysisBlock() {
			return getRuleContext(ImageVideoAnalysisBlockContext.class,0);
		}
		public FileScopedTypeContext fileScopedType() {
			return getRuleContext(FileScopedTypeContext.class,0);
		}
		public HybridDefContext hybridDef() {
			return getRuleContext(HybridDefContext.class,0);
		}
		public InterfaceDefContext interfaceDef() {
			return getRuleContext(InterfaceDefContext.class,0);
		}
		public StatementContext statement() {
			return getRuleContext(StatementContext.class,0);
		}
		public DocCommentContext docComment() {
			return getRuleContext(DocCommentContext.class,0);
		}
		public AttributeDeclContext attributeDecl() {
			return getRuleContext(AttributeDeclContext.class,0);
		}
		public DeclarationContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_declaration; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDeclaration(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDeclaration(this);
		}
	}

	public final DeclarationContext declaration() throws RecognitionException {
		DeclarationContext _localctx = new DeclarationContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_declaration);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(453);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==DOC_COMMENT) {
				{
				setState(452);
				docComment();
				}
			}

			setState(456);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__191) {
				{
				setState(455);
				attributeDecl();
				}
			}

			setState(570);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,3,_ctx) ) {
			case 1:
				{
				setState(458);
				moduleDecl();
				}
				break;
			case 2:
				{
				setState(459);
				importDecl();
				}
				break;
			case 3:
				{
				setState(460);
				exportDecl();
				}
				break;
			case 4:
				{
				setState(461);
				functionDecl();
				}
				break;
			case 5:
				{
				setState(462);
				structDecl();
				}
				break;
			case 6:
				{
				setState(463);
				enumDecl();
				}
				break;
			case 7:
				{
				setState(464);
				traitDecl();
				}
				break;
			case 8:
				{
				setState(465);
				implDecl();
				}
				break;
			case 9:
				{
				setState(466);
				typeAliasDecl();
				}
				break;
			case 10:
				{
				setState(467);
				constStmt();
				}
				break;
			case 11:
				{
				setState(468);
				classDecl();
				}
				break;
			case 12:
				{
				setState(469);
				interfaceDecl();
				}
				break;
			case 13:
				{
				setState(470);
				recordDecl();
				}
				break;
			case 14:
				{
				setState(471);
				quantumCircuitDecl();
				}
				break;
			case 15:
				{
				setState(472);
				nanoAgentDecl();
				}
				break;
			case 16:
				{
				setState(473);
				languageDecl();
				}
				break;
			case 17:
				{
				setState(474);
				effectDecl();
				}
				break;
			case 18:
				{
				setState(475);
				mtsDecl();
				}
				break;
			case 19:
				{
				setState(476);
				sankofaDecl();
				}
				break;
			case 20:
				{
				setState(477);
				agentDecl();
				}
				break;
			case 21:
				{
				setState(478);
				cognitiveBlock();
				}
				break;
			case 22:
				{
				setState(479);
				metaBlock();
				}
				break;
			case 23:
				{
				setState(480);
				hdlModuleDecl();
				}
				break;
			case 24:
				{
				setState(481);
				cloudDecl();
				}
				break;
			case 25:
				{
				setState(482);
				distributedDecl();
				}
				break;
			case 26:
				{
				setState(483);
				onDeviceAgentDecl();
				}
				break;
			case 27:
				{
				setState(484);
				selfEvolveDecl();
				}
				break;
			case 28:
				{
				setState(485);
				optPassDecl();
				}
				break;
			case 29:
				{
				setState(486);
				targetPlatform();
				}
				break;
			case 30:
				{
				setState(487);
				runtimeDecl();
				}
				break;
			case 31:
				{
				setState(488);
				actorDecl();
				}
				break;
			case 32:
				{
				setState(489);
				aiSystemDecl();
				}
				break;
			case 33:
				{
				setState(490);
				agiSystemDecl();
				}
				break;
			case 34:
				{
				setState(491);
				asiSystemDecl();
				}
				break;
			case 35:
				{
				setState(492);
				aesiSystemDecl();
				}
				break;
			case 36:
				{
				setState(493);
				asesiSystemDecl();
				}
				break;
			case 37:
				{
				setState(494);
				adminInterfaceDecl();
				}
				break;
			case 38:
				{
				setState(495);
				paymentGatewayDecl();
				}
				break;
			case 39:
				{
				setState(496);
				userFeedbackDecl();
				}
				break;
			case 40:
				{
				setState(497);
				copyrightNoticeDecl();
				}
				break;
			case 41:
				{
				setState(498);
				tailorMadeFeatureDecl();
				}
				break;
			case 42:
				{
				setState(499);
				programOnceDecl();
				}
				break;
			case 43:
				{
				setState(500);
				maliciousIdeaDetection();
				}
				break;
			case 44:
				{
				setState(501);
				userBlockingDecl();
				}
				break;
			case 45:
				{
				setState(502);
				legalActionDecl();
				}
				break;
			case 46:
				{
				setState(503);
				sandboxDecl();
				}
				break;
			case 47:
				{
				setState(504);
				omniversalSimulationDecl();
				}
				break;
			case 48:
				{
				setState(505);
				omniversalCodeSynthDecl();
				}
				break;
			case 49:
				{
				setState(506);
				omniversalDeployDecl();
				}
				break;
			case 50:
				{
				setState(507);
				omniversalAlignmentDecl();
				}
				break;
			case 51:
				{
				setState(508);
				omniversalContainmentDecl();
				}
				break;
			case 52:
				{
				setState(509);
				omniversalTrustDecl();
				}
				break;
			case 53:
				{
				setState(510);
				omniversalKnowledgeDecl();
				}
				break;
			case 54:
				{
				setState(511);
				omniversalGenerativeDecl();
				}
				break;
			case 55:
				{
				setState(512);
				omniversalSovereigntyDecl();
				}
				break;
			case 56:
				{
				setState(513);
				omniversalGoalDecl();
				}
				break;
			case 57:
				{
				setState(514);
				omniversalBioNanoDecl();
				}
				break;
			case 58:
				{
				setState(515);
				omniversalRealityDecl();
				}
				break;
			case 59:
				{
				setState(516);
				omniversalNlpDecl();
				}
				break;
			case 60:
				{
				setState(517);
				chatArchitectDecl();
				}
				break;
			case 61:
				{
				setState(518);
				greenComputingAttr();
				}
				break;
			case 62:
				{
				setState(519);
				thermalOptDecl();
				}
				break;
			case 63:
				{
				setState(520);
				resourceConserveDecl();
				}
				break;
			case 64:
				{
				setState(521);
				selfDiscoverDecl();
				}
				break;
			case 65:
				{
				setState(522);
				developerAnalyticsDecl();
				}
				break;
			case 66:
				{
				setState(523);
				licenseTrackingDecl();
				}
				break;
			case 67:
				{
				setState(524);
				deploymentDecl();
				}
				break;
			case 68:
				{
				setState(525);
				versionReleaseDecl();
				}
				break;
			case 69:
				{
				setState(526);
				lspServerDecl();
				}
				break;
			case 70:
				{
				setState(527);
				typeClassDecl();
				}
				break;
			case 71:
				{
				setState(528);
				typeClassInstance();
				}
				break;
			case 72:
				{
				setState(529);
				higherKindedTypeDecl();
				}
				break;
			case 73:
				{
				setState(530);
				selfAdjustDecl();
				}
				break;
			case 74:
				{
				setState(531);
				selfVersioningDecl();
				}
				break;
			case 75:
				{
				setState(532);
				extensionMethodDecl();
				}
				break;
			case 76:
				{
				setState(533);
				extensionPropertyDecl();
				}
				break;
			case 77:
				{
				setState(534);
				extensionIndexerDecl();
				}
				break;
			case 78:
				{
				setState(535);
				extensionOperatorDecl();
				}
				break;
			case 79:
				{
				setState(536);
				macroDecl();
				}
				break;
			case 80:
				{
				setState(537);
				domainSpecificLanguageDecl();
				}
				break;
			case 81:
				{
				setState(538);
				aspectDecl();
				}
				break;
			case 82:
				{
				setState(539);
				typeProviderDecl();
				}
				break;
			case 83:
				{
				setState(540);
				dataParallelismDecl();
				}
				break;
			case 84:
				{
				setState(541);
				concurrentDataStructureDecl();
				}
				break;
			case 85:
				{
				setState(542);
				messageHandlerDecl();
				}
				break;
			case 86:
				{
				setState(543);
				musicDecl();
				}
				break;
			case 87:
				{
				setState(544);
				roboticsDecl();
				}
				break;
			case 88:
				{
				setState(545);
				deepLearningDecl();
				}
				break;
			case 89:
				{
				setState(546);
				graphicsDecl();
				}
				break;
			case 90:
				{
				setState(547);
				videoDecl();
				}
				break;
			case 91:
				{
				setState(548);
				tensorDecl();
				}
				break;
			case 92:
				{
				setState(549);
				matrixDecl();
				}
				break;
			case 93:
				{
				setState(550);
				vectorDecl();
				}
				break;
			case 94:
				{
				setState(551);
				mlModelDecl();
				}
				break;
			case 95:
				{
				setState(552);
				quantumMlBlock();
				}
				break;
			case 96:
				{
				setState(553);
				explainableRlBlock();
				}
				break;
			case 97:
				{
				setState(554);
				explainableDeepLearningBlock();
				}
				break;
			case 98:
				{
				setState(555);
				knowledgeGraphBlock();
				}
				break;
			case 99:
				{
				setState(556);
				probabilisticGraphicalModelBlock();
				}
				break;
			case 100:
				{
				setState(557);
				transferLearningBlock();
				}
				break;
			case 101:
				{
				setState(558);
				multiAgentBlock();
				}
				break;
			case 102:
				{
				setState(559);
				autonomousSystemBlock();
				}
				break;
			case 103:
				{
				setState(560);
				graphModelingBlock();
				}
				break;
			case 104:
				{
				setState(561);
				advancedNlpBlock();
				}
				break;
			case 105:
				{
				setState(562);
				cognitiveArchitectureBlock();
				}
				break;
			case 106:
				{
				setState(563);
				aiForBusinessBlock();
				}
				break;
			case 107:
				{
				setState(564);
				vrArInteractionBlock();
				}
				break;
			case 108:
				{
				setState(565);
				imageVideoAnalysisBlock();
				}
				break;
			case 109:
				{
				setState(566);
				fileScopedType();
				}
				break;
			case 110:
				{
				setState(567);
				hybridDef();
				}
				break;
			case 111:
				{
				setState(568);
				interfaceDef();
				}
				break;
			case 112:
				{
				setState(569);
				statement();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ModuleDeclContext extends ParserRuleContext {
		public TerminalNode MODULE() { return getToken(ZamaniParser.MODULE, 0); }
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public List<TerminalNode> COLONCOLON() { return getTokens(ZamaniParser.COLONCOLON); }
		public TerminalNode COLONCOLON(int i) {
			return getToken(ZamaniParser.COLONCOLON, i);
		}
		public ModuleDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_moduleDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterModuleDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitModuleDecl(this);
		}
	}

	public final ModuleDeclContext moduleDecl() throws RecognitionException {
		ModuleDeclContext _localctx = new ModuleDeclContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_moduleDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(572);
			match(MODULE);
			setState(573);
			ident();
			setState(578);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COLONCOLON) {
				{
				{
				setState(574);
				match(COLONCOLON);
				setState(575);
				ident();
				}
				}
				setState(580);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(583);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case LBRACE:
				{
				setState(581);
				blockExpr();
				}
				break;
			case SEMI:
				{
				setState(582);
				match(SEMI);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ImportDeclContext extends ParserRuleContext {
		public TerminalNode IMPORT() { return getToken(ZamaniParser.IMPORT, 0); }
		public ModulePathContext modulePath() {
			return getRuleContext(ModulePathContext.class,0);
		}
		public TerminalNode AS() { return getToken(ZamaniParser.AS, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ImportDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_importDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterImportDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitImportDecl(this);
		}
	}

	public final ImportDeclContext importDecl() throws RecognitionException {
		ImportDeclContext _localctx = new ImportDeclContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_importDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(585);
			match(IMPORT);
			setState(586);
			modulePath();
			setState(589);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==AS) {
				{
				setState(587);
				match(AS);
				setState(588);
				ident();
				}
			}

			setState(592);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==SEMI) {
				{
				setState(591);
				match(SEMI);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExportDeclContext extends ParserRuleContext {
		public TerminalNode EXPORT() { return getToken(ZamaniParser.EXPORT, 0); }
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ExportDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_exportDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExportDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExportDecl(this);
		}
	}

	public final ExportDeclContext exportDecl() throws RecognitionException {
		ExportDeclContext _localctx = new ExportDeclContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_exportDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(594);
			match(EXPORT);
			setState(595);
			ident();
			setState(598);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__0) {
				{
				setState(596);
				match(T__0);
				setState(597);
				ident();
				}
			}

			setState(601);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==SEMI) {
				{
				setState(600);
				match(SEMI);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ModulePathContext extends ParserRuleContext {
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public List<TerminalNode> COLONCOLON() { return getTokens(ZamaniParser.COLONCOLON); }
		public TerminalNode COLONCOLON(int i) {
			return getToken(ZamaniParser.COLONCOLON, i);
		}
		public List<TerminalNode> DOT() { return getTokens(ZamaniParser.DOT); }
		public TerminalNode DOT(int i) {
			return getToken(ZamaniParser.DOT, i);
		}
		public ModulePathContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_modulePath; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterModulePath(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitModulePath(this);
		}
	}

	public final ModulePathContext modulePath() throws RecognitionException {
		ModulePathContext _localctx = new ModulePathContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_modulePath);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(603);
			ident();
			setState(610);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==DOT || _la==COLONCOLON) {
				{
				setState(608);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case COLONCOLON:
					{
					setState(604);
					match(COLONCOLON);
					setState(605);
					ident();
					}
					break;
				case DOT:
					{
					setState(606);
					match(DOT);
					setState(607);
					ident();
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				}
				setState(612);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class UseStmtContext extends ParserRuleContext {
		public TerminalNode USE() { return getToken(ZamaniParser.USE, 0); }
		public UsePathContext usePath() {
			return getRuleContext(UsePathContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public UseStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_useStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterUseStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitUseStmt(this);
		}
	}

	public final UseStmtContext useStmt() throws RecognitionException {
		UseStmtContext _localctx = new UseStmtContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_useStmt);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(613);
			match(USE);
			setState(614);
			usePath();
			setState(616);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==SEMI) {
				{
				setState(615);
				match(SEMI);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class UsePathContext extends ParserRuleContext {
		public List<SegmentContext> segment() {
			return getRuleContexts(SegmentContext.class);
		}
		public SegmentContext segment(int i) {
			return getRuleContext(SegmentContext.class,i);
		}
		public List<TerminalNode> COLONCOLON() { return getTokens(ZamaniParser.COLONCOLON); }
		public TerminalNode COLONCOLON(int i) {
			return getToken(ZamaniParser.COLONCOLON, i);
		}
		public TerminalNode STAR() { return getToken(ZamaniParser.STAR, 0); }
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public UsePathContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_usePath; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterUsePath(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitUsePath(this);
		}
	}

	public final UsePathContext usePath() throws RecognitionException {
		UsePathContext _localctx = new UsePathContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_usePath);
		int _la;
		try {
			int _alt;
			setState(650);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,17,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(618);
				segment();
				setState(623);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,13,_ctx);
				while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
					if ( _alt==1 ) {
						{
						{
						setState(619);
						match(COLONCOLON);
						setState(620);
						segment();
						}
						} 
					}
					setState(625);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,13,_ctx);
				}
				setState(628);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COLONCOLON) {
					{
					setState(626);
					match(COLONCOLON);
					setState(627);
					match(STAR);
					}
				}

				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(630);
				segment();
				setState(635);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,15,_ctx);
				while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
					if ( _alt==1 ) {
						{
						{
						setState(631);
						match(COLONCOLON);
						setState(632);
						segment();
						}
						} 
					}
					setState(637);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,15,_ctx);
				}
				setState(638);
				match(COLONCOLON);
				setState(639);
				match(LBRACE);
				setState(640);
				ident();
				setState(645);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(641);
					match(COMMA);
					setState(642);
					ident();
					}
					}
					setState(647);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(648);
				match(RBRACE);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SegmentContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public SegmentContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_segment; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSegment(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSegment(this);
		}
	}

	public final SegmentContext segment() throws RecognitionException {
		SegmentContext _localctx = new SegmentContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_segment);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(652);
			ident();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class GlobalUsingContext extends ParserRuleContext {
		public TerminalNode GLOBAL() { return getToken(ZamaniParser.GLOBAL, 0); }
		public TerminalNode USING() { return getToken(ZamaniParser.USING, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public GlobalUsingContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_globalUsing; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterGlobalUsing(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitGlobalUsing(this);
		}
	}

	public final GlobalUsingContext globalUsing() throws RecognitionException {
		GlobalUsingContext _localctx = new GlobalUsingContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_globalUsing);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(654);
			match(GLOBAL);
			setState(655);
			match(USING);
			setState(656);
			ident();
			setState(657);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class FunctionDeclContext extends ParserRuleContext {
		public TerminalNode FN() { return getToken(ZamaniParser.FN, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public TerminalNode ARROW() { return getToken(ZamaniParser.ARROW, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public EffectListContext effectList() {
			return getRuleContext(EffectListContext.class,0);
		}
		public FunctionDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_functionDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterFunctionDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitFunctionDecl(this);
		}
	}

	public final FunctionDeclContext functionDecl() throws RecognitionException {
		FunctionDeclContext _localctx = new FunctionDeclContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_functionDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(660);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(659);
				modifiers();
				}
			}

			setState(662);
			match(FN);
			setState(663);
			ident();
			setState(665);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(664);
				typeParams();
				}
			}

			setState(667);
			match(LPAREN);
			setState(669);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(668);
				params();
				}
			}

			setState(671);
			match(RPAREN);
			setState(674);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==ARROW) {
				{
				setState(672);
				match(ARROW);
				setState(673);
				typeExpr(0);
				}
			}

			setState(678);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__1) {
				{
				setState(676);
				match(T__1);
				setState(677);
				effectList();
				}
			}

			setState(680);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ParamsContext extends ParserRuleContext {
		public List<ParamContext> param() {
			return getRuleContexts(ParamContext.class);
		}
		public ParamContext param(int i) {
			return getRuleContext(ParamContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public ParamsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_params; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterParams(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitParams(this);
		}
	}

	public final ParamsContext params() throws RecognitionException {
		ParamsContext _localctx = new ParamsContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_params);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(682);
			param();
			setState(687);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(683);
				match(COMMA);
				setState(684);
				param();
				}
				}
				setState(689);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ParamContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public ParamContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_param; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterParam(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitParam(this);
		}
	}

	public final ParamContext param() throws RecognitionException {
		ParamContext _localctx = new ParamContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_param);
		int _la;
		try {
			setState(706);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__2:
			case QUANTUM:
			case CIRCUIT:
			case THIS:
			case SELF_LOWER:
			case SELF_UPPER:
			case INT_KW:
			case FLOAT_KW:
			case BOOL_KW:
			case STR_KW:
			case STRING_KW:
			case CHAR_KW:
			case VOID:
			case NANO:
			case AGENT:
			case EFFECT:
			case HANDLE:
			case REMEMBER:
			case RECALL:
			case LEARN:
			case INFER:
			case WISDOM:
			case ZAMANI:
			case SASA:
			case ANCESTOR:
			case LINEAR:
			case AFFINE:
			case LANGUAGE:
			case MTS_KW:
			case LEN:
			case PRINT:
			case PRINTLN:
			case ASSERT:
			case PANIC:
			case IDENT:
				enterOuterAlt(_localctx, 1);
				{
				setState(691);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__2) {
					{
					setState(690);
					match(T__2);
					}
				}

				setState(693);
				ident();
				setState(696);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COLON) {
					{
					setState(694);
					match(COLON);
					setState(695);
					typeExpr(0);
					}
				}

				setState(700);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==ASSIGN) {
					{
					setState(698);
					match(ASSIGN);
					setState(699);
					expression();
					}
				}

				}
				break;
			case T__3:
				enterOuterAlt(_localctx, 2);
				{
				setState(702);
				match(T__3);
				setState(703);
				typeExpr(0);
				setState(704);
				ident();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ModifiersContext extends ParserRuleContext {
		public List<ModifierContext> modifier() {
			return getRuleContexts(ModifierContext.class);
		}
		public ModifierContext modifier(int i) {
			return getRuleContext(ModifierContext.class,i);
		}
		public ModifiersContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_modifiers; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterModifiers(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitModifiers(this);
		}
	}

	public final ModifiersContext modifiers() throws RecognitionException {
		ModifiersContext _localctx = new ModifiersContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_modifiers);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(709); 
			_errHandler.sync(this);
			_alt = 1;
			do {
				switch (_alt) {
				case 1:
					{
					{
					setState(708);
					modifier();
					}
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				setState(711); 
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,28,_ctx);
			} while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER );
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ModifierContext extends ParserRuleContext {
		public TerminalNode PUB() { return getToken(ZamaniParser.PUB, 0); }
		public TerminalNode PRIVATE() { return getToken(ZamaniParser.PRIVATE, 0); }
		public TerminalNode PROTECTED() { return getToken(ZamaniParser.PROTECTED, 0); }
		public TerminalNode STATIC() { return getToken(ZamaniParser.STATIC, 0); }
		public TerminalNode CONST() { return getToken(ZamaniParser.CONST, 0); }
		public TerminalNode ASYNC() { return getToken(ZamaniParser.ASYNC, 0); }
		public TerminalNode UNSAFE() { return getToken(ZamaniParser.UNSAFE, 0); }
		public TerminalNode INLINE() { return getToken(ZamaniParser.INLINE, 0); }
		public TerminalNode OVERRIDE() { return getToken(ZamaniParser.OVERRIDE, 0); }
		public TerminalNode FINAL() { return getToken(ZamaniParser.FINAL, 0); }
		public TerminalNode ABSTRACT() { return getToken(ZamaniParser.ABSTRACT, 0); }
		public TerminalNode VIRTUAL() { return getToken(ZamaniParser.VIRTUAL, 0); }
		public TerminalNode SEALED() { return getToken(ZamaniParser.SEALED, 0); }
		public TerminalNode PARTIAL() { return getToken(ZamaniParser.PARTIAL, 0); }
		public TerminalNode FILE() { return getToken(ZamaniParser.FILE, 0); }
		public TerminalNode REQUIRED() { return getToken(ZamaniParser.REQUIRED, 0); }
		public TerminalNode INIT() { return getToken(ZamaniParser.INIT, 0); }
		public ModifierContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_modifier; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterModifier(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitModifier(this);
		}
	}

	public final ModifierContext modifier() throws RecognitionException {
		ModifierContext _localctx = new ModifierContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_modifier);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(713);
			_la = _input.LA(1);
			if ( !(((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ReturnStmtContext extends ParserRuleContext {
		public TerminalNode RETURN() { return getToken(ZamaniParser.RETURN, 0); }
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public ReturnStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_returnStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterReturnStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitReturnStmt(this);
		}
	}

	public final ReturnStmtContext returnStmt() throws RecognitionException {
		ReturnStmtContext _localctx = new ReturnStmtContext(_ctx, getState());
		enterRule(_localctx, 30, RULE_returnStmt);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(715);
			match(RETURN);
			setState(717);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				setState(716);
				expression();
				}
			}

			setState(719);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BreakStmtContext extends ParserRuleContext {
		public TerminalNode BREAK() { return getToken(ZamaniParser.BREAK, 0); }
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public BreakStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_breakStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBreakStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBreakStmt(this);
		}
	}

	public final BreakStmtContext breakStmt() throws RecognitionException {
		BreakStmtContext _localctx = new BreakStmtContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_breakStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(721);
			match(BREAK);
			setState(722);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ContinueStmtContext extends ParserRuleContext {
		public TerminalNode CONTINUE() { return getToken(ZamaniParser.CONTINUE, 0); }
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ContinueStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_continueStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterContinueStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitContinueStmt(this);
		}
	}

	public final ContinueStmtContext continueStmt() throws RecognitionException {
		ContinueStmtContext _localctx = new ContinueStmtContext(_ctx, getState());
		enterRule(_localctx, 34, RULE_continueStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(724);
			match(CONTINUE);
			setState(725);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class WhileStmtContext extends ParserRuleContext {
		public TerminalNode WHILE() { return getToken(ZamaniParser.WHILE, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public WhileStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_whileStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterWhileStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitWhileStmt(this);
		}
	}

	public final WhileStmtContext whileStmt() throws RecognitionException {
		WhileStmtContext _localctx = new WhileStmtContext(_ctx, getState());
		enterRule(_localctx, 36, RULE_whileStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(727);
			match(WHILE);
			setState(728);
			expression();
			setState(729);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ForStmtContext extends ParserRuleContext {
		public TerminalNode FOR() { return getToken(ZamaniParser.FOR, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode IN() { return getToken(ZamaniParser.IN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ForStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_forStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterForStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitForStmt(this);
		}
	}

	public final ForStmtContext forStmt() throws RecognitionException {
		ForStmtContext _localctx = new ForStmtContext(_ctx, getState());
		enterRule(_localctx, 38, RULE_forStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(731);
			match(FOR);
			setState(732);
			ident();
			setState(733);
			match(IN);
			setState(734);
			expression();
			setState(735);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LoopExprContext extends ParserRuleContext {
		public TerminalNode LOOP() { return getToken(ZamaniParser.LOOP, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public LoopExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_loopExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLoopExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLoopExpr(this);
		}
	}

	public final LoopExprContext loopExpr() throws RecognitionException {
		LoopExprContext _localctx = new LoopExprContext(_ctx, getState());
		enterRule(_localctx, 40, RULE_loopExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(737);
			match(LOOP);
			setState(738);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class IfExprContext extends ParserRuleContext {
		public TerminalNode IF() { return getToken(ZamaniParser.IF, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public List<BlockExprContext> blockExpr() {
			return getRuleContexts(BlockExprContext.class);
		}
		public BlockExprContext blockExpr(int i) {
			return getRuleContext(BlockExprContext.class,i);
		}
		public TerminalNode ELSE() { return getToken(ZamaniParser.ELSE, 0); }
		public IfExprContext ifExpr() {
			return getRuleContext(IfExprContext.class,0);
		}
		public IfExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_ifExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIfExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIfExpr(this);
		}
	}

	public final IfExprContext ifExpr() throws RecognitionException {
		IfExprContext _localctx = new IfExprContext(_ctx, getState());
		enterRule(_localctx, 42, RULE_ifExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(740);
			match(IF);
			setState(741);
			expression();
			setState(742);
			blockExpr();
			setState(748);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==ELSE) {
				{
				setState(743);
				match(ELSE);
				setState(746);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case IF:
					{
					setState(744);
					ifExpr();
					}
					break;
				case LBRACE:
					{
					setState(745);
					blockExpr();
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MatchStmtContext extends ParserRuleContext {
		public MatchExprContext matchExpr() {
			return getRuleContext(MatchExprContext.class,0);
		}
		public MatchStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_matchStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMatchStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMatchStmt(this);
		}
	}

	public final MatchStmtContext matchStmt() throws RecognitionException {
		MatchStmtContext _localctx = new MatchStmtContext(_ctx, getState());
		enterRule(_localctx, 44, RULE_matchStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(750);
			matchExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MatchExprContext extends ParserRuleContext {
		public TerminalNode MATCH() { return getToken(ZamaniParser.MATCH, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<MatchCaseContext> matchCase() {
			return getRuleContexts(MatchCaseContext.class);
		}
		public MatchCaseContext matchCase(int i) {
			return getRuleContext(MatchCaseContext.class,i);
		}
		public MatchExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_matchExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMatchExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMatchExpr(this);
		}
	}

	public final MatchExprContext matchExpr() throws RecognitionException {
		MatchExprContext _localctx = new MatchExprContext(_ctx, getState());
		enterRule(_localctx, 46, RULE_matchExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(752);
			match(MATCH);
			setState(753);
			expression();
			setState(754);
			match(LBRACE);
			setState(758);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__4 || _la==T__64 || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & 7L) != 0) || ((((_la - 262)) & ~0x3f) == 0 && ((1L << (_la - 262)) & 4544132024014929921L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 8589934627L) != 0)) {
				{
				{
				setState(755);
				matchCase();
				}
				}
				setState(760);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(761);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MatchCaseContext extends ParserRuleContext {
		public TerminalNode FATARROW() { return getToken(ZamaniParser.FATARROW, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public TerminalNode CASE() { return getToken(ZamaniParser.CASE, 0); }
		public PatternContext pattern() {
			return getRuleContext(PatternContext.class,0);
		}
		public TerminalNode WHEN() { return getToken(ZamaniParser.WHEN, 0); }
		public TerminalNode COMMA() { return getToken(ZamaniParser.COMMA, 0); }
		public MatchCaseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_matchCase; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMatchCase(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMatchCase(this);
		}
	}

	public final MatchCaseContext matchCase() throws RecognitionException {
		MatchCaseContext _localctx = new MatchCaseContext(_ctx, getState());
		enterRule(_localctx, 48, RULE_matchCase);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(766);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case CASE:
				{
				setState(763);
				match(CASE);
				setState(764);
				pattern(0);
				}
				break;
			case T__4:
			case T__64:
			case T__195:
			case T__196:
			case T__197:
			case QUANTUM:
			case CIRCUIT:
			case THIS:
			case SELF_LOWER:
			case SELF_UPPER:
			case INT_KW:
			case FLOAT_KW:
			case BOOL_KW:
			case STR_KW:
			case STRING_KW:
			case CHAR_KW:
			case VOID:
			case NANO:
			case AGENT:
			case EFFECT:
			case HANDLE:
			case REMEMBER:
			case RECALL:
			case LEARN:
			case INFER:
			case WISDOM:
			case ZAMANI:
			case SASA:
			case ANCESTOR:
			case LINEAR:
			case AFFINE:
			case LANGUAGE:
			case MTS_KW:
			case LEN:
			case PRINT:
			case PRINTLN:
			case ASSERT:
			case PANIC:
			case BOOLEAN:
			case NIL:
			case INTEGER:
			case FLOAT:
			case STRING:
			case CHAR:
			case IDENT:
			case LPAREN:
			case LBRACK:
			case PIPE:
				{
				setState(765);
				pattern(0);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			setState(770);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==WHEN) {
				{
				setState(768);
				match(WHEN);
				setState(769);
				expression();
				}
			}

			setState(772);
			match(FATARROW);
			setState(773);
			expression();
			setState(775);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COMMA) {
				{
				setState(774);
				match(COMMA);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PatternContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public LiteralContext literal() {
			return getRuleContext(LiteralContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public List<PatternContext> pattern() {
			return getRuleContexts(PatternContext.class);
		}
		public PatternContext pattern(int i) {
			return getRuleContext(PatternContext.class,i);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode PIPE() { return getToken(ZamaniParser.PIPE, 0); }
		public PatternContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_pattern; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPattern(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPattern(this);
		}
	}

	public final PatternContext pattern() throws RecognitionException {
		return pattern(0);
	}

	private PatternContext pattern(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		PatternContext _localctx = new PatternContext(_ctx, _parentState);
		PatternContext _prevctx = _localctx;
		int _startState = 50;
		enterRecursionRule(_localctx, 50, RULE_pattern, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(820);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,39,_ctx) ) {
			case 1:
				{
				setState(778);
				ident();
				}
				break;
			case 2:
				{
				setState(779);
				literal();
				}
				break;
			case 3:
				{
				setState(780);
				match(T__4);
				}
				break;
			case 4:
				{
				setState(781);
				match(LPAREN);
				setState(782);
				pattern(0);
				setState(787);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(783);
					match(COMMA);
					setState(784);
					pattern(0);
					}
					}
					setState(789);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(790);
				match(RPAREN);
				}
				break;
			case 5:
				{
				setState(792);
				match(LBRACK);
				setState(793);
				pattern(0);
				setState(798);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(794);
					match(COMMA);
					setState(795);
					pattern(0);
					}
					}
					setState(800);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(801);
				match(RBRACK);
				}
				break;
			case 6:
				{
				setState(803);
				match(LBRACK);
				setState(804);
				pattern(0);
				setState(809);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(805);
					match(COMMA);
					setState(806);
					pattern(0);
					}
					}
					setState(811);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(812);
				match(T__3);
				setState(813);
				pattern(0);
				setState(814);
				match(RBRACK);
				}
				break;
			case 7:
				{
				setState(816);
				ident();
				setState(817);
				match(COLON);
				setState(818);
				typeExpr(0);
				}
				break;
			}
			_ctx.stop = _input.LT(-1);
			setState(827);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,40,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					{
					_localctx = new PatternContext(_parentctx, _parentState);
					pushNewRecursionContext(_localctx, _startState, RULE_pattern);
					setState(822);
					if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
					setState(823);
					match(PIPE);
					setState(824);
					pattern(3);
					}
					} 
				}
				setState(829);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,40,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class UnsafeBlockContext extends ParserRuleContext {
		public TerminalNode UNSAFE() { return getToken(ZamaniParser.UNSAFE, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public TerminalNode BANG() { return getToken(ZamaniParser.BANG, 0); }
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public UnsafeBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_unsafeBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterUnsafeBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitUnsafeBlock(this);
		}
	}

	public final UnsafeBlockContext unsafeBlock() throws RecognitionException {
		UnsafeBlockContext _localctx = new UnsafeBlockContext(_ctx, getState());
		enterRule(_localctx, 52, RULE_unsafeBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(830);
			match(UNSAFE);
			setState(843);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case QUANTUM:
			case CIRCUIT:
			case THIS:
			case SELF_LOWER:
			case SELF_UPPER:
			case INT_KW:
			case FLOAT_KW:
			case BOOL_KW:
			case STR_KW:
			case STRING_KW:
			case CHAR_KW:
			case VOID:
			case NANO:
			case AGENT:
			case EFFECT:
			case HANDLE:
			case REMEMBER:
			case RECALL:
			case LEARN:
			case INFER:
			case WISDOM:
			case ZAMANI:
			case SASA:
			case ANCESTOR:
			case LINEAR:
			case AFFINE:
			case LANGUAGE:
			case MTS_KW:
			case LEN:
			case PRINT:
			case PRINTLN:
			case ASSERT:
			case PANIC:
			case IDENT:
			case LBRACE:
				{
				setState(832);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
					{
					setState(831);
					ident();
					}
				}

				setState(834);
				blockExpr();
				}
				break;
			case BANG:
				{
				setState(835);
				match(BANG);
				setState(836);
				match(LPAREN);
				setState(837);
				match(T__5);
				setState(838);
				match(COLON);
				setState(839);
				expression();
				setState(840);
				match(RPAREN);
				setState(841);
				blockExpr();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ThrowStmtContext extends ParserRuleContext {
		public TerminalNode THROW() { return getToken(ZamaniParser.THROW, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ThrowStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_throwStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterThrowStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitThrowStmt(this);
		}
	}

	public final ThrowStmtContext throwStmt() throws RecognitionException {
		ThrowStmtContext _localctx = new ThrowStmtContext(_ctx, getState());
		enterRule(_localctx, 54, RULE_throwStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(845);
			match(THROW);
			setState(846);
			expression();
			setState(847);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TryCatchStmtContext extends ParserRuleContext {
		public TerminalNode TRY() { return getToken(ZamaniParser.TRY, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public List<CatchClauseContext> catchClause() {
			return getRuleContexts(CatchClauseContext.class);
		}
		public CatchClauseContext catchClause(int i) {
			return getRuleContext(CatchClauseContext.class,i);
		}
		public FinallyClauseContext finallyClause() {
			return getRuleContext(FinallyClauseContext.class,0);
		}
		public TryCatchStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_tryCatchStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTryCatchStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTryCatchStmt(this);
		}
	}

	public final TryCatchStmtContext tryCatchStmt() throws RecognitionException {
		TryCatchStmtContext _localctx = new TryCatchStmtContext(_ctx, getState());
		enterRule(_localctx, 56, RULE_tryCatchStmt);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(849);
			match(TRY);
			setState(850);
			blockExpr();
			setState(854);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==CATCH) {
				{
				{
				setState(851);
				catchClause();
				}
				}
				setState(856);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(858);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==FINALLY) {
				{
				setState(857);
				finallyClause();
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CatchClauseContext extends ParserRuleContext {
		public TerminalNode CATCH() { return getToken(ZamaniParser.CATCH, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public ParamContext param() {
			return getRuleContext(ParamContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public CatchClauseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_catchClause; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCatchClause(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCatchClause(this);
		}
	}

	public final CatchClauseContext catchClause() throws RecognitionException {
		CatchClauseContext _localctx = new CatchClauseContext(_ctx, getState());
		enterRule(_localctx, 58, RULE_catchClause);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(860);
			match(CATCH);
			setState(865);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LPAREN) {
				{
				setState(861);
				match(LPAREN);
				setState(862);
				param();
				setState(863);
				match(RPAREN);
				}
			}

			setState(867);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class FinallyClauseContext extends ParserRuleContext {
		public TerminalNode FINALLY() { return getToken(ZamaniParser.FINALLY, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public FinallyClauseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_finallyClause; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterFinallyClause(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitFinallyClause(this);
		}
	}

	public final FinallyClauseContext finallyClause() throws RecognitionException {
		FinallyClauseContext _localctx = new FinallyClauseContext(_ctx, getState());
		enterRule(_localctx, 60, RULE_finallyClause);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(869);
			match(FINALLY);
			setState(870);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BlockExprContext extends ParserRuleContext {
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public BlockExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_blockExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBlockExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBlockExpr(this);
		}
	}

	public final BlockExprContext blockExpr() throws RecognitionException {
		BlockExprContext _localctx = new BlockExprContext(_ctx, getState());
		enterRule(_localctx, 62, RULE_blockExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(872);
			match(LBRACE);
			setState(876);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(873);
				statement();
				}
				}
				setState(878);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(879);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LetStmtContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public TerminalNode LET() { return getToken(ZamaniParser.LET, 0); }
		public TerminalNode VAR() { return getToken(ZamaniParser.VAR, 0); }
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public LetStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_letStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLetStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLetStmt(this);
		}
	}

	public final LetStmtContext letStmt() throws RecognitionException {
		LetStmtContext _localctx = new LetStmtContext(_ctx, getState());
		enterRule(_localctx, 64, RULE_letStmt);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(881);
			_la = _input.LA(1);
			if ( !(_la==LET || _la==VAR) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			setState(883);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2) {
				{
				setState(882);
				match(T__2);
				}
			}

			setState(885);
			ident();
			setState(888);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COLON) {
				{
				setState(886);
				match(COLON);
				setState(887);
				typeExpr(0);
				}
			}

			setState(890);
			match(ASSIGN);
			setState(891);
			expression();
			setState(892);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ConstStmtContext extends ParserRuleContext {
		public TerminalNode CONST() { return getToken(ZamaniParser.CONST, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public ConstStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_constStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterConstStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitConstStmt(this);
		}
	}

	public final ConstStmtContext constStmt() throws RecognitionException {
		ConstStmtContext _localctx = new ConstStmtContext(_ctx, getState());
		enterRule(_localctx, 66, RULE_constStmt);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(894);
			match(CONST);
			setState(895);
			ident();
			setState(898);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COLON) {
				{
				setState(896);
				match(COLON);
				setState(897);
				typeExpr(0);
				}
			}

			setState(900);
			match(ASSIGN);
			setState(901);
			expression();
			setState(902);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExpressionContext extends ParserRuleContext {
		public AssignmentExprContext assignmentExpr() {
			return getRuleContext(AssignmentExprContext.class,0);
		}
		public ExpressionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expression; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExpression(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExpression(this);
		}
	}

	public final ExpressionContext expression() throws RecognitionException {
		ExpressionContext _localctx = new ExpressionContext(_ctx, getState());
		enterRule(_localctx, 68, RULE_expression);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(904);
			assignmentExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AssignmentExprContext extends ParserRuleContext {
		public RangeExprContext rangeExpr() {
			return getRuleContext(RangeExprContext.class,0);
		}
		public AssignOpContext assignOp() {
			return getRuleContext(AssignOpContext.class,0);
		}
		public AssignmentExprContext assignmentExpr() {
			return getRuleContext(AssignmentExprContext.class,0);
		}
		public AssignmentExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_assignmentExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAssignmentExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAssignmentExpr(this);
		}
	}

	public final AssignmentExprContext assignmentExpr() throws RecognitionException {
		AssignmentExprContext _localctx = new AssignmentExprContext(_ctx, getState());
		enterRule(_localctx, 70, RULE_assignmentExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(906);
			rangeExpr();
			setState(910);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,50,_ctx) ) {
			case 1:
				{
				setState(907);
				assignOp();
				setState(908);
				assignmentExpr();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AssignOpContext extends ParserRuleContext {
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public TerminalNode PLUSEQ() { return getToken(ZamaniParser.PLUSEQ, 0); }
		public TerminalNode MINUSEQ() { return getToken(ZamaniParser.MINUSEQ, 0); }
		public TerminalNode STAREQ() { return getToken(ZamaniParser.STAREQ, 0); }
		public TerminalNode SLASHEQ() { return getToken(ZamaniParser.SLASHEQ, 0); }
		public AssignOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_assignOp; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAssignOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAssignOp(this);
		}
	}

	public final AssignOpContext assignOp() throws RecognitionException {
		AssignOpContext _localctx = new AssignOpContext(_ctx, getState());
		enterRule(_localctx, 72, RULE_assignOp);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(912);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 8064L) != 0) || ((((_la - 350)) & ~0x3f) == 0 && ((1L << (_la - 350)) & 491521L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RangeExprContext extends ParserRuleContext {
		public List<LogicalOrExprContext> logicalOrExpr() {
			return getRuleContexts(LogicalOrExprContext.class);
		}
		public LogicalOrExprContext logicalOrExpr(int i) {
			return getRuleContext(LogicalOrExprContext.class,i);
		}
		public TerminalNode DOTDOT() { return getToken(ZamaniParser.DOTDOT, 0); }
		public TerminalNode DOTDOTEQ() { return getToken(ZamaniParser.DOTDOTEQ, 0); }
		public RangeExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_rangeExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRangeExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRangeExpr(this);
		}
	}

	public final RangeExprContext rangeExpr() throws RecognitionException {
		RangeExprContext _localctx = new RangeExprContext(_ctx, getState());
		enterRule(_localctx, 74, RULE_rangeExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(914);
			logicalOrExpr();
			setState(917);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,51,_ctx) ) {
			case 1:
				{
				setState(915);
				_la = _input.LA(1);
				if ( !(_la==DOTDOT || _la==DOTDOTEQ) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(916);
				logicalOrExpr();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LogicalOrExprContext extends ParserRuleContext {
		public List<LogicalAndExprContext> logicalAndExpr() {
			return getRuleContexts(LogicalAndExprContext.class);
		}
		public LogicalAndExprContext logicalAndExpr(int i) {
			return getRuleContext(LogicalAndExprContext.class,i);
		}
		public List<TerminalNode> OROR() { return getTokens(ZamaniParser.OROR); }
		public TerminalNode OROR(int i) {
			return getToken(ZamaniParser.OROR, i);
		}
		public LogicalOrExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_logicalOrExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLogicalOrExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLogicalOrExpr(this);
		}
	}

	public final LogicalOrExprContext logicalOrExpr() throws RecognitionException {
		LogicalOrExprContext _localctx = new LogicalOrExprContext(_ctx, getState());
		enterRule(_localctx, 76, RULE_logicalOrExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(919);
			logicalAndExpr();
			setState(924);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,52,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(920);
					_la = _input.LA(1);
					if ( !(_la==T__12 || _la==OROR) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(921);
					logicalAndExpr();
					}
					} 
				}
				setState(926);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,52,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LogicalAndExprContext extends ParserRuleContext {
		public BitOrExprContext bitOrExpr() {
			return getRuleContext(BitOrExprContext.class,0);
		}
		public List<BitAndExprContext> bitAndExpr() {
			return getRuleContexts(BitAndExprContext.class);
		}
		public BitAndExprContext bitAndExpr(int i) {
			return getRuleContext(BitAndExprContext.class,i);
		}
		public List<TerminalNode> ANDAND() { return getTokens(ZamaniParser.ANDAND); }
		public TerminalNode ANDAND(int i) {
			return getToken(ZamaniParser.ANDAND, i);
		}
		public LogicalAndExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_logicalAndExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLogicalAndExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLogicalAndExpr(this);
		}
	}

	public final LogicalAndExprContext logicalAndExpr() throws RecognitionException {
		LogicalAndExprContext _localctx = new LogicalAndExprContext(_ctx, getState());
		enterRule(_localctx, 78, RULE_logicalAndExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(927);
			bitOrExpr();
			setState(932);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,53,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(928);
					_la = _input.LA(1);
					if ( !(_la==T__13 || _la==ANDAND) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(929);
					bitAndExpr();
					}
					} 
				}
				setState(934);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,53,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BitOrExprContext extends ParserRuleContext {
		public List<BitXorExprContext> bitXorExpr() {
			return getRuleContexts(BitXorExprContext.class);
		}
		public BitXorExprContext bitXorExpr(int i) {
			return getRuleContext(BitXorExprContext.class,i);
		}
		public List<TerminalNode> PIPE() { return getTokens(ZamaniParser.PIPE); }
		public TerminalNode PIPE(int i) {
			return getToken(ZamaniParser.PIPE, i);
		}
		public BitOrExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_bitOrExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBitOrExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBitOrExpr(this);
		}
	}

	public final BitOrExprContext bitOrExpr() throws RecognitionException {
		BitOrExprContext _localctx = new BitOrExprContext(_ctx, getState());
		enterRule(_localctx, 80, RULE_bitOrExpr);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(935);
			bitXorExpr();
			setState(940);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,54,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(936);
					match(PIPE);
					setState(937);
					bitXorExpr();
					}
					} 
				}
				setState(942);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,54,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BitXorExprContext extends ParserRuleContext {
		public List<BitAndExprContext> bitAndExpr() {
			return getRuleContexts(BitAndExprContext.class);
		}
		public BitAndExprContext bitAndExpr(int i) {
			return getRuleContext(BitAndExprContext.class,i);
		}
		public List<TerminalNode> CARET() { return getTokens(ZamaniParser.CARET); }
		public TerminalNode CARET(int i) {
			return getToken(ZamaniParser.CARET, i);
		}
		public BitXorExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_bitXorExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBitXorExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBitXorExpr(this);
		}
	}

	public final BitXorExprContext bitXorExpr() throws RecognitionException {
		BitXorExprContext _localctx = new BitXorExprContext(_ctx, getState());
		enterRule(_localctx, 82, RULE_bitXorExpr);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(943);
			bitAndExpr();
			setState(948);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,55,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(944);
					match(CARET);
					setState(945);
					bitAndExpr();
					}
					} 
				}
				setState(950);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,55,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BitAndExprContext extends ParserRuleContext {
		public List<EqualityExprContext> equalityExpr() {
			return getRuleContexts(EqualityExprContext.class);
		}
		public EqualityExprContext equalityExpr(int i) {
			return getRuleContext(EqualityExprContext.class,i);
		}
		public List<TerminalNode> AMP() { return getTokens(ZamaniParser.AMP); }
		public TerminalNode AMP(int i) {
			return getToken(ZamaniParser.AMP, i);
		}
		public BitAndExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_bitAndExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBitAndExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBitAndExpr(this);
		}
	}

	public final BitAndExprContext bitAndExpr() throws RecognitionException {
		BitAndExprContext _localctx = new BitAndExprContext(_ctx, getState());
		enterRule(_localctx, 84, RULE_bitAndExpr);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(951);
			equalityExpr();
			setState(956);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,56,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(952);
					match(AMP);
					setState(953);
					equalityExpr();
					}
					} 
				}
				setState(958);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,56,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EqualityExprContext extends ParserRuleContext {
		public List<ComparisonExprContext> comparisonExpr() {
			return getRuleContexts(ComparisonExprContext.class);
		}
		public ComparisonExprContext comparisonExpr(int i) {
			return getRuleContext(ComparisonExprContext.class,i);
		}
		public List<TerminalNode> EQ() { return getTokens(ZamaniParser.EQ); }
		public TerminalNode EQ(int i) {
			return getToken(ZamaniParser.EQ, i);
		}
		public List<TerminalNode> NEQ() { return getTokens(ZamaniParser.NEQ); }
		public TerminalNode NEQ(int i) {
			return getToken(ZamaniParser.NEQ, i);
		}
		public EqualityExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_equalityExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEqualityExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEqualityExpr(this);
		}
	}

	public final EqualityExprContext equalityExpr() throws RecognitionException {
		EqualityExprContext _localctx = new EqualityExprContext(_ctx, getState());
		enterRule(_localctx, 86, RULE_equalityExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(959);
			comparisonExpr();
			setState(964);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,57,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(960);
					_la = _input.LA(1);
					if ( !(_la==T__14 || _la==T__15 || _la==EQ || _la==NEQ) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(961);
					comparisonExpr();
					}
					} 
				}
				setState(966);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,57,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ComparisonExprContext extends ParserRuleContext {
		public List<ShiftExprContext> shiftExpr() {
			return getRuleContexts(ShiftExprContext.class);
		}
		public ShiftExprContext shiftExpr(int i) {
			return getRuleContext(ShiftExprContext.class,i);
		}
		public List<TerminalNode> LT() { return getTokens(ZamaniParser.LT); }
		public TerminalNode LT(int i) {
			return getToken(ZamaniParser.LT, i);
		}
		public List<TerminalNode> LE() { return getTokens(ZamaniParser.LE); }
		public TerminalNode LE(int i) {
			return getToken(ZamaniParser.LE, i);
		}
		public List<TerminalNode> GT() { return getTokens(ZamaniParser.GT); }
		public TerminalNode GT(int i) {
			return getToken(ZamaniParser.GT, i);
		}
		public List<TerminalNode> GE() { return getTokens(ZamaniParser.GE); }
		public TerminalNode GE(int i) {
			return getToken(ZamaniParser.GE, i);
		}
		public ComparisonExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_comparisonExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterComparisonExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitComparisonExpr(this);
		}
	}

	public final ComparisonExprContext comparisonExpr() throws RecognitionException {
		ComparisonExprContext _localctx = new ComparisonExprContext(_ctx, getState());
		enterRule(_localctx, 88, RULE_comparisonExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(967);
			shiftExpr();
			setState(972);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,58,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(968);
					_la = _input.LA(1);
					if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 917504L) != 0) || ((((_la - 353)) & ~0x3f) == 0 && ((1L << (_la - 353)) & 15L) != 0)) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(969);
					shiftExpr();
					}
					} 
				}
				setState(974);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,58,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ShiftExprContext extends ParserRuleContext {
		public List<SumExprContext> sumExpr() {
			return getRuleContexts(SumExprContext.class);
		}
		public SumExprContext sumExpr(int i) {
			return getRuleContext(SumExprContext.class,i);
		}
		public List<TerminalNode> SHL() { return getTokens(ZamaniParser.SHL); }
		public TerminalNode SHL(int i) {
			return getToken(ZamaniParser.SHL, i);
		}
		public List<TerminalNode> SHR() { return getTokens(ZamaniParser.SHR); }
		public TerminalNode SHR(int i) {
			return getToken(ZamaniParser.SHR, i);
		}
		public List<TerminalNode> SHRU() { return getTokens(ZamaniParser.SHRU); }
		public TerminalNode SHRU(int i) {
			return getToken(ZamaniParser.SHRU, i);
		}
		public ShiftExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_shiftExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterShiftExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitShiftExpr(this);
		}
	}

	public final ShiftExprContext shiftExpr() throws RecognitionException {
		ShiftExprContext _localctx = new ShiftExprContext(_ctx, getState());
		enterRule(_localctx, 90, RULE_shiftExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(975);
			sumExpr();
			setState(980);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,59,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(976);
					_la = _input.LA(1);
					if ( !(((((_la - 362)) & ~0x3f) == 0 && ((1L << (_la - 362)) & 7L) != 0)) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(977);
					sumExpr();
					}
					} 
				}
				setState(982);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,59,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SumExprContext extends ParserRuleContext {
		public List<ProductExprContext> productExpr() {
			return getRuleContexts(ProductExprContext.class);
		}
		public ProductExprContext productExpr(int i) {
			return getRuleContext(ProductExprContext.class,i);
		}
		public List<TerminalNode> PLUS() { return getTokens(ZamaniParser.PLUS); }
		public TerminalNode PLUS(int i) {
			return getToken(ZamaniParser.PLUS, i);
		}
		public List<TerminalNode> MINUS() { return getTokens(ZamaniParser.MINUS); }
		public TerminalNode MINUS(int i) {
			return getToken(ZamaniParser.MINUS, i);
		}
		public SumExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sumExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSumExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSumExpr(this);
		}
	}

	public final SumExprContext sumExpr() throws RecognitionException {
		SumExprContext _localctx = new SumExprContext(_ctx, getState());
		enterRule(_localctx, 92, RULE_sumExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(983);
			productExpr();
			setState(988);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,60,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(984);
					_la = _input.LA(1);
					if ( !(_la==PLUS || _la==MINUS) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(985);
					productExpr();
					}
					} 
				}
				setState(990);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,60,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProductExprContext extends ParserRuleContext {
		public List<CastExprContext> castExpr() {
			return getRuleContexts(CastExprContext.class);
		}
		public CastExprContext castExpr(int i) {
			return getRuleContext(CastExprContext.class,i);
		}
		public List<TerminalNode> STAR() { return getTokens(ZamaniParser.STAR); }
		public TerminalNode STAR(int i) {
			return getToken(ZamaniParser.STAR, i);
		}
		public List<TerminalNode> SLASH() { return getTokens(ZamaniParser.SLASH); }
		public TerminalNode SLASH(int i) {
			return getToken(ZamaniParser.SLASH, i);
		}
		public List<TerminalNode> PERCENT() { return getTokens(ZamaniParser.PERCENT); }
		public TerminalNode PERCENT(int i) {
			return getToken(ZamaniParser.PERCENT, i);
		}
		public ProductExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_productExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterProductExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitProductExpr(this);
		}
	}

	public final ProductExprContext productExpr() throws RecognitionException {
		ProductExprContext _localctx = new ProductExprContext(_ctx, getState());
		enterRule(_localctx, 94, RULE_productExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(991);
			castExpr();
			setState(996);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,61,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(992);
					_la = _input.LA(1);
					if ( !(((((_la - 347)) & ~0x3f) == 0 && ((1L << (_la - 347)) & 7L) != 0)) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(993);
					castExpr();
					}
					} 
				}
				setState(998);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,61,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CastExprContext extends ParserRuleContext {
		public PrefixExprContext prefixExpr() {
			return getRuleContext(PrefixExprContext.class,0);
		}
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public List<TerminalNode> AS() { return getTokens(ZamaniParser.AS); }
		public TerminalNode AS(int i) {
			return getToken(ZamaniParser.AS, i);
		}
		public List<TerminalNode> COLON() { return getTokens(ZamaniParser.COLON); }
		public TerminalNode COLON(int i) {
			return getToken(ZamaniParser.COLON, i);
		}
		public CastExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_castExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCastExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCastExpr(this);
		}
	}

	public final CastExprContext castExpr() throws RecognitionException {
		CastExprContext _localctx = new CastExprContext(_ctx, getState());
		enterRule(_localctx, 96, RULE_castExpr);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(999);
			prefixExpr();
			setState(1004);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,62,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(1000);
					_la = _input.LA(1);
					if ( !(_la==AS || _la==COLON) ) {
					_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					setState(1001);
					typeExpr(0);
					}
					} 
				}
				setState(1006);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,62,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PrefixExprContext extends ParserRuleContext {
		public PrefixExprContext prefixExpr() {
			return getRuleContext(PrefixExprContext.class,0);
		}
		public TerminalNode MINUS() { return getToken(ZamaniParser.MINUS, 0); }
		public TerminalNode BANG() { return getToken(ZamaniParser.BANG, 0); }
		public TerminalNode TILDE() { return getToken(ZamaniParser.TILDE, 0); }
		public TerminalNode AMP() { return getToken(ZamaniParser.AMP, 0); }
		public TerminalNode STAR() { return getToken(ZamaniParser.STAR, 0); }
		public PostfixExprContext postfixExpr() {
			return getRuleContext(PostfixExprContext.class,0);
		}
		public PrefixExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_prefixExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPrefixExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPrefixExpr(this);
		}
	}

	public final PrefixExprContext prefixExpr() throws RecognitionException {
		PrefixExprContext _localctx = new PrefixExprContext(_ctx, getState());
		enterRule(_localctx, 98, RULE_prefixExpr);
		int _la;
		try {
			setState(1021);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__19:
			case T__20:
			case TILDE:
			case BANG:
			case MINUS:
			case STAR:
			case AMP:
				enterOuterAlt(_localctx, 1);
				{
				setState(1017);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case MINUS:
					{
					setState(1007);
					match(MINUS);
					}
					break;
				case BANG:
					{
					setState(1008);
					match(BANG);
					}
					break;
				case TILDE:
					{
					setState(1009);
					match(TILDE);
					}
					break;
				case AMP:
					{
					setState(1010);
					match(AMP);
					setState(1012);
					_errHandler.sync(this);
					_la = _input.LA(1);
					if (_la==T__2) {
						{
						setState(1011);
						match(T__2);
						}
					}

					}
					break;
				case STAR:
					{
					setState(1014);
					match(STAR);
					}
					break;
				case T__19:
					{
					setState(1015);
					match(T__19);
					}
					break;
				case T__20:
					{
					setState(1016);
					match(T__20);
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				setState(1019);
				prefixExpr();
				}
				break;
			case T__21:
			case T__22:
			case T__23:
			case T__24:
			case T__25:
			case T__26:
			case T__29:
			case T__30:
			case T__31:
			case T__32:
			case T__33:
			case T__34:
			case T__35:
			case T__36:
			case T__37:
			case T__39:
			case T__40:
			case T__41:
			case T__42:
			case T__43:
			case T__44:
			case T__64:
			case T__195:
			case T__196:
			case T__197:
			case T__198:
			case FN:
			case ASYNC:
			case LOOP:
			case IF:
			case MATCH:
			case TRY:
			case QUANTUM:
			case CIRCUIT:
			case THIS:
			case SELF_LOWER:
			case SELF_UPPER:
			case INT_KW:
			case FLOAT_KW:
			case BOOL_KW:
			case STR_KW:
			case STRING_KW:
			case CHAR_KW:
			case VOID:
			case NANO:
			case AGENT:
			case EFFECT:
			case HANDLE:
			case REMEMBER:
			case RECALL:
			case LEARN:
			case INFER:
			case WISDOM:
			case ZAMANI:
			case SASA:
			case ANCESTOR:
			case LINEAR:
			case AFFINE:
			case LANGUAGE:
			case MTS_KW:
			case LEN:
			case PRINT:
			case PRINTLN:
			case ASSERT:
			case PANIC:
			case BOOLEAN:
			case NIL:
			case INTEGER:
			case FLOAT:
			case STRING:
			case CHAR:
			case IDENT:
			case LPAREN:
			case LBRACE:
			case LBRACK:
			case PIPE:
				enterOuterAlt(_localctx, 2);
				{
				setState(1020);
				postfixExpr();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PostfixExprContext extends ParserRuleContext {
		public PrimaryExprContext primaryExpr() {
			return getRuleContext(PrimaryExprContext.class,0);
		}
		public List<PostfixOpContext> postfixOp() {
			return getRuleContexts(PostfixOpContext.class);
		}
		public PostfixOpContext postfixOp(int i) {
			return getRuleContext(PostfixOpContext.class,i);
		}
		public PostfixExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_postfixExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPostfixExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPostfixExpr(this);
		}
	}

	public final PostfixExprContext postfixExpr() throws RecognitionException {
		PostfixExprContext _localctx = new PostfixExprContext(_ctx, getState());
		enterRule(_localctx, 100, RULE_postfixExpr);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(1023);
			primaryExpr();
			setState(1027);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,66,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(1024);
					postfixOp();
					}
					} 
				}
				setState(1029);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,66,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PostfixOpContext extends ParserRuleContext {
		public PostfixOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_postfixOp; }
	 
		public PostfixOpContext() { }
		public void copyFrom(PostfixOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class MemberOpContext extends PostfixOpContext {
		public TerminalNode DOT() { return getToken(ZamaniParser.DOT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public MemberOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMemberOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMemberOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class WithBlockOpContext extends PostfixOpContext {
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public List<TerminalNode> COLON() { return getTokens(ZamaniParser.COLON); }
		public TerminalNode COLON(int i) {
			return getToken(ZamaniParser.COLON, i);
		}
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> SEMI() { return getTokens(ZamaniParser.SEMI); }
		public TerminalNode SEMI(int i) {
			return getToken(ZamaniParser.SEMI, i);
		}
		public WithBlockOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterWithBlockOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitWithBlockOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class CallOpContext extends PostfixOpContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public CallOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCallOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCallOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class PostIncOpContext extends PostfixOpContext {
		public PostIncOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPostIncOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPostIncOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class WithEffectOpContext extends PostfixOpContext {
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public EffectListContext effectList() {
			return getRuleContext(EffectListContext.class,0);
		}
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public WithEffectOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterWithEffectOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitWithEffectOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class IndexOpContext extends PostfixOpContext {
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public IndexOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIndexOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIndexOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class TryPropagateOpContext extends PostfixOpContext {
		public TerminalNode QUESTION() { return getToken(ZamaniParser.QUESTION, 0); }
		public TryPropagateOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTryPropagateOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTryPropagateOp(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class PostDecOpContext extends PostfixOpContext {
		public PostDecOpContext(PostfixOpContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPostDecOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPostDecOp(this);
		}
	}

	public final PostfixOpContext postfixOp() throws RecognitionException {
		PostfixOpContext _localctx = new PostfixOpContext(_ctx, getState());
		enterRule(_localctx, 102, RULE_postfixOp);
		int _la;
		try {
			setState(1069);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,71,_ctx) ) {
			case 1:
				_localctx = new CallOpContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(1030);
				match(LPAREN);
				setState(1032);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
					{
					setState(1031);
					args();
					}
				}

				setState(1034);
				match(RPAREN);
				}
				break;
			case 2:
				_localctx = new IndexOpContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(1035);
				match(LBRACK);
				setState(1036);
				expression();
				setState(1037);
				match(RBRACK);
				}
				break;
			case 3:
				_localctx = new MemberOpContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(1039);
				match(DOT);
				setState(1040);
				ident();
				setState(1046);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,69,_ctx) ) {
				case 1:
					{
					setState(1041);
					match(LPAREN);
					setState(1043);
					_errHandler.sync(this);
					_la = _input.LA(1);
					if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
						{
						setState(1042);
						args();
						}
					}

					setState(1045);
					match(RPAREN);
					}
					break;
				}
				}
				break;
			case 4:
				_localctx = new TryPropagateOpContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(1048);
				match(QUESTION);
				}
				break;
			case 5:
				_localctx = new PostIncOpContext(_localctx);
				enterOuterAlt(_localctx, 5);
				{
				setState(1049);
				match(T__19);
				}
				break;
			case 6:
				_localctx = new PostDecOpContext(_localctx);
				enterOuterAlt(_localctx, 6);
				{
				setState(1050);
				match(T__20);
				}
				break;
			case 7:
				_localctx = new WithEffectOpContext(_localctx);
				enterOuterAlt(_localctx, 7);
				{
				setState(1051);
				match(T__1);
				setState(1052);
				match(LBRACK);
				setState(1053);
				effectList();
				setState(1054);
				match(RBRACK);
				}
				break;
			case 8:
				_localctx = new WithBlockOpContext(_localctx);
				enterOuterAlt(_localctx, 8);
				{
				setState(1056);
				match(T__1);
				setState(1057);
				match(LBRACE);
				setState(1065);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
					{
					{
					setState(1058);
					ident();
					setState(1059);
					match(COLON);
					setState(1060);
					expression();
					setState(1061);
					match(SEMI);
					}
					}
					setState(1067);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(1068);
				match(RBRACE);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ArgsContext extends ParserRuleContext {
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public List<NamedArgumentContext> namedArgument() {
			return getRuleContexts(NamedArgumentContext.class);
		}
		public NamedArgumentContext namedArgument(int i) {
			return getRuleContext(NamedArgumentContext.class,i);
		}
		public ArgsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_args; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterArgs(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitArgs(this);
		}
	}

	public final ArgsContext args() throws RecognitionException {
		ArgsContext _localctx = new ArgsContext(_ctx, getState());
		enterRule(_localctx, 104, RULE_args);
		int _la;
		try {
			setState(1087);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,74,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(1071);
				expression();
				setState(1076);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(1072);
					match(COMMA);
					setState(1073);
					expression();
					}
					}
					setState(1078);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(1079);
				namedArgument();
				setState(1084);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(1080);
					match(COMMA);
					setState(1081);
					namedArgument();
					}
					}
					setState(1086);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class NamedArgumentContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public NamedArgumentContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_namedArgument; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNamedArgument(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNamedArgument(this);
		}
	}

	public final NamedArgumentContext namedArgument() throws RecognitionException {
		NamedArgumentContext _localctx = new NamedArgumentContext(_ctx, getState());
		enterRule(_localctx, 106, RULE_namedArgument);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1089);
			ident();
			setState(1090);
			match(ASSIGN);
			setState(1091);
			expression();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PrimaryExprContext extends ParserRuleContext {
		public PrimaryExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_primaryExpr; }
	 
		public PrimaryExprContext() { }
		public void copyFrom(PrimaryExprContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class NewExprContext extends PrimaryExprContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TypeArgsContext typeArgs() {
			return getRuleContext(TypeArgsContext.class,0);
		}
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public NewExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNewExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNewExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ThisExprContext extends PrimaryExprContext {
		public TerminalNode THIS() { return getToken(ZamaniParser.THIS, 0); }
		public ThisExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterThisExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitThisExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class PerformValExprContext extends PrimaryExprContext {
		public PerformExprContext performExpr() {
			return getRuleContext(PerformExprContext.class,0);
		}
		public PerformValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPerformValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPerformValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class MatchValExprContext extends PrimaryExprContext {
		public MatchExprContext matchExpr() {
			return getRuleContext(MatchExprContext.class,0);
		}
		public MatchValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMatchValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMatchValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LoopValExprContext extends PrimaryExprContext {
		public LoopExprContext loopExpr() {
			return getRuleContext(LoopExprContext.class,0);
		}
		public LoopValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLoopValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLoopValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ZamaniValExprContext extends PrimaryExprContext {
		public ZamaniExprContext zamaniExpr() {
			return getRuleContext(ZamaniExprContext.class,0);
		}
		public ZamaniValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterZamaniValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitZamaniValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class SpawnValExprContext extends PrimaryExprContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public SpawnValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSpawnValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSpawnValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ParenExprContext extends PrimaryExprContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ParenExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterParenExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitParenExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class IdentExprContext extends PrimaryExprContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public StructLiteralTailContext structLiteralTail() {
			return getRuleContext(StructLiteralTailContext.class,0);
		}
		public IdentExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIdentExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIdentExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ConsensusValExprContext extends PrimaryExprContext {
		public ConsensusExprContext consensusExpr() {
			return getRuleContext(ConsensusExprContext.class,0);
		}
		public ConsensusValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterConsensusValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitConsensusValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class YieldExprContext extends PrimaryExprContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public YieldExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterYieldExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitYieldExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LambdaExprContext extends PrimaryExprContext {
		public List<TerminalNode> PIPE() { return getTokens(ZamaniParser.PIPE); }
		public TerminalNode PIPE(int i) {
			return getToken(ZamaniParser.PIPE, i);
		}
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public LambdaExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLambdaExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLambdaExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LiteralExprContext extends PrimaryExprContext {
		public LiteralContext literal() {
			return getRuleContext(LiteralContext.class,0);
		}
		public LiteralExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLiteralExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLiteralExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class AsyncExprContext extends PrimaryExprContext {
		public TerminalNode ASYNC() { return getToken(ZamaniParser.ASYNC, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public AsyncExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAsyncExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAsyncExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class IfValExprContext extends PrimaryExprContext {
		public IfExprContext ifExpr() {
			return getRuleContext(IfExprContext.class,0);
		}
		public IfValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIfValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIfValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ArrayExprContext extends PrimaryExprContext {
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public ArrayExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterArrayExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitArrayExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class AnonFnExprContext extends PrimaryExprContext {
		public TerminalNode FN() { return getToken(ZamaniParser.FN, 0); }
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public TerminalNode ARROW() { return getToken(ZamaniParser.ARROW, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public AnonFnExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAnonFnExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAnonFnExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class InterpStringExprContext extends PrimaryExprContext {
		public InterpolatedStringContext interpolatedString() {
			return getRuleContext(InterpolatedStringContext.class,0);
		}
		public InterpStringExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterInterpStringExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitInterpStringExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class RecallValExprContext extends PrimaryExprContext {
		public RecallExprContext recallExpr() {
			return getRuleContext(RecallExprContext.class,0);
		}
		public RecallValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRecallValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRecallValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class QuantumOpValExprContext extends PrimaryExprContext {
		public QuantumOpExprContext quantumOpExpr() {
			return getRuleContext(QuantumOpExprContext.class,0);
		}
		public QuantumOpValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterQuantumOpValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitQuantumOpValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class SelfExprContext extends PrimaryExprContext {
		public TerminalNode SELF_LOWER() { return getToken(ZamaniParser.SELF_LOWER, 0); }
		public SelfExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSelfExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSelfExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class MapExprContext extends PrimaryExprContext {
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> FATARROW() { return getTokens(ZamaniParser.FATARROW); }
		public TerminalNode FATARROW(int i) {
			return getToken(ZamaniParser.FATARROW, i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public MapExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMapExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMapExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class SasaValExprContext extends PrimaryExprContext {
		public SasaExprContext sasaExpr() {
			return getRuleContext(SasaExprContext.class,0);
		}
		public SasaValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSasaValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSasaValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class MacroCallExprContext extends PrimaryExprContext {
		public MacroCallContext macroCall() {
			return getRuleContext(MacroCallContext.class,0);
		}
		public MacroCallExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMacroCallExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMacroCallExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class AwaitExprContext extends PrimaryExprContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public AwaitExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAwaitExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAwaitExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class MopValExprContext extends PrimaryExprContext {
		public MopExprContext mopExpr() {
			return getRuleContext(MopExprContext.class,0);
		}
		public MopValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMopValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMopValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class TryCatchExprContext extends PrimaryExprContext {
		public TerminalNode TRY() { return getToken(ZamaniParser.TRY, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public List<CatchClauseContext> catchClause() {
			return getRuleContexts(CatchClauseContext.class);
		}
		public CatchClauseContext catchClause(int i) {
			return getRuleContext(CatchClauseContext.class,i);
		}
		public TryCatchExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTryCatchExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTryCatchExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class NanoValExprContext extends PrimaryExprContext {
		public NanoExprContext nanoExpr() {
			return getRuleContext(NanoExprContext.class,0);
		}
		public NanoValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNanoValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNanoValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class SuperExprContext extends PrimaryExprContext {
		public TerminalNode DOT() { return getToken(ZamaniParser.DOT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public SuperExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSuperExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSuperExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class BlockValExprContext extends PrimaryExprContext {
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public BlockValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBlockValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBlockValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class MtsValExprContext extends PrimaryExprContext {
		public MtsExprContext mtsExpr() {
			return getRuleContext(MtsExprContext.class,0);
		}
		public MtsValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMtsValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMtsValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LearnValExprContext extends PrimaryExprContext {
		public LearnExprContext learnExpr() {
			return getRuleContext(LearnExprContext.class,0);
		}
		public LearnValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLearnValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLearnValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class AncestorValExprContext extends PrimaryExprContext {
		public AncestorCallContext ancestorCall() {
			return getRuleContext(AncestorCallContext.class,0);
		}
		public AncestorValExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAncestorValExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAncestorValExpr(this);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class TupleExprContext extends PrimaryExprContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public TupleExprContext(PrimaryExprContext ctx) { copyFrom(ctx); }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTupleExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTupleExpr(this);
		}
	}

	public final PrimaryExprContext primaryExpr() throws RecognitionException {
		PrimaryExprContext _localctx = new PrimaryExprContext(_ctx, getState());
		enterRule(_localctx, 108, RULE_primaryExpr);
		int _la;
		try {
			int _alt;
			setState(1220);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,91,_ctx) ) {
			case 1:
				_localctx = new IdentExprContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(1093);
				ident();
				setState(1095);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,75,_ctx) ) {
				case 1:
					{
					setState(1094);
					structLiteralTail();
					}
					break;
				}
				}
				break;
			case 2:
				_localctx = new LiteralExprContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(1097);
				literal();
				}
				break;
			case 3:
				_localctx = new ParenExprContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(1098);
				match(LPAREN);
				setState(1099);
				expression();
				setState(1100);
				match(RPAREN);
				}
				break;
			case 4:
				_localctx = new TupleExprContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(1102);
				match(LPAREN);
				setState(1103);
				expression();
				setState(1106); 
				_errHandler.sync(this);
				_la = _input.LA(1);
				do {
					{
					{
					setState(1104);
					match(COMMA);
					setState(1105);
					expression();
					}
					}
					setState(1108); 
					_errHandler.sync(this);
					_la = _input.LA(1);
				} while ( _la==COMMA );
				setState(1110);
				match(RPAREN);
				}
				break;
			case 5:
				_localctx = new ArrayExprContext(_localctx);
				enterOuterAlt(_localctx, 5);
				{
				setState(1112);
				match(LBRACK);
				setState(1121);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
					{
					setState(1113);
					expression();
					setState(1118);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==COMMA) {
						{
						{
						setState(1114);
						match(COMMA);
						setState(1115);
						expression();
						}
						}
						setState(1120);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					}
				}

				setState(1123);
				match(RBRACK);
				}
				break;
			case 6:
				_localctx = new MapExprContext(_localctx);
				enterOuterAlt(_localctx, 6);
				{
				setState(1124);
				match(T__21);
				setState(1125);
				match(LBRACE);
				setState(1139);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
					{
					setState(1126);
					expression();
					setState(1127);
					match(FATARROW);
					setState(1128);
					expression();
					setState(1136);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==COMMA) {
						{
						{
						setState(1129);
						match(COMMA);
						setState(1130);
						expression();
						setState(1131);
						match(FATARROW);
						setState(1132);
						expression();
						}
						}
						setState(1138);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					}
				}

				setState(1141);
				match(RBRACE);
				}
				break;
			case 7:
				_localctx = new BlockValExprContext(_localctx);
				enterOuterAlt(_localctx, 7);
				{
				setState(1142);
				blockExpr();
				}
				break;
			case 8:
				_localctx = new LambdaExprContext(_localctx);
				enterOuterAlt(_localctx, 8);
				{
				setState(1143);
				match(PIPE);
				setState(1145);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
					{
					setState(1144);
					params();
					}
				}

				setState(1147);
				match(PIPE);
				setState(1150);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,82,_ctx) ) {
				case 1:
					{
					setState(1148);
					blockExpr();
					}
					break;
				case 2:
					{
					setState(1149);
					expression();
					}
					break;
				}
				}
				break;
			case 9:
				_localctx = new AnonFnExprContext(_localctx);
				enterOuterAlt(_localctx, 9);
				{
				setState(1152);
				match(FN);
				setState(1153);
				match(LPAREN);
				setState(1155);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
					{
					setState(1154);
					params();
					}
				}

				setState(1157);
				match(RPAREN);
				setState(1160);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==ARROW) {
					{
					setState(1158);
					match(ARROW);
					setState(1159);
					typeExpr(0);
					}
				}

				setState(1162);
				blockExpr();
				}
				break;
			case 10:
				_localctx = new IfValExprContext(_localctx);
				enterOuterAlt(_localctx, 10);
				{
				setState(1163);
				ifExpr();
				}
				break;
			case 11:
				_localctx = new MatchValExprContext(_localctx);
				enterOuterAlt(_localctx, 11);
				{
				setState(1164);
				matchExpr();
				}
				break;
			case 12:
				_localctx = new LoopValExprContext(_localctx);
				enterOuterAlt(_localctx, 12);
				{
				setState(1165);
				loopExpr();
				}
				break;
			case 13:
				_localctx = new AsyncExprContext(_localctx);
				enterOuterAlt(_localctx, 13);
				{
				setState(1166);
				match(ASYNC);
				setState(1167);
				expression();
				}
				break;
			case 14:
				_localctx = new AwaitExprContext(_localctx);
				enterOuterAlt(_localctx, 14);
				{
				setState(1168);
				match(T__22);
				setState(1169);
				expression();
				}
				break;
			case 15:
				_localctx = new SpawnValExprContext(_localctx);
				enterOuterAlt(_localctx, 15);
				{
				setState(1170);
				match(T__23);
				setState(1171);
				expression();
				}
				break;
			case 16:
				_localctx = new NewExprContext(_localctx);
				enterOuterAlt(_localctx, 16);
				{
				setState(1172);
				match(T__24);
				setState(1173);
				ident();
				setState(1175);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==LT) {
					{
					setState(1174);
					typeArgs();
					}
				}

				setState(1177);
				match(LPAREN);
				setState(1179);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
					{
					setState(1178);
					args();
					}
				}

				setState(1181);
				match(RPAREN);
				}
				break;
			case 17:
				_localctx = new TryCatchExprContext(_localctx);
				enterOuterAlt(_localctx, 17);
				{
				setState(1183);
				match(TRY);
				setState(1184);
				expression();
				setState(1188);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,87,_ctx);
				while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
					if ( _alt==1 ) {
						{
						{
						setState(1185);
						catchClause();
						}
						} 
					}
					setState(1190);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,87,_ctx);
				}
				}
				break;
			case 18:
				_localctx = new YieldExprContext(_localctx);
				enterOuterAlt(_localctx, 18);
				{
				setState(1191);
				match(T__25);
				setState(1193);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,88,_ctx) ) {
				case 1:
					{
					setState(1192);
					expression();
					}
					break;
				}
				}
				break;
			case 19:
				_localctx = new RecallValExprContext(_localctx);
				enterOuterAlt(_localctx, 19);
				{
				setState(1195);
				recallExpr();
				}
				break;
			case 20:
				_localctx = new LearnValExprContext(_localctx);
				enterOuterAlt(_localctx, 20);
				{
				setState(1196);
				learnExpr();
				}
				break;
			case 21:
				_localctx = new PerformValExprContext(_localctx);
				enterOuterAlt(_localctx, 21);
				{
				setState(1197);
				performExpr();
				}
				break;
			case 22:
				_localctx = new ZamaniValExprContext(_localctx);
				enterOuterAlt(_localctx, 22);
				{
				setState(1198);
				zamaniExpr();
				}
				break;
			case 23:
				_localctx = new SasaValExprContext(_localctx);
				enterOuterAlt(_localctx, 23);
				{
				setState(1199);
				sasaExpr();
				}
				break;
			case 24:
				_localctx = new QuantumOpValExprContext(_localctx);
				enterOuterAlt(_localctx, 24);
				{
				setState(1200);
				quantumOpExpr();
				}
				break;
			case 25:
				_localctx = new NanoValExprContext(_localctx);
				enterOuterAlt(_localctx, 25);
				{
				setState(1201);
				nanoExpr();
				}
				break;
			case 26:
				_localctx = new MtsValExprContext(_localctx);
				enterOuterAlt(_localctx, 26);
				{
				setState(1202);
				mtsExpr();
				}
				break;
			case 27:
				_localctx = new ConsensusValExprContext(_localctx);
				enterOuterAlt(_localctx, 27);
				{
				setState(1203);
				consensusExpr();
				}
				break;
			case 28:
				_localctx = new AncestorValExprContext(_localctx);
				enterOuterAlt(_localctx, 28);
				{
				setState(1204);
				ancestorCall();
				}
				break;
			case 29:
				_localctx = new MopValExprContext(_localctx);
				enterOuterAlt(_localctx, 29);
				{
				setState(1205);
				mopExpr();
				}
				break;
			case 30:
				_localctx = new MacroCallExprContext(_localctx);
				enterOuterAlt(_localctx, 30);
				{
				setState(1206);
				macroCall();
				}
				break;
			case 31:
				_localctx = new SuperExprContext(_localctx);
				enterOuterAlt(_localctx, 31);
				{
				setState(1207);
				match(T__26);
				setState(1215);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,90,_ctx) ) {
				case 1:
					{
					setState(1208);
					match(DOT);
					setState(1209);
					ident();
					}
					break;
				case 2:
					{
					setState(1210);
					match(LPAREN);
					setState(1212);
					_errHandler.sync(this);
					_la = _input.LA(1);
					if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
						{
						setState(1211);
						args();
						}
					}

					setState(1214);
					match(RPAREN);
					}
					break;
				}
				}
				break;
			case 32:
				_localctx = new ThisExprContext(_localctx);
				enterOuterAlt(_localctx, 32);
				{
				setState(1217);
				match(THIS);
				}
				break;
			case 33:
				_localctx = new SelfExprContext(_localctx);
				enterOuterAlt(_localctx, 33);
				{
				setState(1218);
				match(SELF_LOWER);
				}
				break;
			case 34:
				_localctx = new InterpStringExprContext(_localctx);
				enterOuterAlt(_localctx, 34);
				{
				setState(1219);
				interpolatedString();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class StructLiteralTailContext extends ParserRuleContext {
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public List<TerminalNode> COLON() { return getTokens(ZamaniParser.COLON); }
		public TerminalNode COLON(int i) {
			return getToken(ZamaniParser.COLON, i);
		}
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public StructLiteralTailContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_structLiteralTail; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterStructLiteralTail(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitStructLiteralTail(this);
		}
	}

	public final StructLiteralTailContext structLiteralTail() throws RecognitionException {
		StructLiteralTailContext _localctx = new StructLiteralTailContext(_ctx, getState());
		enterRule(_localctx, 110, RULE_structLiteralTail);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1222);
			match(LBRACE);
			setState(1231);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				{
				setState(1223);
				ident();
				setState(1224);
				match(COLON);
				setState(1225);
				expression();
				setState(1227);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COMMA) {
					{
					setState(1226);
					match(COMMA);
					}
				}

				}
				}
				setState(1233);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1234);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RecallExprContext extends ParserRuleContext {
		public TerminalNode RECALL() { return getToken(ZamaniParser.RECALL, 0); }
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public RecallExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_recallExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRecallExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRecallExpr(this);
		}
	}

	public final RecallExprContext recallExpr() throws RecognitionException {
		RecallExprContext _localctx = new RecallExprContext(_ctx, getState());
		enterRule(_localctx, 112, RULE_recallExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1236);
			match(RECALL);
			setState(1242);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,94,_ctx) ) {
			case 1:
				{
				setState(1237);
				match(LPAREN);
				setState(1238);
				expression();
				setState(1239);
				match(RPAREN);
				}
				break;
			case 2:
				{
				setState(1241);
				expression();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LearnExprContext extends ParserRuleContext {
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public TerminalNode LEARN() { return getToken(ZamaniParser.LEARN, 0); }
		public TerminalNode INFER() { return getToken(ZamaniParser.INFER, 0); }
		public LearnExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_learnExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLearnExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLearnExpr(this);
		}
	}

	public final LearnExprContext learnExpr() throws RecognitionException {
		LearnExprContext _localctx = new LearnExprContext(_ctx, getState());
		enterRule(_localctx, 114, RULE_learnExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1244);
			_la = _input.LA(1);
			if ( !(_la==LEARN || _la==INFER) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			setState(1246);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__27) {
				{
				setState(1245);
				match(T__27);
				}
			}

			setState(1248);
			expression();
			setState(1252);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,96,_ctx) ) {
			case 1:
				{
				setState(1249);
				match(T__1);
				setState(1250);
				match(T__28);
				setState(1251);
				expression();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PerformExprContext extends ParserRuleContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public PerformExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_performExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPerformExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPerformExpr(this);
		}
	}

	public final PerformExprContext performExpr() throws RecognitionException {
		PerformExprContext _localctx = new PerformExprContext(_ctx, getState());
		enterRule(_localctx, 116, RULE_performExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1254);
			match(T__29);
			setState(1255);
			expression();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ZamaniExprContext extends ParserRuleContext {
		public TerminalNode ZAMANI() { return getToken(ZamaniParser.ZAMANI, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public ZamaniExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_zamaniExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterZamaniExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitZamaniExpr(this);
		}
	}

	public final ZamaniExprContext zamaniExpr() throws RecognitionException {
		ZamaniExprContext _localctx = new ZamaniExprContext(_ctx, getState());
		enterRule(_localctx, 118, RULE_zamaniExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1257);
			match(ZAMANI);
			setState(1260);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,97,_ctx) ) {
			case 1:
				{
				setState(1258);
				blockExpr();
				}
				break;
			case 2:
				{
				setState(1259);
				expression();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SasaExprContext extends ParserRuleContext {
		public TerminalNode SASA() { return getToken(ZamaniParser.SASA, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public SasaExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sasaExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSasaExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSasaExpr(this);
		}
	}

	public final SasaExprContext sasaExpr() throws RecognitionException {
		SasaExprContext _localctx = new SasaExprContext(_ctx, getState());
		enterRule(_localctx, 120, RULE_sasaExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1262);
			match(SASA);
			setState(1265);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,98,_ctx) ) {
			case 1:
				{
				setState(1263);
				blockExpr();
				}
				break;
			case 2:
				{
				setState(1264);
				expression();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class QuantumOpExprContext extends ParserRuleContext {
		public TerminalNode QUANTUM() { return getToken(ZamaniParser.QUANTUM, 0); }
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public QuantumOpExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_quantumOpExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterQuantumOpExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitQuantumOpExpr(this);
		}
	}

	public final QuantumOpExprContext quantumOpExpr() throws RecognitionException {
		QuantumOpExprContext _localctx = new QuantumOpExprContext(_ctx, getState());
		enterRule(_localctx, 122, RULE_quantumOpExpr);
		int _la;
		try {
			setState(1295);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case QUANTUM:
				enterOuterAlt(_localctx, 1);
				{
				setState(1267);
				match(QUANTUM);
				setState(1268);
				ident();
				setState(1274);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,100,_ctx) ) {
				case 1:
					{
					setState(1269);
					match(LPAREN);
					setState(1271);
					_errHandler.sync(this);
					_la = _input.LA(1);
					if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
						{
						setState(1270);
						args();
						}
					}

					setState(1273);
					match(RPAREN);
					}
					break;
				}
				}
				break;
			case T__30:
				enterOuterAlt(_localctx, 2);
				{
				setState(1276);
				match(T__30);
				setState(1277);
				match(LPAREN);
				setState(1278);
				expression();
				setState(1283);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(1279);
					match(COMMA);
					setState(1280);
					expression();
					}
					}
					setState(1285);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(1286);
				match(RPAREN);
				}
				break;
			case T__31:
				enterOuterAlt(_localctx, 3);
				{
				setState(1288);
				match(T__31);
				setState(1289);
				match(LPAREN);
				setState(1290);
				ident();
				setState(1291);
				match(COMMA);
				setState(1292);
				ident();
				setState(1293);
				match(RPAREN);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class NanoExprContext extends ParserRuleContext {
		public NanoLitContext nanoLit() {
			return getRuleContext(NanoLitContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public NanoExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_nanoExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNanoExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNanoExpr(this);
		}
	}

	public final NanoExprContext nanoExpr() throws RecognitionException {
		NanoExprContext _localctx = new NanoExprContext(_ctx, getState());
		enterRule(_localctx, 124, RULE_nanoExpr);
		try {
			setState(1308);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__195:
			case T__196:
				enterOuterAlt(_localctx, 1);
				{
				setState(1297);
				nanoLit();
				}
				break;
			case T__32:
				enterOuterAlt(_localctx, 2);
				{
				setState(1298);
				match(T__32);
				setState(1299);
				match(LPAREN);
				setState(1300);
				expression();
				setState(1301);
				match(RPAREN);
				}
				break;
			case T__33:
				enterOuterAlt(_localctx, 3);
				{
				setState(1303);
				match(T__33);
				setState(1304);
				match(LPAREN);
				setState(1305);
				expression();
				setState(1306);
				match(RPAREN);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MtsExprContext extends ParserRuleContext {
		public MtsLitContext mtsLit() {
			return getRuleContext(MtsLitContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode COMMA() { return getToken(ZamaniParser.COMMA, 0); }
		public MtsExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mtsExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMtsExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMtsExpr(this);
		}
	}

	public final MtsExprContext mtsExpr() throws RecognitionException {
		MtsExprContext _localctx = new MtsExprContext(_ctx, getState());
		enterRule(_localctx, 126, RULE_mtsExpr);
		try {
			setState(1328);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case MTS_KW:
				enterOuterAlt(_localctx, 1);
				{
				setState(1310);
				mtsLit();
				}
				break;
			case T__34:
				enterOuterAlt(_localctx, 2);
				{
				setState(1311);
				match(T__34);
				setState(1312);
				match(LPAREN);
				setState(1313);
				blockExpr();
				setState(1314);
				match(RPAREN);
				}
				break;
			case T__35:
				enterOuterAlt(_localctx, 3);
				{
				setState(1316);
				match(T__35);
				setState(1317);
				match(LPAREN);
				setState(1318);
				blockExpr();
				setState(1319);
				match(RPAREN);
				}
				break;
			case T__36:
				enterOuterAlt(_localctx, 4);
				{
				setState(1321);
				match(T__36);
				setState(1322);
				match(LPAREN);
				setState(1323);
				expression();
				setState(1324);
				match(COMMA);
				setState(1325);
				blockExpr();
				setState(1326);
				match(RPAREN);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ConsensusExprContext extends ParserRuleContext {
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public ExprListContext exprList() {
			return getRuleContext(ExprListContext.class,0);
		}
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public ConsensusExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_consensusExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterConsensusExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitConsensusExpr(this);
		}
	}

	public final ConsensusExprContext consensusExpr() throws RecognitionException {
		ConsensusExprContext _localctx = new ConsensusExprContext(_ctx, getState());
		enterRule(_localctx, 128, RULE_consensusExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1330);
			match(T__37);
			setState(1331);
			match(LBRACK);
			setState(1332);
			exprList();
			setState(1333);
			match(RBRACK);
			setState(1334);
			match(T__38);
			setState(1335);
			expression();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AncestorCallContext extends ParserRuleContext {
		public TerminalNode ANCESTOR() { return getToken(ZamaniParser.ANCESTOR, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public AncestorCallContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_ancestorCall; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAncestorCall(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAncestorCall(this);
		}
	}

	public final AncestorCallContext ancestorCall() throws RecognitionException {
		AncestorCallContext _localctx = new AncestorCallContext(_ctx, getState());
		enterRule(_localctx, 130, RULE_ancestorCall);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1337);
			match(ANCESTOR);
			setState(1338);
			ident();
			setState(1339);
			match(LPAREN);
			setState(1341);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				setState(1340);
				args();
				}
			}

			setState(1343);
			match(RPAREN);
			setState(1345);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,106,_ctx) ) {
			case 1:
				{
				setState(1344);
				match(SEMI);
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MopExprContext extends ParserRuleContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MopExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mopExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMopExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMopExpr(this);
		}
	}

	public final MopExprContext mopExpr() throws RecognitionException {
		MopExprContext _localctx = new MopExprContext(_ctx, getState());
		enterRule(_localctx, 132, RULE_mopExpr);
		int _la;
		try {
			setState(1381);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__39:
				enterOuterAlt(_localctx, 1);
				{
				setState(1347);
				match(T__39);
				setState(1348);
				match(LPAREN);
				setState(1349);
				expression();
				setState(1350);
				match(RPAREN);
				}
				break;
			case T__40:
				enterOuterAlt(_localctx, 2);
				{
				setState(1352);
				match(T__40);
				setState(1353);
				match(LPAREN);
				setState(1354);
				ident();
				setState(1355);
				match(RPAREN);
				}
				break;
			case T__41:
				enterOuterAlt(_localctx, 3);
				{
				setState(1357);
				match(T__41);
				setState(1358);
				match(LPAREN);
				setState(1359);
				expression();
				setState(1360);
				match(RPAREN);
				}
				break;
			case T__42:
				enterOuterAlt(_localctx, 4);
				{
				setState(1362);
				match(T__42);
				setState(1363);
				match(LBRACE);
				setState(1367);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
					{
					{
					setState(1364);
					statement();
					}
					}
					setState(1369);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(1370);
				match(RBRACE);
				}
				break;
			case T__43:
				enterOuterAlt(_localctx, 5);
				{
				setState(1371);
				match(T__43);
				setState(1372);
				match(LPAREN);
				setState(1373);
				expression();
				setState(1374);
				match(RPAREN);
				}
				break;
			case T__44:
				enterOuterAlt(_localctx, 6);
				{
				setState(1376);
				match(T__44);
				setState(1377);
				match(LPAREN);
				setState(1378);
				expression();
				setState(1379);
				match(RPAREN);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MacroCallContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode BANG() { return getToken(ZamaniParser.BANG, 0); }
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public MacroCallContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_macroCall; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMacroCall(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMacroCall(this);
		}
	}

	public final MacroCallContext macroCall() throws RecognitionException {
		MacroCallContext _localctx = new MacroCallContext(_ctx, getState());
		enterRule(_localctx, 134, RULE_macroCall);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1383);
			ident();
			setState(1384);
			match(BANG);
			setState(1385);
			match(LPAREN);
			setState(1387);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -4611668151363436529L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029859857L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				setState(1386);
				args();
				}
			}

			setState(1389);
			match(RPAREN);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExprListContext extends ParserRuleContext {
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public ExprListContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_exprList; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExprList(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExprList(this);
		}
	}

	public final ExprListContext exprList() throws RecognitionException {
		ExprListContext _localctx = new ExprListContext(_ctx, getState());
		enterRule(_localctx, 136, RULE_exprList);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1391);
			expression();
			setState(1396);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(1392);
				match(COMMA);
				setState(1393);
				expression();
				}
				}
				setState(1398);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExpressionStmtContext extends ParserRuleContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ExpressionStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expressionStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExpressionStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExpressionStmt(this);
		}
	}

	public final ExpressionStmtContext expressionStmt() throws RecognitionException {
		ExpressionStmtContext _localctx = new ExpressionStmtContext(_ctx, getState());
		enterRule(_localctx, 138, RULE_expressionStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1399);
			expression();
			setState(1400);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeExprContext extends ParserRuleContext {
		public BaseTypeContext baseType() {
			return getRuleContext(BaseTypeContext.class,0);
		}
		public TerminalNode LT() { return getToken(ZamaniParser.LT, 0); }
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public TerminalNode GT() { return getToken(ZamaniParser.GT, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TerminalNode ARROW() { return getToken(ZamaniParser.ARROW, 0); }
		public TerminalNode FN() { return getToken(ZamaniParser.FN, 0); }
		public TerminalNode AMP() { return getToken(ZamaniParser.AMP, 0); }
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public TerminalNode STAR() { return getToken(ZamaniParser.STAR, 0); }
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SELF_UPPER() { return getToken(ZamaniParser.SELF_UPPER, 0); }
		public TerminalNode SELF_LOWER() { return getToken(ZamaniParser.SELF_LOWER, 0); }
		public TerminalNode LINEAR() { return getToken(ZamaniParser.LINEAR, 0); }
		public TerminalNode AFFINE() { return getToken(ZamaniParser.AFFINE, 0); }
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<SessionOpContext> sessionOp() {
			return getRuleContexts(SessionOpContext.class);
		}
		public SessionOpContext sessionOp(int i) {
			return getRuleContext(SessionOpContext.class,i);
		}
		public PiTypeContext piType() {
			return getRuleContext(PiTypeContext.class,0);
		}
		public SigmaTypeContext sigmaType() {
			return getRuleContext(SigmaTypeContext.class,0);
		}
		public IdentityTypeContext identityType() {
			return getRuleContext(IdentityTypeContext.class,0);
		}
		public QuantumTypeContext quantumType() {
			return getRuleContext(QuantumTypeContext.class,0);
		}
		public NanoTypeContext nanoType() {
			return getRuleContext(NanoTypeContext.class,0);
		}
		public MtsTypeContext mtsType() {
			return getRuleContext(MtsTypeContext.class,0);
		}
		public SankofaTypeContext sankofaType() {
			return getRuleContext(SankofaTypeContext.class,0);
		}
		public CognitiveTypeContext cognitiveType() {
			return getRuleContext(CognitiveTypeContext.class,0);
		}
		public TypeParamContext typeParam() {
			return getRuleContext(TypeParamContext.class,0);
		}
		public TerminalNode DOT() { return getToken(ZamaniParser.DOT, 0); }
		public TerminalNode QUESTION() { return getToken(ZamaniParser.QUESTION, 0); }
		public List<EffectNameContext> effectName() {
			return getRuleContexts(EffectNameContext.class);
		}
		public EffectNameContext effectName(int i) {
			return getRuleContext(EffectNameContext.class,i);
		}
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TypeExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeExpr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeExpr(this);
		}
	}

	public final TypeExprContext typeExpr() throws RecognitionException {
		return typeExpr(0);
	}

	private TypeExprContext typeExpr(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		TypeExprContext _localctx = new TypeExprContext(_ctx, _parentState);
		TypeExprContext _prevctx = _localctx;
		int _startState = 140;
		enterRecursionRule(_localctx, 140, RULE_typeExpr, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(1528);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,123,_ctx) ) {
			case 1:
				{
				setState(1403);
				baseType();
				setState(1415);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,112,_ctx) ) {
				case 1:
					{
					setState(1404);
					match(LT);
					setState(1405);
					typeExpr(0);
					setState(1410);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==COMMA) {
						{
						{
						setState(1406);
						match(COMMA);
						setState(1407);
						typeExpr(0);
						}
						}
						setState(1412);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(1413);
					match(GT);
					}
					break;
				}
				}
				break;
			case 2:
				{
				setState(1417);
				match(LPAREN);
				setState(1418);
				match(RPAREN);
				}
				break;
			case 3:
				{
				setState(1419);
				match(LPAREN);
				setState(1420);
				typeExpr(0);
				setState(1421);
				match(RPAREN);
				}
				break;
			case 4:
				{
				setState(1423);
				match(LPAREN);
				setState(1424);
				typeExpr(0);
				setState(1427); 
				_errHandler.sync(this);
				_la = _input.LA(1);
				do {
					{
					{
					setState(1425);
					match(COMMA);
					setState(1426);
					typeExpr(0);
					}
					}
					setState(1429); 
					_errHandler.sync(this);
					_la = _input.LA(1);
				} while ( _la==COMMA );
				setState(1431);
				match(RPAREN);
				setState(1434);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,114,_ctx) ) {
				case 1:
					{
					setState(1432);
					match(ARROW);
					setState(1433);
					typeExpr(0);
					}
					break;
				}
				}
				break;
			case 5:
				{
				setState(1436);
				match(FN);
				setState(1437);
				match(LPAREN);
				setState(1446);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 46)) & ~0x3f) == 0 && ((1L << (_la - 46)) & 268434943L) != 0) || ((((_la - 200)) & ~0x3f) == 0 && ((1L << (_la - 200)) & 17196646151L) != 0) || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 19703317089222627L) != 0) || _la==STAR || _la==AMP) {
					{
					setState(1438);
					typeExpr(0);
					setState(1443);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==COMMA) {
						{
						{
						setState(1439);
						match(COMMA);
						setState(1440);
						typeExpr(0);
						}
						}
						setState(1445);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					}
				}

				setState(1448);
				match(RPAREN);
				setState(1451);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,117,_ctx) ) {
				case 1:
					{
					setState(1449);
					match(ARROW);
					setState(1450);
					typeExpr(0);
					}
					break;
				}
				}
				break;
			case 6:
				{
				setState(1453);
				match(AMP);
				setState(1455);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__2) {
					{
					setState(1454);
					match(T__2);
					}
				}

				setState(1457);
				match(LBRACK);
				setState(1458);
				typeExpr(0);
				setState(1459);
				match(RBRACK);
				}
				break;
			case 7:
				{
				setState(1461);
				match(AMP);
				setState(1463);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__2) {
					{
					setState(1462);
					match(T__2);
					}
				}

				setState(1465);
				typeExpr(30);
				}
				break;
			case 8:
				{
				setState(1466);
				match(STAR);
				setState(1468);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__2) {
					{
					setState(1467);
					match(T__2);
					}
				}

				setState(1470);
				typeExpr(29);
				}
				break;
			case 9:
				{
				setState(1471);
				match(LBRACK);
				setState(1472);
				typeExpr(0);
				setState(1475);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==SEMI) {
					{
					setState(1473);
					match(SEMI);
					setState(1474);
					expression();
					}
				}

				setState(1477);
				match(RBRACK);
				}
				break;
			case 10:
				{
				setState(1479);
				match(SELF_UPPER);
				}
				break;
			case 11:
				{
				setState(1480);
				match(SELF_LOWER);
				}
				break;
			case 12:
				{
				setState(1481);
				match(T__45);
				setState(1482);
				match(LT);
				setState(1483);
				typeExpr(0);
				setState(1484);
				match(GT);
				}
				break;
			case 13:
				{
				setState(1486);
				match(LINEAR);
				setState(1487);
				typeExpr(23);
				}
				break;
			case 14:
				{
				setState(1488);
				match(AFFINE);
				setState(1489);
				typeExpr(22);
				}
				break;
			case 15:
				{
				setState(1490);
				match(T__46);
				setState(1491);
				match(LBRACE);
				setState(1495);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (((((_la - 203)) & ~0x3f) == 0 && ((1L << (_la - 203)) & 31L) != 0)) {
					{
					{
					setState(1492);
					sessionOp();
					}
					}
					setState(1497);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(1498);
				match(RBRACE);
				}
				break;
			case 16:
				{
				setState(1499);
				piType();
				}
				break;
			case 17:
				{
				setState(1500);
				sigmaType();
				}
				break;
			case 18:
				{
				setState(1501);
				identityType();
				}
				break;
			case 19:
				{
				setState(1502);
				match(T__47);
				}
				break;
			case 20:
				{
				setState(1503);
				match(T__48);
				}
				break;
			case 21:
				{
				setState(1504);
				match(T__49);
				}
				break;
			case 22:
				{
				setState(1505);
				match(T__50);
				}
				break;
			case 23:
				{
				setState(1506);
				match(T__51);
				}
				break;
			case 24:
				{
				setState(1507);
				match(T__52);
				}
				break;
			case 25:
				{
				setState(1508);
				match(T__53);
				}
				break;
			case 26:
				{
				setState(1509);
				quantumType();
				}
				break;
			case 27:
				{
				setState(1510);
				nanoType();
				}
				break;
			case 28:
				{
				setState(1511);
				mtsType();
				}
				break;
			case 29:
				{
				setState(1512);
				sankofaType();
				}
				break;
			case 30:
				{
				setState(1513);
				cognitiveType();
				}
				break;
			case 31:
				{
				setState(1514);
				match(T__55);
				setState(1515);
				match(LT);
				setState(1516);
				typeParam();
				setState(1517);
				match(DOT);
				setState(1518);
				typeExpr(0);
				setState(1519);
				match(GT);
				}
				break;
			case 32:
				{
				setState(1521);
				match(T__56);
				setState(1522);
				typeParam();
				setState(1523);
				match(DOT);
				setState(1524);
				typeExpr(3);
				}
				break;
			case 33:
				{
				setState(1526);
				match(T__57);
				setState(1527);
				typeExpr(2);
				}
				break;
			}
			_ctx.stop = _input.LT(-1);
			setState(1551);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,126,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(1549);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,125,_ctx) ) {
					case 1:
						{
						_localctx = new TypeExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_typeExpr);
						setState(1530);
						if (!(precpred(_ctx, 25))) throw new FailedPredicateException(this, "precpred(_ctx, 25)");
						setState(1531);
						match(QUESTION);
						}
						break;
					case 2:
						{
						_localctx = new TypeExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_typeExpr);
						setState(1532);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(1533);
						match(T__1);
						setState(1534);
						match(T__54);
						setState(1535);
						match(LBRACE);
						setState(1536);
						effectName();
						setState(1541);
						_errHandler.sync(this);
						_la = _input.LA(1);
						while (_la==COMMA) {
							{
							{
							setState(1537);
							match(COMMA);
							setState(1538);
							effectName();
							}
							}
							setState(1543);
							_errHandler.sync(this);
							_la = _input.LA(1);
						}
						setState(1544);
						match(RBRACE);
						}
						break;
					case 3:
						{
						_localctx = new TypeExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_typeExpr);
						setState(1546);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(1547);
						match(DOT);
						setState(1548);
						ident();
						}
						break;
					}
					} 
				}
				setState(1553);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,126,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BaseTypeContext extends ParserRuleContext {
		public TerminalNode VOID() { return getToken(ZamaniParser.VOID, 0); }
		public TerminalNode INT_KW() { return getToken(ZamaniParser.INT_KW, 0); }
		public TerminalNode FLOAT_KW() { return getToken(ZamaniParser.FLOAT_KW, 0); }
		public TerminalNode BOOL_KW() { return getToken(ZamaniParser.BOOL_KW, 0); }
		public TerminalNode STR_KW() { return getToken(ZamaniParser.STR_KW, 0); }
		public TerminalNode CHAR_KW() { return getToken(ZamaniParser.CHAR_KW, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public BaseTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_baseType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterBaseType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitBaseType(this);
		}
	}

	public final BaseTypeContext baseType() throws RecognitionException {
		BaseTypeContext _localctx = new BaseTypeContext(_ctx, getState());
		enterRule(_localctx, 142, RULE_baseType);
		try {
			setState(1576);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,127,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(1554);
				match(VOID);
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(1555);
				match(INT_KW);
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(1556);
				match(FLOAT_KW);
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(1557);
				match(BOOL_KW);
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				setState(1558);
				match(STR_KW);
				}
				break;
			case 6:
				enterOuterAlt(_localctx, 6);
				{
				setState(1559);
				match(CHAR_KW);
				}
				break;
			case 7:
				enterOuterAlt(_localctx, 7);
				{
				setState(1560);
				match(T__58);
				}
				break;
			case 8:
				enterOuterAlt(_localctx, 8);
				{
				setState(1561);
				match(T__59);
				}
				break;
			case 9:
				enterOuterAlt(_localctx, 9);
				{
				setState(1562);
				match(T__60);
				}
				break;
			case 10:
				enterOuterAlt(_localctx, 10);
				{
				setState(1563);
				match(T__61);
				}
				break;
			case 11:
				enterOuterAlt(_localctx, 11);
				{
				setState(1564);
				match(T__62);
				}
				break;
			case 12:
				enterOuterAlt(_localctx, 12);
				{
				setState(1565);
				match(T__63);
				}
				break;
			case 13:
				enterOuterAlt(_localctx, 13);
				{
				setState(1566);
				match(T__64);
				}
				break;
			case 14:
				enterOuterAlt(_localctx, 14);
				{
				setState(1567);
				match(T__65);
				}
				break;
			case 15:
				enterOuterAlt(_localctx, 15);
				{
				setState(1568);
				match(T__66);
				}
				break;
			case 16:
				enterOuterAlt(_localctx, 16);
				{
				setState(1569);
				match(T__67);
				}
				break;
			case 17:
				enterOuterAlt(_localctx, 17);
				{
				setState(1570);
				match(T__68);
				}
				break;
			case 18:
				enterOuterAlt(_localctx, 18);
				{
				setState(1571);
				match(T__69);
				}
				break;
			case 19:
				enterOuterAlt(_localctx, 19);
				{
				setState(1572);
				match(T__70);
				}
				break;
			case 20:
				enterOuterAlt(_localctx, 20);
				{
				setState(1573);
				match(T__71);
				}
				break;
			case 21:
				enterOuterAlt(_localctx, 21);
				{
				setState(1574);
				match(T__72);
				}
				break;
			case 22:
				enterOuterAlt(_localctx, 22);
				{
				setState(1575);
				ident();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeParamsContext extends ParserRuleContext {
		public TerminalNode LT() { return getToken(ZamaniParser.LT, 0); }
		public List<TypeParamContext> typeParam() {
			return getRuleContexts(TypeParamContext.class);
		}
		public TypeParamContext typeParam(int i) {
			return getRuleContext(TypeParamContext.class,i);
		}
		public TerminalNode GT() { return getToken(ZamaniParser.GT, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public TypeParamsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeParams; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeParams(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeParams(this);
		}
	}

	public final TypeParamsContext typeParams() throws RecognitionException {
		TypeParamsContext _localctx = new TypeParamsContext(_ctx, getState());
		enterRule(_localctx, 144, RULE_typeParams);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1578);
			match(LT);
			setState(1579);
			typeParam();
			setState(1584);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(1580);
				match(COMMA);
				setState(1581);
				typeParam();
				}
				}
				setState(1586);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1587);
			match(GT);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeParamContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TypeParamContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeParam; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeParam(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeParam(this);
		}
	}

	public final TypeParamContext typeParam() throws RecognitionException {
		TypeParamContext _localctx = new TypeParamContext(_ctx, getState());
		enterRule(_localctx, 146, RULE_typeParam);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1589);
			ident();
			setState(1592);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COLON) {
				{
				setState(1590);
				match(COLON);
				setState(1591);
				typeExpr(0);
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeArgsContext extends ParserRuleContext {
		public TerminalNode LT() { return getToken(ZamaniParser.LT, 0); }
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public TerminalNode GT() { return getToken(ZamaniParser.GT, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public TypeArgsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeArgs; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeArgs(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeArgs(this);
		}
	}

	public final TypeArgsContext typeArgs() throws RecognitionException {
		TypeArgsContext _localctx = new TypeArgsContext(_ctx, getState());
		enterRule(_localctx, 148, RULE_typeArgs);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1594);
			match(LT);
			setState(1595);
			typeExpr(0);
			setState(1600);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(1596);
				match(COMMA);
				setState(1597);
				typeExpr(0);
				}
				}
				setState(1602);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1603);
			match(GT);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class StructDeclContext extends ParserRuleContext {
		public TerminalNode STRUCT() { return getToken(ZamaniParser.STRUCT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public List<StructFieldContext> structField() {
			return getRuleContexts(StructFieldContext.class);
		}
		public StructFieldContext structField(int i) {
			return getRuleContext(StructFieldContext.class,i);
		}
		public StructDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_structDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterStructDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitStructDecl(this);
		}
	}

	public final StructDeclContext structDecl() throws RecognitionException {
		StructDeclContext _localctx = new StructDeclContext(_ctx, getState());
		enterRule(_localctx, 150, RULE_structDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1606);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1605);
				modifiers();
				}
			}

			setState(1608);
			match(STRUCT);
			setState(1609);
			ident();
			setState(1611);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1610);
				typeParams();
				}
			}

			setState(1613);
			match(LBRACE);
			setState(1617);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & -255086697512961L) != 0) || ((((_la - 299)) & ~0x3f) == 0 && ((1L << (_la - 299)) & 268468223L) != 0)) {
				{
				{
				setState(1614);
				structField();
				}
				}
				setState(1619);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1620);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class StructFieldContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TerminalNode COMMA() { return getToken(ZamaniParser.COMMA, 0); }
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public StructFieldContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_structField; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterStructField(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitStructField(this);
		}
	}

	public final StructFieldContext structField() throws RecognitionException {
		StructFieldContext _localctx = new StructFieldContext(_ctx, getState());
		enterRule(_localctx, 152, RULE_structField);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1623);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1622);
				modifiers();
				}
			}

			setState(1625);
			ident();
			setState(1626);
			match(COLON);
			setState(1627);
			typeExpr(0);
			setState(1629);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COMMA || _la==SEMI) {
				{
				setState(1628);
				_la = _input.LA(1);
				if ( !(_la==COMMA || _la==SEMI) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EnumDeclContext extends ParserRuleContext {
		public TerminalNode ENUM() { return getToken(ZamaniParser.ENUM, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public List<EnumVariantContext> enumVariant() {
			return getRuleContexts(EnumVariantContext.class);
		}
		public EnumVariantContext enumVariant(int i) {
			return getRuleContext(EnumVariantContext.class,i);
		}
		public EnumDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_enumDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEnumDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEnumDecl(this);
		}
	}

	public final EnumDeclContext enumDecl() throws RecognitionException {
		EnumDeclContext _localctx = new EnumDeclContext(_ctx, getState());
		enterRule(_localctx, 154, RULE_enumDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1632);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1631);
				modifiers();
				}
			}

			setState(1634);
			match(ENUM);
			setState(1635);
			ident();
			setState(1637);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1636);
				typeParams();
				}
			}

			setState(1639);
			match(LBRACE);
			setState(1643);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				{
				setState(1640);
				enumVariant();
				}
				}
				setState(1645);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1646);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EnumVariantContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public EnumVariantContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_enumVariant; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEnumVariant(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEnumVariant(this);
		}
	}

	public final EnumVariantContext enumVariant() throws RecognitionException {
		EnumVariantContext _localctx = new EnumVariantContext(_ctx, getState());
		enterRule(_localctx, 156, RULE_enumVariant);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1648);
			ident();
			setState(1660);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LPAREN) {
				{
				setState(1649);
				match(LPAREN);
				setState(1650);
				typeExpr(0);
				setState(1655);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(1651);
					match(COMMA);
					setState(1652);
					typeExpr(0);
					}
					}
					setState(1657);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(1658);
				match(RPAREN);
				}
			}

			setState(1663);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COMMA || _la==SEMI) {
				{
				setState(1662);
				_la = _input.LA(1);
				if ( !(_la==COMMA || _la==SEMI) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TraitDeclContext extends ParserRuleContext {
		public TerminalNode TRAIT() { return getToken(ZamaniParser.TRAIT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public WhereClauseContext whereClause() {
			return getRuleContext(WhereClauseContext.class,0);
		}
		public List<TraitItemContext> traitItem() {
			return getRuleContexts(TraitItemContext.class);
		}
		public TraitItemContext traitItem(int i) {
			return getRuleContext(TraitItemContext.class,i);
		}
		public TraitDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_traitDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTraitDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTraitDecl(this);
		}
	}

	public final TraitDeclContext traitDecl() throws RecognitionException {
		TraitDeclContext _localctx = new TraitDeclContext(_ctx, getState());
		enterRule(_localctx, 158, RULE_traitDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1666);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1665);
				modifiers();
				}
			}

			setState(1668);
			match(TRAIT);
			setState(1669);
			ident();
			setState(1671);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1670);
				typeParams();
				}
			}

			setState(1675);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__73) {
				{
				setState(1673);
				match(T__73);
				setState(1674);
				whereClause();
				}
			}

			setState(1677);
			match(LBRACE);
			setState(1681);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 234)) & ~0x3f) == 0 && ((1L << (_la - 234)) & 68719738879L) != 0)) {
				{
				{
				setState(1678);
				traitItem();
				}
				}
				setState(1683);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1684);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TraitItemContext extends ParserRuleContext {
		public FunctionDeclContext functionDecl() {
			return getRuleContext(FunctionDeclContext.class,0);
		}
		public TypeAliasDeclContext typeAliasDecl() {
			return getRuleContext(TypeAliasDeclContext.class,0);
		}
		public ConstDeclContext constDecl() {
			return getRuleContext(ConstDeclContext.class,0);
		}
		public TraitItemContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_traitItem; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTraitItem(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTraitItem(this);
		}
	}

	public final TraitItemContext traitItem() throws RecognitionException {
		TraitItemContext _localctx = new TraitItemContext(_ctx, getState());
		enterRule(_localctx, 160, RULE_traitItem);
		try {
			setState(1689);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,146,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(1686);
				functionDecl();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(1687);
				typeAliasDecl();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(1688);
				constDecl();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ImplDeclContext extends ParserRuleContext {
		public TerminalNode IMPL() { return getToken(ZamaniParser.IMPL, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode FOR() { return getToken(ZamaniParser.FOR, 0); }
		public WhereClauseContext whereClause() {
			return getRuleContext(WhereClauseContext.class,0);
		}
		public List<ImplItemContext> implItem() {
			return getRuleContexts(ImplItemContext.class);
		}
		public ImplItemContext implItem(int i) {
			return getRuleContext(ImplItemContext.class,i);
		}
		public ImplDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_implDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterImplDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitImplDecl(this);
		}
	}

	public final ImplDeclContext implDecl() throws RecognitionException {
		ImplDeclContext _localctx = new ImplDeclContext(_ctx, getState());
		enterRule(_localctx, 162, RULE_implDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1691);
			match(IMPL);
			setState(1693);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1692);
				typeParams();
				}
			}

			setState(1698);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,148,_ctx) ) {
			case 1:
				{
				setState(1695);
				ident();
				setState(1696);
				match(FOR);
				}
				break;
			}
			setState(1700);
			typeExpr(0);
			setState(1703);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__73) {
				{
				setState(1701);
				match(T__73);
				setState(1702);
				whereClause();
				}
			}

			setState(1705);
			match(LBRACE);
			setState(1709);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 234)) & ~0x3f) == 0 && ((1L << (_la - 234)) & 68719738879L) != 0)) {
				{
				{
				setState(1706);
				implItem();
				}
				}
				setState(1711);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1712);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ImplItemContext extends ParserRuleContext {
		public FunctionDeclContext functionDecl() {
			return getRuleContext(FunctionDeclContext.class,0);
		}
		public TypeAliasDeclContext typeAliasDecl() {
			return getRuleContext(TypeAliasDeclContext.class,0);
		}
		public ConstDeclContext constDecl() {
			return getRuleContext(ConstDeclContext.class,0);
		}
		public ImplItemContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_implItem; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterImplItem(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitImplItem(this);
		}
	}

	public final ImplItemContext implItem() throws RecognitionException {
		ImplItemContext _localctx = new ImplItemContext(_ctx, getState());
		enterRule(_localctx, 164, RULE_implItem);
		try {
			setState(1717);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,151,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(1714);
				functionDecl();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(1715);
				typeAliasDecl();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(1716);
				constDecl();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeAliasDeclContext extends ParserRuleContext {
		public TerminalNode TYPE() { return getToken(ZamaniParser.TYPE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public TypeAliasDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeAliasDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeAliasDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeAliasDecl(this);
		}
	}

	public final TypeAliasDeclContext typeAliasDecl() throws RecognitionException {
		TypeAliasDeclContext _localctx = new TypeAliasDeclContext(_ctx, getState());
		enterRule(_localctx, 166, RULE_typeAliasDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1720);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1719);
				modifiers();
				}
			}

			setState(1722);
			match(TYPE);
			setState(1723);
			ident();
			setState(1725);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1724);
				typeParams();
				}
			}

			setState(1727);
			match(ASSIGN);
			setState(1728);
			typeExpr(0);
			setState(1729);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ConstDeclContext extends ParserRuleContext {
		public TerminalNode CONST() { return getToken(ZamaniParser.CONST, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(ZamaniParser.ASSIGN, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public ConstDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_constDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterConstDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitConstDecl(this);
		}
	}

	public final ConstDeclContext constDecl() throws RecognitionException {
		ConstDeclContext _localctx = new ConstDeclContext(_ctx, getState());
		enterRule(_localctx, 168, RULE_constDecl);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1732);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,154,_ctx) ) {
			case 1:
				{
				setState(1731);
				modifiers();
				}
				break;
			}
			setState(1734);
			match(CONST);
			setState(1735);
			ident();
			setState(1736);
			match(COLON);
			setState(1737);
			typeExpr(0);
			setState(1738);
			match(ASSIGN);
			setState(1739);
			expression();
			setState(1740);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ClassDeclContext extends ParserRuleContext {
		public TerminalNode CLASS() { return getToken(ZamaniParser.CLASS, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public List<ClassItemContext> classItem() {
			return getRuleContexts(ClassItemContext.class);
		}
		public ClassItemContext classItem(int i) {
			return getRuleContext(ClassItemContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public ClassDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_classDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterClassDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitClassDecl(this);
		}
	}

	public final ClassDeclContext classDecl() throws RecognitionException {
		ClassDeclContext _localctx = new ClassDeclContext(_ctx, getState());
		enterRule(_localctx, 170, RULE_classDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1743);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1742);
				modifiers();
				}
			}

			setState(1745);
			match(CLASS);
			setState(1746);
			ident();
			setState(1748);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1747);
				typeParams();
				}
			}

			setState(1752);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__74) {
				{
				setState(1750);
				match(T__74);
				setState(1751);
				typeExpr(0);
				}
			}

			setState(1763);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__75) {
				{
				setState(1754);
				match(T__75);
				setState(1755);
				typeExpr(0);
				setState(1760);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(1756);
					match(COMMA);
					setState(1757);
					typeExpr(0);
					}
					}
					setState(1762);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				}
			}

			setState(1765);
			match(LBRACE);
			setState(1769);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__76 || ((((_la - 234)) & ~0x3f) == 0 && ((1L << (_la - 234)) & -510173395025921L) != 0) || ((((_la - 298)) & ~0x3f) == 0 && ((1L << (_la - 298)) & 536936447L) != 0)) {
				{
				{
				setState(1766);
				classItem();
				}
				}
				setState(1771);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1772);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ClassItemContext extends ParserRuleContext {
		public FunctionDeclContext functionDecl() {
			return getRuleContext(FunctionDeclContext.class,0);
		}
		public StructFieldContext structField() {
			return getRuleContext(StructFieldContext.class,0);
		}
		public ConstructorDeclContext constructorDecl() {
			return getRuleContext(ConstructorDeclContext.class,0);
		}
		public DestructorDeclContext destructorDecl() {
			return getRuleContext(DestructorDeclContext.class,0);
		}
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public ClassItemContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_classItem; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterClassItem(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitClassItem(this);
		}
	}

	public final ClassItemContext classItem() throws RecognitionException {
		ClassItemContext _localctx = new ClassItemContext(_ctx, getState());
		enterRule(_localctx, 172, RULE_classItem);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1775);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,161,_ctx) ) {
			case 1:
				{
				setState(1774);
				modifiers();
				}
				break;
			}
			setState(1781);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,162,_ctx) ) {
			case 1:
				{
				setState(1777);
				functionDecl();
				}
				break;
			case 2:
				{
				setState(1778);
				structField();
				}
				break;
			case 3:
				{
				setState(1779);
				constructorDecl();
				}
				break;
			case 4:
				{
				setState(1780);
				destructorDecl();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ConstructorDeclContext extends ParserRuleContext {
		public TerminalNode INIT() { return getToken(ZamaniParser.INIT, 0); }
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public ConstructorDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_constructorDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterConstructorDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitConstructorDecl(this);
		}
	}

	public final ConstructorDeclContext constructorDecl() throws RecognitionException {
		ConstructorDeclContext _localctx = new ConstructorDeclContext(_ctx, getState());
		enterRule(_localctx, 174, RULE_constructorDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1783);
			match(INIT);
			setState(1784);
			match(LPAREN);
			setState(1786);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(1785);
				params();
				}
			}

			setState(1788);
			match(RPAREN);
			setState(1789);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DestructorDeclContext extends ParserRuleContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public DestructorDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_destructorDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDestructorDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDestructorDecl(this);
		}
	}

	public final DestructorDeclContext destructorDecl() throws RecognitionException {
		DestructorDeclContext _localctx = new DestructorDeclContext(_ctx, getState());
		enterRule(_localctx, 176, RULE_destructorDecl);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1791);
			match(T__76);
			setState(1792);
			match(LPAREN);
			setState(1793);
			match(RPAREN);
			setState(1794);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class InterfaceDeclContext extends ParserRuleContext {
		public TerminalNode INTERFACE() { return getToken(ZamaniParser.INTERFACE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public List<TraitItemContext> traitItem() {
			return getRuleContexts(TraitItemContext.class);
		}
		public TraitItemContext traitItem(int i) {
			return getRuleContext(TraitItemContext.class,i);
		}
		public InterfaceDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_interfaceDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterInterfaceDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitInterfaceDecl(this);
		}
	}

	public final InterfaceDeclContext interfaceDecl() throws RecognitionException {
		InterfaceDeclContext _localctx = new InterfaceDeclContext(_ctx, getState());
		enterRule(_localctx, 178, RULE_interfaceDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1797);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1796);
				modifiers();
				}
			}

			setState(1799);
			match(INTERFACE);
			setState(1800);
			ident();
			setState(1802);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1801);
				typeParams();
				}
			}

			setState(1804);
			match(LBRACE);
			setState(1808);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 234)) & ~0x3f) == 0 && ((1L << (_la - 234)) & 68719738879L) != 0)) {
				{
				{
				setState(1805);
				traitItem();
				}
				}
				setState(1810);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1811);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RecordDeclContext extends ParserRuleContext {
		public TerminalNode RECORD() { return getToken(ZamaniParser.RECORD, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ModifiersContext modifiers() {
			return getRuleContext(ModifiersContext.class,0);
		}
		public TypeParamsContext typeParams() {
			return getRuleContext(TypeParamsContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public TerminalNode ARROW() { return getToken(ZamaniParser.ARROW, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public RecordDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_recordDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRecordDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRecordDecl(this);
		}
	}

	public final RecordDeclContext recordDecl() throws RecognitionException {
		RecordDeclContext _localctx = new RecordDeclContext(_ctx, getState());
		enterRule(_localctx, 180, RULE_recordDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1814);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (((((_la - 235)) & ~0x3f) == 0 && ((1L << (_la - 235)) & 131071L) != 0)) {
				{
				setState(1813);
				modifiers();
				}
			}

			setState(1816);
			match(RECORD);
			setState(1817);
			ident();
			setState(1819);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LT) {
				{
				setState(1818);
				typeParams();
				}
			}

			setState(1821);
			match(LPAREN);
			setState(1823);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(1822);
				params();
				}
			}

			setState(1825);
			match(RPAREN);
			setState(1828);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==ARROW) {
				{
				setState(1826);
				match(ARROW);
				setState(1827);
				typeExpr(0);
				}
			}

			setState(1832);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case LBRACE:
				{
				setState(1830);
				blockExpr();
				}
				break;
			case SEMI:
				{
				setState(1831);
				match(SEMI);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class QuantumCircuitDeclContext extends ParserRuleContext {
		public TerminalNode QUANTUM() { return getToken(ZamaniParser.QUANTUM, 0); }
		public TerminalNode CIRCUIT() { return getToken(ZamaniParser.CIRCUIT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public QuantumCircuitDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_quantumCircuitDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterQuantumCircuitDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitQuantumCircuitDecl(this);
		}
	}

	public final QuantumCircuitDeclContext quantumCircuitDecl() throws RecognitionException {
		QuantumCircuitDeclContext _localctx = new QuantumCircuitDeclContext(_ctx, getState());
		enterRule(_localctx, 182, RULE_quantumCircuitDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1834);
			match(QUANTUM);
			setState(1835);
			match(CIRCUIT);
			setState(1836);
			ident();
			setState(1837);
			match(LPAREN);
			setState(1839);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(1838);
				params();
				}
			}

			setState(1841);
			match(RPAREN);
			setState(1842);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class NanoAgentDeclContext extends ParserRuleContext {
		public TerminalNode NANO() { return getToken(ZamaniParser.NANO, 0); }
		public TerminalNode AGENT() { return getToken(ZamaniParser.AGENT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public NanoAgentDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_nanoAgentDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNanoAgentDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNanoAgentDecl(this);
		}
	}

	public final NanoAgentDeclContext nanoAgentDecl() throws RecognitionException {
		NanoAgentDeclContext _localctx = new NanoAgentDeclContext(_ctx, getState());
		enterRule(_localctx, 184, RULE_nanoAgentDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1844);
			match(NANO);
			setState(1845);
			match(AGENT);
			setState(1846);
			ident();
			setState(1847);
			match(LPAREN);
			setState(1849);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(1848);
				params();
				}
			}

			setState(1851);
			match(RPAREN);
			setState(1852);
			blockExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LanguageDeclContext extends ParserRuleContext {
		public TerminalNode LANGUAGE() { return getToken(ZamaniParser.LANGUAGE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public LanguageDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_languageDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLanguageDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLanguageDecl(this);
		}
	}

	public final LanguageDeclContext languageDecl() throws RecognitionException {
		LanguageDeclContext _localctx = new LanguageDeclContext(_ctx, getState());
		enterRule(_localctx, 186, RULE_languageDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1854);
			match(LANGUAGE);
			setState(1855);
			ident();
			setState(1856);
			match(LBRACE);
			setState(1860);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1857);
				statement();
				}
				}
				setState(1862);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1863);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EffectDeclContext extends ParserRuleContext {
		public TerminalNode EFFECT() { return getToken(ZamaniParser.EFFECT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<EffectOpContext> effectOp() {
			return getRuleContexts(EffectOpContext.class);
		}
		public EffectOpContext effectOp(int i) {
			return getRuleContext(EffectOpContext.class,i);
		}
		public EffectDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_effectDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEffectDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEffectDecl(this);
		}
	}

	public final EffectDeclContext effectDecl() throws RecognitionException {
		EffectDeclContext _localctx = new EffectDeclContext(_ctx, getState());
		enterRule(_localctx, 188, RULE_effectDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1865);
			match(EFFECT);
			setState(1866);
			ident();
			setState(1867);
			match(LBRACE);
			setState(1871);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__77) {
				{
				{
				setState(1868);
				effectOp();
				}
				}
				setState(1873);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1874);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EffectOpContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TerminalNode ARROW() { return getToken(ZamaniParser.ARROW, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public EffectOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_effectOp; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEffectOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEffectOp(this);
		}
	}

	public final EffectOpContext effectOp() throws RecognitionException {
		EffectOpContext _localctx = new EffectOpContext(_ctx, getState());
		enterRule(_localctx, 190, RULE_effectOp);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1876);
			match(T__77);
			setState(1877);
			ident();
			setState(1878);
			match(LPAREN);
			setState(1880);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(1879);
				params();
				}
			}

			setState(1882);
			match(RPAREN);
			setState(1883);
			match(ARROW);
			setState(1884);
			typeExpr(0);
			setState(1885);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EffectListContext extends ParserRuleContext {
		public List<EffectNameContext> effectName() {
			return getRuleContexts(EffectNameContext.class);
		}
		public EffectNameContext effectName(int i) {
			return getRuleContext(EffectNameContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public EffectListContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_effectList; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEffectList(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEffectList(this);
		}
	}

	public final EffectListContext effectList() throws RecognitionException {
		EffectListContext _localctx = new EffectListContext(_ctx, getState());
		enterRule(_localctx, 192, RULE_effectList);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1887);
			effectName();
			setState(1892);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(1888);
				match(COMMA);
				setState(1889);
				effectName();
				}
				}
				setState(1894);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EffectNameContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public EffectNameContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_effectName; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterEffectName(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitEffectName(this);
		}
	}

	public final EffectNameContext effectName() throws RecognitionException {
		EffectNameContext _localctx = new EffectNameContext(_ctx, getState());
		enterRule(_localctx, 194, RULE_effectName);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1895);
			ident();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MtsDeclContext extends ParserRuleContext {
		public TerminalNode MTS_KW() { return getToken(ZamaniParser.MTS_KW, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MtsDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mtsDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMtsDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMtsDecl(this);
		}
	}

	public final MtsDeclContext mtsDecl() throws RecognitionException {
		MtsDeclContext _localctx = new MtsDeclContext(_ctx, getState());
		enterRule(_localctx, 196, RULE_mtsDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1897);
			match(MTS_KW);
			setState(1898);
			ident();
			setState(1899);
			match(LBRACE);
			setState(1903);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1900);
				statement();
				}
				}
				setState(1905);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1906);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SankofaDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public SankofaDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sankofaDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSankofaDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSankofaDecl(this);
		}
	}

	public final SankofaDeclContext sankofaDecl() throws RecognitionException {
		SankofaDeclContext _localctx = new SankofaDeclContext(_ctx, getState());
		enterRule(_localctx, 198, RULE_sankofaDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1908);
			match(T__78);
			setState(1909);
			ident();
			setState(1910);
			match(LBRACE);
			setState(1914);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1911);
				statement();
				}
				}
				setState(1916);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1917);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AgentDeclContext extends ParserRuleContext {
		public TerminalNode AGENT() { return getToken(ZamaniParser.AGENT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<AgentCapabilityContext> agentCapability() {
			return getRuleContexts(AgentCapabilityContext.class);
		}
		public AgentCapabilityContext agentCapability(int i) {
			return getRuleContext(AgentCapabilityContext.class,i);
		}
		public List<AgentBehaviorContext> agentBehavior() {
			return getRuleContexts(AgentBehaviorContext.class);
		}
		public AgentBehaviorContext agentBehavior(int i) {
			return getRuleContext(AgentBehaviorContext.class,i);
		}
		public AgentDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_agentDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAgentDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAgentDecl(this);
		}
	}

	public final AgentDeclContext agentDecl() throws RecognitionException {
		AgentDeclContext _localctx = new AgentDeclContext(_ctx, getState());
		enterRule(_localctx, 200, RULE_agentDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1919);
			match(AGENT);
			setState(1920);
			ident();
			setState(1921);
			match(LBRACE);
			setState(1926);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__187 || _la==T__188) {
				{
				setState(1924);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case T__187:
					{
					setState(1922);
					agentCapability();
					}
					break;
				case T__188:
					{
					setState(1923);
					agentBehavior();
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				}
				setState(1928);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1929);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CognitiveBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public CognitiveBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_cognitiveBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCognitiveBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCognitiveBlock(this);
		}
	}

	public final CognitiveBlockContext cognitiveBlock() throws RecognitionException {
		CognitiveBlockContext _localctx = new CognitiveBlockContext(_ctx, getState());
		enterRule(_localctx, 202, RULE_cognitiveBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1931);
			match(T__79);
			setState(1932);
			ident();
			setState(1933);
			match(LBRACE);
			setState(1937);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1934);
				statement();
				}
				}
				setState(1939);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1940);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MetaBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MetaBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_metaBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMetaBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMetaBlock(this);
		}
	}

	public final MetaBlockContext metaBlock() throws RecognitionException {
		MetaBlockContext _localctx = new MetaBlockContext(_ctx, getState());
		enterRule(_localctx, 204, RULE_metaBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1942);
			match(T__80);
			setState(1943);
			ident();
			setState(1944);
			match(LBRACE);
			setState(1948);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1945);
				statement();
				}
				}
				setState(1950);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1951);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class HdlModuleDeclContext extends ParserRuleContext {
		public TerminalNode MODULE() { return getToken(ZamaniParser.MODULE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public HdlModuleDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_hdlModuleDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterHdlModuleDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitHdlModuleDecl(this);
		}
	}

	public final HdlModuleDeclContext hdlModuleDecl() throws RecognitionException {
		HdlModuleDeclContext _localctx = new HdlModuleDeclContext(_ctx, getState());
		enterRule(_localctx, 206, RULE_hdlModuleDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1953);
			match(T__81);
			setState(1954);
			match(MODULE);
			setState(1955);
			ident();
			setState(1956);
			match(LPAREN);
			setState(1958);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__2 || _la==T__3 || ((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
				{
				setState(1957);
				params();
				}
			}

			setState(1960);
			match(RPAREN);
			setState(1961);
			match(LBRACE);
			setState(1965);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1962);
				statement();
				}
				}
				setState(1967);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1968);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CloudDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public CloudDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_cloudDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCloudDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCloudDecl(this);
		}
	}

	public final CloudDeclContext cloudDecl() throws RecognitionException {
		CloudDeclContext _localctx = new CloudDeclContext(_ctx, getState());
		enterRule(_localctx, 208, RULE_cloudDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1970);
			match(T__82);
			setState(1971);
			ident();
			setState(1972);
			match(LBRACE);
			setState(1976);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1973);
				statement();
				}
				}
				setState(1978);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1979);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DistributedDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public DistributedDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_distributedDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDistributedDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDistributedDecl(this);
		}
	}

	public final DistributedDeclContext distributedDecl() throws RecognitionException {
		DistributedDeclContext _localctx = new DistributedDeclContext(_ctx, getState());
		enterRule(_localctx, 210, RULE_distributedDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1981);
			match(T__83);
			setState(1982);
			ident();
			setState(1983);
			match(LBRACE);
			setState(1987);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1984);
				statement();
				}
				}
				setState(1989);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(1990);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OnDeviceAgentDeclContext extends ParserRuleContext {
		public TerminalNode AGENT() { return getToken(ZamaniParser.AGENT, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OnDeviceAgentDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_onDeviceAgentDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOnDeviceAgentDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOnDeviceAgentDecl(this);
		}
	}

	public final OnDeviceAgentDeclContext onDeviceAgentDecl() throws RecognitionException {
		OnDeviceAgentDeclContext _localctx = new OnDeviceAgentDeclContext(_ctx, getState());
		enterRule(_localctx, 212, RULE_onDeviceAgentDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(1992);
			match(T__84);
			setState(1993);
			match(AGENT);
			setState(1994);
			ident();
			setState(1995);
			match(LBRACE);
			setState(1999);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(1996);
				statement();
				}
				}
				setState(2001);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2002);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SelfEvolveDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public SelfEvolveDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_selfEvolveDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSelfEvolveDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSelfEvolveDecl(this);
		}
	}

	public final SelfEvolveDeclContext selfEvolveDecl() throws RecognitionException {
		SelfEvolveDeclContext _localctx = new SelfEvolveDeclContext(_ctx, getState());
		enterRule(_localctx, 214, RULE_selfEvolveDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2004);
			match(T__85);
			setState(2005);
			ident();
			setState(2006);
			match(LBRACE);
			setState(2010);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2007);
				statement();
				}
				}
				setState(2012);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2013);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OptPassDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OptPassDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_optPassDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOptPassDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOptPassDecl(this);
		}
	}

	public final OptPassDeclContext optPassDecl() throws RecognitionException {
		OptPassDeclContext _localctx = new OptPassDeclContext(_ctx, getState());
		enterRule(_localctx, 216, RULE_optPassDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2015);
			match(T__86);
			setState(2016);
			ident();
			setState(2017);
			match(LBRACE);
			setState(2021);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2018);
				statement();
				}
				}
				setState(2023);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2024);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TargetPlatformContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TargetPlatformContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_targetPlatform; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTargetPlatform(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTargetPlatform(this);
		}
	}

	public final TargetPlatformContext targetPlatform() throws RecognitionException {
		TargetPlatformContext _localctx = new TargetPlatformContext(_ctx, getState());
		enterRule(_localctx, 218, RULE_targetPlatform);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2026);
			match(T__87);
			setState(2027);
			ident();
			setState(2028);
			match(LBRACE);
			setState(2032);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2029);
				statement();
				}
				}
				setState(2034);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2035);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RuntimeDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public RuntimeDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_runtimeDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRuntimeDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRuntimeDecl(this);
		}
	}

	public final RuntimeDeclContext runtimeDecl() throws RecognitionException {
		RuntimeDeclContext _localctx = new RuntimeDeclContext(_ctx, getState());
		enterRule(_localctx, 220, RULE_runtimeDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2037);
			match(T__88);
			setState(2038);
			ident();
			setState(2039);
			match(LBRACE);
			setState(2043);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2040);
				statement();
				}
				}
				setState(2045);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2046);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ActorDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ActorDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_actorDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterActorDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitActorDecl(this);
		}
	}

	public final ActorDeclContext actorDecl() throws RecognitionException {
		ActorDeclContext _localctx = new ActorDeclContext(_ctx, getState());
		enterRule(_localctx, 222, RULE_actorDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2048);
			match(T__89);
			setState(2049);
			ident();
			setState(2050);
			match(LBRACE);
			setState(2054);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2051);
				statement();
				}
				}
				setState(2056);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2057);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AiSystemDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AiSystemDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aiSystemDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAiSystemDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAiSystemDecl(this);
		}
	}

	public final AiSystemDeclContext aiSystemDecl() throws RecognitionException {
		AiSystemDeclContext _localctx = new AiSystemDeclContext(_ctx, getState());
		enterRule(_localctx, 224, RULE_aiSystemDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2059);
			match(T__90);
			setState(2060);
			match(T__91);
			setState(2061);
			ident();
			setState(2062);
			match(LBRACE);
			setState(2066);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2063);
				statement();
				}
				}
				setState(2068);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2069);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AgiSystemDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AgiSystemDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_agiSystemDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAgiSystemDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAgiSystemDecl(this);
		}
	}

	public final AgiSystemDeclContext agiSystemDecl() throws RecognitionException {
		AgiSystemDeclContext _localctx = new AgiSystemDeclContext(_ctx, getState());
		enterRule(_localctx, 226, RULE_agiSystemDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2071);
			match(T__92);
			setState(2072);
			match(T__91);
			setState(2073);
			ident();
			setState(2074);
			match(LBRACE);
			setState(2078);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2075);
				statement();
				}
				}
				setState(2080);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2081);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AsiSystemDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AsiSystemDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asiSystemDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAsiSystemDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAsiSystemDecl(this);
		}
	}

	public final AsiSystemDeclContext asiSystemDecl() throws RecognitionException {
		AsiSystemDeclContext _localctx = new AsiSystemDeclContext(_ctx, getState());
		enterRule(_localctx, 228, RULE_asiSystemDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2083);
			match(T__93);
			setState(2084);
			match(T__91);
			setState(2085);
			ident();
			setState(2086);
			match(LBRACE);
			setState(2090);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2087);
				statement();
				}
				}
				setState(2092);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2093);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AesiSystemDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AesiSystemDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aesiSystemDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAesiSystemDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAesiSystemDecl(this);
		}
	}

	public final AesiSystemDeclContext aesiSystemDecl() throws RecognitionException {
		AesiSystemDeclContext _localctx = new AesiSystemDeclContext(_ctx, getState());
		enterRule(_localctx, 230, RULE_aesiSystemDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2095);
			match(T__94);
			setState(2096);
			match(T__91);
			setState(2097);
			ident();
			setState(2098);
			match(LBRACE);
			setState(2102);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2099);
				statement();
				}
				}
				setState(2104);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2105);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AsesiSystemDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AsesiSystemDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asesiSystemDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAsesiSystemDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAsesiSystemDecl(this);
		}
	}

	public final AsesiSystemDeclContext asesiSystemDecl() throws RecognitionException {
		AsesiSystemDeclContext _localctx = new AsesiSystemDeclContext(_ctx, getState());
		enterRule(_localctx, 232, RULE_asesiSystemDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2107);
			match(T__95);
			setState(2108);
			match(T__91);
			setState(2109);
			ident();
			setState(2110);
			match(LBRACE);
			setState(2114);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2111);
				statement();
				}
				}
				setState(2116);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2117);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AdminInterfaceDeclContext extends ParserRuleContext {
		public TerminalNode INTERFACE() { return getToken(ZamaniParser.INTERFACE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AdminInterfaceDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_adminInterfaceDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAdminInterfaceDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAdminInterfaceDecl(this);
		}
	}

	public final AdminInterfaceDeclContext adminInterfaceDecl() throws RecognitionException {
		AdminInterfaceDeclContext _localctx = new AdminInterfaceDeclContext(_ctx, getState());
		enterRule(_localctx, 234, RULE_adminInterfaceDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2119);
			match(T__96);
			setState(2120);
			match(INTERFACE);
			setState(2121);
			ident();
			setState(2122);
			match(LBRACE);
			setState(2126);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2123);
				statement();
				}
				}
				setState(2128);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2129);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PaymentGatewayDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public PaymentGatewayDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_paymentGatewayDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPaymentGatewayDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPaymentGatewayDecl(this);
		}
	}

	public final PaymentGatewayDeclContext paymentGatewayDecl() throws RecognitionException {
		PaymentGatewayDeclContext _localctx = new PaymentGatewayDeclContext(_ctx, getState());
		enterRule(_localctx, 236, RULE_paymentGatewayDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2131);
			match(T__97);
			setState(2132);
			match(T__98);
			setState(2133);
			ident();
			setState(2134);
			match(LBRACE);
			setState(2138);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2135);
				statement();
				}
				}
				setState(2140);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2141);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class UserFeedbackDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public UserFeedbackDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_userFeedbackDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterUserFeedbackDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitUserFeedbackDecl(this);
		}
	}

	public final UserFeedbackDeclContext userFeedbackDecl() throws RecognitionException {
		UserFeedbackDeclContext _localctx = new UserFeedbackDeclContext(_ctx, getState());
		enterRule(_localctx, 238, RULE_userFeedbackDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2143);
			match(T__99);
			setState(2144);
			match(T__100);
			setState(2145);
			ident();
			setState(2146);
			match(LBRACE);
			setState(2150);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2147);
				statement();
				}
				}
				setState(2152);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2153);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CopyrightNoticeDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public CopyrightNoticeDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_copyrightNoticeDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCopyrightNoticeDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCopyrightNoticeDecl(this);
		}
	}

	public final CopyrightNoticeDeclContext copyrightNoticeDecl() throws RecognitionException {
		CopyrightNoticeDeclContext _localctx = new CopyrightNoticeDeclContext(_ctx, getState());
		enterRule(_localctx, 240, RULE_copyrightNoticeDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2155);
			match(T__101);
			setState(2156);
			match(T__102);
			setState(2157);
			ident();
			setState(2158);
			match(LBRACE);
			setState(2162);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2159);
				statement();
				}
				}
				setState(2164);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2165);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TailorMadeFeatureDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TailorMadeFeatureDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_tailorMadeFeatureDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTailorMadeFeatureDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTailorMadeFeatureDecl(this);
		}
	}

	public final TailorMadeFeatureDeclContext tailorMadeFeatureDecl() throws RecognitionException {
		TailorMadeFeatureDeclContext _localctx = new TailorMadeFeatureDeclContext(_ctx, getState());
		enterRule(_localctx, 242, RULE_tailorMadeFeatureDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2167);
			match(T__103);
			setState(2168);
			ident();
			setState(2169);
			match(LBRACE);
			setState(2173);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2170);
				statement();
				}
				}
				setState(2175);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2176);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProgramOnceDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ProgramOnceDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_programOnceDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterProgramOnceDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitProgramOnceDecl(this);
		}
	}

	public final ProgramOnceDeclContext programOnceDecl() throws RecognitionException {
		ProgramOnceDeclContext _localctx = new ProgramOnceDeclContext(_ctx, getState());
		enterRule(_localctx, 244, RULE_programOnceDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2178);
			match(T__104);
			setState(2179);
			ident();
			setState(2180);
			match(LBRACE);
			setState(2184);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2181);
				statement();
				}
				}
				setState(2186);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2187);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MaliciousIdeaDetectionContext extends ParserRuleContext {
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MaliciousIdeaDetectionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_maliciousIdeaDetection; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMaliciousIdeaDetection(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMaliciousIdeaDetection(this);
		}
	}

	public final MaliciousIdeaDetectionContext maliciousIdeaDetection() throws RecognitionException {
		MaliciousIdeaDetectionContext _localctx = new MaliciousIdeaDetectionContext(_ctx, getState());
		enterRule(_localctx, 246, RULE_maliciousIdeaDetection);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2189);
			match(T__105);
			setState(2190);
			match(T__106);
			setState(2191);
			match(T__107);
			setState(2192);
			match(LBRACE);
			setState(2196);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2193);
				statement();
				}
				}
				setState(2198);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2199);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class UserBlockingDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public UserBlockingDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_userBlockingDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterUserBlockingDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitUserBlockingDecl(this);
		}
	}

	public final UserBlockingDeclContext userBlockingDecl() throws RecognitionException {
		UserBlockingDeclContext _localctx = new UserBlockingDeclContext(_ctx, getState());
		enterRule(_localctx, 248, RULE_userBlockingDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2201);
			match(T__108);
			setState(2202);
			match(T__99);
			setState(2203);
			ident();
			setState(2204);
			match(LBRACE);
			setState(2208);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2205);
				statement();
				}
				}
				setState(2210);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2211);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LegalActionDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public LegalActionDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_legalActionDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLegalActionDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLegalActionDecl(this);
		}
	}

	public final LegalActionDeclContext legalActionDecl() throws RecognitionException {
		LegalActionDeclContext _localctx = new LegalActionDeclContext(_ctx, getState());
		enterRule(_localctx, 250, RULE_legalActionDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2213);
			match(T__109);
			setState(2214);
			match(T__110);
			setState(2215);
			ident();
			setState(2216);
			match(LBRACE);
			setState(2220);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2217);
				statement();
				}
				}
				setState(2222);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2223);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SandboxDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public SandboxDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sandboxDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSandboxDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSandboxDecl(this);
		}
	}

	public final SandboxDeclContext sandboxDecl() throws RecognitionException {
		SandboxDeclContext _localctx = new SandboxDeclContext(_ctx, getState());
		enterRule(_localctx, 252, RULE_sandboxDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2225);
			match(T__111);
			setState(2226);
			ident();
			setState(2227);
			match(LBRACE);
			setState(2231);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2228);
				statement();
				}
				}
				setState(2233);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2234);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalSimulationDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalSimulationDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalSimulationDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalSimulationDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalSimulationDecl(this);
		}
	}

	public final OmniversalSimulationDeclContext omniversalSimulationDecl() throws RecognitionException {
		OmniversalSimulationDeclContext _localctx = new OmniversalSimulationDeclContext(_ctx, getState());
		enterRule(_localctx, 254, RULE_omniversalSimulationDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2236);
			match(T__112);
			setState(2237);
			match(T__113);
			setState(2238);
			ident();
			setState(2239);
			match(LBRACE);
			setState(2243);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2240);
				statement();
				}
				}
				setState(2245);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2246);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalCodeSynthDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalCodeSynthDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalCodeSynthDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalCodeSynthDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalCodeSynthDecl(this);
		}
	}

	public final OmniversalCodeSynthDeclContext omniversalCodeSynthDecl() throws RecognitionException {
		OmniversalCodeSynthDeclContext _localctx = new OmniversalCodeSynthDeclContext(_ctx, getState());
		enterRule(_localctx, 256, RULE_omniversalCodeSynthDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2248);
			match(T__112);
			setState(2249);
			match(T__114);
			setState(2250);
			ident();
			setState(2251);
			match(LBRACE);
			setState(2255);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2252);
				statement();
				}
				}
				setState(2257);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2258);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalDeployDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalDeployDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalDeployDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalDeployDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalDeployDecl(this);
		}
	}

	public final OmniversalDeployDeclContext omniversalDeployDecl() throws RecognitionException {
		OmniversalDeployDeclContext _localctx = new OmniversalDeployDeclContext(_ctx, getState());
		enterRule(_localctx, 258, RULE_omniversalDeployDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2260);
			match(T__112);
			setState(2261);
			match(T__33);
			setState(2262);
			ident();
			setState(2263);
			match(LBRACE);
			setState(2267);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2264);
				statement();
				}
				}
				setState(2269);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2270);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalAlignmentDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalAlignmentDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalAlignmentDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalAlignmentDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalAlignmentDecl(this);
		}
	}

	public final OmniversalAlignmentDeclContext omniversalAlignmentDecl() throws RecognitionException {
		OmniversalAlignmentDeclContext _localctx = new OmniversalAlignmentDeclContext(_ctx, getState());
		enterRule(_localctx, 260, RULE_omniversalAlignmentDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2272);
			match(T__112);
			setState(2273);
			match(T__115);
			setState(2274);
			ident();
			setState(2275);
			match(LBRACE);
			setState(2279);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2276);
				statement();
				}
				}
				setState(2281);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2282);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalContainmentDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalContainmentDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalContainmentDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalContainmentDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalContainmentDecl(this);
		}
	}

	public final OmniversalContainmentDeclContext omniversalContainmentDecl() throws RecognitionException {
		OmniversalContainmentDeclContext _localctx = new OmniversalContainmentDeclContext(_ctx, getState());
		enterRule(_localctx, 262, RULE_omniversalContainmentDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2284);
			match(T__112);
			setState(2285);
			match(T__116);
			setState(2286);
			ident();
			setState(2287);
			match(LBRACE);
			setState(2291);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2288);
				statement();
				}
				}
				setState(2293);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2294);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalTrustDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalTrustDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalTrustDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalTrustDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalTrustDecl(this);
		}
	}

	public final OmniversalTrustDeclContext omniversalTrustDecl() throws RecognitionException {
		OmniversalTrustDeclContext _localctx = new OmniversalTrustDeclContext(_ctx, getState());
		enterRule(_localctx, 264, RULE_omniversalTrustDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2296);
			match(T__112);
			setState(2297);
			match(T__117);
			setState(2298);
			ident();
			setState(2299);
			match(LBRACE);
			setState(2303);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2300);
				statement();
				}
				}
				setState(2305);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2306);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalKnowledgeDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalKnowledgeDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalKnowledgeDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalKnowledgeDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalKnowledgeDecl(this);
		}
	}

	public final OmniversalKnowledgeDeclContext omniversalKnowledgeDecl() throws RecognitionException {
		OmniversalKnowledgeDeclContext _localctx = new OmniversalKnowledgeDeclContext(_ctx, getState());
		enterRule(_localctx, 266, RULE_omniversalKnowledgeDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2308);
			match(T__112);
			setState(2309);
			match(T__118);
			setState(2310);
			ident();
			setState(2311);
			match(LBRACE);
			setState(2315);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2312);
				statement();
				}
				}
				setState(2317);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2318);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalGenerativeDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalGenerativeDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalGenerativeDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalGenerativeDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalGenerativeDecl(this);
		}
	}

	public final OmniversalGenerativeDeclContext omniversalGenerativeDecl() throws RecognitionException {
		OmniversalGenerativeDeclContext _localctx = new OmniversalGenerativeDeclContext(_ctx, getState());
		enterRule(_localctx, 268, RULE_omniversalGenerativeDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2320);
			match(T__112);
			setState(2321);
			match(T__119);
			setState(2322);
			ident();
			setState(2323);
			match(LBRACE);
			setState(2327);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2324);
				statement();
				}
				}
				setState(2329);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2330);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalSovereigntyDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalSovereigntyDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalSovereigntyDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalSovereigntyDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalSovereigntyDecl(this);
		}
	}

	public final OmniversalSovereigntyDeclContext omniversalSovereigntyDecl() throws RecognitionException {
		OmniversalSovereigntyDeclContext _localctx = new OmniversalSovereigntyDeclContext(_ctx, getState());
		enterRule(_localctx, 270, RULE_omniversalSovereigntyDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2332);
			match(T__112);
			setState(2333);
			match(T__120);
			setState(2334);
			ident();
			setState(2335);
			match(LBRACE);
			setState(2339);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2336);
				statement();
				}
				}
				setState(2341);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2342);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalGoalDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalGoalDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalGoalDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalGoalDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalGoalDecl(this);
		}
	}

	public final OmniversalGoalDeclContext omniversalGoalDecl() throws RecognitionException {
		OmniversalGoalDeclContext _localctx = new OmniversalGoalDeclContext(_ctx, getState());
		enterRule(_localctx, 272, RULE_omniversalGoalDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2344);
			match(T__112);
			setState(2345);
			match(T__121);
			setState(2346);
			ident();
			setState(2347);
			match(LBRACE);
			setState(2351);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2348);
				statement();
				}
				}
				setState(2353);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2354);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalBioNanoDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalBioNanoDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalBioNanoDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalBioNanoDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalBioNanoDecl(this);
		}
	}

	public final OmniversalBioNanoDeclContext omniversalBioNanoDecl() throws RecognitionException {
		OmniversalBioNanoDeclContext _localctx = new OmniversalBioNanoDeclContext(_ctx, getState());
		enterRule(_localctx, 274, RULE_omniversalBioNanoDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2356);
			match(T__112);
			setState(2357);
			match(T__122);
			setState(2358);
			ident();
			setState(2359);
			match(LBRACE);
			setState(2363);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2360);
				statement();
				}
				}
				setState(2365);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2366);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalRealityDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalRealityDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalRealityDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalRealityDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalRealityDecl(this);
		}
	}

	public final OmniversalRealityDeclContext omniversalRealityDecl() throws RecognitionException {
		OmniversalRealityDeclContext _localctx = new OmniversalRealityDeclContext(_ctx, getState());
		enterRule(_localctx, 276, RULE_omniversalRealityDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2368);
			match(T__112);
			setState(2369);
			match(T__123);
			setState(2370);
			ident();
			setState(2371);
			match(LBRACE);
			setState(2375);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2372);
				statement();
				}
				}
				setState(2377);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2378);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OmniversalNlpDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public OmniversalNlpDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_omniversalNlpDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterOmniversalNlpDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitOmniversalNlpDecl(this);
		}
	}

	public final OmniversalNlpDeclContext omniversalNlpDecl() throws RecognitionException {
		OmniversalNlpDeclContext _localctx = new OmniversalNlpDeclContext(_ctx, getState());
		enterRule(_localctx, 278, RULE_omniversalNlpDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2380);
			match(T__112);
			setState(2381);
			match(T__124);
			setState(2382);
			ident();
			setState(2383);
			match(LBRACE);
			setState(2387);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2384);
				statement();
				}
				}
				setState(2389);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2390);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ChatArchitectDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ChatArchitectDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_chatArchitectDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterChatArchitectDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitChatArchitectDecl(this);
		}
	}

	public final ChatArchitectDeclContext chatArchitectDecl() throws RecognitionException {
		ChatArchitectDeclContext _localctx = new ChatArchitectDeclContext(_ctx, getState());
		enterRule(_localctx, 280, RULE_chatArchitectDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2392);
			match(T__125);
			setState(2393);
			match(T__126);
			setState(2394);
			ident();
			setState(2395);
			match(LBRACE);
			setState(2399);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2396);
				statement();
				}
				}
				setState(2401);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2402);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class GreenComputingAttrContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public GreenComputingAttrContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_greenComputingAttr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterGreenComputingAttr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitGreenComputingAttr(this);
		}
	}

	public final GreenComputingAttrContext greenComputingAttr() throws RecognitionException {
		GreenComputingAttrContext _localctx = new GreenComputingAttrContext(_ctx, getState());
		enterRule(_localctx, 282, RULE_greenComputingAttr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2404);
			match(T__127);
			setState(2405);
			ident();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ThermalOptDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ThermalOptDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_thermalOptDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterThermalOptDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitThermalOptDecl(this);
		}
	}

	public final ThermalOptDeclContext thermalOptDecl() throws RecognitionException {
		ThermalOptDeclContext _localctx = new ThermalOptDeclContext(_ctx, getState());
		enterRule(_localctx, 284, RULE_thermalOptDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2407);
			match(T__128);
			setState(2408);
			match(T__129);
			setState(2409);
			ident();
			setState(2410);
			match(LBRACE);
			setState(2414);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2411);
				statement();
				}
				}
				setState(2416);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2417);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ResourceConserveDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ResourceConserveDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_resourceConserveDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterResourceConserveDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitResourceConserveDecl(this);
		}
	}

	public final ResourceConserveDeclContext resourceConserveDecl() throws RecognitionException {
		ResourceConserveDeclContext _localctx = new ResourceConserveDeclContext(_ctx, getState());
		enterRule(_localctx, 286, RULE_resourceConserveDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2419);
			match(T__130);
			setState(2420);
			match(T__131);
			setState(2421);
			ident();
			setState(2422);
			match(LBRACE);
			setState(2426);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2423);
				statement();
				}
				}
				setState(2428);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2429);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SelfDiscoverDeclContext extends ParserRuleContext {
		public TerminalNode SELF_LOWER() { return getToken(ZamaniParser.SELF_LOWER, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public SelfDiscoverDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_selfDiscoverDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSelfDiscoverDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSelfDiscoverDecl(this);
		}
	}

	public final SelfDiscoverDeclContext selfDiscoverDecl() throws RecognitionException {
		SelfDiscoverDeclContext _localctx = new SelfDiscoverDeclContext(_ctx, getState());
		enterRule(_localctx, 288, RULE_selfDiscoverDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2431);
			match(SELF_LOWER);
			setState(2432);
			match(T__132);
			setState(2433);
			ident();
			setState(2434);
			match(LBRACE);
			setState(2438);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2435);
				statement();
				}
				}
				setState(2440);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2441);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DeveloperAnalyticsDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public DeveloperAnalyticsDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_developerAnalyticsDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDeveloperAnalyticsDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDeveloperAnalyticsDecl(this);
		}
	}

	public final DeveloperAnalyticsDeclContext developerAnalyticsDecl() throws RecognitionException {
		DeveloperAnalyticsDeclContext _localctx = new DeveloperAnalyticsDeclContext(_ctx, getState());
		enterRule(_localctx, 290, RULE_developerAnalyticsDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2443);
			match(T__133);
			setState(2444);
			match(T__134);
			setState(2445);
			ident();
			setState(2446);
			match(LBRACE);
			setState(2450);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2447);
				statement();
				}
				}
				setState(2452);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2453);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LicenseTrackingDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public LicenseTrackingDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_licenseTrackingDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLicenseTrackingDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLicenseTrackingDecl(this);
		}
	}

	public final LicenseTrackingDeclContext licenseTrackingDecl() throws RecognitionException {
		LicenseTrackingDeclContext _localctx = new LicenseTrackingDeclContext(_ctx, getState());
		enterRule(_localctx, 292, RULE_licenseTrackingDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2455);
			match(T__135);
			setState(2456);
			match(T__136);
			setState(2457);
			ident();
			setState(2458);
			match(LBRACE);
			setState(2462);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2459);
				statement();
				}
				}
				setState(2464);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2465);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DeploymentDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public DeploymentDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_deploymentDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDeploymentDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDeploymentDecl(this);
		}
	}

	public final DeploymentDeclContext deploymentDecl() throws RecognitionException {
		DeploymentDeclContext _localctx = new DeploymentDeclContext(_ctx, getState());
		enterRule(_localctx, 294, RULE_deploymentDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2467);
			match(T__137);
			setState(2468);
			ident();
			setState(2469);
			match(LBRACE);
			setState(2473);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2470);
				statement();
				}
				}
				setState(2475);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2476);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VersionReleaseDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public VersionReleaseDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_versionReleaseDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterVersionReleaseDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitVersionReleaseDecl(this);
		}
	}

	public final VersionReleaseDeclContext versionReleaseDecl() throws RecognitionException {
		VersionReleaseDeclContext _localctx = new VersionReleaseDeclContext(_ctx, getState());
		enterRule(_localctx, 296, RULE_versionReleaseDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2478);
			match(T__138);
			setState(2479);
			match(T__139);
			setState(2480);
			ident();
			setState(2481);
			match(LBRACE);
			setState(2485);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2482);
				statement();
				}
				}
				setState(2487);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2488);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LspServerDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public LspServerDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_lspServerDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLspServerDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLspServerDecl(this);
		}
	}

	public final LspServerDeclContext lspServerDecl() throws RecognitionException {
		LspServerDeclContext _localctx = new LspServerDeclContext(_ctx, getState());
		enterRule(_localctx, 298, RULE_lspServerDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2490);
			match(T__140);
			setState(2491);
			match(T__141);
			setState(2492);
			ident();
			setState(2493);
			match(LBRACE);
			setState(2497);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2494);
				statement();
				}
				}
				setState(2499);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2500);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeClassDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TypeClassDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeClassDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeClassDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeClassDecl(this);
		}
	}

	public final TypeClassDeclContext typeClassDecl() throws RecognitionException {
		TypeClassDeclContext _localctx = new TypeClassDeclContext(_ctx, getState());
		enterRule(_localctx, 300, RULE_typeClassDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2502);
			match(T__142);
			setState(2503);
			ident();
			setState(2504);
			match(LBRACE);
			setState(2508);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2505);
				statement();
				}
				}
				setState(2510);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2511);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeClassInstanceContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode FOR() { return getToken(ZamaniParser.FOR, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TypeClassInstanceContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeClassInstance; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeClassInstance(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeClassInstance(this);
		}
	}

	public final TypeClassInstanceContext typeClassInstance() throws RecognitionException {
		TypeClassInstanceContext _localctx = new TypeClassInstanceContext(_ctx, getState());
		enterRule(_localctx, 302, RULE_typeClassInstance);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2513);
			match(T__143);
			setState(2514);
			ident();
			setState(2515);
			match(FOR);
			setState(2516);
			typeExpr(0);
			setState(2517);
			match(LBRACE);
			setState(2521);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2518);
				statement();
				}
				}
				setState(2523);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2524);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class HigherKindedTypeDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public HigherKindedTypeDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_higherKindedTypeDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterHigherKindedTypeDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitHigherKindedTypeDecl(this);
		}
	}

	public final HigherKindedTypeDeclContext higherKindedTypeDecl() throws RecognitionException {
		HigherKindedTypeDeclContext _localctx = new HigherKindedTypeDeclContext(_ctx, getState());
		enterRule(_localctx, 304, RULE_higherKindedTypeDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2526);
			match(T__55);
			setState(2527);
			ident();
			setState(2528);
			match(LBRACE);
			setState(2532);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2529);
				statement();
				}
				}
				setState(2534);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2535);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SelfAdjustDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public SelfAdjustDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_selfAdjustDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSelfAdjustDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSelfAdjustDecl(this);
		}
	}

	public final SelfAdjustDeclContext selfAdjustDecl() throws RecognitionException {
		SelfAdjustDeclContext _localctx = new SelfAdjustDeclContext(_ctx, getState());
		enterRule(_localctx, 306, RULE_selfAdjustDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2537);
			match(T__144);
			setState(2538);
			ident();
			setState(2539);
			match(LBRACE);
			setState(2543);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2540);
				statement();
				}
				}
				setState(2545);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2546);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SelfVersioningDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public SelfVersioningDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_selfVersioningDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSelfVersioningDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSelfVersioningDecl(this);
		}
	}

	public final SelfVersioningDeclContext selfVersioningDecl() throws RecognitionException {
		SelfVersioningDeclContext _localctx = new SelfVersioningDeclContext(_ctx, getState());
		enterRule(_localctx, 308, RULE_selfVersioningDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2548);
			match(T__145);
			setState(2549);
			ident();
			setState(2550);
			match(LBRACE);
			setState(2554);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2551);
				statement();
				}
				}
				setState(2556);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2557);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExtensionMethodDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ExtensionMethodDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_extensionMethodDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExtensionMethodDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExtensionMethodDecl(this);
		}
	}

	public final ExtensionMethodDeclContext extensionMethodDecl() throws RecognitionException {
		ExtensionMethodDeclContext _localctx = new ExtensionMethodDeclContext(_ctx, getState());
		enterRule(_localctx, 310, RULE_extensionMethodDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2559);
			match(T__146);
			setState(2560);
			match(T__147);
			setState(2561);
			ident();
			setState(2562);
			match(LBRACE);
			setState(2566);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2563);
				statement();
				}
				}
				setState(2568);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2569);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExtensionPropertyDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ExtensionPropertyDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_extensionPropertyDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExtensionPropertyDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExtensionPropertyDecl(this);
		}
	}

	public final ExtensionPropertyDeclContext extensionPropertyDecl() throws RecognitionException {
		ExtensionPropertyDeclContext _localctx = new ExtensionPropertyDeclContext(_ctx, getState());
		enterRule(_localctx, 312, RULE_extensionPropertyDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2571);
			match(T__146);
			setState(2572);
			match(T__148);
			setState(2573);
			ident();
			setState(2574);
			match(LBRACE);
			setState(2578);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2575);
				statement();
				}
				}
				setState(2580);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2581);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExtensionIndexerDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ExtensionIndexerDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_extensionIndexerDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExtensionIndexerDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExtensionIndexerDecl(this);
		}
	}

	public final ExtensionIndexerDeclContext extensionIndexerDecl() throws RecognitionException {
		ExtensionIndexerDeclContext _localctx = new ExtensionIndexerDeclContext(_ctx, getState());
		enterRule(_localctx, 314, RULE_extensionIndexerDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2583);
			match(T__146);
			setState(2584);
			match(T__149);
			setState(2585);
			ident();
			setState(2586);
			match(LBRACE);
			setState(2590);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2587);
				statement();
				}
				}
				setState(2592);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2593);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExtensionOperatorDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ExtensionOperatorDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_extensionOperatorDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExtensionOperatorDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExtensionOperatorDecl(this);
		}
	}

	public final ExtensionOperatorDeclContext extensionOperatorDecl() throws RecognitionException {
		ExtensionOperatorDeclContext _localctx = new ExtensionOperatorDeclContext(_ctx, getState());
		enterRule(_localctx, 316, RULE_extensionOperatorDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2595);
			match(T__146);
			setState(2596);
			match(T__150);
			setState(2597);
			ident();
			setState(2598);
			match(LBRACE);
			setState(2602);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2599);
				statement();
				}
				}
				setState(2604);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2605);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MacroDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MacroDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_macroDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMacroDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMacroDecl(this);
		}
	}

	public final MacroDeclContext macroDecl() throws RecognitionException {
		MacroDeclContext _localctx = new MacroDeclContext(_ctx, getState());
		enterRule(_localctx, 318, RULE_macroDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2607);
			match(T__151);
			setState(2608);
			ident();
			setState(2609);
			match(LBRACE);
			setState(2613);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2610);
				statement();
				}
				}
				setState(2615);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2616);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DomainSpecificLanguageDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public DomainSpecificLanguageDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_domainSpecificLanguageDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDomainSpecificLanguageDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDomainSpecificLanguageDecl(this);
		}
	}

	public final DomainSpecificLanguageDeclContext domainSpecificLanguageDecl() throws RecognitionException {
		DomainSpecificLanguageDeclContext _localctx = new DomainSpecificLanguageDeclContext(_ctx, getState());
		enterRule(_localctx, 320, RULE_domainSpecificLanguageDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2618);
			match(T__152);
			setState(2619);
			ident();
			setState(2620);
			match(LBRACE);
			setState(2624);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2621);
				statement();
				}
				}
				setState(2626);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2627);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AspectDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AspectDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aspectDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAspectDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAspectDecl(this);
		}
	}

	public final AspectDeclContext aspectDecl() throws RecognitionException {
		AspectDeclContext _localctx = new AspectDeclContext(_ctx, getState());
		enterRule(_localctx, 322, RULE_aspectDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2629);
			match(T__153);
			setState(2630);
			ident();
			setState(2631);
			match(LBRACE);
			setState(2635);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2632);
				statement();
				}
				}
				setState(2637);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2638);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeProviderDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TypeProviderDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_typeProviderDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTypeProviderDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTypeProviderDecl(this);
		}
	}

	public final TypeProviderDeclContext typeProviderDecl() throws RecognitionException {
		TypeProviderDeclContext _localctx = new TypeProviderDeclContext(_ctx, getState());
		enterRule(_localctx, 324, RULE_typeProviderDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2640);
			match(T__154);
			setState(2641);
			ident();
			setState(2642);
			match(LBRACE);
			setState(2646);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2643);
				statement();
				}
				}
				setState(2648);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2649);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DataParallelismDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public DataParallelismDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_dataParallelismDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDataParallelismDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDataParallelismDecl(this);
		}
	}

	public final DataParallelismDeclContext dataParallelismDecl() throws RecognitionException {
		DataParallelismDeclContext _localctx = new DataParallelismDeclContext(_ctx, getState());
		enterRule(_localctx, 326, RULE_dataParallelismDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2651);
			match(T__155);
			setState(2652);
			match(T__156);
			setState(2653);
			ident();
			setState(2654);
			match(LBRACE);
			setState(2658);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2655);
				statement();
				}
				}
				setState(2660);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2661);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ConcurrentDataStructureDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ConcurrentDataStructureDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_concurrentDataStructureDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterConcurrentDataStructureDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitConcurrentDataStructureDecl(this);
		}
	}

	public final ConcurrentDataStructureDeclContext concurrentDataStructureDecl() throws RecognitionException {
		ConcurrentDataStructureDeclContext _localctx = new ConcurrentDataStructureDeclContext(_ctx, getState());
		enterRule(_localctx, 328, RULE_concurrentDataStructureDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2663);
			match(T__157);
			setState(2664);
			match(T__155);
			setState(2665);
			match(T__158);
			setState(2666);
			ident();
			setState(2667);
			match(LBRACE);
			setState(2671);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2668);
				statement();
				}
				}
				setState(2673);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2674);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MessageHandlerDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MessageHandlerDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_messageHandlerDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMessageHandlerDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMessageHandlerDecl(this);
		}
	}

	public final MessageHandlerDeclContext messageHandlerDecl() throws RecognitionException {
		MessageHandlerDeclContext _localctx = new MessageHandlerDeclContext(_ctx, getState());
		enterRule(_localctx, 330, RULE_messageHandlerDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2676);
			match(T__159);
			setState(2677);
			match(T__160);
			setState(2678);
			ident();
			setState(2679);
			match(LBRACE);
			setState(2683);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2680);
				statement();
				}
				}
				setState(2685);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2686);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MusicDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MusicDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_musicDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMusicDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMusicDecl(this);
		}
	}

	public final MusicDeclContext musicDecl() throws RecognitionException {
		MusicDeclContext _localctx = new MusicDeclContext(_ctx, getState());
		enterRule(_localctx, 332, RULE_musicDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2688);
			match(T__161);
			setState(2689);
			ident();
			setState(2690);
			match(LBRACE);
			setState(2694);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2691);
				statement();
				}
				}
				setState(2696);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2697);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RoboticsDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public RoboticsDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_roboticsDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRoboticsDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRoboticsDecl(this);
		}
	}

	public final RoboticsDeclContext roboticsDecl() throws RecognitionException {
		RoboticsDeclContext _localctx = new RoboticsDeclContext(_ctx, getState());
		enterRule(_localctx, 334, RULE_roboticsDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2699);
			match(T__162);
			setState(2700);
			ident();
			setState(2701);
			match(LBRACE);
			setState(2705);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2702);
				statement();
				}
				}
				setState(2707);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2708);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DeepLearningDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public DeepLearningDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_deepLearningDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDeepLearningDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDeepLearningDecl(this);
		}
	}

	public final DeepLearningDeclContext deepLearningDecl() throws RecognitionException {
		DeepLearningDeclContext _localctx = new DeepLearningDeclContext(_ctx, getState());
		enterRule(_localctx, 336, RULE_deepLearningDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2710);
			match(T__163);
			setState(2711);
			ident();
			setState(2712);
			match(LBRACE);
			setState(2716);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2713);
				statement();
				}
				}
				setState(2718);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2719);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class GraphicsDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public GraphicsDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_graphicsDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterGraphicsDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitGraphicsDecl(this);
		}
	}

	public final GraphicsDeclContext graphicsDecl() throws RecognitionException {
		GraphicsDeclContext _localctx = new GraphicsDeclContext(_ctx, getState());
		enterRule(_localctx, 338, RULE_graphicsDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2721);
			match(T__164);
			setState(2722);
			ident();
			setState(2723);
			match(LBRACE);
			setState(2727);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2724);
				statement();
				}
				}
				setState(2729);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2730);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VideoDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public VideoDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_videoDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterVideoDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitVideoDecl(this);
		}
	}

	public final VideoDeclContext videoDecl() throws RecognitionException {
		VideoDeclContext _localctx = new VideoDeclContext(_ctx, getState());
		enterRule(_localctx, 340, RULE_videoDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2732);
			match(T__165);
			setState(2733);
			ident();
			setState(2734);
			match(LBRACE);
			setState(2738);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2735);
				statement();
				}
				}
				setState(2740);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2741);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TensorDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TensorDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_tensorDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTensorDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTensorDecl(this);
		}
	}

	public final TensorDeclContext tensorDecl() throws RecognitionException {
		TensorDeclContext _localctx = new TensorDeclContext(_ctx, getState());
		enterRule(_localctx, 342, RULE_tensorDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2743);
			match(T__166);
			setState(2744);
			ident();
			setState(2745);
			match(LBRACE);
			setState(2749);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2746);
				statement();
				}
				}
				setState(2751);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2752);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MatrixDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MatrixDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_matrixDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMatrixDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMatrixDecl(this);
		}
	}

	public final MatrixDeclContext matrixDecl() throws RecognitionException {
		MatrixDeclContext _localctx = new MatrixDeclContext(_ctx, getState());
		enterRule(_localctx, 344, RULE_matrixDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2754);
			match(T__167);
			setState(2755);
			ident();
			setState(2756);
			match(LBRACE);
			setState(2760);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2757);
				statement();
				}
				}
				setState(2762);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2763);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VectorDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public VectorDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_vectorDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterVectorDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitVectorDecl(this);
		}
	}

	public final VectorDeclContext vectorDecl() throws RecognitionException {
		VectorDeclContext _localctx = new VectorDeclContext(_ctx, getState());
		enterRule(_localctx, 346, RULE_vectorDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2765);
			match(T__168);
			setState(2766);
			ident();
			setState(2767);
			match(LBRACE);
			setState(2771);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2768);
				statement();
				}
				}
				setState(2773);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2774);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MlModelDeclContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MlModelDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mlModelDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMlModelDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMlModelDecl(this);
		}
	}

	public final MlModelDeclContext mlModelDecl() throws RecognitionException {
		MlModelDeclContext _localctx = new MlModelDeclContext(_ctx, getState());
		enterRule(_localctx, 348, RULE_mlModelDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2776);
			match(T__169);
			setState(2777);
			match(T__170);
			setState(2778);
			ident();
			setState(2779);
			match(LBRACE);
			setState(2783);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2780);
				statement();
				}
				}
				setState(2785);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2786);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class QuantumMlBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public QuantumMlBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_quantumMlBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterQuantumMlBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitQuantumMlBlock(this);
		}
	}

	public final QuantumMlBlockContext quantumMlBlock() throws RecognitionException {
		QuantumMlBlockContext _localctx = new QuantumMlBlockContext(_ctx, getState());
		enterRule(_localctx, 350, RULE_quantumMlBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2788);
			match(T__171);
			setState(2789);
			ident();
			setState(2790);
			match(LBRACE);
			setState(2794);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2791);
				statement();
				}
				}
				setState(2796);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2797);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExplainableRlBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ExplainableRlBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_explainableRlBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExplainableRlBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExplainableRlBlock(this);
		}
	}

	public final ExplainableRlBlockContext explainableRlBlock() throws RecognitionException {
		ExplainableRlBlockContext _localctx = new ExplainableRlBlockContext(_ctx, getState());
		enterRule(_localctx, 352, RULE_explainableRlBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2799);
			match(T__172);
			setState(2800);
			ident();
			setState(2801);
			match(LBRACE);
			setState(2805);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2802);
				statement();
				}
				}
				setState(2807);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2808);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExplainableDeepLearningBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ExplainableDeepLearningBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_explainableDeepLearningBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExplainableDeepLearningBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExplainableDeepLearningBlock(this);
		}
	}

	public final ExplainableDeepLearningBlockContext explainableDeepLearningBlock() throws RecognitionException {
		ExplainableDeepLearningBlockContext _localctx = new ExplainableDeepLearningBlockContext(_ctx, getState());
		enterRule(_localctx, 354, RULE_explainableDeepLearningBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2810);
			match(T__173);
			setState(2811);
			ident();
			setState(2812);
			match(LBRACE);
			setState(2816);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2813);
				statement();
				}
				}
				setState(2818);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2819);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class KnowledgeGraphBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public KnowledgeGraphBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_knowledgeGraphBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterKnowledgeGraphBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitKnowledgeGraphBlock(this);
		}
	}

	public final KnowledgeGraphBlockContext knowledgeGraphBlock() throws RecognitionException {
		KnowledgeGraphBlockContext _localctx = new KnowledgeGraphBlockContext(_ctx, getState());
		enterRule(_localctx, 356, RULE_knowledgeGraphBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2821);
			match(T__174);
			setState(2822);
			ident();
			setState(2823);
			match(LBRACE);
			setState(2827);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2824);
				statement();
				}
				}
				setState(2829);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2830);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProbabilisticGraphicalModelBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ProbabilisticGraphicalModelBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_probabilisticGraphicalModelBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterProbabilisticGraphicalModelBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitProbabilisticGraphicalModelBlock(this);
		}
	}

	public final ProbabilisticGraphicalModelBlockContext probabilisticGraphicalModelBlock() throws RecognitionException {
		ProbabilisticGraphicalModelBlockContext _localctx = new ProbabilisticGraphicalModelBlockContext(_ctx, getState());
		enterRule(_localctx, 358, RULE_probabilisticGraphicalModelBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2832);
			match(T__175);
			setState(2833);
			ident();
			setState(2834);
			match(LBRACE);
			setState(2838);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2835);
				statement();
				}
				}
				setState(2840);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2841);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TransferLearningBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public TransferLearningBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_transferLearningBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTransferLearningBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTransferLearningBlock(this);
		}
	}

	public final TransferLearningBlockContext transferLearningBlock() throws RecognitionException {
		TransferLearningBlockContext _localctx = new TransferLearningBlockContext(_ctx, getState());
		enterRule(_localctx, 360, RULE_transferLearningBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2843);
			match(T__176);
			setState(2844);
			ident();
			setState(2845);
			match(LBRACE);
			setState(2849);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2846);
				statement();
				}
				}
				setState(2851);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2852);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MultiAgentBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public MultiAgentBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_multiAgentBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMultiAgentBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMultiAgentBlock(this);
		}
	}

	public final MultiAgentBlockContext multiAgentBlock() throws RecognitionException {
		MultiAgentBlockContext _localctx = new MultiAgentBlockContext(_ctx, getState());
		enterRule(_localctx, 362, RULE_multiAgentBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2854);
			match(T__177);
			setState(2855);
			ident();
			setState(2856);
			match(LBRACE);
			setState(2860);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2857);
				statement();
				}
				}
				setState(2862);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2863);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AutonomousSystemBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AutonomousSystemBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_autonomousSystemBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAutonomousSystemBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAutonomousSystemBlock(this);
		}
	}

	public final AutonomousSystemBlockContext autonomousSystemBlock() throws RecognitionException {
		AutonomousSystemBlockContext _localctx = new AutonomousSystemBlockContext(_ctx, getState());
		enterRule(_localctx, 364, RULE_autonomousSystemBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2865);
			match(T__178);
			setState(2866);
			ident();
			setState(2867);
			match(LBRACE);
			setState(2871);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2868);
				statement();
				}
				}
				setState(2873);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2874);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class GraphModelingBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public GraphModelingBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_graphModelingBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterGraphModelingBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitGraphModelingBlock(this);
		}
	}

	public final GraphModelingBlockContext graphModelingBlock() throws RecognitionException {
		GraphModelingBlockContext _localctx = new GraphModelingBlockContext(_ctx, getState());
		enterRule(_localctx, 366, RULE_graphModelingBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2876);
			match(T__179);
			setState(2877);
			ident();
			setState(2878);
			match(LBRACE);
			setState(2882);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2879);
				statement();
				}
				}
				setState(2884);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2885);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AdvancedNlpBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AdvancedNlpBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_advancedNlpBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAdvancedNlpBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAdvancedNlpBlock(this);
		}
	}

	public final AdvancedNlpBlockContext advancedNlpBlock() throws RecognitionException {
		AdvancedNlpBlockContext _localctx = new AdvancedNlpBlockContext(_ctx, getState());
		enterRule(_localctx, 368, RULE_advancedNlpBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2887);
			match(T__180);
			setState(2888);
			ident();
			setState(2889);
			match(LBRACE);
			setState(2893);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2890);
				statement();
				}
				}
				setState(2895);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2896);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CognitiveArchitectureBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public CognitiveArchitectureBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_cognitiveArchitectureBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCognitiveArchitectureBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCognitiveArchitectureBlock(this);
		}
	}

	public final CognitiveArchitectureBlockContext cognitiveArchitectureBlock() throws RecognitionException {
		CognitiveArchitectureBlockContext _localctx = new CognitiveArchitectureBlockContext(_ctx, getState());
		enterRule(_localctx, 370, RULE_cognitiveArchitectureBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2898);
			match(T__181);
			setState(2899);
			ident();
			setState(2900);
			match(LBRACE);
			setState(2904);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2901);
				statement();
				}
				}
				setState(2906);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2907);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AiForBusinessBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AiForBusinessBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aiForBusinessBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAiForBusinessBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAiForBusinessBlock(this);
		}
	}

	public final AiForBusinessBlockContext aiForBusinessBlock() throws RecognitionException {
		AiForBusinessBlockContext _localctx = new AiForBusinessBlockContext(_ctx, getState());
		enterRule(_localctx, 372, RULE_aiForBusinessBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2909);
			match(T__182);
			setState(2910);
			ident();
			setState(2911);
			match(LBRACE);
			setState(2915);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2912);
				statement();
				}
				}
				setState(2917);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2918);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VrArInteractionBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public VrArInteractionBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_vrArInteractionBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterVrArInteractionBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitVrArInteractionBlock(this);
		}
	}

	public final VrArInteractionBlockContext vrArInteractionBlock() throws RecognitionException {
		VrArInteractionBlockContext _localctx = new VrArInteractionBlockContext(_ctx, getState());
		enterRule(_localctx, 374, RULE_vrArInteractionBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2920);
			match(T__183);
			setState(2921);
			ident();
			setState(2922);
			match(LBRACE);
			setState(2926);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2923);
				statement();
				}
				}
				setState(2928);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2929);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ImageVideoAnalysisBlockContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ImageVideoAnalysisBlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_imageVideoAnalysisBlock; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterImageVideoAnalysisBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitImageVideoAnalysisBlock(this);
		}
	}

	public final ImageVideoAnalysisBlockContext imageVideoAnalysisBlock() throws RecognitionException {
		ImageVideoAnalysisBlockContext _localctx = new ImageVideoAnalysisBlockContext(_ctx, getState());
		enterRule(_localctx, 376, RULE_imageVideoAnalysisBlock);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2931);
			match(T__184);
			setState(2932);
			ident();
			setState(2933);
			match(LBRACE);
			setState(2937);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2934);
				statement();
				}
				}
				setState(2939);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2940);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class FileScopedTypeContext extends ParserRuleContext {
		public TerminalNode FILE() { return getToken(ZamaniParser.FILE, 0); }
		public TerminalNode TYPE() { return getToken(ZamaniParser.TYPE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public FileScopedTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_fileScopedType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterFileScopedType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitFileScopedType(this);
		}
	}

	public final FileScopedTypeContext fileScopedType() throws RecognitionException {
		FileScopedTypeContext _localctx = new FileScopedTypeContext(_ctx, getState());
		enterRule(_localctx, 378, RULE_fileScopedType);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2942);
			match(FILE);
			setState(2943);
			match(T__185);
			setState(2944);
			match(TYPE);
			setState(2945);
			ident();
			setState(2946);
			match(LBRACE);
			setState(2950);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2947);
				statement();
				}
				}
				setState(2952);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2953);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class HybridDefContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public HybridDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_hybridDef; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterHybridDef(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitHybridDef(this);
		}
	}

	public final HybridDefContext hybridDef() throws RecognitionException {
		HybridDefContext _localctx = new HybridDefContext(_ctx, getState());
		enterRule(_localctx, 380, RULE_hybridDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2955);
			match(T__186);
			setState(2956);
			ident();
			setState(2957);
			match(LBRACE);
			setState(2961);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2958);
				statement();
				}
				}
				setState(2963);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2964);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class InterfaceDefContext extends ParserRuleContext {
		public TerminalNode INTERFACE() { return getToken(ZamaniParser.INTERFACE, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public InterfaceDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_interfaceDef; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterInterfaceDef(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitInterfaceDef(this);
		}
	}

	public final InterfaceDefContext interfaceDef() throws RecognitionException {
		InterfaceDefContext _localctx = new InterfaceDefContext(_ctx, getState());
		enterRule(_localctx, 382, RULE_interfaceDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2966);
			match(INTERFACE);
			setState(2967);
			ident();
			setState(2968);
			match(LBRACE);
			setState(2972);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2969);
				statement();
				}
				}
				setState(2974);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2975);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AgentCapabilityContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AgentCapabilityContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_agentCapability; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAgentCapability(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAgentCapability(this);
		}
	}

	public final AgentCapabilityContext agentCapability() throws RecognitionException {
		AgentCapabilityContext _localctx = new AgentCapabilityContext(_ctx, getState());
		enterRule(_localctx, 384, RULE_agentCapability);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2977);
			match(T__187);
			setState(2978);
			ident();
			setState(2979);
			match(LBRACE);
			setState(2983);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2980);
				statement();
				}
				}
				setState(2985);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2986);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AgentBehaviorContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AgentBehaviorContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_agentBehavior; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAgentBehavior(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAgentBehavior(this);
		}
	}

	public final AgentBehaviorContext agentBehavior() throws RecognitionException {
		AgentBehaviorContext _localctx = new AgentBehaviorContext(_ctx, getState());
		enterRule(_localctx, 386, RULE_agentBehavior);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2988);
			match(T__188);
			setState(2989);
			ident();
			setState(2990);
			match(LBRACE);
			setState(2994);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(2991);
				statement();
				}
				}
				setState(2996);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(2997);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExplainStmtContext extends ParserRuleContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public ExplainStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_explainStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterExplainStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitExplainStmt(this);
		}
	}

	public final ExplainStmtContext explainStmt() throws RecognitionException {
		ExplainStmtContext _localctx = new ExplainStmtContext(_ctx, getState());
		enterRule(_localctx, 388, RULE_explainStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(2999);
			match(T__189);
			setState(3000);
			expression();
			setState(3001);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TransparentStmtContext extends ParserRuleContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SEMI() { return getToken(ZamaniParser.SEMI, 0); }
		public TransparentStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_transparentStmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterTransparentStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitTransparentStmt(this);
		}
	}

	public final TransparentStmtContext transparentStmt() throws RecognitionException {
		TransparentStmtContext _localctx = new TransparentStmtContext(_ctx, getState());
		enterRule(_localctx, 390, RULE_transparentStmt);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3003);
			match(T__190);
			setState(3004);
			expression();
			setState(3005);
			match(SEMI);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AsiCapabilityDefContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AsiCapabilityDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asiCapabilityDef; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAsiCapabilityDef(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAsiCapabilityDef(this);
		}
	}

	public final AsiCapabilityDefContext asiCapabilityDef() throws RecognitionException {
		AsiCapabilityDefContext _localctx = new AsiCapabilityDefContext(_ctx, getState());
		enterRule(_localctx, 392, RULE_asiCapabilityDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3007);
			match(T__93);
			setState(3008);
			match(T__187);
			setState(3009);
			ident();
			setState(3010);
			match(LBRACE);
			setState(3014);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(3011);
				statement();
				}
				}
				setState(3016);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(3017);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AesiCapabilityDefContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AesiCapabilityDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aesiCapabilityDef; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAesiCapabilityDef(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAesiCapabilityDef(this);
		}
	}

	public final AesiCapabilityDefContext aesiCapabilityDef() throws RecognitionException {
		AesiCapabilityDefContext _localctx = new AesiCapabilityDefContext(_ctx, getState());
		enterRule(_localctx, 394, RULE_aesiCapabilityDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3019);
			match(T__94);
			setState(3020);
			match(T__187);
			setState(3021);
			ident();
			setState(3022);
			match(LBRACE);
			setState(3026);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(3023);
				statement();
				}
				}
				setState(3028);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(3029);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AsesiCapabilityDefContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public AsesiCapabilityDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asesiCapabilityDef; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAsesiCapabilityDef(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAsesiCapabilityDef(this);
		}
	}

	public final AsesiCapabilityDefContext asesiCapabilityDef() throws RecognitionException {
		AsesiCapabilityDefContext _localctx = new AsesiCapabilityDefContext(_ctx, getState());
		enterRule(_localctx, 396, RULE_asesiCapabilityDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3031);
			match(T__95);
			setState(3032);
			match(T__187);
			setState(3033);
			ident();
			setState(3034);
			match(LBRACE);
			setState(3038);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (((((_la - 20)) & ~0x3f) == 0 && ((1L << (_la - 20)) & 35184438672639L) != 0) || ((((_la - 196)) & ~0x3f) == 0 && ((1L << (_la - 196)) & -2377838755722559473L) != 0) || ((((_la - 261)) & ~0x3f) == 0 && ((1L << (_la - 261)) & 9088264048029860249L) != 0) || ((((_la - 327)) & ~0x3f) == 0 && ((1L << (_la - 327)) & 12886622251L) != 0)) {
				{
				{
				setState(3035);
				statement();
				}
				}
				setState(3040);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(3041);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DocCommentContext extends ParserRuleContext {
		public List<TerminalNode> DOC_COMMENT() { return getTokens(ZamaniParser.DOC_COMMENT); }
		public TerminalNode DOC_COMMENT(int i) {
			return getToken(ZamaniParser.DOC_COMMENT, i);
		}
		public DocCommentContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_docComment; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterDocComment(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitDocComment(this);
		}
	}

	public final DocCommentContext docComment() throws RecognitionException {
		DocCommentContext _localctx = new DocCommentContext(_ctx, getState());
		enterRule(_localctx, 398, RULE_docComment);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3044); 
			_errHandler.sync(this);
			_la = _input.LA(1);
			do {
				{
				{
				setState(3043);
				match(DOC_COMMENT);
				}
				}
				setState(3046); 
				_errHandler.sync(this);
				_la = _input.LA(1);
			} while ( _la==DOC_COMMENT );
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AttributeDeclContext extends ParserRuleContext {
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public AttributeDeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_attributeDecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterAttributeDecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitAttributeDecl(this);
		}
	}

	public final AttributeDeclContext attributeDecl() throws RecognitionException {
		AttributeDeclContext _localctx = new AttributeDeclContext(_ctx, getState());
		enterRule(_localctx, 400, RULE_attributeDecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3048);
			match(T__191);
			setState(3049);
			ident();
			setState(3054);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(3050);
				match(COMMA);
				setState(3051);
				ident();
				}
				}
				setState(3056);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(3057);
			match(RBRACK);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class IdentContext extends ParserRuleContext {
		public TerminalNode IDENT() { return getToken(ZamaniParser.IDENT, 0); }
		public TerminalNode THIS() { return getToken(ZamaniParser.THIS, 0); }
		public TerminalNode SELF_LOWER() { return getToken(ZamaniParser.SELF_LOWER, 0); }
		public TerminalNode SELF_UPPER() { return getToken(ZamaniParser.SELF_UPPER, 0); }
		public TerminalNode INT_KW() { return getToken(ZamaniParser.INT_KW, 0); }
		public TerminalNode FLOAT_KW() { return getToken(ZamaniParser.FLOAT_KW, 0); }
		public TerminalNode BOOL_KW() { return getToken(ZamaniParser.BOOL_KW, 0); }
		public TerminalNode STR_KW() { return getToken(ZamaniParser.STR_KW, 0); }
		public TerminalNode STRING_KW() { return getToken(ZamaniParser.STRING_KW, 0); }
		public TerminalNode CHAR_KW() { return getToken(ZamaniParser.CHAR_KW, 0); }
		public TerminalNode VOID() { return getToken(ZamaniParser.VOID, 0); }
		public TerminalNode QUANTUM() { return getToken(ZamaniParser.QUANTUM, 0); }
		public TerminalNode NANO() { return getToken(ZamaniParser.NANO, 0); }
		public TerminalNode AGENT() { return getToken(ZamaniParser.AGENT, 0); }
		public TerminalNode CIRCUIT() { return getToken(ZamaniParser.CIRCUIT, 0); }
		public TerminalNode EFFECT() { return getToken(ZamaniParser.EFFECT, 0); }
		public TerminalNode HANDLE() { return getToken(ZamaniParser.HANDLE, 0); }
		public TerminalNode REMEMBER() { return getToken(ZamaniParser.REMEMBER, 0); }
		public TerminalNode RECALL() { return getToken(ZamaniParser.RECALL, 0); }
		public TerminalNode LEARN() { return getToken(ZamaniParser.LEARN, 0); }
		public TerminalNode INFER() { return getToken(ZamaniParser.INFER, 0); }
		public TerminalNode WISDOM() { return getToken(ZamaniParser.WISDOM, 0); }
		public TerminalNode ZAMANI() { return getToken(ZamaniParser.ZAMANI, 0); }
		public TerminalNode SASA() { return getToken(ZamaniParser.SASA, 0); }
		public TerminalNode ANCESTOR() { return getToken(ZamaniParser.ANCESTOR, 0); }
		public TerminalNode LINEAR() { return getToken(ZamaniParser.LINEAR, 0); }
		public TerminalNode AFFINE() { return getToken(ZamaniParser.AFFINE, 0); }
		public TerminalNode LANGUAGE() { return getToken(ZamaniParser.LANGUAGE, 0); }
		public TerminalNode MTS_KW() { return getToken(ZamaniParser.MTS_KW, 0); }
		public TerminalNode LEN() { return getToken(ZamaniParser.LEN, 0); }
		public TerminalNode PRINT() { return getToken(ZamaniParser.PRINT, 0); }
		public TerminalNode PRINTLN() { return getToken(ZamaniParser.PRINTLN, 0); }
		public TerminalNode ASSERT() { return getToken(ZamaniParser.ASSERT, 0); }
		public TerminalNode PANIC() { return getToken(ZamaniParser.PANIC, 0); }
		public IdentContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_ident; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIdent(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIdent(this);
		}
	}

	public final IdentContext ident() throws RecognitionException {
		IdentContext _localctx = new IdentContext(_ctx, getState());
		enterRule(_localctx, 402, RULE_ident);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3059);
			_la = _input.LA(1);
			if ( !(((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class StatementContext extends ParserRuleContext {
		public LetStmtContext letStmt() {
			return getRuleContext(LetStmtContext.class,0);
		}
		public ConstStmtContext constStmt() {
			return getRuleContext(ConstStmtContext.class,0);
		}
		public ReturnStmtContext returnStmt() {
			return getRuleContext(ReturnStmtContext.class,0);
		}
		public BreakStmtContext breakStmt() {
			return getRuleContext(BreakStmtContext.class,0);
		}
		public ContinueStmtContext continueStmt() {
			return getRuleContext(ContinueStmtContext.class,0);
		}
		public ThrowStmtContext throwStmt() {
			return getRuleContext(ThrowStmtContext.class,0);
		}
		public ExpressionStmtContext expressionStmt() {
			return getRuleContext(ExpressionStmtContext.class,0);
		}
		public IfExprContext ifExpr() {
			return getRuleContext(IfExprContext.class,0);
		}
		public MatchStmtContext matchStmt() {
			return getRuleContext(MatchStmtContext.class,0);
		}
		public WhileStmtContext whileStmt() {
			return getRuleContext(WhileStmtContext.class,0);
		}
		public ForStmtContext forStmt() {
			return getRuleContext(ForStmtContext.class,0);
		}
		public LoopExprContext loopExpr() {
			return getRuleContext(LoopExprContext.class,0);
		}
		public UnsafeBlockContext unsafeBlock() {
			return getRuleContext(UnsafeBlockContext.class,0);
		}
		public TryCatchStmtContext tryCatchStmt() {
			return getRuleContext(TryCatchStmtContext.class,0);
		}
		public BlockExprContext blockExpr() {
			return getRuleContext(BlockExprContext.class,0);
		}
		public StatementContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_statement; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterStatement(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitStatement(this);
		}
	}

	public final StatementContext statement() throws RecognitionException {
		StatementContext _localctx = new StatementContext(_ctx, getState());
		enterRule(_localctx, 404, RULE_statement);
		try {
			setState(3076);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,280,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(3061);
				letStmt();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(3062);
				constStmt();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(3063);
				returnStmt();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(3064);
				breakStmt();
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				setState(3065);
				continueStmt();
				}
				break;
			case 6:
				enterOuterAlt(_localctx, 6);
				{
				setState(3066);
				throwStmt();
				}
				break;
			case 7:
				enterOuterAlt(_localctx, 7);
				{
				setState(3067);
				expressionStmt();
				}
				break;
			case 8:
				enterOuterAlt(_localctx, 8);
				{
				setState(3068);
				ifExpr();
				}
				break;
			case 9:
				enterOuterAlt(_localctx, 9);
				{
				setState(3069);
				matchStmt();
				}
				break;
			case 10:
				enterOuterAlt(_localctx, 10);
				{
				setState(3070);
				whileStmt();
				}
				break;
			case 11:
				enterOuterAlt(_localctx, 11);
				{
				setState(3071);
				forStmt();
				}
				break;
			case 12:
				enterOuterAlt(_localctx, 12);
				{
				setState(3072);
				loopExpr();
				}
				break;
			case 13:
				enterOuterAlt(_localctx, 13);
				{
				setState(3073);
				unsafeBlock();
				}
				break;
			case 14:
				enterOuterAlt(_localctx, 14);
				{
				setState(3074);
				tryCatchStmt();
				}
				break;
			case 15:
				enterOuterAlt(_localctx, 15);
				{
				setState(3075);
				blockExpr();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class IdentListContext extends ParserRuleContext {
		public List<IdentContext> ident() {
			return getRuleContexts(IdentContext.class);
		}
		public IdentContext ident(int i) {
			return getRuleContext(IdentContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public IdentListContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_identList; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIdentList(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIdentList(this);
		}
	}

	public final IdentListContext identList() throws RecognitionException {
		IdentListContext _localctx = new IdentListContext(_ctx, getState());
		enterRule(_localctx, 406, RULE_identList);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3078);
			ident();
			setState(3083);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(3079);
				match(COMMA);
				setState(3080);
				ident();
				}
				}
				setState(3085);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class WhereClauseContext extends ParserRuleContext {
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public List<TerminalNode> COLON() { return getTokens(ZamaniParser.COLON); }
		public TerminalNode COLON(int i) {
			return getToken(ZamaniParser.COLON, i);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public WhereClauseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_whereClause; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterWhereClause(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitWhereClause(this);
		}
	}

	public final WhereClauseContext whereClause() throws RecognitionException {
		WhereClauseContext _localctx = new WhereClauseContext(_ctx, getState());
		enterRule(_localctx, 408, RULE_whereClause);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3086);
			typeExpr(0);
			setState(3089);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==COLON) {
				{
				setState(3087);
				match(COLON);
				setState(3088);
				typeExpr(0);
				}
			}

			setState(3099);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==COMMA) {
				{
				{
				setState(3091);
				match(COMMA);
				setState(3092);
				typeExpr(0);
				setState(3095);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COLON) {
					{
					setState(3093);
					match(COLON);
					setState(3094);
					typeExpr(0);
					}
				}

				}
				}
				setState(3101);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LiteralContext extends ParserRuleContext {
		public TerminalNode INTEGER() { return getToken(ZamaniParser.INTEGER, 0); }
		public TerminalNode FLOAT() { return getToken(ZamaniParser.FLOAT, 0); }
		public TerminalNode STRING() { return getToken(ZamaniParser.STRING, 0); }
		public TerminalNode CHAR() { return getToken(ZamaniParser.CHAR, 0); }
		public TerminalNode BOOLEAN() { return getToken(ZamaniParser.BOOLEAN, 0); }
		public TerminalNode NIL() { return getToken(ZamaniParser.NIL, 0); }
		public QuantumLitContext quantumLit() {
			return getRuleContext(QuantumLitContext.class,0);
		}
		public NanoLitContext nanoLit() {
			return getRuleContext(NanoLitContext.class,0);
		}
		public MtsLitContext mtsLit() {
			return getRuleContext(MtsLitContext.class,0);
		}
		public RawStringLitContext rawStringLit() {
			return getRuleContext(RawStringLitContext.class,0);
		}
		public Utf8StringLitContext utf8StringLit() {
			return getRuleContext(Utf8StringLitContext.class,0);
		}
		public LiteralContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_literal; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterLiteral(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitLiteral(this);
		}
	}

	public final LiteralContext literal() throws RecognitionException {
		LiteralContext _localctx = new LiteralContext(_ctx, getState());
		enterRule(_localctx, 410, RULE_literal);
		try {
			setState(3113);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INTEGER:
				enterOuterAlt(_localctx, 1);
				{
				setState(3102);
				match(INTEGER);
				}
				break;
			case FLOAT:
				enterOuterAlt(_localctx, 2);
				{
				setState(3103);
				match(FLOAT);
				}
				break;
			case STRING:
				enterOuterAlt(_localctx, 3);
				{
				setState(3104);
				match(STRING);
				}
				break;
			case CHAR:
				enterOuterAlt(_localctx, 4);
				{
				setState(3105);
				match(CHAR);
				}
				break;
			case BOOLEAN:
				enterOuterAlt(_localctx, 5);
				{
				setState(3106);
				match(BOOLEAN);
				}
				break;
			case NIL:
				enterOuterAlt(_localctx, 6);
				{
				setState(3107);
				match(NIL);
				}
				break;
			case PIPE:
				enterOuterAlt(_localctx, 7);
				{
				setState(3108);
				quantumLit();
				}
				break;
			case T__195:
			case T__196:
				enterOuterAlt(_localctx, 8);
				{
				setState(3109);
				nanoLit();
				}
				break;
			case MTS_KW:
				enterOuterAlt(_localctx, 9);
				{
				setState(3110);
				mtsLit();
				}
				break;
			case T__197:
				enterOuterAlt(_localctx, 10);
				{
				setState(3111);
				rawStringLit();
				}
				break;
			case T__64:
				enterOuterAlt(_localctx, 11);
				{
				setState(3112);
				utf8StringLit();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class QuantumLitContext extends ParserRuleContext {
		public TerminalNode PIPE() { return getToken(ZamaniParser.PIPE, 0); }
		public TerminalNode PLUS() { return getToken(ZamaniParser.PLUS, 0); }
		public TerminalNode MINUS() { return getToken(ZamaniParser.MINUS, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public QuantumLitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_quantumLit; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterQuantumLit(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitQuantumLit(this);
		}
	}

	public final QuantumLitContext quantumLit() throws RecognitionException {
		QuantumLitContext _localctx = new QuantumLitContext(_ctx, getState());
		enterRule(_localctx, 412, RULE_quantumLit);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3115);
			match(PIPE);
			setState(3121);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__192:
				{
				setState(3116);
				match(T__192);
				}
				break;
			case T__193:
				{
				setState(3117);
				match(T__193);
				}
				break;
			case PLUS:
				{
				setState(3118);
				match(PLUS);
				}
				break;
			case MINUS:
				{
				setState(3119);
				match(MINUS);
				}
				break;
			case QUANTUM:
			case CIRCUIT:
			case THIS:
			case SELF_LOWER:
			case SELF_UPPER:
			case INT_KW:
			case FLOAT_KW:
			case BOOL_KW:
			case STR_KW:
			case STRING_KW:
			case CHAR_KW:
			case VOID:
			case NANO:
			case AGENT:
			case EFFECT:
			case HANDLE:
			case REMEMBER:
			case RECALL:
			case LEARN:
			case INFER:
			case WISDOM:
			case ZAMANI:
			case SASA:
			case ANCESTOR:
			case LINEAR:
			case AFFINE:
			case LANGUAGE:
			case MTS_KW:
			case LEN:
			case PRINT:
			case PRINTLN:
			case ASSERT:
			case PANIC:
			case IDENT:
				{
				setState(3120);
				ident();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			setState(3123);
			match(T__194);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class NanoLitContext extends ParserRuleContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public TerminalNode ORBITAL() { return getToken(ZamaniParser.ORBITAL, 0); }
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TerminalNode FORMULA() { return getToken(ZamaniParser.FORMULA, 0); }
		public NanoLitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_nanoLit; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNanoLit(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNanoLit(this);
		}
	}

	public final NanoLitContext nanoLit() throws RecognitionException {
		NanoLitContext _localctx = new NanoLitContext(_ctx, getState());
		enterRule(_localctx, 414, RULE_nanoLit);
		try {
			setState(3136);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__195:
				enterOuterAlt(_localctx, 1);
				{
				setState(3125);
				match(T__195);
				setState(3126);
				match(LPAREN);
				setState(3127);
				ident();
				setState(3128);
				match(COLON);
				setState(3129);
				match(ORBITAL);
				setState(3130);
				match(RPAREN);
				}
				break;
			case T__196:
				enterOuterAlt(_localctx, 2);
				{
				setState(3132);
				match(T__196);
				setState(3133);
				match(LPAREN);
				setState(3134);
				match(FORMULA);
				setState(3135);
				match(RPAREN);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MtsLitContext extends ParserRuleContext {
		public TerminalNode MTS_KW() { return getToken(ZamaniParser.MTS_KW, 0); }
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public TerminalNode STRING() { return getToken(ZamaniParser.STRING, 0); }
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public MtsLitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mtsLit; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMtsLit(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMtsLit(this);
		}
	}

	public final MtsLitContext mtsLit() throws RecognitionException {
		MtsLitContext _localctx = new MtsLitContext(_ctx, getState());
		enterRule(_localctx, 416, RULE_mtsLit);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3138);
			match(MTS_KW);
			setState(3139);
			match(LBRACK);
			setState(3140);
			match(STRING);
			setState(3141);
			match(RBRACK);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RawStringLitContext extends ParserRuleContext {
		public TerminalNode STRING() { return getToken(ZamaniParser.STRING, 0); }
		public RawStringLitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_rawStringLit; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterRawStringLit(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitRawStringLit(this);
		}
	}

	public final RawStringLitContext rawStringLit() throws RecognitionException {
		RawStringLitContext _localctx = new RawStringLitContext(_ctx, getState());
		enterRule(_localctx, 418, RULE_rawStringLit);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3143);
			match(T__197);
			setState(3144);
			match(STRING);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Utf8StringLitContext extends ParserRuleContext {
		public TerminalNode STRING() { return getToken(ZamaniParser.STRING, 0); }
		public Utf8StringLitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_utf8StringLit; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterUtf8StringLit(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitUtf8StringLit(this);
		}
	}

	public final Utf8StringLitContext utf8StringLit() throws RecognitionException {
		Utf8StringLitContext _localctx = new Utf8StringLitContext(_ctx, getState());
		enterRule(_localctx, 420, RULE_utf8StringLit);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3146);
			match(T__64);
			setState(3147);
			match(STRING);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class InterpolatedStringContext extends ParserRuleContext {
		public TerminalNode STRING() { return getToken(ZamaniParser.STRING, 0); }
		public InterpolatedStringContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_interpolatedString; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterInterpolatedString(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitInterpolatedString(this);
		}
	}

	public final InterpolatedStringContext interpolatedString() throws RecognitionException {
		InterpolatedStringContext _localctx = new InterpolatedStringContext(_ctx, getState());
		enterRule(_localctx, 422, RULE_interpolatedString);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3149);
			match(T__198);
			setState(3150);
			match(STRING);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PiTypeContext extends ParserRuleContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TerminalNode DOT() { return getToken(ZamaniParser.DOT, 0); }
		public PiTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_piType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterPiType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitPiType(this);
		}
	}

	public final PiTypeContext piType() throws RecognitionException {
		PiTypeContext _localctx = new PiTypeContext(_ctx, getState());
		enterRule(_localctx, 424, RULE_piType);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3152);
			match(T__199);
			setState(3153);
			match(LPAREN);
			setState(3154);
			ident();
			setState(3155);
			match(COLON);
			setState(3156);
			typeExpr(0);
			setState(3157);
			match(RPAREN);
			setState(3158);
			match(DOT);
			setState(3159);
			typeExpr(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SigmaTypeContext extends ParserRuleContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode COLON() { return getToken(ZamaniParser.COLON, 0); }
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public TerminalNode DOT() { return getToken(ZamaniParser.DOT, 0); }
		public SigmaTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sigmaType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSigmaType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSigmaType(this);
		}
	}

	public final SigmaTypeContext sigmaType() throws RecognitionException {
		SigmaTypeContext _localctx = new SigmaTypeContext(_ctx, getState());
		enterRule(_localctx, 426, RULE_sigmaType);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3161);
			match(T__200);
			setState(3162);
			match(LPAREN);
			setState(3163);
			ident();
			setState(3164);
			match(COLON);
			setState(3165);
			typeExpr(0);
			setState(3166);
			match(RPAREN);
			setState(3167);
			match(DOT);
			setState(3168);
			typeExpr(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class IdentityTypeContext extends ParserRuleContext {
		public TerminalNode LPAREN() { return getToken(ZamaniParser.LPAREN, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public List<TerminalNode> COMMA() { return getTokens(ZamaniParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(ZamaniParser.COMMA, i);
		}
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public TerminalNode RPAREN() { return getToken(ZamaniParser.RPAREN, 0); }
		public IdentityTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_identityType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterIdentityType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitIdentityType(this);
		}
	}

	public final IdentityTypeContext identityType() throws RecognitionException {
		IdentityTypeContext _localctx = new IdentityTypeContext(_ctx, getState());
		enterRule(_localctx, 428, RULE_identityType);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3170);
			match(T__201);
			setState(3171);
			match(LPAREN);
			setState(3172);
			typeExpr(0);
			setState(3173);
			match(COMMA);
			setState(3174);
			expression();
			setState(3175);
			match(COMMA);
			setState(3176);
			expression();
			setState(3177);
			match(RPAREN);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SessionOpContext extends ParserRuleContext {
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public TerminalNode LBRACE() { return getToken(ZamaniParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(ZamaniParser.RBRACE, 0); }
		public List<SessionBranchContext> sessionBranch() {
			return getRuleContexts(SessionBranchContext.class);
		}
		public SessionBranchContext sessionBranch(int i) {
			return getRuleContext(SessionBranchContext.class,i);
		}
		public SessionOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sessionOp; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSessionOp(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSessionOp(this);
		}
	}

	public final SessionOpContext sessionOp() throws RecognitionException {
		SessionOpContext _localctx = new SessionOpContext(_ctx, getState());
		enterRule(_localctx, 430, RULE_sessionOp);
		int _la;
		try {
			setState(3202);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__202:
				enterOuterAlt(_localctx, 1);
				{
				setState(3179);
				match(T__202);
				setState(3180);
				typeExpr(0);
				}
				break;
			case T__203:
				enterOuterAlt(_localctx, 2);
				{
				setState(3181);
				match(T__203);
				setState(3182);
				typeExpr(0);
				}
				break;
			case T__204:
				enterOuterAlt(_localctx, 3);
				{
				setState(3183);
				match(T__204);
				setState(3184);
				match(LBRACE);
				setState(3188);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
					{
					{
					setState(3185);
					sessionBranch();
					}
					}
					setState(3190);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(3191);
				match(RBRACE);
				}
				break;
			case T__205:
				enterOuterAlt(_localctx, 4);
				{
				setState(3192);
				match(T__205);
				setState(3193);
				match(LBRACE);
				setState(3197);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (((((_la - 278)) & ~0x3f) == 0 && ((1L << (_la - 278)) & 563018672898019L) != 0)) {
					{
					{
					setState(3194);
					sessionBranch();
					}
					}
					setState(3199);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(3200);
				match(RBRACE);
				}
				break;
			case T__206:
				enterOuterAlt(_localctx, 5);
				{
				setState(3201);
				match(T__206);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SessionBranchContext extends ParserRuleContext {
		public IdentContext ident() {
			return getRuleContext(IdentContext.class,0);
		}
		public TerminalNode ARROW() { return getToken(ZamaniParser.ARROW, 0); }
		public TypeExprContext typeExpr() {
			return getRuleContext(TypeExprContext.class,0);
		}
		public SessionBranchContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sessionBranch; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSessionBranch(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSessionBranch(this);
		}
	}

	public final SessionBranchContext sessionBranch() throws RecognitionException {
		SessionBranchContext _localctx = new SessionBranchContext(_ctx, getState());
		enterRule(_localctx, 432, RULE_sessionBranch);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3204);
			ident();
			setState(3205);
			match(ARROW);
			setState(3206);
			typeExpr(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class QuantumTypeContext extends ParserRuleContext {
		public TerminalNode LBRACK() { return getToken(ZamaniParser.LBRACK, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode RBRACK() { return getToken(ZamaniParser.RBRACK, 0); }
		public TerminalNode LT() { return getToken(ZamaniParser.LT, 0); }
		public List<TypeExprContext> typeExpr() {
			return getRuleContexts(TypeExprContext.class);
		}
		public TypeExprContext typeExpr(int i) {
			return getRuleContext(TypeExprContext.class,i);
		}
		public TerminalNode GT() { return getToken(ZamaniParser.GT, 0); }
		public TerminalNode COMMA() { return getToken(ZamaniParser.COMMA, 0); }
		public QuantumTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_quantumType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterQuantumType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitQuantumType(this);
		}
	}

	public final QuantumTypeContext quantumType() throws RecognitionException {
		QuantumTypeContext _localctx = new QuantumTypeContext(_ctx, getState());
		enterRule(_localctx, 434, RULE_quantumType);
		try {
			setState(3226);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__207:
				enterOuterAlt(_localctx, 1);
				{
				setState(3208);
				match(T__207);
				}
				break;
			case T__208:
				enterOuterAlt(_localctx, 2);
				{
				setState(3209);
				match(T__208);
				setState(3210);
				match(LBRACK);
				setState(3211);
				expression();
				setState(3212);
				match(RBRACK);
				}
				break;
			case T__209:
				enterOuterAlt(_localctx, 3);
				{
				setState(3214);
				match(T__209);
				setState(3215);
				match(LT);
				setState(3216);
				typeExpr(0);
				setState(3217);
				match(GT);
				}
				break;
			case T__210:
				enterOuterAlt(_localctx, 4);
				{
				setState(3219);
				match(T__210);
				setState(3220);
				match(LT);
				setState(3221);
				typeExpr(0);
				setState(3222);
				match(COMMA);
				setState(3223);
				typeExpr(0);
				setState(3224);
				match(GT);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class NanoTypeContext extends ParserRuleContext {
		public NanoTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_nanoType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterNanoType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitNanoType(this);
		}
	}

	public final NanoTypeContext nanoType() throws RecognitionException {
		NanoTypeContext _localctx = new NanoTypeContext(_ctx, getState());
		enterRule(_localctx, 436, RULE_nanoType);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3228);
			_la = _input.LA(1);
			if ( !(((((_la - 212)) & ~0x3f) == 0 && ((1L << (_la - 212)) & 7L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MtsTypeContext extends ParserRuleContext {
		public MtsTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mtsType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterMtsType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitMtsType(this);
		}
	}

	public final MtsTypeContext mtsType() throws RecognitionException {
		MtsTypeContext _localctx = new MtsTypeContext(_ctx, getState());
		enterRule(_localctx, 438, RULE_mtsType);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3230);
			_la = _input.LA(1);
			if ( !(((((_la - 215)) & ~0x3f) == 0 && ((1L << (_la - 215)) & 7L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class SankofaTypeContext extends ParserRuleContext {
		public SankofaTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_sankofaType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterSankofaType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitSankofaType(this);
		}
	}

	public final SankofaTypeContext sankofaType() throws RecognitionException {
		SankofaTypeContext _localctx = new SankofaTypeContext(_ctx, getState());
		enterRule(_localctx, 440, RULE_sankofaType);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3232);
			_la = _input.LA(1);
			if ( !(((((_la - 218)) & ~0x3f) == 0 && ((1L << (_la - 218)) & 7L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CognitiveTypeContext extends ParserRuleContext {
		public CognitiveTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_cognitiveType; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).enterCognitiveType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof ZamaniListener ) ((ZamaniListener)listener).exitCognitiveType(this);
		}
	}

	public final CognitiveTypeContext cognitiveType() throws RecognitionException {
		CognitiveTypeContext _localctx = new CognitiveTypeContext(_ctx, getState());
		enterRule(_localctx, 442, RULE_cognitiveType);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3234);
			_la = _input.LA(1);
			if ( !(((((_la - 221)) & ~0x3f) == 0 && ((1L << (_la - 221)) & 7L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 25:
			return pattern_sempred((PatternContext)_localctx, predIndex);
		case 70:
			return typeExpr_sempred((TypeExprContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean pattern_sempred(PatternContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 2);
		}
		return true;
	}
	private boolean typeExpr_sempred(TypeExprContext _localctx, int predIndex) {
		switch (predIndex) {
		case 1:
			return precpred(_ctx, 25);
		case 2:
			return precpred(_ctx, 5);
		case 3:
			return precpred(_ctx, 1);
		}
		return true;
	}

	private static final String _serializedATNSegment0 =
		"\u0004\u0001\u0178\u0ca5\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001"+
		"\u0002\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004"+
		"\u0002\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007"+
		"\u0002\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b"+
		"\u0002\f\u0007\f\u0002\r\u0007\r\u0002\u000e\u0007\u000e\u0002\u000f\u0007"+
		"\u000f\u0002\u0010\u0007\u0010\u0002\u0011\u0007\u0011\u0002\u0012\u0007"+
		"\u0012\u0002\u0013\u0007\u0013\u0002\u0014\u0007\u0014\u0002\u0015\u0007"+
		"\u0015\u0002\u0016\u0007\u0016\u0002\u0017\u0007\u0017\u0002\u0018\u0007"+
		"\u0018\u0002\u0019\u0007\u0019\u0002\u001a\u0007\u001a\u0002\u001b\u0007"+
		"\u001b\u0002\u001c\u0007\u001c\u0002\u001d\u0007\u001d\u0002\u001e\u0007"+
		"\u001e\u0002\u001f\u0007\u001f\u0002 \u0007 \u0002!\u0007!\u0002\"\u0007"+
		"\"\u0002#\u0007#\u0002$\u0007$\u0002%\u0007%\u0002&\u0007&\u0002\'\u0007"+
		"\'\u0002(\u0007(\u0002)\u0007)\u0002*\u0007*\u0002+\u0007+\u0002,\u0007"+
		",\u0002-\u0007-\u0002.\u0007.\u0002/\u0007/\u00020\u00070\u00021\u0007"+
		"1\u00022\u00072\u00023\u00073\u00024\u00074\u00025\u00075\u00026\u0007"+
		"6\u00027\u00077\u00028\u00078\u00029\u00079\u0002:\u0007:\u0002;\u0007"+
		";\u0002<\u0007<\u0002=\u0007=\u0002>\u0007>\u0002?\u0007?\u0002@\u0007"+
		"@\u0002A\u0007A\u0002B\u0007B\u0002C\u0007C\u0002D\u0007D\u0002E\u0007"+
		"E\u0002F\u0007F\u0002G\u0007G\u0002H\u0007H\u0002I\u0007I\u0002J\u0007"+
		"J\u0002K\u0007K\u0002L\u0007L\u0002M\u0007M\u0002N\u0007N\u0002O\u0007"+
		"O\u0002P\u0007P\u0002Q\u0007Q\u0002R\u0007R\u0002S\u0007S\u0002T\u0007"+
		"T\u0002U\u0007U\u0002V\u0007V\u0002W\u0007W\u0002X\u0007X\u0002Y\u0007"+
		"Y\u0002Z\u0007Z\u0002[\u0007[\u0002\\\u0007\\\u0002]\u0007]\u0002^\u0007"+
		"^\u0002_\u0007_\u0002`\u0007`\u0002a\u0007a\u0002b\u0007b\u0002c\u0007"+
		"c\u0002d\u0007d\u0002e\u0007e\u0002f\u0007f\u0002g\u0007g\u0002h\u0007"+
		"h\u0002i\u0007i\u0002j\u0007j\u0002k\u0007k\u0002l\u0007l\u0002m\u0007"+
		"m\u0002n\u0007n\u0002o\u0007o\u0002p\u0007p\u0002q\u0007q\u0002r\u0007"+
		"r\u0002s\u0007s\u0002t\u0007t\u0002u\u0007u\u0002v\u0007v\u0002w\u0007"+
		"w\u0002x\u0007x\u0002y\u0007y\u0002z\u0007z\u0002{\u0007{\u0002|\u0007"+
		"|\u0002}\u0007}\u0002~\u0007~\u0002\u007f\u0007\u007f\u0002\u0080\u0007"+
		"\u0080\u0002\u0081\u0007\u0081\u0002\u0082\u0007\u0082\u0002\u0083\u0007"+
		"\u0083\u0002\u0084\u0007\u0084\u0002\u0085\u0007\u0085\u0002\u0086\u0007"+
		"\u0086\u0002\u0087\u0007\u0087\u0002\u0088\u0007\u0088\u0002\u0089\u0007"+
		"\u0089\u0002\u008a\u0007\u008a\u0002\u008b\u0007\u008b\u0002\u008c\u0007"+
		"\u008c\u0002\u008d\u0007\u008d\u0002\u008e\u0007\u008e\u0002\u008f\u0007"+
		"\u008f\u0002\u0090\u0007\u0090\u0002\u0091\u0007\u0091\u0002\u0092\u0007"+
		"\u0092\u0002\u0093\u0007\u0093\u0002\u0094\u0007\u0094\u0002\u0095\u0007"+
		"\u0095\u0002\u0096\u0007\u0096\u0002\u0097\u0007\u0097\u0002\u0098\u0007"+
		"\u0098\u0002\u0099\u0007\u0099\u0002\u009a\u0007\u009a\u0002\u009b\u0007"+
		"\u009b\u0002\u009c\u0007\u009c\u0002\u009d\u0007\u009d\u0002\u009e\u0007"+
		"\u009e\u0002\u009f\u0007\u009f\u0002\u00a0\u0007\u00a0\u0002\u00a1\u0007"+
		"\u00a1\u0002\u00a2\u0007\u00a2\u0002\u00a3\u0007\u00a3\u0002\u00a4\u0007"+
		"\u00a4\u0002\u00a5\u0007\u00a5\u0002\u00a6\u0007\u00a6\u0002\u00a7\u0007"+
		"\u00a7\u0002\u00a8\u0007\u00a8\u0002\u00a9\u0007\u00a9\u0002\u00aa\u0007"+
		"\u00aa\u0002\u00ab\u0007\u00ab\u0002\u00ac\u0007\u00ac\u0002\u00ad\u0007"+
		"\u00ad\u0002\u00ae\u0007\u00ae\u0002\u00af\u0007\u00af\u0002\u00b0\u0007"+
		"\u00b0\u0002\u00b1\u0007\u00b1\u0002\u00b2\u0007\u00b2\u0002\u00b3\u0007"+
		"\u00b3\u0002\u00b4\u0007\u00b4\u0002\u00b5\u0007\u00b5\u0002\u00b6\u0007"+
		"\u00b6\u0002\u00b7\u0007\u00b7\u0002\u00b8\u0007\u00b8\u0002\u00b9\u0007"+
		"\u00b9\u0002\u00ba\u0007\u00ba\u0002\u00bb\u0007\u00bb\u0002\u00bc\u0007"+
		"\u00bc\u0002\u00bd\u0007\u00bd\u0002\u00be\u0007\u00be\u0002\u00bf\u0007"+
		"\u00bf\u0002\u00c0\u0007\u00c0\u0002\u00c1\u0007\u00c1\u0002\u00c2\u0007"+
		"\u00c2\u0002\u00c3\u0007\u00c3\u0002\u00c4\u0007\u00c4\u0002\u00c5\u0007"+
		"\u00c5\u0002\u00c6\u0007\u00c6\u0002\u00c7\u0007\u00c7\u0002\u00c8\u0007"+
		"\u00c8\u0002\u00c9\u0007\u00c9\u0002\u00ca\u0007\u00ca\u0002\u00cb\u0007"+
		"\u00cb\u0002\u00cc\u0007\u00cc\u0002\u00cd\u0007\u00cd\u0002\u00ce\u0007"+
		"\u00ce\u0002\u00cf\u0007\u00cf\u0002\u00d0\u0007\u00d0\u0002\u00d1\u0007"+
		"\u00d1\u0002\u00d2\u0007\u00d2\u0002\u00d3\u0007\u00d3\u0002\u00d4\u0007"+
		"\u00d4\u0002\u00d5\u0007\u00d5\u0002\u00d6\u0007\u00d6\u0002\u00d7\u0007"+
		"\u00d7\u0002\u00d8\u0007\u00d8\u0002\u00d9\u0007\u00d9\u0002\u00da\u0007"+
		"\u00da\u0002\u00db\u0007\u00db\u0002\u00dc\u0007\u00dc\u0002\u00dd\u0007"+
		"\u00dd\u0001\u0000\u0005\u0000\u01be\b\u0000\n\u0000\f\u0000\u01c1\t\u0000"+
		"\u0001\u0000\u0001\u0000\u0001\u0001\u0003\u0001\u01c6\b\u0001\u0001\u0001"+
		"\u0003\u0001\u01c9\b\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0003\u0001\u023b\b\u0001\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0002"+
		"\u0005\u0002\u0241\b\u0002\n\u0002\f\u0002\u0244\t\u0002\u0001\u0002\u0001"+
		"\u0002\u0003\u0002\u0248\b\u0002\u0001\u0003\u0001\u0003\u0001\u0003\u0001"+
		"\u0003\u0003\u0003\u024e\b\u0003\u0001\u0003\u0003\u0003\u0251\b\u0003"+
		"\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0004\u0003\u0004\u0257\b\u0004"+
		"\u0001\u0004\u0003\u0004\u025a\b\u0004\u0001\u0005\u0001\u0005\u0001\u0005"+
		"\u0001\u0005\u0001\u0005\u0005\u0005\u0261\b\u0005\n\u0005\f\u0005\u0264"+
		"\t\u0005\u0001\u0006\u0001\u0006\u0001\u0006\u0003\u0006\u0269\b\u0006"+
		"\u0001\u0007\u0001\u0007\u0001\u0007\u0005\u0007\u026e\b\u0007\n\u0007"+
		"\f\u0007\u0271\t\u0007\u0001\u0007\u0001\u0007\u0003\u0007\u0275\b\u0007"+
		"\u0001\u0007\u0001\u0007\u0001\u0007\u0005\u0007\u027a\b\u0007\n\u0007"+
		"\f\u0007\u027d\t\u0007\u0001\u0007\u0001\u0007\u0001\u0007\u0001\u0007"+
		"\u0001\u0007\u0005\u0007\u0284\b\u0007\n\u0007\f\u0007\u0287\t\u0007\u0001"+
		"\u0007\u0001\u0007\u0003\u0007\u028b\b\u0007\u0001\b\u0001\b\u0001\t\u0001"+
		"\t\u0001\t\u0001\t\u0001\t\u0001\n\u0003\n\u0295\b\n\u0001\n\u0001\n\u0001"+
		"\n\u0003\n\u029a\b\n\u0001\n\u0001\n\u0003\n\u029e\b\n\u0001\n\u0001\n"+
		"\u0001\n\u0003\n\u02a3\b\n\u0001\n\u0001\n\u0003\n\u02a7\b\n\u0001\n\u0001"+
		"\n\u0001\u000b\u0001\u000b\u0001\u000b\u0005\u000b\u02ae\b\u000b\n\u000b"+
		"\f\u000b\u02b1\t\u000b\u0001\f\u0003\f\u02b4\b\f\u0001\f\u0001\f\u0001"+
		"\f\u0003\f\u02b9\b\f\u0001\f\u0001\f\u0003\f\u02bd\b\f\u0001\f\u0001\f"+
		"\u0001\f\u0001\f\u0003\f\u02c3\b\f\u0001\r\u0004\r\u02c6\b\r\u000b\r\f"+
		"\r\u02c7\u0001\u000e\u0001\u000e\u0001\u000f\u0001\u000f\u0003\u000f\u02ce"+
		"\b\u000f\u0001\u000f\u0001\u000f\u0001\u0010\u0001\u0010\u0001\u0010\u0001"+
		"\u0011\u0001\u0011\u0001\u0011\u0001\u0012\u0001\u0012\u0001\u0012\u0001"+
		"\u0012\u0001\u0013\u0001\u0013\u0001\u0013\u0001\u0013\u0001\u0013\u0001"+
		"\u0013\u0001\u0014\u0001\u0014\u0001\u0014\u0001\u0015\u0001\u0015\u0001"+
		"\u0015\u0001\u0015\u0001\u0015\u0001\u0015\u0003\u0015\u02eb\b\u0015\u0003"+
		"\u0015\u02ed\b\u0015\u0001\u0016\u0001\u0016\u0001\u0017\u0001\u0017\u0001"+
		"\u0017\u0001\u0017\u0005\u0017\u02f5\b\u0017\n\u0017\f\u0017\u02f8\t\u0017"+
		"\u0001\u0017\u0001\u0017\u0001\u0018\u0001\u0018\u0001\u0018\u0003\u0018"+
		"\u02ff\b\u0018\u0001\u0018\u0001\u0018\u0003\u0018\u0303\b\u0018\u0001"+
		"\u0018\u0001\u0018\u0001\u0018\u0003\u0018\u0308\b\u0018\u0001\u0019\u0001"+
		"\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0001"+
		"\u0019\u0005\u0019\u0312\b\u0019\n\u0019\f\u0019\u0315\t\u0019\u0001\u0019"+
		"\u0001\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0005\u0019"+
		"\u031d\b\u0019\n\u0019\f\u0019\u0320\t\u0019\u0001\u0019\u0001\u0019\u0001"+
		"\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0005\u0019\u0328\b\u0019\n"+
		"\u0019\f\u0019\u032b\t\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0001"+
		"\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0003\u0019\u0335"+
		"\b\u0019\u0001\u0019\u0001\u0019\u0001\u0019\u0005\u0019\u033a\b\u0019"+
		"\n\u0019\f\u0019\u033d\t\u0019\u0001\u001a\u0001\u001a\u0003\u001a\u0341"+
		"\b\u001a\u0001\u001a\u0001\u001a\u0001\u001a\u0001\u001a\u0001\u001a\u0001"+
		"\u001a\u0001\u001a\u0001\u001a\u0001\u001a\u0003\u001a\u034c\b\u001a\u0001"+
		"\u001b\u0001\u001b\u0001\u001b\u0001\u001b\u0001\u001c\u0001\u001c\u0001"+
		"\u001c\u0005\u001c\u0355\b\u001c\n\u001c\f\u001c\u0358\t\u001c\u0001\u001c"+
		"\u0003\u001c\u035b\b\u001c\u0001\u001d\u0001\u001d\u0001\u001d\u0001\u001d"+
		"\u0001\u001d\u0003\u001d\u0362\b\u001d\u0001\u001d\u0001\u001d\u0001\u001e"+
		"\u0001\u001e\u0001\u001e\u0001\u001f\u0001\u001f\u0005\u001f\u036b\b\u001f"+
		"\n\u001f\f\u001f\u036e\t\u001f\u0001\u001f\u0001\u001f\u0001 \u0001 \u0003"+
		" \u0374\b \u0001 \u0001 \u0001 \u0003 \u0379\b \u0001 \u0001 \u0001 \u0001"+
		" \u0001!\u0001!\u0001!\u0001!\u0003!\u0383\b!\u0001!\u0001!\u0001!\u0001"+
		"!\u0001\"\u0001\"\u0001#\u0001#\u0001#\u0001#\u0003#\u038f\b#\u0001$\u0001"+
		"$\u0001%\u0001%\u0001%\u0003%\u0396\b%\u0001&\u0001&\u0001&\u0005&\u039b"+
		"\b&\n&\f&\u039e\t&\u0001\'\u0001\'\u0001\'\u0005\'\u03a3\b\'\n\'\f\'\u03a6"+
		"\t\'\u0001(\u0001(\u0001(\u0005(\u03ab\b(\n(\f(\u03ae\t(\u0001)\u0001"+
		")\u0001)\u0005)\u03b3\b)\n)\f)\u03b6\t)\u0001*\u0001*\u0001*\u0005*\u03bb"+
		"\b*\n*\f*\u03be\t*\u0001+\u0001+\u0001+\u0005+\u03c3\b+\n+\f+\u03c6\t"+
		"+\u0001,\u0001,\u0001,\u0005,\u03cb\b,\n,\f,\u03ce\t,\u0001-\u0001-\u0001"+
		"-\u0005-\u03d3\b-\n-\f-\u03d6\t-\u0001.\u0001.\u0001.\u0005.\u03db\b."+
		"\n.\f.\u03de\t.\u0001/\u0001/\u0001/\u0005/\u03e3\b/\n/\f/\u03e6\t/\u0001"+
		"0\u00010\u00010\u00050\u03eb\b0\n0\f0\u03ee\t0\u00011\u00011\u00011\u0001"+
		"1\u00011\u00031\u03f5\b1\u00011\u00011\u00011\u00031\u03fa\b1\u00011\u0001"+
		"1\u00031\u03fe\b1\u00012\u00012\u00052\u0402\b2\n2\f2\u0405\t2\u00013"+
		"\u00013\u00033\u0409\b3\u00013\u00013\u00013\u00013\u00013\u00013\u0001"+
		"3\u00013\u00013\u00033\u0414\b3\u00013\u00033\u0417\b3\u00013\u00013\u0001"+
		"3\u00013\u00013\u00013\u00013\u00013\u00013\u00013\u00013\u00013\u0001"+
		"3\u00013\u00013\u00053\u0428\b3\n3\f3\u042b\t3\u00013\u00033\u042e\b3"+
		"\u00014\u00014\u00014\u00054\u0433\b4\n4\f4\u0436\t4\u00014\u00014\u0001"+
		"4\u00054\u043b\b4\n4\f4\u043e\t4\u00034\u0440\b4\u00015\u00015\u00015"+
		"\u00015\u00016\u00016\u00036\u0448\b6\u00016\u00016\u00016\u00016\u0001"+
		"6\u00016\u00016\u00016\u00016\u00046\u0453\b6\u000b6\f6\u0454\u00016\u0001"+
		"6\u00016\u00016\u00016\u00016\u00056\u045d\b6\n6\f6\u0460\t6\u00036\u0462"+
		"\b6\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u0001"+
		"6\u00016\u00056\u046f\b6\n6\f6\u0472\t6\u00036\u0474\b6\u00016\u00016"+
		"\u00016\u00016\u00036\u047a\b6\u00016\u00016\u00016\u00036\u047f\b6\u0001"+
		"6\u00016\u00016\u00036\u0484\b6\u00016\u00016\u00016\u00036\u0489\b6\u0001"+
		"6\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u0001"+
		"6\u00016\u00016\u00036\u0498\b6\u00016\u00016\u00036\u049c\b6\u00016\u0001"+
		"6\u00016\u00016\u00016\u00056\u04a3\b6\n6\f6\u04a6\t6\u00016\u00016\u0003"+
		"6\u04aa\b6\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u0001"+
		"6\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00016\u00036\u04bd"+
		"\b6\u00016\u00036\u04c0\b6\u00016\u00016\u00016\u00036\u04c5\b6\u0001"+
		"7\u00017\u00017\u00017\u00017\u00037\u04cc\b7\u00057\u04ce\b7\n7\f7\u04d1"+
		"\t7\u00017\u00017\u00018\u00018\u00018\u00018\u00018\u00018\u00038\u04db"+
		"\b8\u00019\u00019\u00039\u04df\b9\u00019\u00019\u00019\u00019\u00039\u04e5"+
		"\b9\u0001:\u0001:\u0001:\u0001;\u0001;\u0001;\u0003;\u04ed\b;\u0001<\u0001"+
		"<\u0001<\u0003<\u04f2\b<\u0001=\u0001=\u0001=\u0001=\u0003=\u04f8\b=\u0001"+
		"=\u0003=\u04fb\b=\u0001=\u0001=\u0001=\u0001=\u0001=\u0005=\u0502\b=\n"+
		"=\f=\u0505\t=\u0001=\u0001=\u0001=\u0001=\u0001=\u0001=\u0001=\u0001="+
		"\u0001=\u0003=\u0510\b=\u0001>\u0001>\u0001>\u0001>\u0001>\u0001>\u0001"+
		">\u0001>\u0001>\u0001>\u0001>\u0003>\u051d\b>\u0001?\u0001?\u0001?\u0001"+
		"?\u0001?\u0001?\u0001?\u0001?\u0001?\u0001?\u0001?\u0001?\u0001?\u0001"+
		"?\u0001?\u0001?\u0001?\u0001?\u0003?\u0531\b?\u0001@\u0001@\u0001@\u0001"+
		"@\u0001@\u0001@\u0001@\u0001A\u0001A\u0001A\u0001A\u0003A\u053e\bA\u0001"+
		"A\u0001A\u0003A\u0542\bA\u0001B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001"+
		"B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001"+
		"B\u0001B\u0005B\u0556\bB\nB\fB\u0559\tB\u0001B\u0001B\u0001B\u0001B\u0001"+
		"B\u0001B\u0001B\u0001B\u0001B\u0001B\u0001B\u0003B\u0566\bB\u0001C\u0001"+
		"C\u0001C\u0001C\u0003C\u056c\bC\u0001C\u0001C\u0001D\u0001D\u0001D\u0005"+
		"D\u0573\bD\nD\fD\u0576\tD\u0001E\u0001E\u0001E\u0001F\u0001F\u0001F\u0001"+
		"F\u0001F\u0001F\u0005F\u0581\bF\nF\fF\u0584\tF\u0001F\u0001F\u0003F\u0588"+
		"\bF\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0004F\u0594\bF\u000bF\fF\u0595\u0001F\u0001F\u0001F\u0003F\u059b\b"+
		"F\u0001F\u0001F\u0001F\u0001F\u0001F\u0005F\u05a2\bF\nF\fF\u05a5\tF\u0003"+
		"F\u05a7\bF\u0001F\u0001F\u0001F\u0003F\u05ac\bF\u0001F\u0001F\u0003F\u05b0"+
		"\bF\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0003F\u05b8\bF\u0001F\u0001"+
		"F\u0001F\u0003F\u05bd\bF\u0001F\u0001F\u0001F\u0001F\u0001F\u0003F\u05c4"+
		"\bF\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0005F\u05d6\bF\nF\fF\u05d9"+
		"\tF\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0003F\u05f9\bF\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0001F\u0005F\u0604\bF\nF\fF\u0607\tF\u0001F\u0001F\u0001F\u0001F\u0001"+
		"F\u0005F\u060e\bF\nF\fF\u0611\tF\u0001G\u0001G\u0001G\u0001G\u0001G\u0001"+
		"G\u0001G\u0001G\u0001G\u0001G\u0001G\u0001G\u0001G\u0001G\u0001G\u0001"+
		"G\u0001G\u0001G\u0001G\u0001G\u0001G\u0001G\u0003G\u0629\bG\u0001H\u0001"+
		"H\u0001H\u0001H\u0005H\u062f\bH\nH\fH\u0632\tH\u0001H\u0001H\u0001I\u0001"+
		"I\u0001I\u0003I\u0639\bI\u0001J\u0001J\u0001J\u0001J\u0005J\u063f\bJ\n"+
		"J\fJ\u0642\tJ\u0001J\u0001J\u0001K\u0003K\u0647\bK\u0001K\u0001K\u0001"+
		"K\u0003K\u064c\bK\u0001K\u0001K\u0005K\u0650\bK\nK\fK\u0653\tK\u0001K"+
		"\u0001K\u0001L\u0003L\u0658\bL\u0001L\u0001L\u0001L\u0001L\u0003L\u065e"+
		"\bL\u0001M\u0003M\u0661\bM\u0001M\u0001M\u0001M\u0003M\u0666\bM\u0001"+
		"M\u0001M\u0005M\u066a\bM\nM\fM\u066d\tM\u0001M\u0001M\u0001N\u0001N\u0001"+
		"N\u0001N\u0001N\u0005N\u0676\bN\nN\fN\u0679\tN\u0001N\u0001N\u0003N\u067d"+
		"\bN\u0001N\u0003N\u0680\bN\u0001O\u0003O\u0683\bO\u0001O\u0001O\u0001"+
		"O\u0003O\u0688\bO\u0001O\u0001O\u0003O\u068c\bO\u0001O\u0001O\u0005O\u0690"+
		"\bO\nO\fO\u0693\tO\u0001O\u0001O\u0001P\u0001P\u0001P\u0003P\u069a\bP"+
		"\u0001Q\u0001Q\u0003Q\u069e\bQ\u0001Q\u0001Q\u0001Q\u0003Q\u06a3\bQ\u0001"+
		"Q\u0001Q\u0001Q\u0003Q\u06a8\bQ\u0001Q\u0001Q\u0005Q\u06ac\bQ\nQ\fQ\u06af"+
		"\tQ\u0001Q\u0001Q\u0001R\u0001R\u0001R\u0003R\u06b6\bR\u0001S\u0003S\u06b9"+
		"\bS\u0001S\u0001S\u0001S\u0003S\u06be\bS\u0001S\u0001S\u0001S\u0001S\u0001"+
		"T\u0003T\u06c5\bT\u0001T\u0001T\u0001T\u0001T\u0001T\u0001T\u0001T\u0001"+
		"T\u0001U\u0003U\u06d0\bU\u0001U\u0001U\u0001U\u0003U\u06d5\bU\u0001U\u0001"+
		"U\u0003U\u06d9\bU\u0001U\u0001U\u0001U\u0001U\u0005U\u06df\bU\nU\fU\u06e2"+
		"\tU\u0003U\u06e4\bU\u0001U\u0001U\u0005U\u06e8\bU\nU\fU\u06eb\tU\u0001"+
		"U\u0001U\u0001V\u0003V\u06f0\bV\u0001V\u0001V\u0001V\u0001V\u0003V\u06f6"+
		"\bV\u0001W\u0001W\u0001W\u0003W\u06fb\bW\u0001W\u0001W\u0001W\u0001X\u0001"+
		"X\u0001X\u0001X\u0001X\u0001Y\u0003Y\u0706\bY\u0001Y\u0001Y\u0001Y\u0003"+
		"Y\u070b\bY\u0001Y\u0001Y\u0005Y\u070f\bY\nY\fY\u0712\tY\u0001Y\u0001Y"+
		"\u0001Z\u0003Z\u0717\bZ\u0001Z\u0001Z\u0001Z\u0003Z\u071c\bZ\u0001Z\u0001"+
		"Z\u0003Z\u0720\bZ\u0001Z\u0001Z\u0001Z\u0003Z\u0725\bZ\u0001Z\u0001Z\u0003"+
		"Z\u0729\bZ\u0001[\u0001[\u0001[\u0001[\u0001[\u0003[\u0730\b[\u0001[\u0001"+
		"[\u0001[\u0001\\\u0001\\\u0001\\\u0001\\\u0001\\\u0003\\\u073a\b\\\u0001"+
		"\\\u0001\\\u0001\\\u0001]\u0001]\u0001]\u0001]\u0005]\u0743\b]\n]\f]\u0746"+
		"\t]\u0001]\u0001]\u0001^\u0001^\u0001^\u0001^\u0005^\u074e\b^\n^\f^\u0751"+
		"\t^\u0001^\u0001^\u0001_\u0001_\u0001_\u0001_\u0003_\u0759\b_\u0001_\u0001"+
		"_\u0001_\u0001_\u0001_\u0001`\u0001`\u0001`\u0005`\u0763\b`\n`\f`\u0766"+
		"\t`\u0001a\u0001a\u0001b\u0001b\u0001b\u0001b\u0005b\u076e\bb\nb\fb\u0771"+
		"\tb\u0001b\u0001b\u0001c\u0001c\u0001c\u0001c\u0005c\u0779\bc\nc\fc\u077c"+
		"\tc\u0001c\u0001c\u0001d\u0001d\u0001d\u0001d\u0001d\u0005d\u0785\bd\n"+
		"d\fd\u0788\td\u0001d\u0001d\u0001e\u0001e\u0001e\u0001e\u0005e\u0790\b"+
		"e\ne\fe\u0793\te\u0001e\u0001e\u0001f\u0001f\u0001f\u0001f\u0005f\u079b"+
		"\bf\nf\ff\u079e\tf\u0001f\u0001f\u0001g\u0001g\u0001g\u0001g\u0001g\u0003"+
		"g\u07a7\bg\u0001g\u0001g\u0001g\u0005g\u07ac\bg\ng\fg\u07af\tg\u0001g"+
		"\u0001g\u0001h\u0001h\u0001h\u0001h\u0005h\u07b7\bh\nh\fh\u07ba\th\u0001"+
		"h\u0001h\u0001i\u0001i\u0001i\u0001i\u0005i\u07c2\bi\ni\fi\u07c5\ti\u0001"+
		"i\u0001i\u0001j\u0001j\u0001j\u0001j\u0001j\u0005j\u07ce\bj\nj\fj\u07d1"+
		"\tj\u0001j\u0001j\u0001k\u0001k\u0001k\u0001k\u0005k\u07d9\bk\nk\fk\u07dc"+
		"\tk\u0001k\u0001k\u0001l\u0001l\u0001l\u0001l\u0005l\u07e4\bl\nl\fl\u07e7"+
		"\tl\u0001l\u0001l\u0001m\u0001m\u0001m\u0001m\u0005m\u07ef\bm\nm\fm\u07f2"+
		"\tm\u0001m\u0001m\u0001n\u0001n\u0001n\u0001n\u0005n\u07fa\bn\nn\fn\u07fd"+
		"\tn\u0001n\u0001n\u0001o\u0001o\u0001o\u0001o\u0005o\u0805\bo\no\fo\u0808"+
		"\to\u0001o\u0001o\u0001p\u0001p\u0001p\u0001p\u0001p\u0005p\u0811\bp\n"+
		"p\fp\u0814\tp\u0001p\u0001p\u0001q\u0001q\u0001q\u0001q\u0001q\u0005q"+
		"\u081d\bq\nq\fq\u0820\tq\u0001q\u0001q\u0001r\u0001r\u0001r\u0001r\u0001"+
		"r\u0005r\u0829\br\nr\fr\u082c\tr\u0001r\u0001r\u0001s\u0001s\u0001s\u0001"+
		"s\u0001s\u0005s\u0835\bs\ns\fs\u0838\ts\u0001s\u0001s\u0001t\u0001t\u0001"+
		"t\u0001t\u0001t\u0005t\u0841\bt\nt\ft\u0844\tt\u0001t\u0001t\u0001u\u0001"+
		"u\u0001u\u0001u\u0001u\u0005u\u084d\bu\nu\fu\u0850\tu\u0001u\u0001u\u0001"+
		"v\u0001v\u0001v\u0001v\u0001v\u0005v\u0859\bv\nv\fv\u085c\tv\u0001v\u0001"+
		"v\u0001w\u0001w\u0001w\u0001w\u0001w\u0005w\u0865\bw\nw\fw\u0868\tw\u0001"+
		"w\u0001w\u0001x\u0001x\u0001x\u0001x\u0001x\u0005x\u0871\bx\nx\fx\u0874"+
		"\tx\u0001x\u0001x\u0001y\u0001y\u0001y\u0001y\u0005y\u087c\by\ny\fy\u087f"+
		"\ty\u0001y\u0001y\u0001z\u0001z\u0001z\u0001z\u0005z\u0887\bz\nz\fz\u088a"+
		"\tz\u0001z\u0001z\u0001{\u0001{\u0001{\u0001{\u0001{\u0005{\u0893\b{\n"+
		"{\f{\u0896\t{\u0001{\u0001{\u0001|\u0001|\u0001|\u0001|\u0001|\u0005|"+
		"\u089f\b|\n|\f|\u08a2\t|\u0001|\u0001|\u0001}\u0001}\u0001}\u0001}\u0001"+
		"}\u0005}\u08ab\b}\n}\f}\u08ae\t}\u0001}\u0001}\u0001~\u0001~\u0001~\u0001"+
		"~\u0005~\u08b6\b~\n~\f~\u08b9\t~\u0001~\u0001~\u0001\u007f\u0001\u007f"+
		"\u0001\u007f\u0001\u007f\u0001\u007f\u0005\u007f\u08c2\b\u007f\n\u007f"+
		"\f\u007f\u08c5\t\u007f\u0001\u007f\u0001\u007f\u0001\u0080\u0001\u0080"+
		"\u0001\u0080\u0001\u0080\u0001\u0080\u0005\u0080\u08ce\b\u0080\n\u0080"+
		"\f\u0080\u08d1\t\u0080\u0001\u0080\u0001\u0080\u0001\u0081\u0001\u0081"+
		"\u0001\u0081\u0001\u0081\u0001\u0081\u0005\u0081\u08da\b\u0081\n\u0081"+
		"\f\u0081\u08dd\t\u0081\u0001\u0081\u0001\u0081\u0001\u0082\u0001\u0082"+
		"\u0001\u0082\u0001\u0082\u0001\u0082\u0005\u0082\u08e6\b\u0082\n\u0082"+
		"\f\u0082\u08e9\t\u0082\u0001\u0082\u0001\u0082\u0001\u0083\u0001\u0083"+
		"\u0001\u0083\u0001\u0083\u0001\u0083\u0005\u0083\u08f2\b\u0083\n\u0083"+
		"\f\u0083\u08f5\t\u0083\u0001\u0083\u0001\u0083\u0001\u0084\u0001\u0084"+
		"\u0001\u0084\u0001\u0084\u0001\u0084\u0005\u0084\u08fe\b\u0084\n\u0084"+
		"\f\u0084\u0901\t\u0084\u0001\u0084\u0001\u0084\u0001\u0085\u0001\u0085"+
		"\u0001\u0085\u0001\u0085\u0001\u0085\u0005\u0085\u090a\b\u0085\n\u0085"+
		"\f\u0085\u090d\t\u0085\u0001\u0085\u0001\u0085\u0001\u0086\u0001\u0086"+
		"\u0001\u0086\u0001\u0086\u0001\u0086\u0005\u0086\u0916\b\u0086\n\u0086"+
		"\f\u0086\u0919\t\u0086\u0001\u0086\u0001\u0086\u0001\u0087\u0001\u0087"+
		"\u0001\u0087\u0001\u0087\u0001\u0087\u0005\u0087\u0922\b\u0087\n\u0087"+
		"\f\u0087\u0925\t\u0087\u0001\u0087\u0001\u0087\u0001\u0088\u0001\u0088"+
		"\u0001\u0088\u0001\u0088\u0001\u0088\u0005\u0088\u092e\b\u0088\n\u0088"+
		"\f\u0088\u0931\t\u0088\u0001\u0088\u0001\u0088\u0001\u0089\u0001\u0089"+
		"\u0001\u0089\u0001\u0089\u0001\u0089\u0005\u0089\u093a\b\u0089\n\u0089"+
		"\f\u0089\u093d\t\u0089\u0001\u0089\u0001\u0089\u0001\u008a\u0001\u008a"+
		"\u0001\u008a\u0001\u008a\u0001\u008a\u0005\u008a\u0946\b\u008a\n\u008a"+
		"\f\u008a\u0949\t\u008a\u0001\u008a\u0001\u008a\u0001\u008b\u0001\u008b"+
		"\u0001\u008b\u0001\u008b\u0001\u008b\u0005\u008b\u0952\b\u008b\n\u008b"+
		"\f\u008b\u0955\t\u008b\u0001\u008b\u0001\u008b\u0001\u008c\u0001\u008c"+
		"\u0001\u008c\u0001\u008c\u0001\u008c\u0005\u008c\u095e\b\u008c\n\u008c"+
		"\f\u008c\u0961\t\u008c\u0001\u008c\u0001\u008c\u0001\u008d\u0001\u008d"+
		"\u0001\u008d\u0001\u008e\u0001\u008e\u0001\u008e\u0001\u008e\u0001\u008e"+
		"\u0005\u008e\u096d\b\u008e\n\u008e\f\u008e\u0970\t\u008e\u0001\u008e\u0001"+
		"\u008e\u0001\u008f\u0001\u008f\u0001\u008f\u0001\u008f\u0001\u008f\u0005"+
		"\u008f\u0979\b\u008f\n\u008f\f\u008f\u097c\t\u008f\u0001\u008f\u0001\u008f"+
		"\u0001\u0090\u0001\u0090\u0001\u0090\u0001\u0090\u0001\u0090\u0005\u0090"+
		"\u0985\b\u0090\n\u0090\f\u0090\u0988\t\u0090\u0001\u0090\u0001\u0090\u0001"+
		"\u0091\u0001\u0091\u0001\u0091\u0001\u0091\u0001\u0091\u0005\u0091\u0991"+
		"\b\u0091\n\u0091\f\u0091\u0994\t\u0091\u0001\u0091\u0001\u0091\u0001\u0092"+
		"\u0001\u0092\u0001\u0092\u0001\u0092\u0001\u0092\u0005\u0092\u099d\b\u0092"+
		"\n\u0092\f\u0092\u09a0\t\u0092\u0001\u0092\u0001\u0092\u0001\u0093\u0001"+
		"\u0093\u0001\u0093\u0001\u0093\u0005\u0093\u09a8\b\u0093\n\u0093\f\u0093"+
		"\u09ab\t\u0093\u0001\u0093\u0001\u0093\u0001\u0094\u0001\u0094\u0001\u0094"+
		"\u0001\u0094\u0001\u0094\u0005\u0094\u09b4\b\u0094\n\u0094\f\u0094\u09b7"+
		"\t\u0094\u0001\u0094\u0001\u0094\u0001\u0095\u0001\u0095\u0001\u0095\u0001"+
		"\u0095\u0001\u0095\u0005\u0095\u09c0\b\u0095\n\u0095\f\u0095\u09c3\t\u0095"+
		"\u0001\u0095\u0001\u0095\u0001\u0096\u0001\u0096\u0001\u0096\u0001\u0096"+
		"\u0005\u0096\u09cb\b\u0096\n\u0096\f\u0096\u09ce\t\u0096\u0001\u0096\u0001"+
		"\u0096\u0001\u0097\u0001\u0097\u0001\u0097\u0001\u0097\u0001\u0097\u0001"+
		"\u0097\u0005\u0097\u09d8\b\u0097\n\u0097\f\u0097\u09db\t\u0097\u0001\u0097"+
		"\u0001\u0097\u0001\u0098\u0001\u0098\u0001\u0098\u0001\u0098\u0005\u0098"+
		"\u09e3\b\u0098\n\u0098\f\u0098\u09e6\t\u0098\u0001\u0098\u0001\u0098\u0001"+
		"\u0099\u0001\u0099\u0001\u0099\u0001\u0099\u0005\u0099\u09ee\b\u0099\n"+
		"\u0099\f\u0099\u09f1\t\u0099\u0001\u0099\u0001\u0099\u0001\u009a\u0001"+
		"\u009a\u0001\u009a\u0001\u009a\u0005\u009a\u09f9\b\u009a\n\u009a\f\u009a"+
		"\u09fc\t\u009a\u0001\u009a\u0001\u009a\u0001\u009b\u0001\u009b\u0001\u009b"+
		"\u0001\u009b\u0001\u009b\u0005\u009b\u0a05\b\u009b\n\u009b\f\u009b\u0a08"+
		"\t\u009b\u0001\u009b\u0001\u009b\u0001\u009c\u0001\u009c\u0001\u009c\u0001"+
		"\u009c\u0001\u009c\u0005\u009c\u0a11\b\u009c\n\u009c\f\u009c\u0a14\t\u009c"+
		"\u0001\u009c\u0001\u009c\u0001\u009d\u0001\u009d\u0001\u009d\u0001\u009d"+
		"\u0001\u009d\u0005\u009d\u0a1d\b\u009d\n\u009d\f\u009d\u0a20\t\u009d\u0001"+
		"\u009d\u0001\u009d\u0001\u009e\u0001\u009e\u0001\u009e\u0001\u009e\u0001"+
		"\u009e\u0005\u009e\u0a29\b\u009e\n\u009e\f\u009e\u0a2c\t\u009e\u0001\u009e"+
		"\u0001\u009e\u0001\u009f\u0001\u009f\u0001\u009f\u0001\u009f\u0005\u009f"+
		"\u0a34\b\u009f\n\u009f\f\u009f\u0a37\t\u009f\u0001\u009f\u0001\u009f\u0001"+
		"\u00a0\u0001\u00a0\u0001\u00a0\u0001\u00a0\u0005\u00a0\u0a3f\b\u00a0\n"+
		"\u00a0\f\u00a0\u0a42\t\u00a0\u0001\u00a0\u0001\u00a0\u0001\u00a1\u0001"+
		"\u00a1\u0001\u00a1\u0001\u00a1\u0005\u00a1\u0a4a\b\u00a1\n\u00a1\f\u00a1"+
		"\u0a4d\t\u00a1\u0001\u00a1\u0001\u00a1\u0001\u00a2\u0001\u00a2\u0001\u00a2"+
		"\u0001\u00a2\u0005\u00a2\u0a55\b\u00a2\n\u00a2\f\u00a2\u0a58\t\u00a2\u0001"+
		"\u00a2\u0001\u00a2\u0001\u00a3\u0001\u00a3\u0001\u00a3\u0001\u00a3\u0001"+
		"\u00a3\u0005\u00a3\u0a61\b\u00a3\n\u00a3\f\u00a3\u0a64\t\u00a3\u0001\u00a3"+
		"\u0001\u00a3\u0001\u00a4\u0001\u00a4\u0001\u00a4\u0001\u00a4\u0001\u00a4"+
		"\u0001\u00a4\u0005\u00a4\u0a6e\b\u00a4\n\u00a4\f\u00a4\u0a71\t\u00a4\u0001"+
		"\u00a4\u0001\u00a4\u0001\u00a5\u0001\u00a5\u0001\u00a5\u0001\u00a5\u0001"+
		"\u00a5\u0005\u00a5\u0a7a\b\u00a5\n\u00a5\f\u00a5\u0a7d\t\u00a5\u0001\u00a5"+
		"\u0001\u00a5\u0001\u00a6\u0001\u00a6\u0001\u00a6\u0001\u00a6\u0005\u00a6"+
		"\u0a85\b\u00a6\n\u00a6\f\u00a6\u0a88\t\u00a6\u0001\u00a6\u0001\u00a6\u0001"+
		"\u00a7\u0001\u00a7\u0001\u00a7\u0001\u00a7\u0005\u00a7\u0a90\b\u00a7\n"+
		"\u00a7\f\u00a7\u0a93\t\u00a7\u0001\u00a7\u0001\u00a7\u0001\u00a8\u0001"+
		"\u00a8\u0001\u00a8\u0001\u00a8\u0005\u00a8\u0a9b\b\u00a8\n\u00a8\f\u00a8"+
		"\u0a9e\t\u00a8\u0001\u00a8\u0001\u00a8\u0001\u00a9\u0001\u00a9\u0001\u00a9"+
		"\u0001\u00a9\u0005\u00a9\u0aa6\b\u00a9\n\u00a9\f\u00a9\u0aa9\t\u00a9\u0001"+
		"\u00a9\u0001\u00a9\u0001\u00aa\u0001\u00aa\u0001\u00aa\u0001\u00aa\u0005"+
		"\u00aa\u0ab1\b\u00aa\n\u00aa\f\u00aa\u0ab4\t\u00aa\u0001\u00aa\u0001\u00aa"+
		"\u0001\u00ab\u0001\u00ab\u0001\u00ab\u0001\u00ab\u0005\u00ab\u0abc\b\u00ab"+
		"\n\u00ab\f\u00ab\u0abf\t\u00ab\u0001\u00ab\u0001\u00ab\u0001\u00ac\u0001"+
		"\u00ac\u0001\u00ac\u0001\u00ac\u0005\u00ac\u0ac7\b\u00ac\n\u00ac\f\u00ac"+
		"\u0aca\t\u00ac\u0001\u00ac\u0001\u00ac\u0001\u00ad\u0001\u00ad\u0001\u00ad"+
		"\u0001\u00ad\u0005\u00ad\u0ad2\b\u00ad\n\u00ad\f\u00ad\u0ad5\t\u00ad\u0001"+
		"\u00ad\u0001\u00ad\u0001\u00ae\u0001\u00ae\u0001\u00ae\u0001\u00ae\u0001"+
		"\u00ae\u0005\u00ae\u0ade\b\u00ae\n\u00ae\f\u00ae\u0ae1\t\u00ae\u0001\u00ae"+
		"\u0001\u00ae\u0001\u00af\u0001\u00af\u0001\u00af\u0001\u00af\u0005\u00af"+
		"\u0ae9\b\u00af\n\u00af\f\u00af\u0aec\t\u00af\u0001\u00af\u0001\u00af\u0001"+
		"\u00b0\u0001\u00b0\u0001\u00b0\u0001\u00b0\u0005\u00b0\u0af4\b\u00b0\n"+
		"\u00b0\f\u00b0\u0af7\t\u00b0\u0001\u00b0\u0001\u00b0\u0001\u00b1\u0001"+
		"\u00b1\u0001\u00b1\u0001\u00b1\u0005\u00b1\u0aff\b\u00b1\n\u00b1\f\u00b1"+
		"\u0b02\t\u00b1\u0001\u00b1\u0001\u00b1\u0001\u00b2\u0001\u00b2\u0001\u00b2"+
		"\u0001\u00b2\u0005\u00b2\u0b0a\b\u00b2\n\u00b2\f\u00b2\u0b0d\t\u00b2\u0001"+
		"\u00b2\u0001\u00b2\u0001\u00b3\u0001\u00b3\u0001\u00b3\u0001\u00b3\u0005"+
		"\u00b3\u0b15\b\u00b3\n\u00b3\f\u00b3\u0b18\t\u00b3\u0001\u00b3\u0001\u00b3"+
		"\u0001\u00b4\u0001\u00b4\u0001\u00b4\u0001\u00b4\u0005\u00b4\u0b20\b\u00b4"+
		"\n\u00b4\f\u00b4\u0b23\t\u00b4\u0001\u00b4\u0001\u00b4\u0001\u00b5\u0001"+
		"\u00b5\u0001\u00b5\u0001\u00b5\u0005\u00b5\u0b2b\b\u00b5\n\u00b5\f\u00b5"+
		"\u0b2e\t\u00b5\u0001\u00b5\u0001\u00b5\u0001\u00b6\u0001\u00b6\u0001\u00b6"+
		"\u0001\u00b6\u0005\u00b6\u0b36\b\u00b6\n\u00b6\f\u00b6\u0b39\t\u00b6\u0001"+
		"\u00b6\u0001\u00b6\u0001\u00b7\u0001\u00b7\u0001\u00b7\u0001\u00b7\u0005"+
		"\u00b7\u0b41\b\u00b7\n\u00b7\f\u00b7\u0b44\t\u00b7\u0001\u00b7\u0001\u00b7"+
		"\u0001\u00b8\u0001\u00b8\u0001\u00b8\u0001\u00b8\u0005\u00b8\u0b4c\b\u00b8"+
		"\n\u00b8\f\u00b8\u0b4f\t\u00b8\u0001\u00b8\u0001\u00b8\u0001\u00b9\u0001"+
		"\u00b9\u0001\u00b9\u0001\u00b9\u0005\u00b9\u0b57\b\u00b9\n\u00b9\f\u00b9"+
		"\u0b5a\t\u00b9\u0001\u00b9\u0001\u00b9\u0001\u00ba\u0001\u00ba\u0001\u00ba"+
		"\u0001\u00ba\u0005\u00ba\u0b62\b\u00ba\n\u00ba\f\u00ba\u0b65\t\u00ba\u0001"+
		"\u00ba\u0001\u00ba\u0001\u00bb\u0001\u00bb\u0001\u00bb\u0001\u00bb\u0005"+
		"\u00bb\u0b6d\b\u00bb\n\u00bb\f\u00bb\u0b70\t\u00bb\u0001\u00bb\u0001\u00bb"+
		"\u0001\u00bc\u0001\u00bc\u0001\u00bc\u0001\u00bc\u0005\u00bc\u0b78\b\u00bc"+
		"\n\u00bc\f\u00bc\u0b7b\t\u00bc\u0001\u00bc\u0001\u00bc\u0001\u00bd\u0001"+
		"\u00bd\u0001\u00bd\u0001\u00bd\u0001\u00bd\u0001\u00bd\u0005\u00bd\u0b85"+
		"\b\u00bd\n\u00bd\f\u00bd\u0b88\t\u00bd\u0001\u00bd\u0001\u00bd\u0001\u00be"+
		"\u0001\u00be\u0001\u00be\u0001\u00be\u0005\u00be\u0b90\b\u00be\n\u00be"+
		"\f\u00be\u0b93\t\u00be\u0001\u00be\u0001\u00be\u0001\u00bf\u0001\u00bf"+
		"\u0001\u00bf\u0001\u00bf\u0005\u00bf\u0b9b\b\u00bf\n\u00bf\f\u00bf\u0b9e"+
		"\t\u00bf\u0001\u00bf\u0001\u00bf\u0001\u00c0\u0001\u00c0\u0001\u00c0\u0001"+
		"\u00c0\u0005\u00c0\u0ba6\b\u00c0\n\u00c0\f\u00c0\u0ba9\t\u00c0\u0001\u00c0"+
		"\u0001\u00c0\u0001\u00c1\u0001\u00c1\u0001\u00c1\u0001\u00c1\u0005\u00c1"+
		"\u0bb1\b\u00c1\n\u00c1\f\u00c1\u0bb4\t\u00c1\u0001\u00c1\u0001\u00c1\u0001"+
		"\u00c2\u0001\u00c2\u0001\u00c2\u0001\u00c2\u0001\u00c3\u0001\u00c3\u0001"+
		"\u00c3\u0001\u00c3\u0001\u00c4\u0001\u00c4\u0001\u00c4\u0001\u00c4\u0001"+
		"\u00c4\u0005\u00c4\u0bc5\b\u00c4\n\u00c4\f\u00c4\u0bc8\t\u00c4\u0001\u00c4"+
		"\u0001\u00c4\u0001\u00c5\u0001\u00c5\u0001\u00c5\u0001\u00c5\u0001\u00c5"+
		"\u0005\u00c5\u0bd1\b\u00c5\n\u00c5\f\u00c5\u0bd4\t\u00c5\u0001\u00c5\u0001"+
		"\u00c5\u0001\u00c6\u0001\u00c6\u0001\u00c6\u0001\u00c6\u0001\u00c6\u0005"+
		"\u00c6\u0bdd\b\u00c6\n\u00c6\f\u00c6\u0be0\t\u00c6\u0001\u00c6\u0001\u00c6"+
		"\u0001\u00c7\u0004\u00c7\u0be5\b\u00c7\u000b\u00c7\f\u00c7\u0be6\u0001"+
		"\u00c8\u0001\u00c8\u0001\u00c8\u0001\u00c8\u0005\u00c8\u0bed\b\u00c8\n"+
		"\u00c8\f\u00c8\u0bf0\t\u00c8\u0001\u00c8\u0001\u00c8\u0001\u00c9\u0001"+
		"\u00c9\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0001"+
		"\u00ca\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0001"+
		"\u00ca\u0001\u00ca\u0001\u00ca\u0001\u00ca\u0003\u00ca\u0c05\b\u00ca\u0001"+
		"\u00cb\u0001\u00cb\u0001\u00cb\u0005\u00cb\u0c0a\b\u00cb\n\u00cb\f\u00cb"+
		"\u0c0d\t\u00cb\u0001\u00cc\u0001\u00cc\u0001\u00cc\u0003\u00cc\u0c12\b"+
		"\u00cc\u0001\u00cc\u0001\u00cc\u0001\u00cc\u0001\u00cc\u0003\u00cc\u0c18"+
		"\b\u00cc\u0005\u00cc\u0c1a\b\u00cc\n\u00cc\f\u00cc\u0c1d\t\u00cc\u0001"+
		"\u00cd\u0001\u00cd\u0001\u00cd\u0001\u00cd\u0001\u00cd\u0001\u00cd\u0001"+
		"\u00cd\u0001\u00cd\u0001\u00cd\u0001\u00cd\u0001\u00cd\u0003\u00cd\u0c2a"+
		"\b\u00cd\u0001\u00ce\u0001\u00ce\u0001\u00ce\u0001\u00ce\u0001\u00ce\u0001"+
		"\u00ce\u0003\u00ce\u0c32\b\u00ce\u0001\u00ce\u0001\u00ce\u0001\u00cf\u0001"+
		"\u00cf\u0001\u00cf\u0001\u00cf\u0001\u00cf\u0001\u00cf\u0001\u00cf\u0001"+
		"\u00cf\u0001\u00cf\u0001\u00cf\u0001\u00cf\u0003\u00cf\u0c41\b\u00cf\u0001"+
		"\u00d0\u0001\u00d0\u0001\u00d0\u0001\u00d0\u0001\u00d0\u0001\u00d1\u0001"+
		"\u00d1\u0001\u00d1\u0001\u00d2\u0001\u00d2\u0001\u00d2\u0001\u00d3\u0001"+
		"\u00d3\u0001\u00d3\u0001\u00d4\u0001\u00d4\u0001\u00d4\u0001\u00d4\u0001"+
		"\u00d4\u0001\u00d4\u0001\u00d4\u0001\u00d4\u0001\u00d4\u0001\u00d5\u0001"+
		"\u00d5\u0001\u00d5\u0001\u00d5\u0001\u00d5\u0001\u00d5\u0001\u00d5\u0001"+
		"\u00d5\u0001\u00d5\u0001\u00d6\u0001\u00d6\u0001\u00d6\u0001\u00d6\u0001"+
		"\u00d6\u0001\u00d6\u0001\u00d6\u0001\u00d6\u0001\u00d6\u0001\u00d7\u0001"+
		"\u00d7\u0001\u00d7\u0001\u00d7\u0001\u00d7\u0001\u00d7\u0001\u00d7\u0005"+
		"\u00d7\u0c73\b\u00d7\n\u00d7\f\u00d7\u0c76\t\u00d7\u0001\u00d7\u0001\u00d7"+
		"\u0001\u00d7\u0001\u00d7\u0005\u00d7\u0c7c\b\u00d7\n\u00d7\f\u00d7\u0c7f"+
		"\t\u00d7\u0001\u00d7\u0001\u00d7\u0003\u00d7\u0c83\b\u00d7\u0001\u00d8"+
		"\u0001\u00d8\u0001\u00d8\u0001\u00d8\u0001\u00d9\u0001\u00d9\u0001\u00d9"+
		"\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0001\u00d9"+
		"\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0001\u00d9"+
		"\u0001\u00d9\u0001\u00d9\u0001\u00d9\u0003\u00d9\u0c9b\b\u00d9\u0001\u00da"+
		"\u0001\u00da\u0001\u00db\u0001\u00db\u0001\u00dc\u0001\u00dc\u0001\u00dd"+
		"\u0001\u00dd\u0001\u00dd\u0000\u00022\u008c\u00de\u0000\u0002\u0004\u0006"+
		"\b\n\f\u000e\u0010\u0012\u0014\u0016\u0018\u001a\u001c\u001e \"$&(*,."+
		"02468:<>@BDFHJLNPRTVXZ\\^`bdfhjlnprtvxz|~\u0080\u0082\u0084\u0086\u0088"+
		"\u008a\u008c\u008e\u0090\u0092\u0094\u0096\u0098\u009a\u009c\u009e\u00a0"+
		"\u00a2\u00a4\u00a6\u00a8\u00aa\u00ac\u00ae\u00b0\u00b2\u00b4\u00b6\u00b8"+
		"\u00ba\u00bc\u00be\u00c0\u00c2\u00c4\u00c6\u00c8\u00ca\u00cc\u00ce\u00d0"+
		"\u00d2\u00d4\u00d6\u00d8\u00da\u00dc\u00de\u00e0\u00e2\u00e4\u00e6\u00e8"+
		"\u00ea\u00ec\u00ee\u00f0\u00f2\u00f4\u00f6\u00f8\u00fa\u00fc\u00fe\u0100"+
		"\u0102\u0104\u0106\u0108\u010a\u010c\u010e\u0110\u0112\u0114\u0116\u0118"+
		"\u011a\u011c\u011e\u0120\u0122\u0124\u0126\u0128\u012a\u012c\u012e\u0130"+
		"\u0132\u0134\u0136\u0138\u013a\u013c\u013e\u0140\u0142\u0144\u0146\u0148"+
		"\u014a\u014c\u014e\u0150\u0152\u0154\u0156\u0158\u015a\u015c\u015e\u0160"+
		"\u0162\u0164\u0166\u0168\u016a\u016c\u016e\u0170\u0172\u0174\u0176\u0178"+
		"\u017a\u017c\u017e\u0180\u0182\u0184\u0186\u0188\u018a\u018c\u018e\u0190"+
		"\u0192\u0194\u0196\u0198\u019a\u019c\u019e\u01a0\u01a2\u01a4\u01a6\u01a8"+
		"\u01aa\u01ac\u01ae\u01b0\u01b2\u01b4\u01b6\u01b8\u01ba\u0000\u0013\u0001"+
		"\u0000\u00eb\u00fb\u0001\u0000\u010c\u010d\u0003\u0000\u0007\f\u015e\u015e"+
		"\u016d\u0170\u0001\u0000\u0171\u0172\u0002\u0000\r\r\u0166\u0166\u0002"+
		"\u0000\u000e\u000e\u0165\u0165\u0002\u0000\u000f\u0010\u015f\u0160\u0002"+
		"\u0000\u0011\u0013\u0161\u0164\u0001\u0000\u016a\u016c\u0001\u0000\u0159"+
		"\u015a\u0001\u0000\u015b\u015d\u0002\u0000\u00e6\u00e6\u0151\u0151\u0001"+
		"\u0000\u012b\u012c\u0002\u0000\u014e\u014e\u0150\u0150\u0003\u0000\u0116"+
		"\u0117\u011b\u0139\u0147\u0147\u0001\u0000\u00d4\u00d6\u0001\u0000\u00d7"+
		"\u00d9\u0001\u0000\u00da\u00dc\u0001\u0000\u00dd\u00df\u0de7\u0000\u01bf"+
		"\u0001\u0000\u0000\u0000\u0002\u01c5\u0001\u0000\u0000\u0000\u0004\u023c"+
		"\u0001\u0000\u0000\u0000\u0006\u0249\u0001\u0000\u0000\u0000\b\u0252\u0001"+
		"\u0000\u0000\u0000\n\u025b\u0001\u0000\u0000\u0000\f\u0265\u0001\u0000"+
		"\u0000\u0000\u000e\u028a\u0001\u0000\u0000\u0000\u0010\u028c\u0001\u0000"+
		"\u0000\u0000\u0012\u028e\u0001\u0000\u0000\u0000\u0014\u0294\u0001\u0000"+
		"\u0000\u0000\u0016\u02aa\u0001\u0000\u0000\u0000\u0018\u02c2\u0001\u0000"+
		"\u0000\u0000\u001a\u02c5\u0001\u0000\u0000\u0000\u001c\u02c9\u0001\u0000"+
		"\u0000\u0000\u001e\u02cb\u0001\u0000\u0000\u0000 \u02d1\u0001\u0000\u0000"+
		"\u0000\"\u02d4\u0001\u0000\u0000\u0000$\u02d7\u0001\u0000\u0000\u0000"+
		"&\u02db\u0001\u0000\u0000\u0000(\u02e1\u0001\u0000\u0000\u0000*\u02e4"+
		"\u0001\u0000\u0000\u0000,\u02ee\u0001\u0000\u0000\u0000.\u02f0\u0001\u0000"+
		"\u0000\u00000\u02fe\u0001\u0000\u0000\u00002\u0334\u0001\u0000\u0000\u0000"+
		"4\u033e\u0001\u0000\u0000\u00006\u034d\u0001\u0000\u0000\u00008\u0351"+
		"\u0001\u0000\u0000\u0000:\u035c\u0001\u0000\u0000\u0000<\u0365\u0001\u0000"+
		"\u0000\u0000>\u0368\u0001\u0000\u0000\u0000@\u0371\u0001\u0000\u0000\u0000"+
		"B\u037e\u0001\u0000\u0000\u0000D\u0388\u0001\u0000\u0000\u0000F\u038a"+
		"\u0001\u0000\u0000\u0000H\u0390\u0001\u0000\u0000\u0000J\u0392\u0001\u0000"+
		"\u0000\u0000L\u0397\u0001\u0000\u0000\u0000N\u039f\u0001\u0000\u0000\u0000"+
		"P\u03a7\u0001\u0000\u0000\u0000R\u03af\u0001\u0000\u0000\u0000T\u03b7"+
		"\u0001\u0000\u0000\u0000V\u03bf\u0001\u0000\u0000\u0000X\u03c7\u0001\u0000"+
		"\u0000\u0000Z\u03cf\u0001\u0000\u0000\u0000\\\u03d7\u0001\u0000\u0000"+
		"\u0000^\u03df\u0001\u0000\u0000\u0000`\u03e7\u0001\u0000\u0000\u0000b"+
		"\u03fd\u0001\u0000\u0000\u0000d\u03ff\u0001\u0000\u0000\u0000f\u042d\u0001"+
		"\u0000\u0000\u0000h\u043f\u0001\u0000\u0000\u0000j\u0441\u0001\u0000\u0000"+
		"\u0000l\u04c4\u0001\u0000\u0000\u0000n\u04c6\u0001\u0000\u0000\u0000p"+
		"\u04d4\u0001\u0000\u0000\u0000r\u04dc\u0001\u0000\u0000\u0000t\u04e6\u0001"+
		"\u0000\u0000\u0000v\u04e9\u0001\u0000\u0000\u0000x\u04ee\u0001\u0000\u0000"+
		"\u0000z\u050f\u0001\u0000\u0000\u0000|\u051c\u0001\u0000\u0000\u0000~"+
		"\u0530\u0001\u0000\u0000\u0000\u0080\u0532\u0001\u0000\u0000\u0000\u0082"+
		"\u0539\u0001\u0000\u0000\u0000\u0084\u0565\u0001\u0000\u0000\u0000\u0086"+
		"\u0567\u0001\u0000\u0000\u0000\u0088\u056f\u0001\u0000\u0000\u0000\u008a"+
		"\u0577\u0001\u0000\u0000\u0000\u008c\u05f8\u0001\u0000\u0000\u0000\u008e"+
		"\u0628\u0001\u0000\u0000\u0000\u0090\u062a\u0001\u0000\u0000\u0000\u0092"+
		"\u0635\u0001\u0000\u0000\u0000\u0094\u063a\u0001\u0000\u0000\u0000\u0096"+
		"\u0646\u0001\u0000\u0000\u0000\u0098\u0657\u0001\u0000\u0000\u0000\u009a"+
		"\u0660\u0001\u0000\u0000\u0000\u009c\u0670\u0001\u0000\u0000\u0000\u009e"+
		"\u0682\u0001\u0000\u0000\u0000\u00a0\u0699\u0001\u0000\u0000\u0000\u00a2"+
		"\u069b\u0001\u0000\u0000\u0000\u00a4\u06b5\u0001\u0000\u0000\u0000\u00a6"+
		"\u06b8\u0001\u0000\u0000\u0000\u00a8\u06c4\u0001\u0000\u0000\u0000\u00aa"+
		"\u06cf\u0001\u0000\u0000\u0000\u00ac\u06ef\u0001\u0000\u0000\u0000\u00ae"+
		"\u06f7\u0001\u0000\u0000\u0000\u00b0\u06ff\u0001\u0000\u0000\u0000\u00b2"+
		"\u0705\u0001\u0000\u0000\u0000\u00b4\u0716\u0001\u0000\u0000\u0000\u00b6"+
		"\u072a\u0001\u0000\u0000\u0000\u00b8\u0734\u0001\u0000\u0000\u0000\u00ba"+
		"\u073e\u0001\u0000\u0000\u0000\u00bc\u0749\u0001\u0000\u0000\u0000\u00be"+
		"\u0754\u0001\u0000\u0000\u0000\u00c0\u075f\u0001\u0000\u0000\u0000\u00c2"+
		"\u0767\u0001\u0000\u0000\u0000\u00c4\u0769\u0001\u0000\u0000\u0000\u00c6"+
		"\u0774\u0001\u0000\u0000\u0000\u00c8\u077f\u0001\u0000\u0000\u0000\u00ca"+
		"\u078b\u0001\u0000\u0000\u0000\u00cc\u0796\u0001\u0000\u0000\u0000\u00ce"+
		"\u07a1\u0001\u0000\u0000\u0000\u00d0\u07b2\u0001\u0000\u0000\u0000\u00d2"+
		"\u07bd\u0001\u0000\u0000\u0000\u00d4\u07c8\u0001\u0000\u0000\u0000\u00d6"+
		"\u07d4\u0001\u0000\u0000\u0000\u00d8\u07df\u0001\u0000\u0000\u0000\u00da"+
		"\u07ea\u0001\u0000\u0000\u0000\u00dc\u07f5\u0001\u0000\u0000\u0000\u00de"+
		"\u0800\u0001\u0000\u0000\u0000\u00e0\u080b\u0001\u0000\u0000\u0000\u00e2"+
		"\u0817\u0001\u0000\u0000\u0000\u00e4\u0823\u0001\u0000\u0000\u0000\u00e6"+
		"\u082f\u0001\u0000\u0000\u0000\u00e8\u083b\u0001\u0000\u0000\u0000\u00ea"+
		"\u0847\u0001\u0000\u0000\u0000\u00ec\u0853\u0001\u0000\u0000\u0000\u00ee"+
		"\u085f\u0001\u0000\u0000\u0000\u00f0\u086b\u0001\u0000\u0000\u0000\u00f2"+
		"\u0877\u0001\u0000\u0000\u0000\u00f4\u0882\u0001\u0000\u0000\u0000\u00f6"+
		"\u088d\u0001\u0000\u0000\u0000\u00f8\u0899\u0001\u0000\u0000\u0000\u00fa"+
		"\u08a5\u0001\u0000\u0000\u0000\u00fc\u08b1\u0001\u0000\u0000\u0000\u00fe"+
		"\u08bc\u0001\u0000\u0000\u0000\u0100\u08c8\u0001\u0000\u0000\u0000\u0102"+
		"\u08d4\u0001\u0000\u0000\u0000\u0104\u08e0\u0001\u0000\u0000\u0000\u0106"+
		"\u08ec\u0001\u0000\u0000\u0000\u0108\u08f8\u0001\u0000\u0000\u0000\u010a"+
		"\u0904\u0001\u0000\u0000\u0000\u010c\u0910\u0001\u0000\u0000\u0000\u010e"+
		"\u091c\u0001\u0000\u0000\u0000\u0110\u0928\u0001\u0000\u0000\u0000\u0112"+
		"\u0934\u0001\u0000\u0000\u0000\u0114\u0940\u0001\u0000\u0000\u0000\u0116"+
		"\u094c\u0001\u0000\u0000\u0000\u0118\u0958\u0001\u0000\u0000\u0000\u011a"+
		"\u0964\u0001\u0000\u0000\u0000\u011c\u0967\u0001\u0000\u0000\u0000\u011e"+
		"\u0973\u0001\u0000\u0000\u0000\u0120\u097f\u0001\u0000\u0000\u0000\u0122"+
		"\u098b\u0001\u0000\u0000\u0000\u0124\u0997\u0001\u0000\u0000\u0000\u0126"+
		"\u09a3\u0001\u0000\u0000\u0000\u0128\u09ae\u0001\u0000\u0000\u0000\u012a"+
		"\u09ba\u0001\u0000\u0000\u0000\u012c\u09c6\u0001\u0000\u0000\u0000\u012e"+
		"\u09d1\u0001\u0000\u0000\u0000\u0130\u09de\u0001\u0000\u0000\u0000\u0132"+
		"\u09e9\u0001\u0000\u0000\u0000\u0134\u09f4\u0001\u0000\u0000\u0000\u0136"+
		"\u09ff\u0001\u0000\u0000\u0000\u0138\u0a0b\u0001\u0000\u0000\u0000\u013a"+
		"\u0a17\u0001\u0000\u0000\u0000\u013c\u0a23\u0001\u0000\u0000\u0000\u013e"+
		"\u0a2f\u0001\u0000\u0000\u0000\u0140\u0a3a\u0001\u0000\u0000\u0000\u0142"+
		"\u0a45\u0001\u0000\u0000\u0000\u0144\u0a50\u0001\u0000\u0000\u0000\u0146"+
		"\u0a5b\u0001\u0000\u0000\u0000\u0148\u0a67\u0001\u0000\u0000\u0000\u014a"+
		"\u0a74\u0001\u0000\u0000\u0000\u014c\u0a80\u0001\u0000\u0000\u0000\u014e"+
		"\u0a8b\u0001\u0000\u0000\u0000\u0150\u0a96\u0001\u0000\u0000\u0000\u0152"+
		"\u0aa1\u0001\u0000\u0000\u0000\u0154\u0aac\u0001\u0000\u0000\u0000\u0156"+
		"\u0ab7\u0001\u0000\u0000\u0000\u0158\u0ac2\u0001\u0000\u0000\u0000\u015a"+
		"\u0acd\u0001\u0000\u0000\u0000\u015c\u0ad8\u0001\u0000\u0000\u0000\u015e"+
		"\u0ae4\u0001\u0000\u0000\u0000\u0160\u0aef\u0001\u0000\u0000\u0000\u0162"+
		"\u0afa\u0001\u0000\u0000\u0000\u0164\u0b05\u0001\u0000\u0000\u0000\u0166"+
		"\u0b10\u0001\u0000\u0000\u0000\u0168\u0b1b\u0001\u0000\u0000\u0000\u016a"+
		"\u0b26\u0001\u0000\u0000\u0000\u016c\u0b31\u0001\u0000\u0000\u0000\u016e"+
		"\u0b3c\u0001\u0000\u0000\u0000\u0170\u0b47\u0001\u0000\u0000\u0000\u0172"+
		"\u0b52\u0001\u0000\u0000\u0000\u0174\u0b5d\u0001\u0000\u0000\u0000\u0176"+
		"\u0b68\u0001\u0000\u0000\u0000\u0178\u0b73\u0001\u0000\u0000\u0000\u017a"+
		"\u0b7e\u0001\u0000\u0000\u0000\u017c\u0b8b\u0001\u0000\u0000\u0000\u017e"+
		"\u0b96\u0001\u0000\u0000\u0000\u0180\u0ba1\u0001\u0000\u0000\u0000\u0182"+
		"\u0bac\u0001\u0000\u0000\u0000\u0184\u0bb7\u0001\u0000\u0000\u0000\u0186"+
		"\u0bbb\u0001\u0000\u0000\u0000\u0188\u0bbf\u0001\u0000\u0000\u0000\u018a"+
		"\u0bcb\u0001\u0000\u0000\u0000\u018c\u0bd7\u0001\u0000\u0000\u0000\u018e"+
		"\u0be4\u0001\u0000\u0000\u0000\u0190\u0be8\u0001\u0000\u0000\u0000\u0192"+
		"\u0bf3\u0001\u0000\u0000\u0000\u0194\u0c04\u0001\u0000\u0000\u0000\u0196"+
		"\u0c06\u0001\u0000\u0000\u0000\u0198\u0c0e\u0001\u0000\u0000\u0000\u019a"+
		"\u0c29\u0001\u0000\u0000\u0000\u019c\u0c2b\u0001\u0000\u0000\u0000\u019e"+
		"\u0c40\u0001\u0000\u0000\u0000\u01a0\u0c42\u0001\u0000\u0000\u0000\u01a2"+
		"\u0c47\u0001\u0000\u0000\u0000\u01a4\u0c4a\u0001\u0000\u0000\u0000\u01a6"+
		"\u0c4d\u0001\u0000\u0000\u0000\u01a8\u0c50\u0001\u0000\u0000\u0000\u01aa"+
		"\u0c59\u0001\u0000\u0000\u0000\u01ac\u0c62\u0001\u0000\u0000\u0000\u01ae"+
		"\u0c82\u0001\u0000\u0000\u0000\u01b0\u0c84\u0001\u0000\u0000\u0000\u01b2"+
		"\u0c9a\u0001\u0000\u0000\u0000\u01b4\u0c9c\u0001\u0000\u0000\u0000\u01b6"+
		"\u0c9e\u0001\u0000\u0000\u0000\u01b8\u0ca0\u0001\u0000\u0000\u0000\u01ba"+
		"\u0ca2\u0001\u0000\u0000\u0000\u01bc\u01be\u0003\u0002\u0001\u0000\u01bd"+
		"\u01bc\u0001\u0000\u0000\u0000\u01be\u01c1\u0001\u0000\u0000\u0000\u01bf"+
		"\u01bd\u0001\u0000\u0000\u0000\u01bf\u01c0\u0001\u0000\u0000\u0000\u01c0"+
		"\u01c2\u0001\u0000\u0000\u0000\u01c1\u01bf\u0001\u0000\u0000\u0000\u01c2"+
		"\u01c3\u0005\u0000\u0000\u0001\u01c3\u0001\u0001\u0000\u0000\u0000\u01c4"+
		"\u01c6\u0003\u018e\u00c7\u0000\u01c5\u01c4\u0001\u0000\u0000\u0000\u01c5"+
		"\u01c6\u0001\u0000\u0000\u0000\u01c6\u01c8\u0001\u0000\u0000\u0000\u01c7"+
		"\u01c9\u0003\u0190\u00c8\u0000\u01c8\u01c7\u0001\u0000\u0000\u0000\u01c8"+
		"\u01c9\u0001\u0000\u0000\u0000\u01c9\u023a\u0001\u0000\u0000\u0000\u01ca"+
		"\u023b\u0003\u0004\u0002\u0000\u01cb\u023b\u0003\u0006\u0003\u0000\u01cc"+
		"\u023b\u0003\b\u0004\u0000\u01cd\u023b\u0003\u0014\n\u0000\u01ce\u023b"+
		"\u0003\u0096K\u0000\u01cf\u023b\u0003\u009aM\u0000\u01d0\u023b\u0003\u009e"+
		"O\u0000\u01d1\u023b\u0003\u00a2Q\u0000\u01d2\u023b\u0003\u00a6S\u0000"+
		"\u01d3\u023b\u0003B!\u0000\u01d4\u023b\u0003\u00aaU\u0000\u01d5\u023b"+
		"\u0003\u00b2Y\u0000\u01d6\u023b\u0003\u00b4Z\u0000\u01d7\u023b\u0003\u00b6"+
		"[\u0000\u01d8\u023b\u0003\u00b8\\\u0000\u01d9\u023b\u0003\u00ba]\u0000"+
		"\u01da\u023b\u0003\u00bc^\u0000\u01db\u023b\u0003\u00c4b\u0000\u01dc\u023b"+
		"\u0003\u00c6c\u0000\u01dd\u023b\u0003\u00c8d\u0000\u01de\u023b\u0003\u00ca"+
		"e\u0000\u01df\u023b\u0003\u00ccf\u0000\u01e0\u023b\u0003\u00ceg\u0000"+
		"\u01e1\u023b\u0003\u00d0h\u0000\u01e2\u023b\u0003\u00d2i\u0000\u01e3\u023b"+
		"\u0003\u00d4j\u0000\u01e4\u023b\u0003\u00d6k\u0000\u01e5\u023b\u0003\u00d8"+
		"l\u0000\u01e6\u023b\u0003\u00dam\u0000\u01e7\u023b\u0003\u00dcn\u0000"+
		"\u01e8\u023b\u0003\u00deo\u0000\u01e9\u023b\u0003\u00e0p\u0000\u01ea\u023b"+
		"\u0003\u00e2q\u0000\u01eb\u023b\u0003\u00e4r\u0000\u01ec\u023b\u0003\u00e6"+
		"s\u0000\u01ed\u023b\u0003\u00e8t\u0000\u01ee\u023b\u0003\u00eau\u0000"+
		"\u01ef\u023b\u0003\u00ecv\u0000\u01f0\u023b\u0003\u00eew\u0000\u01f1\u023b"+
		"\u0003\u00f0x\u0000\u01f2\u023b\u0003\u00f2y\u0000\u01f3\u023b\u0003\u00f4"+
		"z\u0000\u01f4\u023b\u0003\u00f6{\u0000\u01f5\u023b\u0003\u00f8|\u0000"+
		"\u01f6\u023b\u0003\u00fa}\u0000\u01f7\u023b\u0003\u00fc~\u0000\u01f8\u023b"+
		"\u0003\u00fe\u007f\u0000\u01f9\u023b\u0003\u0100\u0080\u0000\u01fa\u023b"+
		"\u0003\u0102\u0081\u0000\u01fb\u023b\u0003\u0104\u0082\u0000\u01fc\u023b"+
		"\u0003\u0106\u0083\u0000\u01fd\u023b\u0003\u0108\u0084\u0000\u01fe\u023b"+
		"\u0003\u010a\u0085\u0000\u01ff\u023b\u0003\u010c\u0086\u0000\u0200\u023b"+
		"\u0003\u010e\u0087\u0000\u0201\u023b\u0003\u0110\u0088\u0000\u0202\u023b"+
		"\u0003\u0112\u0089\u0000\u0203\u023b\u0003\u0114\u008a\u0000\u0204\u023b"+
		"\u0003\u0116\u008b\u0000\u0205\u023b\u0003\u0118\u008c\u0000\u0206\u023b"+
		"\u0003\u011a\u008d\u0000\u0207\u023b\u0003\u011c\u008e\u0000\u0208\u023b"+
		"\u0003\u011e\u008f\u0000\u0209\u023b\u0003\u0120\u0090\u0000\u020a\u023b"+
		"\u0003\u0122\u0091\u0000\u020b\u023b\u0003\u0124\u0092\u0000\u020c\u023b"+
		"\u0003\u0126\u0093\u0000\u020d\u023b\u0003\u0128\u0094\u0000\u020e\u023b"+
		"\u0003\u012a\u0095\u0000\u020f\u023b\u0003\u012c\u0096\u0000\u0210\u023b"+
		"\u0003\u012e\u0097\u0000\u0211\u023b\u0003\u0130\u0098\u0000\u0212\u023b"+
		"\u0003\u0132\u0099\u0000\u0213\u023b\u0003\u0134\u009a\u0000\u0214\u023b"+
		"\u0003\u0136\u009b\u0000\u0215\u023b\u0003\u0138\u009c\u0000\u0216\u023b"+
		"\u0003\u013a\u009d\u0000\u0217\u023b\u0003\u013c\u009e\u0000\u0218\u023b"+
		"\u0003\u013e\u009f\u0000\u0219\u023b\u0003\u0140\u00a0\u0000\u021a\u023b"+
		"\u0003\u0142\u00a1\u0000\u021b\u023b\u0003\u0144\u00a2\u0000\u021c\u023b"+
		"\u0003\u0146\u00a3\u0000\u021d\u023b\u0003\u0148\u00a4\u0000\u021e\u023b"+
		"\u0003\u014a\u00a5\u0000\u021f\u023b\u0003\u014c\u00a6\u0000\u0220\u023b"+
		"\u0003\u014e\u00a7\u0000\u0221\u023b\u0003\u0150\u00a8\u0000\u0222\u023b"+
		"\u0003\u0152\u00a9\u0000\u0223\u023b\u0003\u0154\u00aa\u0000\u0224\u023b"+
		"\u0003\u0156\u00ab\u0000\u0225\u023b\u0003\u0158\u00ac\u0000\u0226\u023b"+
		"\u0003\u015a\u00ad\u0000\u0227\u023b\u0003\u015c\u00ae\u0000\u0228\u023b"+
		"\u0003\u015e\u00af\u0000\u0229\u023b\u0003\u0160\u00b0\u0000\u022a\u023b"+
		"\u0003\u0162\u00b1\u0000\u022b\u023b\u0003\u0164\u00b2\u0000\u022c\u023b"+
		"\u0003\u0166\u00b3\u0000\u022d\u023b\u0003\u0168\u00b4\u0000\u022e\u023b"+
		"\u0003\u016a\u00b5\u0000\u022f\u023b\u0003\u016c\u00b6\u0000\u0230\u023b"+
		"\u0003\u016e\u00b7\u0000\u0231\u023b\u0003\u0170\u00b8\u0000\u0232\u023b"+
		"\u0003\u0172\u00b9\u0000\u0233\u023b\u0003\u0174\u00ba\u0000\u0234\u023b"+
		"\u0003\u0176\u00bb\u0000\u0235\u023b\u0003\u0178\u00bc\u0000\u0236\u023b"+
		"\u0003\u017a\u00bd\u0000\u0237\u023b\u0003\u017c\u00be\u0000\u0238\u023b"+
		"\u0003\u017e\u00bf\u0000\u0239\u023b\u0003\u0194\u00ca\u0000\u023a\u01ca"+
		"\u0001\u0000\u0000\u0000\u023a\u01cb\u0001\u0000\u0000\u0000\u023a\u01cc"+
		"\u0001\u0000\u0000\u0000\u023a\u01cd\u0001\u0000\u0000\u0000\u023a\u01ce"+
		"\u0001\u0000\u0000\u0000\u023a\u01cf\u0001\u0000\u0000\u0000\u023a\u01d0"+
		"\u0001\u0000\u0000\u0000\u023a\u01d1\u0001\u0000\u0000\u0000\u023a\u01d2"+
		"\u0001\u0000\u0000\u0000\u023a\u01d3\u0001\u0000\u0000\u0000\u023a\u01d4"+
		"\u0001\u0000\u0000\u0000\u023a\u01d5\u0001\u0000\u0000\u0000\u023a\u01d6"+
		"\u0001\u0000\u0000\u0000\u023a\u01d7\u0001\u0000\u0000\u0000\u023a\u01d8"+
		"\u0001\u0000\u0000\u0000\u023a\u01d9\u0001\u0000\u0000\u0000\u023a\u01da"+
		"\u0001\u0000\u0000\u0000\u023a\u01db\u0001\u0000\u0000\u0000\u023a\u01dc"+
		"\u0001\u0000\u0000\u0000\u023a\u01dd\u0001\u0000\u0000\u0000\u023a\u01de"+
		"\u0001\u0000\u0000\u0000\u023a\u01df\u0001\u0000\u0000\u0000\u023a\u01e0"+
		"\u0001\u0000\u0000\u0000\u023a\u01e1\u0001\u0000\u0000\u0000\u023a\u01e2"+
		"\u0001\u0000\u0000\u0000\u023a\u01e3\u0001\u0000\u0000\u0000\u023a\u01e4"+
		"\u0001\u0000\u0000\u0000\u023a\u01e5\u0001\u0000\u0000\u0000\u023a\u01e6"+
		"\u0001\u0000\u0000\u0000\u023a\u01e7\u0001\u0000\u0000\u0000\u023a\u01e8"+
		"\u0001\u0000\u0000\u0000\u023a\u01e9\u0001\u0000\u0000\u0000\u023a\u01ea"+
		"\u0001\u0000\u0000\u0000\u023a\u01eb\u0001\u0000\u0000\u0000\u023a\u01ec"+
		"\u0001\u0000\u0000\u0000\u023a\u01ed\u0001\u0000\u0000\u0000\u023a\u01ee"+
		"\u0001\u0000\u0000\u0000\u023a\u01ef\u0001\u0000\u0000\u0000\u023a\u01f0"+
		"\u0001\u0000\u0000\u0000\u023a\u01f1\u0001\u0000\u0000\u0000\u023a\u01f2"+
		"\u0001\u0000\u0000\u0000\u023a\u01f3\u0001\u0000\u0000\u0000\u023a\u01f4"+
		"\u0001\u0000\u0000\u0000\u023a\u01f5\u0001\u0000\u0000\u0000\u023a\u01f6"+
		"\u0001\u0000\u0000\u0000\u023a\u01f7\u0001\u0000\u0000\u0000\u023a\u01f8"+
		"\u0001\u0000\u0000\u0000\u023a\u01f9\u0001\u0000\u0000\u0000\u023a\u01fa"+
		"\u0001\u0000\u0000\u0000\u023a\u01fb\u0001\u0000\u0000\u0000\u023a\u01fc"+
		"\u0001\u0000\u0000\u0000\u023a\u01fd\u0001\u0000\u0000\u0000\u023a\u01fe"+
		"\u0001\u0000\u0000\u0000\u023a\u01ff\u0001\u0000\u0000\u0000\u023a\u0200"+
		"\u0001\u0000\u0000\u0000\u023a\u0201\u0001\u0000\u0000\u0000\u023a\u0202"+
		"\u0001\u0000\u0000\u0000\u023a\u0203\u0001\u0000\u0000\u0000\u023a\u0204"+
		"\u0001\u0000\u0000\u0000\u023a\u0205\u0001\u0000\u0000\u0000\u023a\u0206"+
		"\u0001\u0000\u0000\u0000\u023a\u0207\u0001\u0000\u0000\u0000\u023a\u0208"+
		"\u0001\u0000\u0000\u0000\u023a\u0209\u0001\u0000\u0000\u0000\u023a\u020a"+
		"\u0001\u0000\u0000\u0000\u023a\u020b\u0001\u0000\u0000\u0000\u023a\u020c"+
		"\u0001\u0000\u0000\u0000\u023a\u020d\u0001\u0000\u0000\u0000\u023a\u020e"+
		"\u0001\u0000\u0000\u0000\u023a\u020f\u0001\u0000\u0000\u0000\u023a\u0210"+
		"\u0001\u0000\u0000\u0000\u023a\u0211\u0001\u0000\u0000\u0000\u023a\u0212"+
		"\u0001\u0000\u0000\u0000\u023a\u0213\u0001\u0000\u0000\u0000\u023a\u0214"+
		"\u0001\u0000\u0000\u0000\u023a\u0215\u0001\u0000\u0000\u0000\u023a\u0216"+
		"\u0001\u0000\u0000\u0000\u023a\u0217\u0001\u0000\u0000\u0000\u023a\u0218"+
		"\u0001\u0000\u0000\u0000\u023a\u0219\u0001\u0000\u0000\u0000\u023a\u021a"+
		"\u0001\u0000\u0000\u0000\u023a\u021b\u0001\u0000\u0000\u0000\u023a\u021c"+
		"\u0001\u0000\u0000\u0000\u023a\u021d\u0001\u0000\u0000\u0000\u023a\u021e"+
		"\u0001\u0000\u0000\u0000\u023a\u021f\u0001\u0000\u0000\u0000\u023a\u0220"+
		"\u0001\u0000\u0000\u0000\u023a\u0221\u0001\u0000\u0000\u0000\u023a\u0222"+
		"\u0001\u0000\u0000\u0000\u023a\u0223\u0001\u0000\u0000\u0000\u023a\u0224"+
		"\u0001\u0000\u0000\u0000\u023a\u0225\u0001\u0000\u0000\u0000\u023a\u0226"+
		"\u0001\u0000\u0000\u0000\u023a\u0227\u0001\u0000\u0000\u0000\u023a\u0228"+
		"\u0001\u0000\u0000\u0000\u023a\u0229\u0001\u0000\u0000\u0000\u023a\u022a"+
		"\u0001\u0000\u0000\u0000\u023a\u022b\u0001\u0000\u0000\u0000\u023a\u022c"+
		"\u0001\u0000\u0000\u0000\u023a\u022d\u0001\u0000\u0000\u0000\u023a\u022e"+
		"\u0001\u0000\u0000\u0000\u023a\u022f\u0001\u0000\u0000\u0000\u023a\u0230"+
		"\u0001\u0000\u0000\u0000\u023a\u0231\u0001\u0000\u0000\u0000\u023a\u0232"+
		"\u0001\u0000\u0000\u0000\u023a\u0233\u0001\u0000\u0000\u0000\u023a\u0234"+
		"\u0001\u0000\u0000\u0000\u023a\u0235\u0001\u0000\u0000\u0000\u023a\u0236"+
		"\u0001\u0000\u0000\u0000\u023a\u0237\u0001\u0000\u0000\u0000\u023a\u0238"+
		"\u0001\u0000\u0000\u0000\u023a\u0239\u0001\u0000\u0000\u0000\u023b\u0003"+
		"\u0001\u0000\u0000\u0000\u023c\u023d\u0005\u00e3\u0000\u0000\u023d\u0242"+
		"\u0003\u0192\u00c9\u0000\u023e\u023f\u0005\u0154\u0000\u0000\u023f\u0241"+
		"\u0003\u0192\u00c9\u0000\u0240\u023e\u0001\u0000\u0000\u0000\u0241\u0244"+
		"\u0001\u0000\u0000\u0000\u0242\u0240\u0001\u0000\u0000\u0000\u0242\u0243"+
		"\u0001\u0000\u0000\u0000\u0243\u0247\u0001\u0000\u0000\u0000\u0244\u0242"+
		"\u0001\u0000\u0000\u0000\u0245\u0248\u0003>\u001f\u0000\u0246\u0248\u0005"+
		"\u0150\u0000\u0000\u0247\u0245\u0001\u0000\u0000\u0000\u0247\u0246\u0001"+
		"\u0000\u0000\u0000\u0248\u0005\u0001\u0000\u0000\u0000\u0249\u024a\u0005"+
		"\u00e4\u0000\u0000\u024a\u024d\u0003\n\u0005\u0000\u024b\u024c\u0005\u00e6"+
		"\u0000\u0000\u024c\u024e\u0003\u0192\u00c9\u0000\u024d\u024b\u0001\u0000"+
		"\u0000\u0000\u024d\u024e\u0001\u0000\u0000\u0000\u024e\u0250\u0001\u0000"+
		"\u0000\u0000\u024f\u0251\u0005\u0150\u0000\u0000\u0250\u024f\u0001\u0000"+
		"\u0000\u0000\u0250\u0251\u0001\u0000\u0000\u0000\u0251\u0007\u0001\u0000"+
		"\u0000\u0000\u0252\u0253\u0005\u00e5\u0000\u0000\u0253\u0256\u0003\u0192"+
		"\u00c9\u0000\u0254\u0255\u0005\u0001\u0000\u0000\u0255\u0257\u0003\u0192"+
		"\u00c9\u0000\u0256\u0254\u0001\u0000\u0000\u0000\u0256\u0257\u0001\u0000"+
		"\u0000\u0000\u0257\u0259\u0001\u0000\u0000\u0000\u0258\u025a\u0005\u0150"+
		"\u0000\u0000\u0259\u0258\u0001\u0000\u0000\u0000\u0259\u025a\u0001\u0000"+
		"\u0000\u0000\u025a\t\u0001\u0000\u0000\u0000\u025b\u0262\u0003\u0192\u00c9"+
		"\u0000\u025c\u025d\u0005\u0154\u0000\u0000\u025d\u0261\u0003\u0192\u00c9"+
		"\u0000\u025e\u025f\u0005\u014f\u0000\u0000\u025f\u0261\u0003\u0192\u00c9"+
		"\u0000\u0260\u025c\u0001\u0000\u0000\u0000\u0260\u025e\u0001\u0000\u0000"+
		"\u0000\u0261\u0264\u0001\u0000\u0000\u0000\u0262\u0260\u0001\u0000\u0000"+
		"\u0000\u0262\u0263\u0001\u0000\u0000\u0000\u0263\u000b\u0001\u0000\u0000"+
		"\u0000\u0264\u0262\u0001\u0000\u0000\u0000\u0265\u0266\u0005\u00e7\u0000"+
		"\u0000\u0266\u0268\u0003\u000e\u0007\u0000\u0267\u0269\u0005\u0150\u0000"+
		"\u0000\u0268\u0267\u0001\u0000\u0000\u0000\u0268\u0269\u0001\u0000\u0000"+
		"\u0000\u0269\r\u0001\u0000\u0000\u0000\u026a\u026f\u0003\u0010\b\u0000"+
		"\u026b\u026c\u0005\u0154\u0000\u0000\u026c\u026e\u0003\u0010\b\u0000\u026d"+
		"\u026b\u0001\u0000\u0000\u0000\u026e\u0271\u0001\u0000\u0000\u0000\u026f"+
		"\u026d\u0001\u0000\u0000\u0000\u026f\u0270\u0001\u0000\u0000\u0000\u0270"+
		"\u0274\u0001\u0000\u0000\u0000\u0271\u026f\u0001\u0000\u0000\u0000\u0272"+
		"\u0273\u0005\u0154\u0000\u0000\u0273\u0275\u0005\u015b\u0000\u0000\u0274"+
		"\u0272\u0001\u0000\u0000\u0000\u0274\u0275\u0001\u0000\u0000\u0000\u0275"+
		"\u028b\u0001\u0000\u0000\u0000\u0276\u027b\u0003\u0010\b\u0000\u0277\u0278"+
		"\u0005\u0154\u0000\u0000\u0278\u027a\u0003\u0010\b\u0000\u0279\u0277\u0001"+
		"\u0000\u0000\u0000\u027a\u027d\u0001\u0000\u0000\u0000\u027b\u0279\u0001"+
		"\u0000\u0000\u0000\u027b\u027c\u0001\u0000\u0000\u0000\u027c\u027e\u0001"+
		"\u0000\u0000\u0000\u027d\u027b\u0001\u0000\u0000\u0000\u027e\u027f\u0005"+
		"\u0154\u0000\u0000\u027f\u0280\u0005\u014a\u0000\u0000\u0280\u0285\u0003"+
		"\u0192\u00c9\u0000\u0281\u0282\u0005\u014e\u0000\u0000\u0282\u0284\u0003"+
		"\u0192\u00c9\u0000\u0283\u0281\u0001\u0000\u0000\u0000\u0284\u0287\u0001"+
		"\u0000\u0000\u0000\u0285\u0283\u0001\u0000\u0000\u0000\u0285\u0286\u0001"+
		"\u0000\u0000\u0000\u0286\u0288\u0001\u0000\u0000\u0000\u0287\u0285\u0001"+
		"\u0000\u0000\u0000\u0288\u0289\u0005\u014b\u0000\u0000\u0289\u028b\u0001"+
		"\u0000\u0000\u0000\u028a\u026a\u0001\u0000\u0000\u0000\u028a\u0276\u0001"+
		"\u0000\u0000\u0000\u028b\u000f\u0001\u0000\u0000\u0000\u028c\u028d\u0003"+
		"\u0192\u00c9\u0000\u028d\u0011\u0001\u0000\u0000\u0000\u028e\u028f\u0005"+
		"\u00e8\u0000\u0000\u028f\u0290\u0005\u00e9\u0000\u0000\u0290\u0291\u0003"+
		"\u0192\u00c9\u0000\u0291\u0292\u0005\u0150\u0000\u0000\u0292\u0013\u0001"+
		"\u0000\u0000\u0000\u0293\u0295\u0003\u001a\r\u0000\u0294\u0293\u0001\u0000"+
		"\u0000\u0000\u0294\u0295\u0001\u0000\u0000\u0000\u0295\u0296\u0001\u0000"+
		"\u0000\u0000\u0296\u0297\u0005\u00ea\u0000\u0000\u0297\u0299\u0003\u0192"+
		"\u00c9\u0000\u0298\u029a\u0003\u0090H\u0000\u0299\u0298\u0001\u0000\u0000"+
		"\u0000\u0299\u029a\u0001\u0000\u0000\u0000\u029a\u029b\u0001\u0000\u0000"+
		"\u0000\u029b\u029d\u0005\u0148\u0000\u0000\u029c\u029e\u0003\u0016\u000b"+
		"\u0000\u029d\u029c\u0001\u0000\u0000\u0000\u029d\u029e\u0001\u0000\u0000"+
		"\u0000\u029e\u029f\u0001\u0000\u0000\u0000\u029f\u02a2\u0005\u0149\u0000"+
		"\u0000\u02a0\u02a1\u0005\u0152\u0000\u0000\u02a1\u02a3\u0003\u008cF\u0000"+
		"\u02a2\u02a0\u0001\u0000\u0000\u0000\u02a2\u02a3\u0001\u0000\u0000\u0000"+
		"\u02a3\u02a6\u0001\u0000\u0000\u0000\u02a4\u02a5\u0005\u0002\u0000\u0000"+
		"\u02a5\u02a7\u0003\u00c0`\u0000\u02a6\u02a4\u0001\u0000\u0000\u0000\u02a6"+
		"\u02a7\u0001\u0000\u0000\u0000\u02a7\u02a8\u0001\u0000\u0000\u0000\u02a8"+
		"\u02a9\u0003>\u001f\u0000\u02a9\u0015\u0001\u0000\u0000\u0000\u02aa\u02af"+
		"\u0003\u0018\f\u0000\u02ab\u02ac\u0005\u014e\u0000\u0000\u02ac\u02ae\u0003"+
		"\u0018\f\u0000\u02ad\u02ab\u0001\u0000\u0000\u0000\u02ae\u02b1\u0001\u0000"+
		"\u0000\u0000\u02af\u02ad\u0001\u0000\u0000\u0000\u02af\u02b0\u0001\u0000"+
		"\u0000\u0000\u02b0\u0017\u0001\u0000\u0000\u0000\u02b1\u02af\u0001\u0000"+
		"\u0000\u0000\u02b2\u02b4\u0005\u0003\u0000\u0000\u02b3\u02b2\u0001\u0000"+
		"\u0000\u0000\u02b3\u02b4\u0001\u0000\u0000\u0000\u02b4\u02b5\u0001\u0000"+
		"\u0000\u0000\u02b5\u02b8\u0003\u0192\u00c9\u0000\u02b6\u02b7\u0005\u0151"+
		"\u0000\u0000\u02b7\u02b9\u0003\u008cF\u0000\u02b8\u02b6\u0001\u0000\u0000"+
		"\u0000\u02b8\u02b9\u0001\u0000\u0000\u0000\u02b9\u02bc\u0001\u0000\u0000"+
		"\u0000\u02ba\u02bb\u0005\u015e\u0000\u0000\u02bb\u02bd\u0003D\"\u0000"+
		"\u02bc\u02ba\u0001\u0000\u0000\u0000\u02bc\u02bd\u0001\u0000\u0000\u0000"+
		"\u02bd\u02c3\u0001\u0000\u0000\u0000\u02be\u02bf\u0005\u0004\u0000\u0000"+
		"\u02bf\u02c0\u0003\u008cF\u0000\u02c0\u02c1\u0003\u0192\u00c9\u0000\u02c1"+
		"\u02c3\u0001\u0000\u0000\u0000\u02c2\u02b3\u0001\u0000\u0000\u0000\u02c2"+
		"\u02be\u0001\u0000\u0000\u0000\u02c3\u0019\u0001\u0000\u0000\u0000\u02c4"+
		"\u02c6\u0003\u001c\u000e\u0000\u02c5\u02c4\u0001\u0000\u0000\u0000\u02c6"+
		"\u02c7\u0001\u0000\u0000\u0000\u02c7\u02c5\u0001\u0000\u0000\u0000\u02c7"+
		"\u02c8\u0001\u0000\u0000\u0000\u02c8\u001b\u0001\u0000\u0000\u0000\u02c9"+
		"\u02ca\u0007\u0000\u0000\u0000\u02ca\u001d\u0001\u0000\u0000\u0000\u02cb"+
		"\u02cd\u0005\u00fc\u0000\u0000\u02cc\u02ce\u0003D\"\u0000\u02cd\u02cc"+
		"\u0001\u0000\u0000\u0000\u02cd\u02ce\u0001\u0000\u0000\u0000\u02ce\u02cf"+
		"\u0001\u0000\u0000\u0000\u02cf\u02d0\u0005\u0150\u0000\u0000\u02d0\u001f"+
		"\u0001\u0000\u0000\u0000\u02d1\u02d2\u0005\u00fd\u0000\u0000\u02d2\u02d3"+
		"\u0005\u0150\u0000\u0000\u02d3!\u0001\u0000\u0000\u0000\u02d4\u02d5\u0005"+
		"\u00fe\u0000\u0000\u02d5\u02d6\u0005\u0150\u0000\u0000\u02d6#\u0001\u0000"+
		"\u0000\u0000\u02d7\u02d8\u0005\u00ff\u0000\u0000\u02d8\u02d9\u0003D\""+
		"\u0000\u02d9\u02da\u0003>\u001f\u0000\u02da%\u0001\u0000\u0000\u0000\u02db"+
		"\u02dc\u0005\u0100\u0000\u0000\u02dc\u02dd\u0003\u0192\u00c9\u0000\u02dd"+
		"\u02de\u0005\u0101\u0000\u0000\u02de\u02df\u0003D\"\u0000\u02df\u02e0"+
		"\u0003>\u001f\u0000\u02e0\'\u0001\u0000\u0000\u0000\u02e1\u02e2\u0005"+
		"\u0102\u0000\u0000\u02e2\u02e3\u0003>\u001f\u0000\u02e3)\u0001\u0000\u0000"+
		"\u0000\u02e4\u02e5\u0005\u0103\u0000\u0000\u02e5\u02e6\u0003D\"\u0000"+
		"\u02e6\u02ec\u0003>\u001f\u0000\u02e7\u02ea\u0005\u0104\u0000\u0000\u02e8"+
		"\u02eb\u0003*\u0015\u0000\u02e9\u02eb\u0003>\u001f\u0000\u02ea\u02e8\u0001"+
		"\u0000\u0000\u0000\u02ea\u02e9\u0001\u0000\u0000\u0000\u02eb\u02ed\u0001"+
		"\u0000\u0000\u0000\u02ec\u02e7\u0001\u0000\u0000\u0000\u02ec\u02ed\u0001"+
		"\u0000\u0000\u0000\u02ed+\u0001\u0000\u0000\u0000\u02ee\u02ef\u0003.\u0017"+
		"\u0000\u02ef-\u0001\u0000\u0000\u0000\u02f0\u02f1\u0005\u0105\u0000\u0000"+
		"\u02f1\u02f2\u0003D\"\u0000\u02f2\u02f6\u0005\u014a\u0000\u0000\u02f3"+
		"\u02f5\u00030\u0018\u0000\u02f4\u02f3\u0001\u0000\u0000\u0000\u02f5\u02f8"+
		"\u0001\u0000\u0000\u0000\u02f6\u02f4\u0001\u0000\u0000\u0000\u02f6\u02f7"+
		"\u0001\u0000\u0000\u0000\u02f7\u02f9\u0001\u0000\u0000\u0000\u02f8\u02f6"+
		"\u0001\u0000\u0000\u0000\u02f9\u02fa\u0005\u014b\u0000\u0000\u02fa/\u0001"+
		"\u0000\u0000\u0000\u02fb\u02fc\u0005\u0106\u0000\u0000\u02fc\u02ff\u0003"+
		"2\u0019\u0000\u02fd\u02ff\u00032\u0019\u0000\u02fe\u02fb\u0001\u0000\u0000"+
		"\u0000\u02fe\u02fd\u0001\u0000\u0000\u0000\u02ff\u0302\u0001\u0000\u0000"+
		"\u0000\u0300\u0301\u0005\u0107\u0000\u0000\u0301\u0303\u0003D\"\u0000"+
		"\u0302\u0300\u0001\u0000\u0000\u0000\u0302\u0303\u0001\u0000\u0000\u0000"+
		"\u0303\u0304\u0001\u0000\u0000\u0000\u0304\u0305\u0005\u0153\u0000\u0000"+
		"\u0305\u0307\u0003D\"\u0000\u0306\u0308\u0005\u014e\u0000\u0000\u0307"+
		"\u0306\u0001\u0000\u0000\u0000\u0307\u0308\u0001\u0000\u0000\u0000\u0308"+
		"1\u0001\u0000\u0000\u0000\u0309\u030a\u0006\u0019\uffff\uffff\u0000\u030a"+
		"\u0335\u0003\u0192\u00c9\u0000\u030b\u0335\u0003\u019a\u00cd\u0000\u030c"+
		"\u0335\u0005\u0005\u0000\u0000\u030d\u030e\u0005\u0148\u0000\u0000\u030e"+
		"\u0313\u00032\u0019\u0000\u030f\u0310\u0005\u014e\u0000\u0000\u0310\u0312"+
		"\u00032\u0019\u0000\u0311\u030f\u0001\u0000\u0000\u0000\u0312\u0315\u0001"+
		"\u0000\u0000\u0000\u0313\u0311\u0001\u0000\u0000\u0000\u0313\u0314\u0001"+
		"\u0000\u0000\u0000\u0314\u0316\u0001\u0000\u0000\u0000\u0315\u0313\u0001"+
		"\u0000\u0000\u0000\u0316\u0317\u0005\u0149\u0000\u0000\u0317\u0335\u0001"+
		"\u0000\u0000\u0000\u0318\u0319\u0005\u014c\u0000\u0000\u0319\u031e\u0003"+
		"2\u0019\u0000\u031a\u031b\u0005\u014e\u0000\u0000\u031b\u031d\u00032\u0019"+
		"\u0000\u031c\u031a\u0001\u0000\u0000\u0000\u031d\u0320\u0001\u0000\u0000"+
		"\u0000\u031e\u031c\u0001\u0000\u0000\u0000\u031e\u031f\u0001\u0000\u0000"+
		"\u0000\u031f\u0321\u0001\u0000\u0000\u0000\u0320\u031e\u0001\u0000\u0000"+
		"\u0000\u0321\u0322\u0005\u014d\u0000\u0000\u0322\u0335\u0001\u0000\u0000"+
		"\u0000\u0323\u0324\u0005\u014c\u0000\u0000\u0324\u0329\u00032\u0019\u0000"+
		"\u0325\u0326\u0005\u014e\u0000\u0000\u0326\u0328\u00032\u0019\u0000\u0327"+
		"\u0325\u0001\u0000\u0000\u0000\u0328\u032b\u0001\u0000\u0000\u0000\u0329"+
		"\u0327\u0001\u0000\u0000\u0000\u0329\u032a\u0001\u0000\u0000\u0000\u032a"+
		"\u032c\u0001\u0000\u0000\u0000\u032b\u0329\u0001\u0000\u0000\u0000\u032c"+
		"\u032d\u0005\u0004\u0000\u0000\u032d\u032e\u00032\u0019\u0000\u032e\u032f"+
		"\u0005\u014d\u0000\u0000\u032f\u0335\u0001\u0000\u0000\u0000\u0330\u0331"+
		"\u0003\u0192\u00c9\u0000\u0331\u0332\u0005\u0151\u0000\u0000\u0332\u0333"+
		"\u0003\u008cF\u0000\u0333\u0335\u0001\u0000\u0000\u0000\u0334\u0309\u0001"+
		"\u0000\u0000\u0000\u0334\u030b\u0001\u0000\u0000\u0000\u0334\u030c\u0001"+
		"\u0000\u0000\u0000\u0334\u030d\u0001\u0000\u0000\u0000\u0334\u0318\u0001"+
		"\u0000\u0000\u0000\u0334\u0323\u0001\u0000\u0000\u0000\u0334\u0330\u0001"+
		"\u0000\u0000\u0000\u0335\u033b\u0001\u0000\u0000\u0000\u0336\u0337\n\u0002"+
		"\u0000\u0000\u0337\u0338\u0005\u0168\u0000\u0000\u0338\u033a\u00032\u0019"+
		"\u0003\u0339\u0336\u0001\u0000\u0000\u0000\u033a\u033d\u0001\u0000\u0000"+
		"\u0000\u033b\u0339\u0001\u0000\u0000\u0000\u033b\u033c\u0001\u0000\u0000"+
		"\u0000\u033c3\u0001\u0000\u0000\u0000\u033d\u033b\u0001\u0000\u0000\u0000"+
		"\u033e\u034b\u0005\u00f1\u0000\u0000\u033f\u0341\u0003\u0192\u00c9\u0000"+
		"\u0340\u033f\u0001\u0000\u0000\u0000\u0340\u0341\u0001\u0000\u0000\u0000"+
		"\u0341\u0342\u0001\u0000\u0000\u0000\u0342\u034c\u0003>\u001f\u0000\u0343"+
		"\u0344\u0005\u0158\u0000\u0000\u0344\u0345\u0005\u0148\u0000\u0000\u0345"+
		"\u0346\u0005\u0006\u0000\u0000\u0346\u0347\u0005\u0151\u0000\u0000\u0347"+
		"\u0348\u0003D\"\u0000\u0348\u0349\u0005\u0149\u0000\u0000\u0349\u034a"+
		"\u0003>\u001f\u0000\u034a\u034c\u0001\u0000\u0000\u0000\u034b\u0340\u0001"+
		"\u0000\u0000\u0000\u034b\u0343\u0001\u0000\u0000\u0000\u034c5\u0001\u0000"+
		"\u0000\u0000\u034d\u034e\u0005\u0108\u0000\u0000\u034e\u034f\u0003D\""+
		"\u0000\u034f\u0350\u0005\u0150\u0000\u0000\u03507\u0001\u0000\u0000\u0000"+
		"\u0351\u0352\u0005\u0109\u0000\u0000\u0352\u0356\u0003>\u001f\u0000\u0353"+
		"\u0355\u0003:\u001d\u0000\u0354\u0353\u0001\u0000\u0000\u0000\u0355\u0358"+
		"\u0001\u0000\u0000\u0000\u0356\u0354\u0001\u0000\u0000\u0000\u0356\u0357"+
		"\u0001\u0000\u0000\u0000\u0357\u035a\u0001\u0000\u0000\u0000\u0358\u0356"+
		"\u0001\u0000\u0000\u0000\u0359\u035b\u0003<\u001e\u0000\u035a\u0359\u0001"+
		"\u0000\u0000\u0000\u035a\u035b\u0001\u0000\u0000\u0000\u035b9\u0001\u0000"+
		"\u0000\u0000\u035c\u0361\u0005\u010a\u0000\u0000\u035d\u035e\u0005\u0148"+
		"\u0000\u0000\u035e\u035f\u0003\u0018\f\u0000\u035f\u0360\u0005\u0149\u0000"+
		"\u0000\u0360\u0362\u0001\u0000\u0000\u0000\u0361\u035d\u0001\u0000\u0000"+
		"\u0000\u0361\u0362\u0001\u0000\u0000\u0000\u0362\u0363\u0001\u0000\u0000"+
		"\u0000\u0363\u0364\u0003>\u001f\u0000\u0364;\u0001\u0000\u0000\u0000\u0365"+
		"\u0366\u0005\u010b\u0000\u0000\u0366\u0367\u0003>\u001f\u0000\u0367=\u0001"+
		"\u0000\u0000\u0000\u0368\u036c\u0005\u014a\u0000\u0000\u0369\u036b\u0003"+
		"\u0194\u00ca\u0000\u036a\u0369\u0001\u0000\u0000\u0000\u036b\u036e\u0001"+
		"\u0000\u0000\u0000\u036c\u036a\u0001\u0000\u0000\u0000\u036c\u036d\u0001"+
		"\u0000\u0000\u0000\u036d\u036f\u0001\u0000\u0000\u0000\u036e\u036c\u0001"+
		"\u0000\u0000\u0000\u036f\u0370\u0005\u014b\u0000\u0000\u0370?\u0001\u0000"+
		"\u0000\u0000\u0371\u0373\u0007\u0001\u0000\u0000\u0372\u0374\u0005\u0003"+
		"\u0000\u0000\u0373\u0372\u0001\u0000\u0000\u0000\u0373\u0374\u0001\u0000"+
		"\u0000\u0000\u0374\u0375\u0001\u0000\u0000\u0000\u0375\u0378\u0003\u0192"+
		"\u00c9\u0000\u0376\u0377\u0005\u0151\u0000\u0000\u0377\u0379\u0003\u008c"+
		"F\u0000\u0378\u0376\u0001\u0000\u0000\u0000\u0378\u0379\u0001\u0000\u0000"+
		"\u0000\u0379\u037a\u0001\u0000\u0000\u0000\u037a\u037b\u0005\u015e\u0000"+
		"\u0000\u037b\u037c\u0003D\"\u0000\u037c\u037d\u0005\u0150\u0000\u0000"+
		"\u037dA\u0001\u0000\u0000\u0000\u037e\u037f\u0005\u00ef\u0000\u0000\u037f"+
		"\u0382\u0003\u0192\u00c9\u0000\u0380\u0381\u0005\u0151\u0000\u0000\u0381"+
		"\u0383\u0003\u008cF\u0000\u0382\u0380\u0001\u0000\u0000\u0000\u0382\u0383"+
		"\u0001\u0000\u0000\u0000\u0383\u0384\u0001\u0000\u0000\u0000\u0384\u0385"+
		"\u0005\u015e\u0000\u0000\u0385\u0386\u0003D\"\u0000\u0386\u0387\u0005"+
		"\u0150\u0000\u0000\u0387C\u0001\u0000\u0000\u0000\u0388\u0389\u0003F#"+
		"\u0000\u0389E\u0001\u0000\u0000\u0000\u038a\u038e\u0003J%\u0000\u038b"+
		"\u038c\u0003H$\u0000\u038c\u038d\u0003F#\u0000\u038d\u038f\u0001\u0000"+
		"\u0000\u0000\u038e\u038b\u0001\u0000\u0000\u0000\u038e\u038f\u0001\u0000"+
		"\u0000\u0000\u038fG\u0001\u0000\u0000\u0000\u0390\u0391\u0007\u0002\u0000"+
		"\u0000\u0391I\u0001\u0000\u0000\u0000\u0392\u0395\u0003L&\u0000\u0393"+
		"\u0394\u0007\u0003\u0000\u0000\u0394\u0396\u0003L&\u0000\u0395\u0393\u0001"+
		"\u0000\u0000\u0000\u0395\u0396\u0001\u0000\u0000\u0000\u0396K\u0001\u0000"+
		"\u0000\u0000\u0397\u039c\u0003N\'\u0000\u0398\u0399\u0007\u0004\u0000"+
		"\u0000\u0399\u039b\u0003N\'\u0000\u039a\u0398\u0001\u0000\u0000\u0000"+
		"\u039b\u039e\u0001\u0000\u0000\u0000\u039c\u039a\u0001\u0000\u0000\u0000"+
		"\u039c\u039d\u0001\u0000\u0000\u0000\u039dM\u0001\u0000\u0000\u0000\u039e"+
		"\u039c\u0001\u0000\u0000\u0000\u039f\u03a4\u0003P(\u0000\u03a0\u03a1\u0007"+
		"\u0005\u0000\u0000\u03a1\u03a3\u0003T*\u0000\u03a2\u03a0\u0001\u0000\u0000"+
		"\u0000\u03a3\u03a6\u0001\u0000\u0000\u0000\u03a4\u03a2\u0001\u0000\u0000"+
		"\u0000\u03a4\u03a5\u0001\u0000\u0000\u0000\u03a5O\u0001\u0000\u0000\u0000"+
		"\u03a6\u03a4\u0001\u0000\u0000\u0000\u03a7\u03ac\u0003R)\u0000\u03a8\u03a9"+
		"\u0005\u0168\u0000\u0000\u03a9\u03ab\u0003R)\u0000\u03aa\u03a8\u0001\u0000"+
		"\u0000\u0000\u03ab\u03ae\u0001\u0000\u0000\u0000\u03ac\u03aa\u0001\u0000"+
		"\u0000\u0000\u03ac\u03ad\u0001\u0000\u0000\u0000\u03adQ\u0001\u0000\u0000"+
		"\u0000\u03ae\u03ac\u0001\u0000\u0000\u0000\u03af\u03b4\u0003T*\u0000\u03b0"+
		"\u03b1\u0005\u0169\u0000\u0000\u03b1\u03b3\u0003T*\u0000\u03b2\u03b0\u0001"+
		"\u0000\u0000\u0000\u03b3\u03b6\u0001\u0000\u0000\u0000\u03b4\u03b2\u0001"+
		"\u0000\u0000\u0000\u03b4\u03b5\u0001\u0000\u0000\u0000\u03b5S\u0001\u0000"+
		"\u0000\u0000\u03b6\u03b4\u0001\u0000\u0000\u0000\u03b7\u03bc\u0003V+\u0000"+
		"\u03b8\u03b9\u0005\u0167\u0000\u0000\u03b9\u03bb\u0003V+\u0000\u03ba\u03b8"+
		"\u0001\u0000\u0000\u0000\u03bb\u03be\u0001\u0000\u0000\u0000\u03bc\u03ba"+
		"\u0001\u0000\u0000\u0000\u03bc\u03bd\u0001\u0000\u0000\u0000\u03bdU\u0001"+
		"\u0000\u0000\u0000\u03be\u03bc\u0001\u0000\u0000\u0000\u03bf\u03c4\u0003"+
		"X,\u0000\u03c0\u03c1\u0007\u0006\u0000\u0000\u03c1\u03c3\u0003X,\u0000"+
		"\u03c2\u03c0\u0001\u0000\u0000\u0000\u03c3\u03c6\u0001\u0000\u0000\u0000"+
		"\u03c4\u03c2\u0001\u0000\u0000\u0000\u03c4\u03c5\u0001\u0000\u0000\u0000"+
		"\u03c5W\u0001\u0000\u0000\u0000\u03c6\u03c4\u0001\u0000\u0000\u0000\u03c7"+
		"\u03cc\u0003Z-\u0000\u03c8\u03c9\u0007\u0007\u0000\u0000\u03c9\u03cb\u0003"+
		"Z-\u0000\u03ca\u03c8\u0001\u0000\u0000\u0000\u03cb\u03ce\u0001\u0000\u0000"+
		"\u0000\u03cc\u03ca\u0001\u0000\u0000\u0000\u03cc\u03cd\u0001\u0000\u0000"+
		"\u0000\u03cdY\u0001\u0000\u0000\u0000\u03ce\u03cc\u0001\u0000\u0000\u0000"+
		"\u03cf\u03d4\u0003\\.\u0000\u03d0\u03d1\u0007\b\u0000\u0000\u03d1\u03d3"+
		"\u0003\\.\u0000\u03d2\u03d0\u0001\u0000\u0000\u0000\u03d3\u03d6\u0001"+
		"\u0000\u0000\u0000\u03d4\u03d2\u0001\u0000\u0000\u0000\u03d4\u03d5\u0001"+
		"\u0000\u0000\u0000\u03d5[\u0001\u0000\u0000\u0000\u03d6\u03d4\u0001\u0000"+
		"\u0000\u0000\u03d7\u03dc\u0003^/\u0000\u03d8\u03d9\u0007\t\u0000\u0000"+
		"\u03d9\u03db\u0003^/\u0000\u03da\u03d8\u0001\u0000\u0000\u0000\u03db\u03de"+
		"\u0001\u0000\u0000\u0000\u03dc\u03da\u0001\u0000\u0000\u0000\u03dc\u03dd"+
		"\u0001\u0000\u0000\u0000\u03dd]\u0001\u0000\u0000\u0000\u03de\u03dc\u0001"+
		"\u0000\u0000\u0000\u03df\u03e4\u0003`0\u0000\u03e0\u03e1\u0007\n\u0000"+
		"\u0000\u03e1\u03e3\u0003`0\u0000\u03e2\u03e0\u0001\u0000\u0000\u0000\u03e3"+
		"\u03e6\u0001\u0000\u0000\u0000\u03e4\u03e2\u0001\u0000\u0000\u0000\u03e4"+
		"\u03e5\u0001\u0000\u0000\u0000\u03e5_\u0001\u0000\u0000\u0000\u03e6\u03e4"+
		"\u0001\u0000\u0000\u0000\u03e7\u03ec\u0003b1\u0000\u03e8\u03e9\u0007\u000b"+
		"\u0000\u0000\u03e9\u03eb\u0003\u008cF\u0000\u03ea\u03e8\u0001\u0000\u0000"+
		"\u0000\u03eb\u03ee\u0001\u0000\u0000\u0000\u03ec\u03ea\u0001\u0000\u0000"+
		"\u0000\u03ec\u03ed\u0001\u0000\u0000\u0000\u03eda\u0001\u0000\u0000\u0000"+
		"\u03ee\u03ec\u0001\u0000\u0000\u0000\u03ef\u03fa\u0005\u015a\u0000\u0000"+
		"\u03f0\u03fa\u0005\u0158\u0000\u0000\u03f1\u03fa\u0005\u0155\u0000\u0000"+
		"\u03f2\u03f4\u0005\u0167\u0000\u0000\u03f3\u03f5\u0005\u0003\u0000\u0000"+
		"\u03f4\u03f3\u0001\u0000\u0000\u0000\u03f4\u03f5\u0001\u0000\u0000\u0000"+
		"\u03f5\u03fa\u0001\u0000\u0000\u0000\u03f6\u03fa\u0005\u015b\u0000\u0000"+
		"\u03f7\u03fa\u0005\u0014\u0000\u0000\u03f8\u03fa\u0005\u0015\u0000\u0000"+
		"\u03f9\u03ef\u0001\u0000\u0000\u0000\u03f9\u03f0\u0001\u0000\u0000\u0000"+
		"\u03f9\u03f1\u0001\u0000\u0000\u0000\u03f9\u03f2\u0001\u0000\u0000\u0000"+
		"\u03f9\u03f6\u0001\u0000\u0000\u0000\u03f9\u03f7\u0001\u0000\u0000\u0000"+
		"\u03f9\u03f8\u0001\u0000\u0000\u0000\u03fa\u03fb\u0001\u0000\u0000\u0000"+
		"\u03fb\u03fe\u0003b1\u0000\u03fc\u03fe\u0003d2\u0000\u03fd\u03f9\u0001"+
		"\u0000\u0000\u0000\u03fd\u03fc\u0001\u0000\u0000\u0000\u03fec\u0001\u0000"+
		"\u0000\u0000\u03ff\u0403\u0003l6\u0000\u0400\u0402\u0003f3\u0000\u0401"+
		"\u0400\u0001\u0000\u0000\u0000\u0402\u0405\u0001\u0000\u0000\u0000\u0403"+
		"\u0401\u0001\u0000\u0000\u0000\u0403\u0404\u0001\u0000\u0000\u0000\u0404"+
		"e\u0001\u0000\u0000\u0000\u0405\u0403\u0001\u0000\u0000\u0000\u0406\u0408"+
		"\u0005\u0148\u0000\u0000\u0407\u0409\u0003h4\u0000\u0408\u0407\u0001\u0000"+
		"\u0000\u0000\u0408\u0409\u0001\u0000\u0000\u0000\u0409\u040a\u0001\u0000"+
		"\u0000\u0000\u040a\u042e\u0005\u0149\u0000\u0000\u040b\u040c\u0005\u014c"+
		"\u0000\u0000\u040c\u040d\u0003D\"\u0000\u040d\u040e\u0005\u014d\u0000"+
		"\u0000\u040e\u042e\u0001\u0000\u0000\u0000\u040f\u0410\u0005\u014f\u0000"+
		"\u0000\u0410\u0416\u0003\u0192\u00c9\u0000\u0411\u0413\u0005\u0148\u0000"+
		"\u0000\u0412\u0414\u0003h4\u0000\u0413\u0412\u0001\u0000\u0000\u0000\u0413"+
		"\u0414\u0001\u0000\u0000\u0000\u0414\u0415\u0001\u0000\u0000\u0000\u0415"+
		"\u0417\u0005\u0149\u0000\u0000\u0416\u0411\u0001\u0000\u0000\u0000\u0416"+
		"\u0417\u0001\u0000\u0000\u0000\u0417\u042e\u0001\u0000\u0000\u0000\u0418"+
		"\u042e\u0005\u0173\u0000\u0000\u0419\u042e\u0005\u0014\u0000\u0000\u041a"+
		"\u042e\u0005\u0015\u0000\u0000\u041b\u041c\u0005\u0002\u0000\u0000\u041c"+
		"\u041d\u0005\u014c\u0000\u0000\u041d\u041e\u0003\u00c0`\u0000\u041e\u041f"+
		"\u0005\u014d\u0000\u0000\u041f\u042e\u0001\u0000\u0000\u0000\u0420\u0421"+
		"\u0005\u0002\u0000\u0000\u0421\u0429\u0005\u014a\u0000\u0000\u0422\u0423"+
		"\u0003\u0192\u00c9\u0000\u0423\u0424\u0005\u0151\u0000\u0000\u0424\u0425"+
		"\u0003D\"\u0000\u0425\u0426\u0005\u0150\u0000\u0000\u0426\u0428\u0001"+
		"\u0000\u0000\u0000\u0427\u0422\u0001\u0000\u0000\u0000\u0428\u042b\u0001"+
		"\u0000\u0000\u0000\u0429\u0427\u0001\u0000\u0000\u0000\u0429\u042a\u0001"+
		"\u0000\u0000\u0000\u042a\u042c\u0001\u0000\u0000\u0000\u042b\u0429\u0001"+
		"\u0000\u0000\u0000\u042c\u042e\u0005\u014b\u0000\u0000\u042d\u0406\u0001"+
		"\u0000\u0000\u0000\u042d\u040b\u0001\u0000\u0000\u0000\u042d\u040f\u0001"+
		"\u0000\u0000\u0000\u042d\u0418\u0001\u0000\u0000\u0000\u042d\u0419\u0001"+
		"\u0000\u0000\u0000\u042d\u041a\u0001\u0000\u0000\u0000\u042d\u041b\u0001"+
		"\u0000\u0000\u0000\u042d\u0420\u0001\u0000\u0000\u0000\u042eg\u0001\u0000"+
		"\u0000\u0000\u042f\u0434\u0003D\"\u0000\u0430\u0431\u0005\u014e\u0000"+
		"\u0000\u0431\u0433\u0003D\"\u0000\u0432\u0430\u0001\u0000\u0000\u0000"+
		"\u0433\u0436\u0001\u0000\u0000\u0000\u0434\u0432\u0001\u0000\u0000\u0000"+
		"\u0434\u0435\u0001\u0000\u0000\u0000\u0435\u0440\u0001\u0000\u0000\u0000"+
		"\u0436\u0434\u0001\u0000\u0000\u0000\u0437\u043c\u0003j5\u0000\u0438\u0439"+
		"\u0005\u014e\u0000\u0000\u0439\u043b\u0003j5\u0000\u043a\u0438\u0001\u0000"+
		"\u0000\u0000\u043b\u043e\u0001\u0000\u0000\u0000\u043c\u043a\u0001\u0000"+
		"\u0000\u0000\u043c\u043d\u0001\u0000\u0000\u0000\u043d\u0440\u0001\u0000"+
		"\u0000\u0000\u043e\u043c\u0001\u0000\u0000\u0000\u043f\u042f\u0001\u0000"+
		"\u0000\u0000\u043f\u0437\u0001\u0000\u0000\u0000\u0440i\u0001\u0000\u0000"+
		"\u0000\u0441\u0442\u0003\u0192\u00c9\u0000\u0442\u0443\u0005\u015e\u0000"+
		"\u0000\u0443\u0444\u0003D\"\u0000\u0444k\u0001\u0000\u0000\u0000\u0445"+
		"\u0447\u0003\u0192\u00c9\u0000\u0446\u0448\u0003n7\u0000\u0447\u0446\u0001"+
		"\u0000\u0000\u0000\u0447\u0448\u0001\u0000\u0000\u0000\u0448\u04c5\u0001"+
		"\u0000\u0000\u0000\u0449\u04c5\u0003\u019a\u00cd\u0000\u044a\u044b\u0005"+
		"\u0148\u0000\u0000\u044b\u044c\u0003D\"\u0000\u044c\u044d\u0005\u0149"+
		"\u0000\u0000\u044d\u04c5\u0001\u0000\u0000\u0000\u044e\u044f\u0005\u0148"+
		"\u0000\u0000\u044f\u0452\u0003D\"\u0000\u0450\u0451\u0005\u014e\u0000"+
		"\u0000\u0451\u0453\u0003D\"\u0000\u0452\u0450\u0001\u0000\u0000\u0000"+
		"\u0453\u0454\u0001\u0000\u0000\u0000\u0454\u0452\u0001\u0000\u0000\u0000"+
		"\u0454\u0455\u0001\u0000\u0000\u0000\u0455\u0456\u0001\u0000\u0000\u0000"+
		"\u0456\u0457\u0005\u0149\u0000\u0000\u0457\u04c5\u0001\u0000\u0000\u0000"+
		"\u0458\u0461\u0005\u014c\u0000\u0000\u0459\u045e\u0003D\"\u0000\u045a"+
		"\u045b\u0005\u014e\u0000\u0000\u045b\u045d\u0003D\"\u0000\u045c\u045a"+
		"\u0001\u0000\u0000\u0000\u045d\u0460\u0001\u0000\u0000\u0000\u045e\u045c"+
		"\u0001\u0000\u0000\u0000\u045e\u045f\u0001\u0000\u0000\u0000\u045f\u0462"+
		"\u0001\u0000\u0000\u0000\u0460\u045e\u0001\u0000\u0000\u0000\u0461\u0459"+
		"\u0001\u0000\u0000\u0000\u0461\u0462\u0001\u0000\u0000\u0000\u0462\u0463"+
		"\u0001\u0000\u0000\u0000\u0463\u04c5\u0005\u014d\u0000\u0000\u0464\u0465"+
		"\u0005\u0016\u0000\u0000\u0465\u0473\u0005\u014a\u0000\u0000\u0466\u0467"+
		"\u0003D\"\u0000\u0467\u0468\u0005\u0153\u0000\u0000\u0468\u0470\u0003"+
		"D\"\u0000\u0469\u046a\u0005\u014e\u0000\u0000\u046a\u046b\u0003D\"\u0000"+
		"\u046b\u046c\u0005\u0153\u0000\u0000\u046c\u046d\u0003D\"\u0000\u046d"+
		"\u046f\u0001\u0000\u0000\u0000\u046e\u0469\u0001\u0000\u0000\u0000\u046f"+
		"\u0472\u0001\u0000\u0000\u0000\u0470\u046e\u0001\u0000\u0000\u0000\u0470"+
		"\u0471\u0001\u0000\u0000\u0000\u0471\u0474\u0001\u0000\u0000\u0000\u0472"+
		"\u0470\u0001\u0000\u0000\u0000\u0473\u0466\u0001\u0000\u0000\u0000\u0473"+
		"\u0474\u0001\u0000\u0000\u0000\u0474\u0475\u0001\u0000\u0000\u0000\u0475"+
		"\u04c5\u0005\u014b\u0000\u0000\u0476\u04c5\u0003>\u001f\u0000\u0477\u0479"+
		"\u0005\u0168\u0000\u0000\u0478\u047a\u0003\u0016\u000b\u0000\u0479\u0478"+
		"\u0001\u0000\u0000\u0000\u0479\u047a\u0001\u0000\u0000\u0000\u047a\u047b"+
		"\u0001\u0000\u0000\u0000\u047b\u047e\u0005\u0168\u0000\u0000\u047c\u047f"+
		"\u0003>\u001f\u0000\u047d\u047f\u0003D\"\u0000\u047e\u047c\u0001\u0000"+
		"\u0000\u0000\u047e\u047d\u0001\u0000\u0000\u0000\u047f\u04c5\u0001\u0000"+
		"\u0000\u0000\u0480\u0481\u0005\u00ea\u0000\u0000\u0481\u0483\u0005\u0148"+
		"\u0000\u0000\u0482\u0484\u0003\u0016\u000b\u0000\u0483\u0482\u0001\u0000"+
		"\u0000\u0000\u0483\u0484\u0001\u0000\u0000\u0000\u0484\u0485\u0001\u0000"+
		"\u0000\u0000\u0485\u0488\u0005\u0149\u0000\u0000\u0486\u0487\u0005\u0152"+
		"\u0000\u0000\u0487\u0489\u0003\u008cF\u0000\u0488\u0486\u0001\u0000\u0000"+
		"\u0000\u0488\u0489\u0001\u0000\u0000\u0000\u0489\u048a\u0001\u0000\u0000"+
		"\u0000\u048a\u04c5\u0003>\u001f\u0000\u048b\u04c5\u0003*\u0015\u0000\u048c"+
		"\u04c5\u0003.\u0017\u0000\u048d\u04c5\u0003(\u0014\u0000\u048e\u048f\u0005"+
		"\u00f0\u0000\u0000\u048f\u04c5\u0003D\"\u0000\u0490\u0491\u0005\u0017"+
		"\u0000\u0000\u0491\u04c5\u0003D\"\u0000\u0492\u0493\u0005\u0018\u0000"+
		"\u0000\u0493\u04c5\u0003D\"\u0000\u0494\u0495\u0005\u0019\u0000\u0000"+
		"\u0495\u0497\u0003\u0192\u00c9\u0000\u0496\u0498\u0003\u0094J\u0000\u0497"+
		"\u0496\u0001\u0000\u0000\u0000\u0497\u0498\u0001\u0000\u0000\u0000\u0498"+
		"\u0499\u0001\u0000\u0000\u0000\u0499\u049b\u0005\u0148\u0000\u0000\u049a"+
		"\u049c\u0003h4\u0000\u049b\u049a\u0001\u0000\u0000\u0000\u049b\u049c\u0001"+
		"\u0000\u0000\u0000\u049c\u049d\u0001\u0000\u0000\u0000\u049d\u049e\u0005"+
		"\u0149\u0000\u0000\u049e\u04c5\u0001\u0000\u0000\u0000\u049f\u04a0\u0005"+
		"\u0109\u0000\u0000\u04a0\u04a4\u0003D\"\u0000\u04a1\u04a3\u0003:\u001d"+
		"\u0000\u04a2\u04a1\u0001\u0000\u0000\u0000\u04a3\u04a6\u0001\u0000\u0000"+
		"\u0000\u04a4\u04a2\u0001\u0000\u0000\u0000\u04a4\u04a5\u0001\u0000\u0000"+
		"\u0000\u04a5\u04c5\u0001\u0000\u0000\u0000\u04a6\u04a4\u0001\u0000\u0000"+
		"\u0000\u04a7\u04a9\u0005\u001a\u0000\u0000\u04a8\u04aa\u0003D\"\u0000"+
		"\u04a9\u04a8\u0001\u0000\u0000\u0000\u04a9\u04aa\u0001\u0000\u0000\u0000"+
		"\u04aa\u04c5\u0001\u0000\u0000\u0000\u04ab\u04c5\u0003p8\u0000\u04ac\u04c5"+
		"\u0003r9\u0000\u04ad\u04c5\u0003t:\u0000\u04ae\u04c5\u0003v;\u0000\u04af"+
		"\u04c5\u0003x<\u0000\u04b0\u04c5\u0003z=\u0000\u04b1\u04c5\u0003|>\u0000"+
		"\u04b2\u04c5\u0003~?\u0000\u04b3\u04c5\u0003\u0080@\u0000\u04b4\u04c5"+
		"\u0003\u0082A\u0000\u04b5\u04c5\u0003\u0084B\u0000\u04b6\u04c5\u0003\u0086"+
		"C\u0000\u04b7\u04bf\u0005\u001b\u0000\u0000\u04b8\u04b9\u0005\u014f\u0000"+
		"\u0000\u04b9\u04c0\u0003\u0192\u00c9\u0000\u04ba\u04bc\u0005\u0148\u0000"+
		"\u0000\u04bb\u04bd\u0003h4\u0000\u04bc\u04bb\u0001\u0000\u0000\u0000\u04bc"+
		"\u04bd\u0001\u0000\u0000\u0000\u04bd\u04be\u0001\u0000\u0000\u0000\u04be"+
		"\u04c0\u0005\u0149\u0000\u0000\u04bf\u04b8\u0001\u0000\u0000\u0000\u04bf"+
		"\u04ba\u0001\u0000\u0000\u0000\u04bf\u04c0\u0001\u0000\u0000\u0000\u04c0"+
		"\u04c5\u0001\u0000\u0000\u0000\u04c1\u04c5\u0005\u011b\u0000\u0000\u04c2"+
		"\u04c5\u0005\u011c\u0000\u0000\u04c3\u04c5\u0003\u01a6\u00d3\u0000\u04c4"+
		"\u0445\u0001\u0000\u0000\u0000\u04c4\u0449\u0001\u0000\u0000\u0000\u04c4"+
		"\u044a\u0001\u0000\u0000\u0000\u04c4\u044e\u0001\u0000\u0000\u0000\u04c4"+
		"\u0458\u0001\u0000\u0000\u0000\u04c4\u0464\u0001\u0000\u0000\u0000\u04c4"+
		"\u0476\u0001\u0000\u0000\u0000\u04c4\u0477\u0001\u0000\u0000\u0000\u04c4"+
		"\u0480\u0001\u0000\u0000\u0000\u04c4\u048b\u0001\u0000\u0000\u0000\u04c4"+
		"\u048c\u0001\u0000\u0000\u0000\u04c4\u048d\u0001\u0000\u0000\u0000\u04c4"+
		"\u048e\u0001\u0000\u0000\u0000\u04c4\u0490\u0001\u0000\u0000\u0000\u04c4"+
		"\u0492\u0001\u0000\u0000\u0000\u04c4\u0494\u0001\u0000\u0000\u0000\u04c4"+
		"\u049f\u0001\u0000\u0000\u0000\u04c4\u04a7\u0001\u0000\u0000\u0000\u04c4"+
		"\u04ab\u0001\u0000\u0000\u0000\u04c4\u04ac\u0001\u0000\u0000\u0000\u04c4"+
		"\u04ad\u0001\u0000\u0000\u0000\u04c4\u04ae\u0001\u0000\u0000\u0000\u04c4"+
		"\u04af\u0001\u0000\u0000\u0000\u04c4\u04b0\u0001\u0000\u0000\u0000\u04c4"+
		"\u04b1\u0001\u0000\u0000\u0000\u04c4\u04b2\u0001\u0000\u0000\u0000\u04c4"+
		"\u04b3\u0001\u0000\u0000\u0000\u04c4\u04b4\u0001\u0000\u0000\u0000\u04c4"+
		"\u04b5\u0001\u0000\u0000\u0000\u04c4\u04b6\u0001\u0000\u0000\u0000\u04c4"+
		"\u04b7\u0001\u0000\u0000\u0000\u04c4\u04c1\u0001\u0000\u0000\u0000\u04c4"+
		"\u04c2\u0001\u0000\u0000\u0000\u04c4\u04c3\u0001\u0000\u0000\u0000\u04c5"+
		"m\u0001\u0000\u0000\u0000\u04c6\u04cf\u0005\u014a\u0000\u0000\u04c7\u04c8"+
		"\u0003\u0192\u00c9\u0000\u04c8\u04c9\u0005\u0151\u0000\u0000\u04c9\u04cb"+
		"\u0003D\"\u0000\u04ca\u04cc\u0005\u014e\u0000\u0000\u04cb\u04ca\u0001"+
		"\u0000\u0000\u0000\u04cb\u04cc\u0001\u0000\u0000\u0000\u04cc\u04ce\u0001"+
		"\u0000\u0000\u0000\u04cd\u04c7\u0001\u0000\u0000\u0000\u04ce\u04d1\u0001"+
		"\u0000\u0000\u0000\u04cf\u04cd\u0001\u0000\u0000\u0000\u04cf\u04d0\u0001"+
		"\u0000\u0000\u0000\u04d0\u04d2\u0001\u0000\u0000\u0000\u04d1\u04cf\u0001"+
		"\u0000\u0000\u0000\u04d2\u04d3\u0005\u014b\u0000\u0000\u04d3o\u0001\u0000"+
		"\u0000\u0000\u04d4\u04da\u0005\u012a\u0000\u0000\u04d5\u04d6\u0005\u0148"+
		"\u0000\u0000\u04d6\u04d7\u0003D\"\u0000\u04d7\u04d8\u0005\u0149\u0000"+
		"\u0000\u04d8\u04db\u0001\u0000\u0000\u0000\u04d9\u04db\u0003D\"\u0000"+
		"\u04da\u04d5\u0001\u0000\u0000\u0000\u04da\u04d9\u0001\u0000\u0000\u0000"+
		"\u04dbq\u0001\u0000\u0000\u0000\u04dc\u04de\u0007\f\u0000\u0000\u04dd"+
		"\u04df\u0005\u001c\u0000\u0000\u04de\u04dd\u0001\u0000\u0000\u0000\u04de"+
		"\u04df\u0001\u0000\u0000\u0000\u04df\u04e0\u0001\u0000\u0000\u0000\u04e0"+
		"\u04e4\u0003D\"\u0000\u04e1\u04e2\u0005\u0002\u0000\u0000\u04e2\u04e3"+
		"\u0005\u001d\u0000\u0000\u04e3\u04e5\u0003D\"\u0000\u04e4\u04e1\u0001"+
		"\u0000\u0000\u0000\u04e4\u04e5\u0001\u0000\u0000\u0000\u04e5s\u0001\u0000"+
		"\u0000\u0000\u04e6\u04e7\u0005\u001e\u0000\u0000\u04e7\u04e8\u0003D\""+
		"\u0000\u04e8u\u0001\u0000\u0000\u0000\u04e9\u04ec\u0005\u012e\u0000\u0000"+
		"\u04ea\u04ed\u0003>\u001f\u0000\u04eb\u04ed\u0003D\"\u0000\u04ec\u04ea"+
		"\u0001\u0000\u0000\u0000\u04ec\u04eb\u0001\u0000\u0000\u0000\u04edw\u0001"+
		"\u0000\u0000\u0000\u04ee\u04f1\u0005\u012f\u0000\u0000\u04ef\u04f2\u0003"+
		">\u001f\u0000\u04f0\u04f2\u0003D\"\u0000\u04f1\u04ef\u0001\u0000\u0000"+
		"\u0000\u04f1\u04f0\u0001\u0000\u0000\u0000\u04f2y\u0001\u0000\u0000\u0000"+
		"\u04f3\u04f4\u0005\u0116\u0000\u0000\u04f4\u04fa\u0003\u0192\u00c9\u0000"+
		"\u04f5\u04f7\u0005\u0148\u0000\u0000\u04f6\u04f8\u0003h4\u0000\u04f7\u04f6"+
		"\u0001\u0000\u0000\u0000\u04f7\u04f8\u0001\u0000\u0000\u0000\u04f8\u04f9"+
		"\u0001\u0000\u0000\u0000\u04f9\u04fb\u0005\u0149\u0000\u0000\u04fa\u04f5"+
		"\u0001\u0000\u0000\u0000\u04fa\u04fb\u0001\u0000\u0000\u0000\u04fb\u0510"+
		"\u0001\u0000\u0000\u0000\u04fc\u04fd\u0005\u001f\u0000\u0000\u04fd\u04fe"+
		"\u0005\u0148\u0000\u0000\u04fe\u0503\u0003D\"\u0000\u04ff\u0500\u0005"+
		"\u014e\u0000\u0000\u0500\u0502\u0003D\"\u0000\u0501\u04ff\u0001\u0000"+
		"\u0000\u0000\u0502\u0505\u0001\u0000\u0000\u0000\u0503\u0501\u0001\u0000"+
		"\u0000\u0000\u0503\u0504\u0001\u0000\u0000\u0000\u0504\u0506\u0001\u0000"+
		"\u0000\u0000\u0505\u0503\u0001\u0000\u0000\u0000\u0506\u0507\u0005\u0149"+
		"\u0000\u0000\u0507\u0510\u0001\u0000\u0000\u0000\u0508\u0509\u0005 \u0000"+
		"\u0000\u0509\u050a\u0005\u0148\u0000\u0000\u050a\u050b\u0003\u0192\u00c9"+
		"\u0000\u050b\u050c\u0005\u014e\u0000\u0000\u050c\u050d\u0003\u0192\u00c9"+
		"\u0000\u050d\u050e\u0005\u0149\u0000\u0000\u050e\u0510\u0001\u0000\u0000"+
		"\u0000\u050f\u04f3\u0001\u0000\u0000\u0000\u050f\u04fc\u0001\u0000\u0000"+
		"\u0000\u050f\u0508\u0001\u0000\u0000\u0000\u0510{\u0001\u0000\u0000\u0000"+
		"\u0511\u051d\u0003\u019e\u00cf\u0000\u0512\u0513\u0005!\u0000\u0000\u0513"+
		"\u0514\u0005\u0148\u0000\u0000\u0514\u0515\u0003D\"\u0000\u0515\u0516"+
		"\u0005\u0149\u0000\u0000\u0516\u051d\u0001\u0000\u0000\u0000\u0517\u0518"+
		"\u0005\"\u0000\u0000\u0518\u0519\u0005\u0148\u0000\u0000\u0519\u051a\u0003"+
		"D\"\u0000\u051a\u051b\u0005\u0149\u0000\u0000\u051b\u051d\u0001\u0000"+
		"\u0000\u0000\u051c\u0511\u0001\u0000\u0000\u0000\u051c\u0512\u0001\u0000"+
		"\u0000\u0000\u051c\u0517\u0001\u0000\u0000\u0000\u051d}\u0001\u0000\u0000"+
		"\u0000\u051e\u0531\u0003\u01a0\u00d0\u0000\u051f\u0520\u0005#\u0000\u0000"+
		"\u0520\u0521\u0005\u0148\u0000\u0000\u0521\u0522\u0003>\u001f\u0000\u0522"+
		"\u0523\u0005\u0149\u0000\u0000\u0523\u0531\u0001\u0000\u0000\u0000\u0524"+
		"\u0525\u0005$\u0000\u0000\u0525\u0526\u0005\u0148\u0000\u0000\u0526\u0527"+
		"\u0003>\u001f\u0000\u0527\u0528\u0005\u0149\u0000\u0000\u0528\u0531\u0001"+
		"\u0000\u0000\u0000\u0529\u052a\u0005%\u0000\u0000\u052a\u052b\u0005\u0148"+
		"\u0000\u0000\u052b\u052c\u0003D\"\u0000\u052c\u052d\u0005\u014e\u0000"+
		"\u0000\u052d\u052e\u0003>\u001f\u0000\u052e\u052f\u0005\u0149\u0000\u0000"+
		"\u052f\u0531\u0001\u0000\u0000\u0000\u0530\u051e\u0001\u0000\u0000\u0000"+
		"\u0530\u051f\u0001\u0000\u0000\u0000\u0530\u0524\u0001\u0000\u0000\u0000"+
		"\u0530\u0529\u0001\u0000\u0000\u0000\u0531\u007f\u0001\u0000\u0000\u0000"+
		"\u0532\u0533\u0005&\u0000\u0000\u0533\u0534\u0005\u014c\u0000\u0000\u0534"+
		"\u0535\u0003\u0088D\u0000\u0535\u0536\u0005\u014d\u0000\u0000\u0536\u0537"+
		"\u0005\'\u0000\u0000\u0537\u0538\u0003D\"\u0000\u0538\u0081\u0001\u0000"+
		"\u0000\u0000\u0539\u053a\u0005\u0130\u0000\u0000\u053a\u053b\u0003\u0192"+
		"\u00c9\u0000\u053b\u053d\u0005\u0148\u0000\u0000\u053c\u053e\u0003h4\u0000"+
		"\u053d\u053c\u0001\u0000\u0000\u0000\u053d\u053e\u0001\u0000\u0000\u0000"+
		"\u053e\u053f\u0001\u0000\u0000\u0000\u053f\u0541\u0005\u0149\u0000\u0000"+
		"\u0540\u0542\u0005\u0150\u0000\u0000\u0541\u0540\u0001\u0000\u0000\u0000"+
		"\u0541\u0542\u0001\u0000\u0000\u0000\u0542\u0083\u0001\u0000\u0000\u0000"+
		"\u0543\u0544\u0005(\u0000\u0000\u0544\u0545\u0005\u0148\u0000\u0000\u0545"+
		"\u0546\u0003D\"\u0000\u0546\u0547\u0005\u0149\u0000\u0000\u0547\u0566"+
		"\u0001\u0000\u0000\u0000\u0548\u0549\u0005)\u0000\u0000\u0549\u054a\u0005"+
		"\u0148\u0000\u0000\u054a\u054b\u0003\u0192\u00c9\u0000\u054b\u054c\u0005"+
		"\u0149\u0000\u0000\u054c\u0566\u0001\u0000\u0000\u0000\u054d\u054e\u0005"+
		"*\u0000\u0000\u054e\u054f\u0005\u0148\u0000\u0000\u054f\u0550\u0003D\""+
		"\u0000\u0550\u0551\u0005\u0149\u0000\u0000\u0551\u0566\u0001\u0000\u0000"+
		"\u0000\u0552\u0553\u0005+\u0000\u0000\u0553\u0557\u0005\u014a\u0000\u0000"+
		"\u0554\u0556\u0003\u0194\u00ca\u0000\u0555\u0554\u0001\u0000\u0000\u0000"+
		"\u0556\u0559\u0001\u0000\u0000\u0000\u0557\u0555\u0001\u0000\u0000\u0000"+
		"\u0557\u0558\u0001\u0000\u0000\u0000\u0558\u055a\u0001\u0000\u0000\u0000"+
		"\u0559\u0557\u0001\u0000\u0000\u0000\u055a\u0566\u0005\u014b\u0000\u0000"+
		"\u055b\u055c\u0005,\u0000\u0000\u055c\u055d\u0005\u0148\u0000\u0000\u055d"+
		"\u055e\u0003D\"\u0000\u055e\u055f\u0005\u0149\u0000\u0000\u055f\u0566"+
		"\u0001\u0000\u0000\u0000\u0560\u0561\u0005-\u0000\u0000\u0561\u0562\u0005"+
		"\u0148\u0000\u0000\u0562\u0563\u0003D\"\u0000\u0563\u0564\u0005\u0149"+
		"\u0000\u0000\u0564\u0566\u0001\u0000\u0000\u0000\u0565\u0543\u0001\u0000"+
		"\u0000\u0000\u0565\u0548\u0001\u0000\u0000\u0000\u0565\u054d\u0001\u0000"+
		"\u0000\u0000\u0565\u0552\u0001\u0000\u0000\u0000\u0565\u055b\u0001\u0000"+
		"\u0000\u0000\u0565\u0560\u0001\u0000\u0000\u0000\u0566\u0085\u0001\u0000"+
		"\u0000\u0000\u0567\u0568\u0003\u0192\u00c9\u0000\u0568\u0569\u0005\u0158"+
		"\u0000\u0000\u0569\u056b\u0005\u0148\u0000\u0000\u056a\u056c\u0003h4\u0000"+
		"\u056b\u056a\u0001\u0000\u0000\u0000\u056b\u056c\u0001\u0000\u0000\u0000"+
		"\u056c\u056d\u0001\u0000\u0000\u0000\u056d\u056e\u0005\u0149\u0000\u0000"+
		"\u056e\u0087\u0001\u0000\u0000\u0000\u056f\u0574\u0003D\"\u0000\u0570"+
		"\u0571\u0005\u014e\u0000\u0000\u0571\u0573\u0003D\"\u0000\u0572\u0570"+
		"\u0001\u0000\u0000\u0000\u0573\u0576\u0001\u0000\u0000\u0000\u0574\u0572"+
		"\u0001\u0000\u0000\u0000\u0574\u0575\u0001\u0000\u0000\u0000\u0575\u0089"+
		"\u0001\u0000\u0000\u0000\u0576\u0574\u0001\u0000\u0000\u0000\u0577\u0578"+
		"\u0003D\"\u0000\u0578\u0579\u0005\u0150\u0000\u0000\u0579\u008b\u0001"+
		"\u0000\u0000\u0000\u057a\u057b\u0006F\uffff\uffff\u0000\u057b\u0587\u0003"+
		"\u008eG\u0000\u057c\u057d\u0005\u0161\u0000\u0000\u057d\u0582\u0003\u008c"+
		"F\u0000\u057e\u057f\u0005\u014e\u0000\u0000\u057f\u0581\u0003\u008cF\u0000"+
		"\u0580\u057e\u0001\u0000\u0000\u0000\u0581\u0584\u0001\u0000\u0000\u0000"+
		"\u0582\u0580\u0001\u0000\u0000\u0000\u0582\u0583\u0001\u0000\u0000\u0000"+
		"\u0583\u0585\u0001\u0000\u0000\u0000\u0584\u0582\u0001\u0000\u0000\u0000"+
		"\u0585\u0586\u0005\u0162\u0000\u0000\u0586\u0588\u0001\u0000\u0000\u0000"+
		"\u0587\u057c\u0001\u0000\u0000\u0000\u0587\u0588\u0001\u0000\u0000\u0000"+
		"\u0588\u05f9\u0001\u0000\u0000\u0000\u0589\u058a\u0005\u0148\u0000\u0000"+
		"\u058a\u05f9\u0005\u0149\u0000\u0000\u058b\u058c\u0005\u0148\u0000\u0000"+
		"\u058c\u058d\u0003\u008cF\u0000\u058d\u058e\u0005\u0149\u0000\u0000\u058e"+
		"\u05f9\u0001\u0000\u0000\u0000\u058f\u0590\u0005\u0148\u0000\u0000\u0590"+
		"\u0593\u0003\u008cF\u0000\u0591\u0592\u0005\u014e\u0000\u0000\u0592\u0594"+
		"\u0003\u008cF\u0000\u0593\u0591\u0001\u0000\u0000\u0000\u0594\u0595\u0001"+
		"\u0000\u0000\u0000\u0595\u0593\u0001\u0000\u0000\u0000\u0595\u0596\u0001"+
		"\u0000\u0000\u0000\u0596\u0597\u0001\u0000\u0000\u0000\u0597\u059a\u0005"+
		"\u0149\u0000\u0000\u0598\u0599\u0005\u0152\u0000\u0000\u0599\u059b\u0003"+
		"\u008cF\u0000\u059a\u0598\u0001\u0000\u0000\u0000\u059a\u059b\u0001\u0000"+
		"\u0000\u0000\u059b\u05f9\u0001\u0000\u0000\u0000\u059c\u059d\u0005\u00ea"+
		"\u0000\u0000\u059d\u05a6\u0005\u0148\u0000\u0000\u059e\u05a3\u0003\u008c"+
		"F\u0000\u059f\u05a0\u0005\u014e\u0000\u0000\u05a0\u05a2\u0003\u008cF\u0000"+
		"\u05a1\u059f\u0001\u0000\u0000\u0000\u05a2\u05a5\u0001\u0000\u0000\u0000"+
		"\u05a3\u05a1\u0001\u0000\u0000\u0000\u05a3\u05a4\u0001\u0000\u0000\u0000"+
		"\u05a4\u05a7\u0001\u0000\u0000\u0000\u05a5\u05a3\u0001\u0000\u0000\u0000"+
		"\u05a6\u059e\u0001\u0000\u0000\u0000\u05a6\u05a7\u0001\u0000\u0000\u0000"+
		"\u05a7\u05a8\u0001\u0000\u0000\u0000\u05a8\u05ab\u0005\u0149\u0000\u0000"+
		"\u05a9\u05aa\u0005\u0152\u0000\u0000\u05aa\u05ac\u0003\u008cF\u0000\u05ab"+
		"\u05a9\u0001\u0000\u0000\u0000\u05ab\u05ac\u0001\u0000\u0000\u0000\u05ac"+
		"\u05f9\u0001\u0000\u0000\u0000\u05ad\u05af\u0005\u0167\u0000\u0000\u05ae"+
		"\u05b0\u0005\u0003\u0000\u0000\u05af\u05ae\u0001\u0000\u0000\u0000\u05af"+
		"\u05b0\u0001\u0000\u0000\u0000\u05b0\u05b1\u0001\u0000\u0000\u0000\u05b1"+
		"\u05b2\u0005\u014c\u0000\u0000\u05b2\u05b3\u0003\u008cF\u0000\u05b3\u05b4"+
		"\u0005\u014d\u0000\u0000\u05b4\u05f9\u0001\u0000\u0000\u0000\u05b5\u05b7"+
		"\u0005\u0167\u0000\u0000\u05b6\u05b8\u0005\u0003\u0000\u0000\u05b7\u05b6"+
		"\u0001\u0000\u0000\u0000\u05b7\u05b8\u0001\u0000\u0000\u0000\u05b8\u05b9"+
		"\u0001\u0000\u0000\u0000\u05b9\u05f9\u0003\u008cF\u001e\u05ba\u05bc\u0005"+
		"\u015b\u0000\u0000\u05bb\u05bd\u0005\u0003\u0000\u0000\u05bc\u05bb\u0001"+
		"\u0000\u0000\u0000\u05bc\u05bd\u0001\u0000\u0000\u0000\u05bd\u05be\u0001"+
		"\u0000\u0000\u0000\u05be\u05f9\u0003\u008cF\u001d\u05bf\u05c0\u0005\u014c"+
		"\u0000\u0000\u05c0\u05c3\u0003\u008cF\u0000\u05c1\u05c2\u0005\u0150\u0000"+
		"\u0000\u05c2\u05c4\u0003D\"\u0000\u05c3\u05c1\u0001\u0000\u0000\u0000"+
		"\u05c3\u05c4\u0001\u0000\u0000\u0000\u05c4\u05c5\u0001\u0000\u0000\u0000"+
		"\u05c5\u05c6\u0005\u014d\u0000\u0000\u05c6\u05f9\u0001\u0000\u0000\u0000"+
		"\u05c7\u05f9\u0005\u011d\u0000\u0000\u05c8\u05f9\u0005\u011c\u0000\u0000"+
		"\u05c9\u05ca\u0005.\u0000\u0000\u05ca\u05cb\u0005\u0161\u0000\u0000\u05cb"+
		"\u05cc\u0003\u008cF\u0000\u05cc\u05cd\u0005\u0162\u0000\u0000\u05cd\u05f9"+
		"\u0001\u0000\u0000\u0000\u05ce\u05cf\u0005\u0131\u0000\u0000\u05cf\u05f9"+
		"\u0003\u008cF\u0017\u05d0\u05d1\u0005\u0132\u0000\u0000\u05d1\u05f9\u0003"+
		"\u008cF\u0016\u05d2\u05d3\u0005/\u0000\u0000\u05d3\u05d7\u0005\u014a\u0000"+
		"\u0000\u05d4\u05d6\u0003\u01ae\u00d7\u0000\u05d5\u05d4\u0001\u0000\u0000"+
		"\u0000\u05d6\u05d9\u0001\u0000\u0000\u0000\u05d7\u05d5\u0001\u0000\u0000"+
		"\u0000\u05d7\u05d8\u0001\u0000\u0000\u0000\u05d8\u05da\u0001\u0000\u0000"+
		"\u0000\u05d9\u05d7\u0001\u0000\u0000\u0000\u05da\u05f9\u0005\u014b\u0000"+
		"\u0000\u05db\u05f9\u0003\u01a8\u00d4\u0000\u05dc\u05f9\u0003\u01aa\u00d5"+
		"\u0000\u05dd\u05f9\u0003\u01ac\u00d6\u0000\u05de\u05f9\u00050\u0000\u0000"+
		"\u05df\u05f9\u00051\u0000\u0000\u05e0\u05f9\u00052\u0000\u0000\u05e1\u05f9"+
		"\u00053\u0000\u0000\u05e2\u05f9\u00054\u0000\u0000\u05e3\u05f9\u00055"+
		"\u0000\u0000\u05e4\u05f9\u00056\u0000\u0000\u05e5\u05f9\u0003\u01b2\u00d9"+
		"\u0000\u05e6\u05f9\u0003\u01b4\u00da\u0000\u05e7\u05f9\u0003\u01b6\u00db"+
		"\u0000\u05e8\u05f9\u0003\u01b8\u00dc\u0000\u05e9\u05f9\u0003\u01ba\u00dd"+
		"\u0000\u05ea\u05eb\u00058\u0000\u0000\u05eb\u05ec\u0005\u0161\u0000\u0000"+
		"\u05ec\u05ed\u0003\u0092I\u0000\u05ed\u05ee\u0005\u014f\u0000\u0000\u05ee"+
		"\u05ef\u0003\u008cF\u0000\u05ef\u05f0\u0005\u0162\u0000\u0000\u05f0\u05f9"+
		"\u0001\u0000\u0000\u0000\u05f1\u05f2\u00059\u0000\u0000\u05f2\u05f3\u0003"+
		"\u0092I\u0000\u05f3\u05f4\u0005\u014f\u0000\u0000\u05f4\u05f5\u0003\u008c"+
		"F\u0003\u05f5\u05f9\u0001\u0000\u0000\u0000\u05f6\u05f7\u0005:\u0000\u0000"+
		"\u05f7\u05f9\u0003\u008cF\u0002\u05f8\u057a\u0001\u0000\u0000\u0000\u05f8"+
		"\u0589\u0001\u0000\u0000\u0000\u05f8\u058b\u0001\u0000\u0000\u0000\u05f8"+
		"\u058f\u0001\u0000\u0000\u0000\u05f8\u059c\u0001\u0000\u0000\u0000\u05f8"+
		"\u05ad\u0001\u0000\u0000\u0000\u05f8\u05b5\u0001\u0000\u0000\u0000\u05f8"+
		"\u05ba\u0001\u0000\u0000\u0000\u05f8\u05bf\u0001\u0000\u0000\u0000\u05f8"+
		"\u05c7\u0001\u0000\u0000\u0000\u05f8\u05c8\u0001\u0000\u0000\u0000\u05f8"+
		"\u05c9\u0001\u0000\u0000\u0000\u05f8\u05ce\u0001\u0000\u0000\u0000\u05f8"+
		"\u05d0\u0001\u0000\u0000\u0000\u05f8\u05d2\u0001\u0000\u0000\u0000\u05f8"+
		"\u05db\u0001\u0000\u0000\u0000\u05f8\u05dc\u0001\u0000\u0000\u0000\u05f8"+
		"\u05dd\u0001\u0000\u0000\u0000\u05f8\u05de\u0001\u0000\u0000\u0000\u05f8"+
		"\u05df\u0001\u0000\u0000\u0000\u05f8\u05e0\u0001\u0000\u0000\u0000\u05f8"+
		"\u05e1\u0001\u0000\u0000\u0000\u05f8\u05e2\u0001\u0000\u0000\u0000\u05f8"+
		"\u05e3\u0001\u0000\u0000\u0000\u05f8\u05e4\u0001\u0000\u0000\u0000\u05f8"+
		"\u05e5\u0001\u0000\u0000\u0000\u05f8\u05e6\u0001\u0000\u0000\u0000\u05f8"+
		"\u05e7\u0001\u0000\u0000\u0000\u05f8\u05e8\u0001\u0000\u0000\u0000\u05f8"+
		"\u05e9\u0001\u0000\u0000\u0000\u05f8\u05ea\u0001\u0000\u0000\u0000\u05f8"+
		"\u05f1\u0001\u0000\u0000\u0000\u05f8\u05f6\u0001\u0000\u0000\u0000\u05f9"+
		"\u060f\u0001\u0000\u0000\u0000\u05fa\u05fb\n\u0019\u0000\u0000\u05fb\u060e"+
		"\u0005\u0173\u0000\u0000\u05fc\u05fd\n\u0005\u0000\u0000\u05fd\u05fe\u0005"+
		"\u0002\u0000\u0000\u05fe\u05ff\u00057\u0000\u0000\u05ff\u0600\u0005\u014a"+
		"\u0000\u0000\u0600\u0605\u0003\u00c2a\u0000\u0601\u0602\u0005\u014e\u0000"+
		"\u0000\u0602\u0604\u0003\u00c2a\u0000\u0603\u0601\u0001\u0000\u0000\u0000"+
		"\u0604\u0607\u0001\u0000\u0000\u0000\u0605\u0603\u0001\u0000\u0000\u0000"+
		"\u0605\u0606\u0001\u0000\u0000\u0000\u0606\u0608\u0001\u0000\u0000\u0000"+
		"\u0607\u0605\u0001\u0000\u0000\u0000\u0608\u0609\u0005\u014b\u0000\u0000"+
		"\u0609\u060e\u0001\u0000\u0000\u0000\u060a\u060b\n\u0001\u0000\u0000\u060b"+
		"\u060c\u0005\u014f\u0000\u0000\u060c\u060e\u0003\u0192\u00c9\u0000\u060d"+
		"\u05fa\u0001\u0000\u0000\u0000\u060d\u05fc\u0001\u0000\u0000\u0000\u060d"+
		"\u060a\u0001\u0000\u0000\u0000\u060e\u0611\u0001\u0000\u0000\u0000\u060f"+
		"\u060d\u0001\u0000\u0000\u0000\u060f\u0610\u0001\u0000\u0000\u0000\u0610"+
		"\u008d\u0001\u0000\u0000\u0000\u0611\u060f\u0001\u0000\u0000\u0000\u0612"+
		"\u0629\u0005\u0124\u0000\u0000\u0613\u0629\u0005\u011e\u0000\u0000\u0614"+
		"\u0629\u0005\u011f\u0000\u0000\u0615\u0629\u0005\u0120\u0000\u0000\u0616"+
		"\u0629\u0005\u0121\u0000\u0000\u0617\u0629\u0005\u0123\u0000\u0000\u0618"+
		"\u0629\u0005;\u0000\u0000\u0619\u0629\u0005<\u0000\u0000\u061a\u0629\u0005"+
		"=\u0000\u0000\u061b\u0629\u0005>\u0000\u0000\u061c\u0629\u0005?\u0000"+
		"\u0000\u061d\u0629\u0005@\u0000\u0000\u061e\u0629\u0005A\u0000\u0000\u061f"+
		"\u0629\u0005B\u0000\u0000\u0620\u0629\u0005C\u0000\u0000\u0621\u0629\u0005"+
		"D\u0000\u0000\u0622\u0629\u0005E\u0000\u0000\u0623\u0629\u0005F\u0000"+
		"\u0000\u0624\u0629\u0005G\u0000\u0000\u0625\u0629\u0005H\u0000\u0000\u0626"+
		"\u0629\u0005I\u0000\u0000\u0627\u0629\u0003\u0192\u00c9\u0000\u0628\u0612"+
		"\u0001\u0000\u0000\u0000\u0628\u0613\u0001\u0000\u0000\u0000\u0628\u0614"+
		"\u0001\u0000\u0000\u0000\u0628\u0615\u0001\u0000\u0000\u0000\u0628\u0616"+
		"\u0001\u0000\u0000\u0000\u0628\u0617\u0001\u0000\u0000\u0000\u0628\u0618"+
		"\u0001\u0000\u0000\u0000\u0628\u0619\u0001\u0000\u0000\u0000\u0628\u061a"+
		"\u0001\u0000\u0000\u0000\u0628\u061b\u0001\u0000\u0000\u0000\u0628\u061c"+
		"\u0001\u0000\u0000\u0000\u0628\u061d\u0001\u0000\u0000\u0000\u0628\u061e"+
		"\u0001\u0000\u0000\u0000\u0628\u061f\u0001\u0000\u0000\u0000\u0628\u0620"+
		"\u0001\u0000\u0000\u0000\u0628\u0621\u0001\u0000\u0000\u0000\u0628\u0622"+
		"\u0001\u0000\u0000\u0000\u0628\u0623\u0001\u0000\u0000\u0000\u0628\u0624"+
		"\u0001\u0000\u0000\u0000\u0628\u0625\u0001\u0000\u0000\u0000\u0628\u0626"+
		"\u0001\u0000\u0000\u0000\u0628\u0627\u0001\u0000\u0000\u0000\u0629\u008f"+
		"\u0001\u0000\u0000\u0000\u062a\u062b\u0005\u0161\u0000\u0000\u062b\u0630"+
		"\u0003\u0092I\u0000\u062c\u062d\u0005\u014e\u0000\u0000\u062d\u062f\u0003"+
		"\u0092I\u0000\u062e\u062c\u0001\u0000\u0000\u0000\u062f\u0632\u0001\u0000"+
		"\u0000\u0000\u0630\u062e\u0001\u0000\u0000\u0000\u0630\u0631\u0001\u0000"+
		"\u0000\u0000\u0631\u0633\u0001\u0000\u0000\u0000\u0632\u0630\u0001\u0000"+
		"\u0000\u0000\u0633\u0634\u0005\u0162\u0000\u0000\u0634\u0091\u0001\u0000"+
		"\u0000\u0000\u0635\u0638\u0003\u0192\u00c9\u0000\u0636\u0637\u0005\u0151"+
		"\u0000\u0000\u0637\u0639\u0003\u008cF\u0000\u0638\u0636\u0001\u0000\u0000"+
		"\u0000\u0638\u0639\u0001\u0000\u0000\u0000\u0639\u0093\u0001\u0000\u0000"+
		"\u0000\u063a\u063b\u0005\u0161\u0000\u0000\u063b\u0640\u0003\u008cF\u0000"+
		"\u063c\u063d\u0005\u014e\u0000\u0000\u063d\u063f\u0003\u008cF\u0000\u063e"+
		"\u063c\u0001\u0000\u0000\u0000\u063f\u0642\u0001\u0000\u0000\u0000\u0640"+
		"\u063e\u0001\u0000\u0000\u0000\u0640\u0641\u0001\u0000\u0000\u0000\u0641"+
		"\u0643\u0001\u0000\u0000\u0000\u0642\u0640\u0001\u0000\u0000\u0000\u0643"+
		"\u0644\u0005\u0162\u0000\u0000\u0644\u0095\u0001\u0000\u0000\u0000\u0645"+
		"\u0647\u0003\u001a\r\u0000\u0646\u0645\u0001\u0000\u0000\u0000\u0646\u0647"+
		"\u0001\u0000\u0000\u0000\u0647\u0648\u0001\u0000\u0000\u0000\u0648\u0649"+
		"\u0005\u010f\u0000\u0000\u0649\u064b\u0003\u0192\u00c9\u0000\u064a\u064c"+
		"\u0003\u0090H\u0000\u064b\u064a\u0001\u0000\u0000\u0000\u064b\u064c\u0001"+
		"\u0000\u0000\u0000\u064c\u064d\u0001\u0000\u0000\u0000\u064d\u0651\u0005"+
		"\u014a\u0000\u0000\u064e\u0650\u0003\u0098L\u0000\u064f\u064e\u0001\u0000"+
		"\u0000\u0000\u0650\u0653\u0001\u0000\u0000\u0000\u0651\u064f\u0001\u0000"+
		"\u0000\u0000\u0651\u0652\u0001\u0000\u0000\u0000\u0652\u0654\u0001\u0000"+
		"\u0000\u0000\u0653\u0651\u0001\u0000\u0000\u0000\u0654\u0655\u0005\u014b"+
		"\u0000\u0000\u0655\u0097\u0001\u0000\u0000\u0000\u0656\u0658\u0003\u001a"+
		"\r\u0000\u0657\u0656\u0001\u0000\u0000\u0000\u0657\u0658\u0001\u0000\u0000"+
		"\u0000\u0658\u0659\u0001\u0000\u0000\u0000\u0659\u065a\u0003\u0192\u00c9"+
		"\u0000\u065a\u065b\u0005\u0151\u0000\u0000\u065b\u065d\u0003\u008cF\u0000"+
		"\u065c\u065e\u0007\r\u0000\u0000\u065d\u065c\u0001\u0000\u0000\u0000\u065d"+
		"\u065e\u0001\u0000\u0000\u0000\u065e\u0099\u0001\u0000\u0000\u0000\u065f"+
		"\u0661\u0003\u001a\r\u0000\u0660\u065f\u0001\u0000\u0000\u0000\u0660\u0661"+
		"\u0001\u0000\u0000\u0000\u0661\u0662\u0001\u0000\u0000\u0000\u0662\u0663"+
		"\u0005\u0110\u0000\u0000\u0663\u0665\u0003\u0192\u00c9\u0000\u0664\u0666"+
		"\u0003\u0090H\u0000\u0665\u0664\u0001\u0000\u0000\u0000\u0665\u0666\u0001"+
		"\u0000\u0000\u0000\u0666\u0667\u0001\u0000\u0000\u0000\u0667\u066b\u0005"+
		"\u014a\u0000\u0000\u0668\u066a\u0003\u009cN\u0000\u0669\u0668\u0001\u0000"+
		"\u0000\u0000\u066a\u066d\u0001\u0000\u0000\u0000\u066b\u0669\u0001\u0000"+
		"\u0000\u0000\u066b\u066c\u0001\u0000\u0000\u0000\u066c\u066e\u0001\u0000"+
		"\u0000\u0000\u066d\u066b\u0001\u0000\u0000\u0000\u066e\u066f\u0005\u014b"+
		"\u0000\u0000\u066f\u009b\u0001\u0000\u0000\u0000\u0670\u067c\u0003\u0192"+
		"\u00c9\u0000\u0671\u0672\u0005\u0148\u0000\u0000\u0672\u0677\u0003\u008c"+
		"F\u0000\u0673\u0674\u0005\u014e\u0000\u0000\u0674\u0676\u0003\u008cF\u0000"+
		"\u0675\u0673\u0001\u0000\u0000\u0000\u0676\u0679\u0001\u0000\u0000\u0000"+
		"\u0677\u0675\u0001\u0000\u0000\u0000\u0677\u0678\u0001\u0000\u0000\u0000"+
		"\u0678\u067a\u0001\u0000\u0000\u0000\u0679\u0677\u0001\u0000\u0000\u0000"+
		"\u067a\u067b\u0005\u0149\u0000\u0000\u067b\u067d\u0001\u0000\u0000\u0000"+
		"\u067c\u0671\u0001\u0000\u0000\u0000\u067c\u067d\u0001\u0000\u0000\u0000"+
		"\u067d\u067f\u0001\u0000\u0000\u0000\u067e\u0680\u0007\r\u0000\u0000\u067f"+
		"\u067e\u0001\u0000\u0000\u0000\u067f\u0680\u0001\u0000\u0000\u0000\u0680"+
		"\u009d\u0001\u0000\u0000\u0000\u0681\u0683\u0003\u001a\r\u0000\u0682\u0681"+
		"\u0001\u0000\u0000\u0000\u0682\u0683\u0001\u0000\u0000\u0000\u0683\u0684"+
		"\u0001\u0000\u0000\u0000\u0684\u0685\u0005\u0111\u0000\u0000\u0685\u0687"+
		"\u0003\u0192\u00c9\u0000\u0686\u0688\u0003\u0090H\u0000\u0687\u0686\u0001"+
		"\u0000\u0000\u0000\u0687\u0688\u0001\u0000\u0000\u0000\u0688\u068b\u0001"+
		"\u0000\u0000\u0000\u0689\u068a\u0005J\u0000\u0000\u068a\u068c\u0003\u0198"+
		"\u00cc\u0000\u068b\u0689\u0001\u0000\u0000\u0000\u068b\u068c\u0001\u0000"+
		"\u0000\u0000\u068c\u068d\u0001\u0000\u0000\u0000\u068d\u0691\u0005\u014a"+
		"\u0000\u0000\u068e\u0690\u0003\u00a0P\u0000\u068f\u068e\u0001\u0000\u0000"+
		"\u0000\u0690\u0693\u0001\u0000\u0000\u0000\u0691\u068f\u0001\u0000\u0000"+
		"\u0000\u0691\u0692\u0001\u0000\u0000\u0000\u0692\u0694\u0001\u0000\u0000"+
		"\u0000\u0693\u0691\u0001\u0000\u0000\u0000\u0694\u0695\u0005\u014b\u0000"+
		"\u0000\u0695\u009f\u0001\u0000\u0000\u0000\u0696\u069a\u0003\u0014\n\u0000"+
		"\u0697\u069a\u0003\u00a6S\u0000\u0698\u069a\u0003\u00a8T\u0000\u0699\u0696"+
		"\u0001\u0000\u0000\u0000\u0699\u0697\u0001\u0000\u0000\u0000\u0699\u0698"+
		"\u0001\u0000\u0000\u0000\u069a\u00a1\u0001\u0000\u0000\u0000\u069b\u069d"+
		"\u0005\u0112\u0000\u0000\u069c\u069e\u0003\u0090H\u0000\u069d\u069c\u0001"+
		"\u0000\u0000\u0000\u069d\u069e\u0001\u0000\u0000\u0000\u069e\u06a2\u0001"+
		"\u0000\u0000\u0000\u069f\u06a0\u0003\u0192\u00c9\u0000\u06a0\u06a1\u0005"+
		"\u0100\u0000\u0000\u06a1\u06a3\u0001\u0000\u0000\u0000\u06a2\u069f\u0001"+
		"\u0000\u0000\u0000\u06a2\u06a3\u0001\u0000\u0000\u0000\u06a3\u06a4\u0001"+
		"\u0000\u0000\u0000\u06a4\u06a7\u0003\u008cF\u0000\u06a5\u06a6\u0005J\u0000"+
		"\u0000\u06a6\u06a8\u0003\u0198\u00cc\u0000\u06a7\u06a5\u0001\u0000\u0000"+
		"\u0000\u06a7\u06a8\u0001\u0000\u0000\u0000\u06a8\u06a9\u0001\u0000\u0000"+
		"\u0000\u06a9\u06ad\u0005\u014a\u0000\u0000\u06aa\u06ac\u0003\u00a4R\u0000"+
		"\u06ab\u06aa\u0001\u0000\u0000\u0000\u06ac\u06af\u0001\u0000\u0000\u0000"+
		"\u06ad\u06ab\u0001\u0000\u0000\u0000\u06ad\u06ae\u0001\u0000\u0000\u0000"+
		"\u06ae\u06b0\u0001\u0000\u0000\u0000\u06af\u06ad\u0001\u0000\u0000\u0000"+
		"\u06b0\u06b1\u0005\u014b\u0000\u0000\u06b1\u00a3\u0001\u0000\u0000\u0000"+
		"\u06b2\u06b6\u0003\u0014\n\u0000\u06b3\u06b6\u0003\u00a6S\u0000\u06b4"+
		"\u06b6\u0003\u00a8T\u0000\u06b5\u06b2\u0001\u0000\u0000\u0000\u06b5\u06b3"+
		"\u0001\u0000\u0000\u0000\u06b5\u06b4\u0001\u0000\u0000\u0000\u06b6\u00a5"+
		"\u0001\u0000\u0000\u0000\u06b7\u06b9\u0003\u001a\r\u0000\u06b8\u06b7\u0001"+
		"\u0000\u0000\u0000\u06b8\u06b9\u0001\u0000\u0000\u0000\u06b9\u06ba\u0001"+
		"\u0000\u0000\u0000\u06ba\u06bb\u0005\u010e\u0000\u0000\u06bb\u06bd\u0003"+
		"\u0192\u00c9\u0000\u06bc\u06be\u0003\u0090H\u0000\u06bd\u06bc\u0001\u0000"+
		"\u0000\u0000\u06bd\u06be\u0001\u0000\u0000\u0000\u06be\u06bf\u0001\u0000"+
		"\u0000\u0000\u06bf\u06c0\u0005\u015e\u0000\u0000\u06c0\u06c1\u0003\u008c"+
		"F\u0000\u06c1\u06c2\u0005\u0150\u0000\u0000\u06c2\u00a7\u0001\u0000\u0000"+
		"\u0000\u06c3\u06c5\u0003\u001a\r\u0000\u06c4\u06c3\u0001\u0000\u0000\u0000"+
		"\u06c4\u06c5\u0001\u0000\u0000\u0000\u06c5\u06c6\u0001\u0000\u0000\u0000"+
		"\u06c6\u06c7\u0005\u00ef\u0000\u0000\u06c7\u06c8\u0003\u0192\u00c9\u0000"+
		"\u06c8\u06c9\u0005\u0151\u0000\u0000\u06c9\u06ca\u0003\u008cF\u0000\u06ca"+
		"\u06cb\u0005\u015e\u0000\u0000\u06cb\u06cc\u0003D\"\u0000\u06cc\u06cd"+
		"\u0005\u0150\u0000\u0000\u06cd\u00a9\u0001\u0000\u0000\u0000\u06ce\u06d0"+
		"\u0003\u001a\r\u0000\u06cf\u06ce\u0001\u0000\u0000\u0000\u06cf\u06d0\u0001"+
		"\u0000\u0000\u0000\u06d0\u06d1\u0001\u0000\u0000\u0000\u06d1\u06d2\u0005"+
		"\u0113\u0000\u0000\u06d2\u06d4\u0003\u0192\u00c9\u0000\u06d3\u06d5\u0003"+
		"\u0090H\u0000\u06d4\u06d3\u0001\u0000\u0000\u0000\u06d4\u06d5\u0001\u0000"+
		"\u0000\u0000\u06d5\u06d8\u0001\u0000\u0000\u0000\u06d6\u06d7\u0005K\u0000"+
		"\u0000\u06d7\u06d9\u0003\u008cF\u0000\u06d8\u06d6\u0001\u0000\u0000\u0000"+
		"\u06d8\u06d9\u0001\u0000\u0000\u0000\u06d9\u06e3\u0001\u0000\u0000\u0000"+
		"\u06da\u06db\u0005L\u0000\u0000\u06db\u06e0\u0003\u008cF\u0000\u06dc\u06dd"+
		"\u0005\u014e\u0000\u0000\u06dd\u06df\u0003\u008cF\u0000\u06de\u06dc\u0001"+
		"\u0000\u0000\u0000\u06df\u06e2\u0001\u0000\u0000\u0000\u06e0\u06de\u0001"+
		"\u0000\u0000\u0000\u06e0\u06e1\u0001\u0000\u0000\u0000\u06e1\u06e4\u0001"+
		"\u0000\u0000\u0000\u06e2\u06e0\u0001\u0000\u0000\u0000\u06e3\u06da\u0001"+
		"\u0000\u0000\u0000\u06e3\u06e4\u0001\u0000\u0000\u0000\u06e4\u06e5\u0001"+
		"\u0000\u0000\u0000\u06e5\u06e9\u0005\u014a\u0000\u0000\u06e6\u06e8\u0003"+
		"\u00acV\u0000\u06e7\u06e6\u0001\u0000\u0000\u0000\u06e8\u06eb\u0001\u0000"+
		"\u0000\u0000\u06e9\u06e7\u0001\u0000\u0000\u0000\u06e9\u06ea\u0001\u0000"+
		"\u0000\u0000\u06ea\u06ec\u0001\u0000\u0000\u0000\u06eb\u06e9\u0001\u0000"+
		"\u0000\u0000\u06ec\u06ed\u0005\u014b\u0000\u0000\u06ed\u00ab\u0001\u0000"+
		"\u0000\u0000\u06ee\u06f0\u0003\u001a\r\u0000\u06ef\u06ee\u0001\u0000\u0000"+
		"\u0000\u06ef\u06f0\u0001\u0000\u0000\u0000\u06f0\u06f5\u0001\u0000\u0000"+
		"\u0000\u06f1\u06f6\u0003\u0014\n\u0000\u06f2\u06f6\u0003\u0098L\u0000"+
		"\u06f3\u06f6\u0003\u00aeW\u0000\u06f4\u06f6\u0003\u00b0X\u0000\u06f5\u06f1"+
		"\u0001\u0000\u0000\u0000\u06f5\u06f2\u0001\u0000\u0000\u0000\u06f5\u06f3"+
		"\u0001\u0000\u0000\u0000\u06f5\u06f4\u0001\u0000\u0000\u0000\u06f6\u00ad"+
		"\u0001\u0000\u0000\u0000\u06f7\u06f8\u0005\u00fb\u0000\u0000\u06f8\u06fa"+
		"\u0005\u0148\u0000\u0000\u06f9\u06fb\u0003\u0016\u000b\u0000\u06fa\u06f9"+
		"\u0001\u0000\u0000\u0000\u06fa\u06fb\u0001\u0000\u0000\u0000\u06fb\u06fc"+
		"\u0001\u0000\u0000\u0000\u06fc\u06fd\u0005\u0149\u0000\u0000\u06fd\u06fe"+
		"\u0003>\u001f\u0000\u06fe\u00af\u0001\u0000\u0000\u0000\u06ff\u0700\u0005"+
		"M\u0000\u0000\u0700\u0701\u0005\u0148\u0000\u0000\u0701\u0702\u0005\u0149"+
		"\u0000\u0000\u0702\u0703\u0003>\u001f\u0000\u0703\u00b1\u0001\u0000\u0000"+
		"\u0000\u0704\u0706\u0003\u001a\r\u0000\u0705\u0704\u0001\u0000\u0000\u0000"+
		"\u0705\u0706\u0001\u0000\u0000\u0000\u0706\u0707\u0001\u0000\u0000\u0000"+
		"\u0707\u0708\u0005\u0114\u0000\u0000\u0708\u070a\u0003\u0192\u00c9\u0000"+
		"\u0709\u070b\u0003\u0090H\u0000\u070a\u0709\u0001\u0000\u0000\u0000\u070a"+
		"\u070b\u0001\u0000\u0000\u0000\u070b\u070c\u0001\u0000\u0000\u0000\u070c"+
		"\u0710\u0005\u014a\u0000\u0000\u070d\u070f\u0003\u00a0P\u0000\u070e\u070d"+
		"\u0001\u0000\u0000\u0000\u070f\u0712\u0001\u0000\u0000\u0000\u0710\u070e"+
		"\u0001\u0000\u0000\u0000\u0710\u0711\u0001\u0000\u0000\u0000\u0711\u0713"+
		"\u0001\u0000\u0000\u0000\u0712\u0710\u0001\u0000\u0000\u0000\u0713\u0714"+
		"\u0005\u014b\u0000\u0000\u0714\u00b3\u0001\u0000\u0000\u0000\u0715\u0717"+
		"\u0003\u001a\r\u0000\u0716\u0715\u0001\u0000\u0000\u0000\u0716\u0717\u0001"+
		"\u0000\u0000\u0000\u0717\u0718\u0001\u0000\u0000\u0000\u0718\u0719\u0005"+
		"\u0115\u0000\u0000\u0719\u071b\u0003\u0192\u00c9\u0000\u071a\u071c\u0003"+
		"\u0090H\u0000\u071b\u071a\u0001\u0000\u0000\u0000\u071b\u071c\u0001\u0000"+
		"\u0000\u0000\u071c\u071d\u0001\u0000\u0000\u0000\u071d\u071f\u0005\u0148"+
		"\u0000\u0000\u071e\u0720\u0003\u0016\u000b\u0000\u071f\u071e\u0001\u0000"+
		"\u0000\u0000\u071f\u0720\u0001\u0000\u0000\u0000\u0720\u0721\u0001\u0000"+
		"\u0000\u0000\u0721\u0724\u0005\u0149\u0000\u0000\u0722\u0723\u0005\u0152"+
		"\u0000\u0000\u0723\u0725\u0003\u008cF\u0000\u0724\u0722\u0001\u0000\u0000"+
		"\u0000\u0724\u0725\u0001\u0000\u0000\u0000\u0725\u0728\u0001\u0000\u0000"+
		"\u0000\u0726\u0729\u0003>\u001f\u0000\u0727\u0729\u0005\u0150\u0000\u0000"+
		"\u0728\u0726\u0001\u0000\u0000\u0000\u0728\u0727\u0001\u0000\u0000\u0000"+
		"\u0729\u00b5\u0001\u0000\u0000\u0000\u072a\u072b\u0005\u0116\u0000\u0000"+
		"\u072b\u072c\u0005\u0117\u0000\u0000\u072c\u072d\u0003\u0192\u00c9\u0000"+
		"\u072d\u072f\u0005\u0148\u0000\u0000\u072e\u0730\u0003\u0016\u000b\u0000"+
		"\u072f\u072e\u0001\u0000\u0000\u0000\u072f\u0730\u0001\u0000\u0000\u0000"+
		"\u0730\u0731\u0001\u0000\u0000\u0000\u0731\u0732\u0005\u0149\u0000\u0000"+
		"\u0732\u0733\u0003>\u001f\u0000\u0733\u00b7\u0001\u0000\u0000\u0000\u0734"+
		"\u0735\u0005\u0125\u0000\u0000\u0735\u0736\u0005\u0126\u0000\u0000\u0736"+
		"\u0737\u0003\u0192\u00c9\u0000\u0737\u0739\u0005\u0148\u0000\u0000\u0738"+
		"\u073a\u0003\u0016\u000b\u0000\u0739\u0738\u0001\u0000\u0000\u0000\u0739"+
		"\u073a\u0001\u0000\u0000\u0000\u073a\u073b\u0001\u0000\u0000\u0000\u073b"+
		"\u073c\u0005\u0149\u0000\u0000\u073c\u073d\u0003>\u001f\u0000\u073d\u00b9"+
		"\u0001\u0000\u0000\u0000\u073e\u073f\u0005\u0133\u0000\u0000\u073f\u0740"+
		"\u0003\u0192\u00c9\u0000\u0740\u0744\u0005\u014a\u0000\u0000\u0741\u0743"+
		"\u0003\u0194\u00ca\u0000\u0742\u0741\u0001\u0000\u0000\u0000\u0743\u0746"+
		"\u0001\u0000\u0000\u0000\u0744\u0742\u0001\u0000\u0000\u0000\u0744\u0745"+
		"\u0001\u0000\u0000\u0000\u0745\u0747\u0001\u0000\u0000\u0000\u0746\u0744"+
		"\u0001\u0000\u0000\u0000\u0747\u0748\u0005\u014b\u0000\u0000\u0748\u00bb"+
		"\u0001\u0000\u0000\u0000\u0749\u074a\u0005\u0127\u0000\u0000\u074a\u074b"+
		"\u0003\u0192\u00c9\u0000\u074b\u074f\u0005\u014a\u0000\u0000\u074c\u074e"+
		"\u0003\u00be_\u0000\u074d\u074c\u0001\u0000\u0000\u0000\u074e\u0751\u0001"+
		"\u0000\u0000\u0000\u074f\u074d\u0001\u0000\u0000\u0000\u074f\u0750\u0001"+
		"\u0000\u0000\u0000\u0750\u0752\u0001\u0000\u0000\u0000\u0751\u074f\u0001"+
		"\u0000\u0000\u0000\u0752\u0753\u0005\u014b\u0000\u0000\u0753\u00bd\u0001"+
		"\u0000\u0000\u0000\u0754\u0755\u0005N\u0000\u0000\u0755\u0756\u0003\u0192"+
		"\u00c9\u0000\u0756\u0758\u0005\u0148\u0000\u0000\u0757\u0759\u0003\u0016"+
		"\u000b\u0000\u0758\u0757\u0001\u0000\u0000\u0000\u0758\u0759\u0001\u0000"+
		"\u0000\u0000\u0759\u075a\u0001\u0000\u0000\u0000\u075a\u075b\u0005\u0149"+
		"\u0000\u0000\u075b\u075c\u0005\u0152\u0000\u0000\u075c\u075d\u0003\u008c"+
		"F\u0000\u075d\u075e\u0005\u0150\u0000\u0000\u075e\u00bf\u0001\u0000\u0000"+
		"\u0000\u075f\u0764\u0003\u00c2a\u0000\u0760\u0761\u0005\u014e\u0000\u0000"+
		"\u0761\u0763\u0003\u00c2a\u0000\u0762\u0760\u0001\u0000\u0000\u0000\u0763"+
		"\u0766\u0001\u0000\u0000\u0000\u0764\u0762\u0001\u0000\u0000\u0000\u0764"+
		"\u0765\u0001\u0000\u0000\u0000\u0765\u00c1\u0001\u0000\u0000\u0000\u0766"+
		"\u0764\u0001\u0000\u0000\u0000\u0767\u0768\u0003\u0192\u00c9\u0000\u0768"+
		"\u00c3\u0001\u0000\u0000\u0000\u0769\u076a\u0005\u0134\u0000\u0000\u076a"+
		"\u076b\u0003\u0192\u00c9\u0000\u076b\u076f\u0005\u014a\u0000\u0000\u076c"+
		"\u076e\u0003\u0194\u00ca\u0000\u076d\u076c\u0001\u0000\u0000\u0000\u076e"+
		"\u0771\u0001\u0000\u0000\u0000\u076f\u076d\u0001\u0000\u0000\u0000\u076f"+
		"\u0770\u0001\u0000\u0000\u0000\u0770\u0772\u0001\u0000\u0000\u0000\u0771"+
		"\u076f\u0001\u0000\u0000\u0000\u0772\u0773\u0005\u014b\u0000\u0000\u0773"+
		"\u00c5\u0001\u0000\u0000\u0000\u0774\u0775\u0005O\u0000\u0000\u0775\u0776"+
		"\u0003\u0192\u00c9\u0000\u0776\u077a\u0005\u014a\u0000\u0000\u0777\u0779"+
		"\u0003\u0194\u00ca\u0000\u0778\u0777\u0001\u0000\u0000\u0000\u0779\u077c"+
		"\u0001\u0000\u0000\u0000\u077a\u0778\u0001\u0000\u0000\u0000\u077a\u077b"+
		"\u0001\u0000\u0000\u0000\u077b\u077d\u0001\u0000\u0000\u0000\u077c\u077a"+
		"\u0001\u0000\u0000\u0000\u077d\u077e\u0005\u014b\u0000\u0000\u077e\u00c7"+
		"\u0001\u0000\u0000\u0000\u077f\u0780\u0005\u0126\u0000\u0000\u0780\u0781"+
		"\u0003\u0192\u00c9\u0000\u0781\u0786\u0005\u014a\u0000\u0000\u0782\u0785"+
		"\u0003\u0180\u00c0\u0000\u0783\u0785\u0003\u0182\u00c1\u0000\u0784\u0782"+
		"\u0001\u0000\u0000\u0000\u0784\u0783\u0001\u0000\u0000\u0000\u0785\u0788"+
		"\u0001\u0000\u0000\u0000\u0786\u0784\u0001\u0000\u0000\u0000\u0786\u0787"+
		"\u0001\u0000\u0000\u0000\u0787\u0789\u0001\u0000\u0000\u0000\u0788\u0786"+
		"\u0001\u0000\u0000\u0000\u0789\u078a\u0005\u014b\u0000\u0000\u078a\u00c9"+
		"\u0001\u0000\u0000\u0000\u078b\u078c\u0005P\u0000\u0000\u078c\u078d\u0003"+
		"\u0192\u00c9\u0000\u078d\u0791\u0005\u014a\u0000\u0000\u078e\u0790\u0003"+
		"\u0194\u00ca\u0000\u078f\u078e\u0001\u0000\u0000\u0000\u0790\u0793\u0001"+
		"\u0000\u0000\u0000\u0791\u078f\u0001\u0000\u0000\u0000\u0791\u0792\u0001"+
		"\u0000\u0000\u0000\u0792\u0794\u0001\u0000\u0000\u0000\u0793\u0791\u0001"+
		"\u0000\u0000\u0000\u0794\u0795\u0005\u014b\u0000\u0000\u0795\u00cb\u0001"+
		"\u0000\u0000\u0000\u0796\u0797\u0005Q\u0000\u0000\u0797\u0798\u0003\u0192"+
		"\u00c9\u0000\u0798\u079c\u0005\u014a\u0000\u0000\u0799\u079b\u0003\u0194"+
		"\u00ca\u0000\u079a\u0799\u0001\u0000\u0000\u0000\u079b\u079e\u0001\u0000"+
		"\u0000\u0000\u079c\u079a\u0001\u0000\u0000\u0000\u079c\u079d\u0001\u0000"+
		"\u0000\u0000\u079d\u079f\u0001\u0000\u0000\u0000\u079e\u079c\u0001\u0000"+
		"\u0000\u0000\u079f\u07a0\u0005\u014b\u0000\u0000\u07a0\u00cd\u0001\u0000"+
		"\u0000\u0000\u07a1\u07a2\u0005R\u0000\u0000\u07a2\u07a3\u0005\u00e3\u0000"+
		"\u0000\u07a3\u07a4\u0003\u0192\u00c9\u0000\u07a4\u07a6\u0005\u0148\u0000"+
		"\u0000\u07a5\u07a7\u0003\u0016\u000b\u0000\u07a6\u07a5\u0001\u0000\u0000"+
		"\u0000\u07a6\u07a7\u0001\u0000\u0000\u0000\u07a7\u07a8\u0001\u0000\u0000"+
		"\u0000\u07a8\u07a9\u0005\u0149\u0000\u0000\u07a9\u07ad\u0005\u014a\u0000"+
		"\u0000\u07aa\u07ac\u0003\u0194\u00ca\u0000\u07ab\u07aa\u0001\u0000\u0000"+
		"\u0000\u07ac\u07af\u0001\u0000\u0000\u0000\u07ad\u07ab\u0001\u0000\u0000"+
		"\u0000\u07ad\u07ae\u0001\u0000\u0000\u0000\u07ae\u07b0\u0001\u0000\u0000"+
		"\u0000\u07af\u07ad\u0001\u0000\u0000\u0000\u07b0\u07b1\u0005\u014b\u0000"+
		"\u0000\u07b1\u00cf\u0001\u0000\u0000\u0000\u07b2\u07b3\u0005S\u0000\u0000"+
		"\u07b3\u07b4\u0003\u0192\u00c9\u0000\u07b4\u07b8\u0005\u014a\u0000\u0000"+
		"\u07b5\u07b7\u0003\u0194\u00ca\u0000\u07b6\u07b5\u0001\u0000\u0000\u0000"+
		"\u07b7\u07ba\u0001\u0000\u0000\u0000\u07b8\u07b6\u0001\u0000\u0000\u0000"+
		"\u07b8\u07b9\u0001\u0000\u0000\u0000\u07b9\u07bb\u0001\u0000\u0000\u0000"+
		"\u07ba\u07b8\u0001\u0000\u0000\u0000\u07bb\u07bc\u0005\u014b\u0000\u0000"+
		"\u07bc\u00d1\u0001\u0000\u0000\u0000\u07bd\u07be\u0005T\u0000\u0000\u07be"+
		"\u07bf\u0003\u0192\u00c9\u0000\u07bf\u07c3\u0005\u014a\u0000\u0000\u07c0"+
		"\u07c2\u0003\u0194\u00ca\u0000\u07c1\u07c0\u0001\u0000\u0000\u0000\u07c2"+
		"\u07c5\u0001\u0000\u0000\u0000\u07c3\u07c1\u0001\u0000\u0000\u0000\u07c3"+
		"\u07c4\u0001\u0000\u0000\u0000\u07c4\u07c6\u0001\u0000\u0000\u0000\u07c5"+
		"\u07c3\u0001\u0000\u0000\u0000\u07c6\u07c7\u0005\u014b\u0000\u0000\u07c7"+
		"\u00d3\u0001\u0000\u0000\u0000\u07c8\u07c9\u0005U\u0000\u0000\u07c9\u07ca"+
		"\u0005\u0126\u0000\u0000\u07ca\u07cb\u0003\u0192\u00c9\u0000\u07cb\u07cf"+
		"\u0005\u014a\u0000\u0000\u07cc\u07ce\u0003\u0194\u00ca\u0000\u07cd\u07cc"+
		"\u0001\u0000\u0000\u0000\u07ce\u07d1\u0001\u0000\u0000\u0000\u07cf\u07cd"+
		"\u0001\u0000\u0000\u0000\u07cf\u07d0\u0001\u0000\u0000\u0000\u07d0\u07d2"+
		"\u0001\u0000\u0000\u0000\u07d1\u07cf\u0001\u0000\u0000\u0000\u07d2\u07d3"+
		"\u0005\u014b\u0000\u0000\u07d3\u00d5\u0001\u0000\u0000\u0000\u07d4\u07d5"+
		"\u0005V\u0000\u0000\u07d5\u07d6\u0003\u0192\u00c9\u0000\u07d6\u07da\u0005"+
		"\u014a\u0000\u0000\u07d7\u07d9\u0003\u0194\u00ca\u0000\u07d8\u07d7\u0001"+
		"\u0000\u0000\u0000\u07d9\u07dc\u0001\u0000\u0000\u0000\u07da\u07d8\u0001"+
		"\u0000\u0000\u0000\u07da\u07db\u0001\u0000\u0000\u0000\u07db\u07dd\u0001"+
		"\u0000\u0000\u0000\u07dc\u07da\u0001\u0000\u0000\u0000\u07dd\u07de\u0005"+
		"\u014b\u0000\u0000\u07de\u00d7\u0001\u0000\u0000\u0000\u07df\u07e0\u0005"+
		"W\u0000\u0000\u07e0\u07e1\u0003\u0192\u00c9\u0000\u07e1\u07e5\u0005\u014a"+
		"\u0000\u0000\u07e2\u07e4\u0003\u0194\u00ca\u0000\u07e3\u07e2\u0001\u0000"+
		"\u0000\u0000\u07e4\u07e7\u0001\u0000\u0000\u0000\u07e5\u07e3\u0001\u0000"+
		"\u0000\u0000\u07e5\u07e6\u0001\u0000\u0000\u0000\u07e6\u07e8\u0001\u0000"+
		"\u0000\u0000\u07e7\u07e5\u0001\u0000\u0000\u0000\u07e8\u07e9\u0005\u014b"+
		"\u0000\u0000\u07e9\u00d9\u0001\u0000\u0000\u0000\u07ea\u07eb\u0005X\u0000"+
		"\u0000\u07eb\u07ec\u0003\u0192\u00c9\u0000\u07ec\u07f0\u0005\u014a\u0000"+
		"\u0000\u07ed\u07ef\u0003\u0194\u00ca\u0000\u07ee\u07ed\u0001\u0000\u0000"+
		"\u0000\u07ef\u07f2\u0001\u0000\u0000\u0000\u07f0\u07ee\u0001\u0000\u0000"+
		"\u0000\u07f0\u07f1\u0001\u0000\u0000\u0000\u07f1\u07f3\u0001\u0000\u0000"+
		"\u0000\u07f2\u07f0\u0001\u0000\u0000\u0000\u07f3\u07f4\u0005\u014b\u0000"+
		"\u0000\u07f4\u00db\u0001\u0000\u0000\u0000\u07f5\u07f6\u0005Y\u0000\u0000"+
		"\u07f6\u07f7\u0003\u0192\u00c9\u0000\u07f7\u07fb\u0005\u014a\u0000\u0000"+
		"\u07f8\u07fa\u0003\u0194\u00ca\u0000\u07f9\u07f8\u0001\u0000\u0000\u0000"+
		"\u07fa\u07fd\u0001\u0000\u0000\u0000\u07fb\u07f9\u0001\u0000\u0000\u0000"+
		"\u07fb\u07fc\u0001\u0000\u0000\u0000\u07fc\u07fe\u0001\u0000\u0000\u0000"+
		"\u07fd\u07fb\u0001\u0000\u0000\u0000\u07fe\u07ff\u0005\u014b\u0000\u0000"+
		"\u07ff\u00dd\u0001\u0000\u0000\u0000\u0800\u0801\u0005Z\u0000\u0000\u0801"+
		"\u0802\u0003\u0192\u00c9\u0000\u0802\u0806\u0005\u014a\u0000\u0000\u0803"+
		"\u0805\u0003\u0194\u00ca\u0000\u0804\u0803\u0001\u0000\u0000\u0000\u0805"+
		"\u0808\u0001\u0000\u0000\u0000\u0806\u0804\u0001\u0000\u0000\u0000\u0806"+
		"\u0807\u0001\u0000\u0000\u0000\u0807\u0809\u0001\u0000\u0000\u0000\u0808"+
		"\u0806\u0001\u0000\u0000\u0000\u0809\u080a\u0005\u014b\u0000\u0000\u080a"+
		"\u00df\u0001\u0000\u0000\u0000\u080b\u080c\u0005[\u0000\u0000\u080c\u080d"+
		"\u0005\\\u0000\u0000\u080d\u080e\u0003\u0192\u00c9\u0000\u080e\u0812\u0005"+
		"\u014a\u0000\u0000\u080f\u0811\u0003\u0194\u00ca\u0000\u0810\u080f\u0001"+
		"\u0000\u0000\u0000\u0811\u0814\u0001\u0000\u0000\u0000\u0812\u0810\u0001"+
		"\u0000\u0000\u0000\u0812\u0813\u0001\u0000\u0000\u0000\u0813\u0815\u0001"+
		"\u0000\u0000\u0000\u0814\u0812\u0001\u0000\u0000\u0000\u0815\u0816\u0005"+
		"\u014b\u0000\u0000\u0816\u00e1\u0001\u0000\u0000\u0000\u0817\u0818\u0005"+
		"]\u0000\u0000\u0818\u0819\u0005\\\u0000\u0000\u0819\u081a\u0003\u0192"+
		"\u00c9\u0000\u081a\u081e\u0005\u014a\u0000\u0000\u081b\u081d\u0003\u0194"+
		"\u00ca\u0000\u081c\u081b\u0001\u0000\u0000\u0000\u081d\u0820\u0001\u0000"+
		"\u0000\u0000\u081e\u081c\u0001\u0000\u0000\u0000\u081e\u081f\u0001\u0000"+
		"\u0000\u0000\u081f\u0821\u0001\u0000\u0000\u0000\u0820\u081e\u0001\u0000"+
		"\u0000\u0000\u0821\u0822\u0005\u014b\u0000\u0000\u0822\u00e3\u0001\u0000"+
		"\u0000\u0000\u0823\u0824\u0005^\u0000\u0000\u0824\u0825\u0005\\\u0000"+
		"\u0000\u0825\u0826\u0003\u0192\u00c9\u0000\u0826\u082a\u0005\u014a\u0000"+
		"\u0000\u0827\u0829\u0003\u0194\u00ca\u0000\u0828\u0827\u0001\u0000\u0000"+
		"\u0000\u0829\u082c\u0001\u0000\u0000\u0000\u082a\u0828\u0001\u0000\u0000"+
		"\u0000\u082a\u082b\u0001\u0000\u0000\u0000\u082b\u082d\u0001\u0000\u0000"+
		"\u0000\u082c\u082a\u0001\u0000\u0000\u0000\u082d\u082e\u0005\u014b\u0000"+
		"\u0000\u082e\u00e5\u0001\u0000\u0000\u0000\u082f\u0830\u0005_\u0000\u0000"+
		"\u0830\u0831\u0005\\\u0000\u0000\u0831\u0832\u0003\u0192\u00c9\u0000\u0832"+
		"\u0836\u0005\u014a\u0000\u0000\u0833\u0835\u0003\u0194\u00ca\u0000\u0834"+
		"\u0833\u0001\u0000\u0000\u0000\u0835\u0838\u0001\u0000\u0000\u0000\u0836"+
		"\u0834\u0001\u0000\u0000\u0000\u0836\u0837\u0001\u0000\u0000\u0000\u0837"+
		"\u0839\u0001\u0000\u0000\u0000\u0838\u0836\u0001\u0000\u0000\u0000\u0839"+
		"\u083a\u0005\u014b\u0000\u0000\u083a\u00e7\u0001\u0000\u0000\u0000\u083b"+
		"\u083c\u0005`\u0000\u0000\u083c\u083d\u0005\\\u0000\u0000\u083d\u083e"+
		"\u0003\u0192\u00c9\u0000\u083e\u0842\u0005\u014a\u0000\u0000\u083f\u0841"+
		"\u0003\u0194\u00ca\u0000\u0840\u083f\u0001\u0000\u0000\u0000\u0841\u0844"+
		"\u0001\u0000\u0000\u0000\u0842\u0840\u0001\u0000\u0000\u0000\u0842\u0843"+
		"\u0001\u0000\u0000\u0000\u0843\u0845\u0001\u0000\u0000\u0000\u0844\u0842"+
		"\u0001\u0000\u0000\u0000\u0845\u0846\u0005\u014b\u0000\u0000\u0846\u00e9"+
		"\u0001\u0000\u0000\u0000\u0847\u0848\u0005a\u0000\u0000\u0848\u0849\u0005"+
		"\u0114\u0000\u0000\u0849\u084a\u0003\u0192\u00c9\u0000\u084a\u084e\u0005"+
		"\u014a\u0000\u0000\u084b\u084d\u0003\u0194\u00ca\u0000\u084c\u084b\u0001"+
		"\u0000\u0000\u0000\u084d\u0850\u0001\u0000\u0000\u0000\u084e\u084c\u0001"+
		"\u0000\u0000\u0000\u084e\u084f\u0001\u0000\u0000\u0000\u084f\u0851\u0001"+
		"\u0000\u0000\u0000\u0850\u084e\u0001\u0000\u0000\u0000\u0851\u0852\u0005"+
		"\u014b\u0000\u0000\u0852\u00eb\u0001\u0000\u0000\u0000\u0853\u0854\u0005"+
		"b\u0000\u0000\u0854\u0855\u0005c\u0000\u0000\u0855\u0856\u0003\u0192\u00c9"+
		"\u0000\u0856\u085a\u0005\u014a\u0000\u0000\u0857\u0859\u0003\u0194\u00ca"+
		"\u0000\u0858\u0857\u0001\u0000\u0000\u0000\u0859\u085c\u0001\u0000\u0000"+
		"\u0000\u085a\u0858\u0001\u0000\u0000\u0000\u085a\u085b\u0001\u0000\u0000"+
		"\u0000\u085b\u085d\u0001\u0000\u0000\u0000\u085c\u085a\u0001\u0000\u0000"+
		"\u0000\u085d\u085e\u0005\u014b\u0000\u0000\u085e\u00ed\u0001\u0000\u0000"+
		"\u0000\u085f\u0860\u0005d\u0000\u0000\u0860\u0861\u0005e\u0000\u0000\u0861"+
		"\u0862\u0003\u0192\u00c9\u0000\u0862\u0866\u0005\u014a\u0000\u0000\u0863"+
		"\u0865\u0003\u0194\u00ca\u0000\u0864\u0863\u0001\u0000\u0000\u0000\u0865"+
		"\u0868\u0001\u0000\u0000\u0000\u0866\u0864\u0001\u0000\u0000\u0000\u0866"+
		"\u0867\u0001\u0000\u0000\u0000\u0867\u0869\u0001\u0000\u0000\u0000\u0868"+
		"\u0866\u0001\u0000\u0000\u0000\u0869\u086a\u0005\u014b\u0000\u0000\u086a"+
		"\u00ef\u0001\u0000\u0000\u0000\u086b\u086c\u0005f\u0000\u0000\u086c\u086d"+
		"\u0005g\u0000\u0000\u086d\u086e\u0003\u0192\u00c9\u0000\u086e\u0872\u0005"+
		"\u014a\u0000\u0000\u086f\u0871\u0003\u0194\u00ca\u0000\u0870\u086f\u0001"+
		"\u0000\u0000\u0000\u0871\u0874\u0001\u0000\u0000\u0000\u0872\u0870\u0001"+
		"\u0000\u0000\u0000\u0872\u0873\u0001\u0000\u0000\u0000\u0873\u0875\u0001"+
		"\u0000\u0000\u0000\u0874\u0872\u0001\u0000\u0000\u0000\u0875\u0876\u0005"+
		"\u014b\u0000\u0000\u0876\u00f1\u0001\u0000\u0000\u0000\u0877\u0878\u0005"+
		"h\u0000\u0000\u0878\u0879\u0003\u0192\u00c9\u0000\u0879\u087d\u0005\u014a"+
		"\u0000\u0000\u087a\u087c\u0003\u0194\u00ca\u0000\u087b\u087a\u0001\u0000"+
		"\u0000\u0000\u087c\u087f\u0001\u0000\u0000\u0000\u087d\u087b\u0001\u0000"+
		"\u0000\u0000\u087d\u087e\u0001\u0000\u0000\u0000\u087e\u0880\u0001\u0000"+
		"\u0000\u0000\u087f\u087d\u0001\u0000\u0000\u0000\u0880\u0881\u0005\u014b"+
		"\u0000\u0000\u0881\u00f3\u0001\u0000\u0000\u0000\u0882\u0883\u0005i\u0000"+
		"\u0000\u0883\u0884\u0003\u0192\u00c9\u0000\u0884\u0888\u0005\u014a\u0000"+
		"\u0000\u0885\u0887\u0003\u0194\u00ca\u0000\u0886\u0885\u0001\u0000\u0000"+
		"\u0000\u0887\u088a\u0001\u0000\u0000\u0000\u0888\u0886\u0001\u0000\u0000"+
		"\u0000\u0888\u0889\u0001\u0000\u0000\u0000\u0889\u088b\u0001\u0000\u0000"+
		"\u0000\u088a\u0888\u0001\u0000\u0000\u0000\u088b\u088c\u0005\u014b\u0000"+
		"\u0000\u088c\u00f5\u0001\u0000\u0000\u0000\u088d\u088e\u0005j\u0000\u0000"+
		"\u088e\u088f\u0005k\u0000\u0000\u088f\u0890\u0005l\u0000\u0000\u0890\u0894"+
		"\u0005\u014a\u0000\u0000\u0891\u0893\u0003\u0194\u00ca\u0000\u0892\u0891"+
		"\u0001\u0000\u0000\u0000\u0893\u0896\u0001\u0000\u0000\u0000\u0894\u0892"+
		"\u0001\u0000\u0000\u0000\u0894\u0895\u0001\u0000\u0000\u0000\u0895\u0897"+
		"\u0001\u0000\u0000\u0000\u0896\u0894\u0001\u0000\u0000\u0000\u0897\u0898"+
		"\u0005\u014b\u0000\u0000\u0898\u00f7\u0001\u0000\u0000\u0000\u0899\u089a"+
		"\u0005m\u0000\u0000\u089a\u089b\u0005d\u0000\u0000\u089b\u089c\u0003\u0192"+
		"\u00c9\u0000\u089c\u08a0\u0005\u014a\u0000\u0000\u089d\u089f\u0003\u0194"+
		"\u00ca\u0000\u089e\u089d\u0001\u0000\u0000\u0000\u089f\u08a2\u0001\u0000"+
		"\u0000\u0000\u08a0\u089e\u0001\u0000\u0000\u0000\u08a0\u08a1\u0001\u0000"+
		"\u0000\u0000\u08a1\u08a3\u0001\u0000\u0000\u0000\u08a2\u08a0\u0001\u0000"+
		"\u0000\u0000\u08a3\u08a4\u0005\u014b\u0000\u0000\u08a4\u00f9\u0001\u0000"+
		"\u0000\u0000\u08a5\u08a6\u0005n\u0000\u0000\u08a6\u08a7\u0005o\u0000\u0000"+
		"\u08a7\u08a8\u0003\u0192\u00c9\u0000\u08a8\u08ac\u0005\u014a\u0000\u0000"+
		"\u08a9\u08ab\u0003\u0194\u00ca\u0000\u08aa\u08a9\u0001\u0000\u0000\u0000"+
		"\u08ab\u08ae\u0001\u0000\u0000\u0000\u08ac\u08aa\u0001\u0000\u0000\u0000"+
		"\u08ac\u08ad\u0001\u0000\u0000\u0000\u08ad\u08af\u0001\u0000\u0000\u0000"+
		"\u08ae\u08ac\u0001";
	private static final String _serializedATNSegment1 =
		"\u0000\u0000\u0000\u08af\u08b0\u0005\u014b\u0000\u0000\u08b0\u00fb\u0001"+
		"\u0000\u0000\u0000\u08b1\u08b2\u0005p\u0000\u0000\u08b2\u08b3\u0003\u0192"+
		"\u00c9\u0000\u08b3\u08b7\u0005\u014a\u0000\u0000\u08b4\u08b6\u0003\u0194"+
		"\u00ca\u0000\u08b5\u08b4\u0001\u0000\u0000\u0000\u08b6\u08b9\u0001\u0000"+
		"\u0000\u0000\u08b7\u08b5\u0001\u0000\u0000\u0000\u08b7\u08b8\u0001\u0000"+
		"\u0000\u0000\u08b8\u08ba\u0001\u0000\u0000\u0000\u08b9\u08b7\u0001\u0000"+
		"\u0000\u0000\u08ba\u08bb\u0005\u014b\u0000\u0000\u08bb\u00fd\u0001\u0000"+
		"\u0000\u0000\u08bc\u08bd\u0005q\u0000\u0000\u08bd\u08be\u0005r\u0000\u0000"+
		"\u08be\u08bf\u0003\u0192\u00c9\u0000\u08bf\u08c3\u0005\u014a\u0000\u0000"+
		"\u08c0\u08c2\u0003\u0194\u00ca\u0000\u08c1\u08c0\u0001\u0000\u0000\u0000"+
		"\u08c2\u08c5\u0001\u0000\u0000\u0000\u08c3\u08c1\u0001\u0000\u0000\u0000"+
		"\u08c3\u08c4\u0001\u0000\u0000\u0000\u08c4\u08c6\u0001\u0000\u0000\u0000"+
		"\u08c5\u08c3\u0001\u0000\u0000\u0000\u08c6\u08c7\u0005\u014b\u0000\u0000"+
		"\u08c7\u00ff\u0001\u0000\u0000\u0000\u08c8\u08c9\u0005q\u0000\u0000\u08c9"+
		"\u08ca\u0005s\u0000\u0000\u08ca\u08cb\u0003\u0192\u00c9\u0000\u08cb\u08cf"+
		"\u0005\u014a\u0000\u0000\u08cc\u08ce\u0003\u0194\u00ca\u0000\u08cd\u08cc"+
		"\u0001\u0000\u0000\u0000\u08ce\u08d1\u0001\u0000\u0000\u0000\u08cf\u08cd"+
		"\u0001\u0000\u0000\u0000\u08cf\u08d0\u0001\u0000\u0000\u0000\u08d0\u08d2"+
		"\u0001\u0000\u0000\u0000\u08d1\u08cf\u0001\u0000\u0000\u0000\u08d2\u08d3"+
		"\u0005\u014b\u0000\u0000\u08d3\u0101\u0001\u0000\u0000\u0000\u08d4\u08d5"+
		"\u0005q\u0000\u0000\u08d5\u08d6\u0005\"\u0000\u0000\u08d6\u08d7\u0003"+
		"\u0192\u00c9\u0000\u08d7\u08db\u0005\u014a\u0000\u0000\u08d8\u08da\u0003"+
		"\u0194\u00ca\u0000\u08d9\u08d8\u0001\u0000\u0000\u0000\u08da\u08dd\u0001"+
		"\u0000\u0000\u0000\u08db\u08d9\u0001\u0000\u0000\u0000\u08db\u08dc\u0001"+
		"\u0000\u0000\u0000\u08dc\u08de\u0001\u0000\u0000\u0000\u08dd\u08db\u0001"+
		"\u0000\u0000\u0000\u08de\u08df\u0005\u014b\u0000\u0000\u08df\u0103\u0001"+
		"\u0000\u0000\u0000\u08e0\u08e1\u0005q\u0000\u0000\u08e1\u08e2\u0005t\u0000"+
		"\u0000\u08e2\u08e3\u0003\u0192\u00c9\u0000\u08e3\u08e7\u0005\u014a\u0000"+
		"\u0000\u08e4\u08e6\u0003\u0194\u00ca\u0000\u08e5\u08e4\u0001\u0000\u0000"+
		"\u0000\u08e6\u08e9\u0001\u0000\u0000\u0000\u08e7\u08e5\u0001\u0000\u0000"+
		"\u0000\u08e7\u08e8\u0001\u0000\u0000\u0000\u08e8\u08ea\u0001\u0000\u0000"+
		"\u0000\u08e9\u08e7\u0001\u0000\u0000\u0000\u08ea\u08eb\u0005\u014b\u0000"+
		"\u0000\u08eb\u0105\u0001\u0000\u0000\u0000\u08ec\u08ed\u0005q\u0000\u0000"+
		"\u08ed\u08ee\u0005u\u0000\u0000\u08ee\u08ef\u0003\u0192\u00c9\u0000\u08ef"+
		"\u08f3\u0005\u014a\u0000\u0000\u08f0\u08f2\u0003\u0194\u00ca\u0000\u08f1"+
		"\u08f0\u0001\u0000\u0000\u0000\u08f2\u08f5\u0001\u0000\u0000\u0000\u08f3"+
		"\u08f1\u0001\u0000\u0000\u0000\u08f3\u08f4\u0001\u0000\u0000\u0000\u08f4"+
		"\u08f6\u0001\u0000\u0000\u0000\u08f5\u08f3\u0001\u0000\u0000\u0000\u08f6"+
		"\u08f7\u0005\u014b\u0000\u0000\u08f7\u0107\u0001\u0000\u0000\u0000\u08f8"+
		"\u08f9\u0005q\u0000\u0000\u08f9\u08fa\u0005v\u0000\u0000\u08fa\u08fb\u0003"+
		"\u0192\u00c9\u0000\u08fb\u08ff\u0005\u014a\u0000\u0000\u08fc\u08fe\u0003"+
		"\u0194\u00ca\u0000\u08fd\u08fc\u0001\u0000\u0000\u0000\u08fe\u0901\u0001"+
		"\u0000\u0000\u0000\u08ff\u08fd\u0001\u0000\u0000\u0000\u08ff\u0900\u0001"+
		"\u0000\u0000\u0000\u0900\u0902\u0001\u0000\u0000\u0000\u0901\u08ff\u0001"+
		"\u0000\u0000\u0000\u0902\u0903\u0005\u014b\u0000\u0000\u0903\u0109\u0001"+
		"\u0000\u0000\u0000\u0904\u0905\u0005q\u0000\u0000\u0905\u0906\u0005w\u0000"+
		"\u0000\u0906\u0907\u0003\u0192\u00c9\u0000\u0907\u090b\u0005\u014a\u0000"+
		"\u0000\u0908\u090a\u0003\u0194\u00ca\u0000\u0909\u0908\u0001\u0000\u0000"+
		"\u0000\u090a\u090d\u0001\u0000\u0000\u0000\u090b\u0909\u0001\u0000\u0000"+
		"\u0000\u090b\u090c\u0001\u0000\u0000\u0000\u090c\u090e\u0001\u0000\u0000"+
		"\u0000\u090d\u090b\u0001\u0000\u0000\u0000\u090e\u090f\u0005\u014b\u0000"+
		"\u0000\u090f\u010b\u0001\u0000\u0000\u0000\u0910\u0911\u0005q\u0000\u0000"+
		"\u0911\u0912\u0005x\u0000\u0000\u0912\u0913\u0003\u0192\u00c9\u0000\u0913"+
		"\u0917\u0005\u014a\u0000\u0000\u0914\u0916\u0003\u0194\u00ca\u0000\u0915"+
		"\u0914\u0001\u0000\u0000\u0000\u0916\u0919\u0001\u0000\u0000\u0000\u0917"+
		"\u0915\u0001\u0000\u0000\u0000\u0917\u0918\u0001\u0000\u0000\u0000\u0918"+
		"\u091a\u0001\u0000\u0000\u0000\u0919\u0917\u0001\u0000\u0000\u0000\u091a"+
		"\u091b\u0005\u014b\u0000\u0000\u091b\u010d\u0001\u0000\u0000\u0000\u091c"+
		"\u091d\u0005q\u0000\u0000\u091d\u091e\u0005y\u0000\u0000\u091e\u091f\u0003"+
		"\u0192\u00c9\u0000\u091f\u0923\u0005\u014a\u0000\u0000\u0920\u0922\u0003"+
		"\u0194\u00ca\u0000\u0921\u0920\u0001\u0000\u0000\u0000\u0922\u0925\u0001"+
		"\u0000\u0000\u0000\u0923\u0921\u0001\u0000\u0000\u0000\u0923\u0924\u0001"+
		"\u0000\u0000\u0000\u0924\u0926\u0001\u0000\u0000\u0000\u0925\u0923\u0001"+
		"\u0000\u0000\u0000\u0926\u0927\u0005\u014b\u0000\u0000\u0927\u010f\u0001"+
		"\u0000\u0000\u0000\u0928\u0929\u0005q\u0000\u0000\u0929\u092a\u0005z\u0000"+
		"\u0000\u092a\u092b\u0003\u0192\u00c9\u0000\u092b\u092f\u0005\u014a\u0000"+
		"\u0000\u092c\u092e\u0003\u0194\u00ca\u0000\u092d\u092c\u0001\u0000\u0000"+
		"\u0000\u092e\u0931\u0001\u0000\u0000\u0000\u092f\u092d\u0001\u0000\u0000"+
		"\u0000\u092f\u0930\u0001\u0000\u0000\u0000\u0930\u0932\u0001\u0000\u0000"+
		"\u0000\u0931\u092f\u0001\u0000\u0000\u0000\u0932\u0933\u0005\u014b\u0000"+
		"\u0000\u0933\u0111\u0001\u0000\u0000\u0000\u0934\u0935\u0005q\u0000\u0000"+
		"\u0935\u0936\u0005{\u0000\u0000\u0936\u0937\u0003\u0192\u00c9\u0000\u0937"+
		"\u093b\u0005\u014a\u0000\u0000\u0938\u093a\u0003\u0194\u00ca\u0000\u0939"+
		"\u0938\u0001\u0000\u0000\u0000\u093a\u093d\u0001\u0000\u0000\u0000\u093b"+
		"\u0939\u0001\u0000\u0000\u0000\u093b\u093c\u0001\u0000\u0000\u0000\u093c"+
		"\u093e\u0001\u0000\u0000\u0000\u093d\u093b\u0001\u0000\u0000\u0000\u093e"+
		"\u093f\u0005\u014b\u0000\u0000\u093f\u0113\u0001\u0000\u0000\u0000\u0940"+
		"\u0941\u0005q\u0000\u0000\u0941\u0942\u0005|\u0000\u0000\u0942\u0943\u0003"+
		"\u0192\u00c9\u0000\u0943\u0947\u0005\u014a\u0000\u0000\u0944\u0946\u0003"+
		"\u0194\u00ca\u0000\u0945\u0944\u0001\u0000\u0000\u0000\u0946\u0949\u0001"+
		"\u0000\u0000\u0000\u0947\u0945\u0001\u0000\u0000\u0000\u0947\u0948\u0001"+
		"\u0000\u0000\u0000\u0948\u094a\u0001\u0000\u0000\u0000\u0949\u0947\u0001"+
		"\u0000\u0000\u0000\u094a\u094b\u0005\u014b\u0000\u0000\u094b\u0115\u0001"+
		"\u0000\u0000\u0000\u094c\u094d\u0005q\u0000\u0000\u094d\u094e\u0005}\u0000"+
		"\u0000\u094e\u094f\u0003\u0192\u00c9\u0000\u094f\u0953\u0005\u014a\u0000"+
		"\u0000\u0950\u0952\u0003\u0194\u00ca\u0000\u0951\u0950\u0001\u0000\u0000"+
		"\u0000\u0952\u0955\u0001\u0000\u0000\u0000\u0953\u0951\u0001\u0000\u0000"+
		"\u0000\u0953\u0954\u0001\u0000\u0000\u0000\u0954\u0956\u0001\u0000\u0000"+
		"\u0000\u0955\u0953\u0001\u0000\u0000\u0000\u0956\u0957\u0005\u014b\u0000"+
		"\u0000\u0957\u0117\u0001\u0000\u0000\u0000\u0958\u0959\u0005~\u0000\u0000"+
		"\u0959\u095a\u0005\u007f\u0000\u0000\u095a\u095b\u0003\u0192\u00c9\u0000"+
		"\u095b\u095f\u0005\u014a\u0000\u0000\u095c\u095e\u0003\u0194\u00ca\u0000"+
		"\u095d\u095c\u0001\u0000\u0000\u0000\u095e\u0961\u0001\u0000\u0000\u0000"+
		"\u095f\u095d\u0001\u0000\u0000\u0000\u095f\u0960\u0001\u0000\u0000\u0000"+
		"\u0960\u0962\u0001\u0000\u0000\u0000\u0961\u095f\u0001\u0000\u0000\u0000"+
		"\u0962\u0963\u0005\u014b\u0000\u0000\u0963\u0119\u0001\u0000\u0000\u0000"+
		"\u0964\u0965\u0005\u0080\u0000\u0000\u0965\u0966\u0003\u0192\u00c9\u0000"+
		"\u0966\u011b\u0001\u0000\u0000\u0000\u0967\u0968\u0005\u0081\u0000\u0000"+
		"\u0968\u0969\u0005\u0082\u0000\u0000\u0969\u096a\u0003\u0192\u00c9\u0000"+
		"\u096a\u096e\u0005\u014a\u0000\u0000\u096b\u096d\u0003\u0194\u00ca\u0000"+
		"\u096c\u096b\u0001\u0000\u0000\u0000\u096d\u0970\u0001\u0000\u0000\u0000"+
		"\u096e\u096c\u0001\u0000\u0000\u0000\u096e\u096f\u0001\u0000\u0000\u0000"+
		"\u096f\u0971\u0001\u0000\u0000\u0000\u0970\u096e\u0001\u0000\u0000\u0000"+
		"\u0971\u0972\u0005\u014b\u0000\u0000\u0972\u011d\u0001\u0000\u0000\u0000"+
		"\u0973\u0974\u0005\u0083\u0000\u0000\u0974\u0975\u0005\u0084\u0000\u0000"+
		"\u0975\u0976\u0003\u0192\u00c9\u0000\u0976\u097a\u0005\u014a\u0000\u0000"+
		"\u0977\u0979\u0003\u0194\u00ca\u0000\u0978\u0977\u0001\u0000\u0000\u0000"+
		"\u0979\u097c\u0001\u0000\u0000\u0000\u097a\u0978\u0001\u0000\u0000\u0000"+
		"\u097a\u097b\u0001\u0000\u0000\u0000\u097b\u097d\u0001\u0000\u0000\u0000"+
		"\u097c\u097a\u0001\u0000\u0000\u0000\u097d\u097e\u0005\u014b\u0000\u0000"+
		"\u097e\u011f\u0001\u0000\u0000\u0000\u097f\u0980\u0005\u011c\u0000\u0000"+
		"\u0980\u0981\u0005\u0085\u0000\u0000\u0981\u0982\u0003\u0192\u00c9\u0000"+
		"\u0982\u0986\u0005\u014a\u0000\u0000\u0983\u0985\u0003\u0194\u00ca\u0000"+
		"\u0984\u0983\u0001\u0000\u0000\u0000\u0985\u0988\u0001\u0000\u0000\u0000"+
		"\u0986\u0984\u0001\u0000\u0000\u0000\u0986\u0987\u0001\u0000\u0000\u0000"+
		"\u0987\u0989\u0001\u0000\u0000\u0000\u0988\u0986\u0001\u0000\u0000\u0000"+
		"\u0989\u098a\u0005\u014b\u0000\u0000\u098a\u0121\u0001\u0000\u0000\u0000"+
		"\u098b\u098c\u0005\u0086\u0000\u0000\u098c\u098d\u0005\u0087\u0000\u0000"+
		"\u098d\u098e\u0003\u0192\u00c9\u0000\u098e\u0992\u0005\u014a\u0000\u0000"+
		"\u098f\u0991\u0003\u0194\u00ca\u0000\u0990\u098f\u0001\u0000\u0000\u0000"+
		"\u0991\u0994\u0001\u0000\u0000\u0000\u0992\u0990\u0001\u0000\u0000\u0000"+
		"\u0992\u0993\u0001\u0000\u0000\u0000\u0993\u0995\u0001\u0000\u0000\u0000"+
		"\u0994\u0992\u0001\u0000\u0000\u0000\u0995\u0996\u0005\u014b\u0000\u0000"+
		"\u0996\u0123\u0001\u0000\u0000\u0000\u0997\u0998\u0005\u0088\u0000\u0000"+
		"\u0998\u0999\u0005\u0089\u0000\u0000\u0999\u099a\u0003\u0192\u00c9\u0000"+
		"\u099a\u099e\u0005\u014a\u0000\u0000\u099b\u099d\u0003\u0194\u00ca\u0000"+
		"\u099c\u099b\u0001\u0000\u0000\u0000\u099d\u09a0\u0001\u0000\u0000\u0000"+
		"\u099e\u099c\u0001\u0000\u0000\u0000\u099e\u099f\u0001\u0000\u0000\u0000"+
		"\u099f\u09a1\u0001\u0000\u0000\u0000\u09a0\u099e\u0001\u0000\u0000\u0000"+
		"\u09a1\u09a2\u0005\u014b\u0000\u0000\u09a2\u0125\u0001\u0000\u0000\u0000"+
		"\u09a3\u09a4\u0005\u008a\u0000\u0000\u09a4\u09a5\u0003\u0192\u00c9\u0000"+
		"\u09a5\u09a9\u0005\u014a\u0000\u0000\u09a6\u09a8\u0003\u0194\u00ca\u0000"+
		"\u09a7\u09a6\u0001\u0000\u0000\u0000\u09a8\u09ab\u0001\u0000\u0000\u0000"+
		"\u09a9\u09a7\u0001\u0000\u0000\u0000\u09a9\u09aa\u0001\u0000\u0000\u0000"+
		"\u09aa\u09ac\u0001\u0000\u0000\u0000\u09ab\u09a9\u0001\u0000\u0000\u0000"+
		"\u09ac\u09ad\u0005\u014b\u0000\u0000\u09ad\u0127\u0001\u0000\u0000\u0000"+
		"\u09ae\u09af\u0005\u008b\u0000\u0000\u09af\u09b0\u0005\u008c\u0000\u0000"+
		"\u09b0\u09b1\u0003\u0192\u00c9\u0000\u09b1\u09b5\u0005\u014a\u0000\u0000"+
		"\u09b2\u09b4\u0003\u0194\u00ca\u0000\u09b3\u09b2\u0001\u0000\u0000\u0000"+
		"\u09b4\u09b7\u0001\u0000\u0000\u0000\u09b5\u09b3\u0001\u0000\u0000\u0000"+
		"\u09b5\u09b6\u0001\u0000\u0000\u0000\u09b6\u09b8\u0001\u0000\u0000\u0000"+
		"\u09b7\u09b5\u0001\u0000\u0000\u0000\u09b8\u09b9\u0005\u014b\u0000\u0000"+
		"\u09b9\u0129\u0001\u0000\u0000\u0000\u09ba\u09bb\u0005\u008d\u0000\u0000"+
		"\u09bb\u09bc\u0005\u008e\u0000\u0000\u09bc\u09bd\u0003\u0192\u00c9\u0000"+
		"\u09bd\u09c1\u0005\u014a\u0000\u0000\u09be\u09c0\u0003\u0194\u00ca\u0000"+
		"\u09bf\u09be\u0001\u0000\u0000\u0000\u09c0\u09c3\u0001\u0000\u0000\u0000"+
		"\u09c1\u09bf\u0001\u0000\u0000\u0000\u09c1\u09c2\u0001\u0000\u0000\u0000"+
		"\u09c2\u09c4\u0001\u0000\u0000\u0000\u09c3\u09c1\u0001\u0000\u0000\u0000"+
		"\u09c4\u09c5\u0005\u014b\u0000\u0000\u09c5\u012b\u0001\u0000\u0000\u0000"+
		"\u09c6\u09c7\u0005\u008f\u0000\u0000\u09c7\u09c8\u0003\u0192\u00c9\u0000"+
		"\u09c8\u09cc\u0005\u014a\u0000\u0000\u09c9\u09cb\u0003\u0194\u00ca\u0000"+
		"\u09ca\u09c9\u0001\u0000\u0000\u0000\u09cb\u09ce\u0001\u0000\u0000\u0000"+
		"\u09cc\u09ca\u0001\u0000\u0000\u0000\u09cc\u09cd\u0001\u0000\u0000\u0000"+
		"\u09cd\u09cf\u0001\u0000\u0000\u0000\u09ce\u09cc\u0001\u0000\u0000\u0000"+
		"\u09cf\u09d0\u0005\u014b\u0000\u0000\u09d0\u012d\u0001\u0000\u0000\u0000"+
		"\u09d1\u09d2\u0005\u0090\u0000\u0000\u09d2\u09d3\u0003\u0192\u00c9\u0000"+
		"\u09d3\u09d4\u0005\u0100\u0000\u0000\u09d4\u09d5\u0003\u008cF\u0000\u09d5"+
		"\u09d9\u0005\u014a\u0000\u0000\u09d6\u09d8\u0003\u0194\u00ca\u0000\u09d7"+
		"\u09d6\u0001\u0000\u0000\u0000\u09d8\u09db\u0001\u0000\u0000\u0000\u09d9"+
		"\u09d7\u0001\u0000\u0000\u0000\u09d9\u09da\u0001\u0000\u0000\u0000\u09da"+
		"\u09dc\u0001\u0000\u0000\u0000\u09db\u09d9\u0001\u0000\u0000\u0000\u09dc"+
		"\u09dd\u0005\u014b\u0000\u0000\u09dd\u012f\u0001\u0000\u0000\u0000\u09de"+
		"\u09df\u00058\u0000\u0000\u09df\u09e0\u0003\u0192\u00c9\u0000\u09e0\u09e4"+
		"\u0005\u014a\u0000\u0000\u09e1\u09e3\u0003\u0194\u00ca\u0000\u09e2\u09e1"+
		"\u0001\u0000\u0000\u0000\u09e3\u09e6\u0001\u0000\u0000\u0000\u09e4\u09e2"+
		"\u0001\u0000\u0000\u0000\u09e4\u09e5\u0001\u0000\u0000\u0000\u09e5\u09e7"+
		"\u0001\u0000\u0000\u0000\u09e6\u09e4\u0001\u0000\u0000\u0000\u09e7\u09e8"+
		"\u0005\u014b\u0000\u0000\u09e8\u0131\u0001\u0000\u0000\u0000\u09e9\u09ea"+
		"\u0005\u0091\u0000\u0000\u09ea\u09eb\u0003\u0192\u00c9\u0000\u09eb\u09ef"+
		"\u0005\u014a\u0000\u0000\u09ec\u09ee\u0003\u0194\u00ca\u0000\u09ed\u09ec"+
		"\u0001\u0000\u0000\u0000\u09ee\u09f1\u0001\u0000\u0000\u0000\u09ef\u09ed"+
		"\u0001\u0000\u0000\u0000\u09ef\u09f0\u0001\u0000\u0000\u0000\u09f0\u09f2"+
		"\u0001\u0000\u0000\u0000\u09f1\u09ef\u0001\u0000\u0000\u0000\u09f2\u09f3"+
		"\u0005\u014b\u0000\u0000\u09f3\u0133\u0001\u0000\u0000\u0000\u09f4\u09f5"+
		"\u0005\u0092\u0000\u0000\u09f5\u09f6\u0003\u0192\u00c9\u0000\u09f6\u09fa"+
		"\u0005\u014a\u0000\u0000\u09f7\u09f9\u0003\u0194\u00ca\u0000\u09f8\u09f7"+
		"\u0001\u0000\u0000\u0000\u09f9\u09fc\u0001\u0000\u0000\u0000\u09fa\u09f8"+
		"\u0001\u0000\u0000\u0000\u09fa\u09fb\u0001\u0000\u0000\u0000\u09fb\u09fd"+
		"\u0001\u0000\u0000\u0000\u09fc\u09fa\u0001\u0000\u0000\u0000\u09fd\u09fe"+
		"\u0005\u014b\u0000\u0000\u09fe\u0135\u0001\u0000\u0000\u0000\u09ff\u0a00"+
		"\u0005\u0093\u0000\u0000\u0a00\u0a01\u0005\u0094\u0000\u0000\u0a01\u0a02"+
		"\u0003\u0192\u00c9\u0000\u0a02\u0a06\u0005\u014a\u0000\u0000\u0a03\u0a05"+
		"\u0003\u0194\u00ca\u0000\u0a04\u0a03\u0001\u0000\u0000\u0000\u0a05\u0a08"+
		"\u0001\u0000\u0000\u0000\u0a06\u0a04\u0001\u0000\u0000\u0000\u0a06\u0a07"+
		"\u0001\u0000\u0000\u0000\u0a07\u0a09\u0001\u0000\u0000\u0000\u0a08\u0a06"+
		"\u0001\u0000\u0000\u0000\u0a09\u0a0a\u0005\u014b\u0000\u0000\u0a0a\u0137"+
		"\u0001\u0000\u0000\u0000\u0a0b\u0a0c\u0005\u0093\u0000\u0000\u0a0c\u0a0d"+
		"\u0005\u0095\u0000\u0000\u0a0d\u0a0e\u0003\u0192\u00c9\u0000\u0a0e\u0a12"+
		"\u0005\u014a\u0000\u0000\u0a0f\u0a11\u0003\u0194\u00ca\u0000\u0a10\u0a0f"+
		"\u0001\u0000\u0000\u0000\u0a11\u0a14\u0001\u0000\u0000\u0000\u0a12\u0a10"+
		"\u0001\u0000\u0000\u0000\u0a12\u0a13\u0001\u0000\u0000\u0000\u0a13\u0a15"+
		"\u0001\u0000\u0000\u0000\u0a14\u0a12\u0001\u0000\u0000\u0000\u0a15\u0a16"+
		"\u0005\u014b\u0000\u0000\u0a16\u0139\u0001\u0000\u0000\u0000\u0a17\u0a18"+
		"\u0005\u0093\u0000\u0000\u0a18\u0a19\u0005\u0096\u0000\u0000\u0a19\u0a1a"+
		"\u0003\u0192\u00c9\u0000\u0a1a\u0a1e\u0005\u014a\u0000\u0000\u0a1b\u0a1d"+
		"\u0003\u0194\u00ca\u0000\u0a1c\u0a1b\u0001\u0000\u0000\u0000\u0a1d\u0a20"+
		"\u0001\u0000\u0000\u0000\u0a1e\u0a1c\u0001\u0000\u0000\u0000\u0a1e\u0a1f"+
		"\u0001\u0000\u0000\u0000\u0a1f\u0a21\u0001\u0000\u0000\u0000\u0a20\u0a1e"+
		"\u0001\u0000\u0000\u0000\u0a21\u0a22\u0005\u014b\u0000\u0000\u0a22\u013b"+
		"\u0001\u0000\u0000\u0000\u0a23\u0a24\u0005\u0093\u0000\u0000\u0a24\u0a25"+
		"\u0005\u0097\u0000\u0000\u0a25\u0a26\u0003\u0192\u00c9\u0000\u0a26\u0a2a"+
		"\u0005\u014a\u0000\u0000\u0a27\u0a29\u0003\u0194\u00ca\u0000\u0a28\u0a27"+
		"\u0001\u0000\u0000\u0000\u0a29\u0a2c\u0001\u0000\u0000\u0000\u0a2a\u0a28"+
		"\u0001\u0000\u0000\u0000\u0a2a\u0a2b\u0001\u0000\u0000\u0000\u0a2b\u0a2d"+
		"\u0001\u0000\u0000\u0000\u0a2c\u0a2a\u0001\u0000\u0000\u0000\u0a2d\u0a2e"+
		"\u0005\u014b\u0000\u0000\u0a2e\u013d\u0001\u0000\u0000\u0000\u0a2f\u0a30"+
		"\u0005\u0098\u0000\u0000\u0a30\u0a31\u0003\u0192\u00c9\u0000\u0a31\u0a35"+
		"\u0005\u014a\u0000\u0000\u0a32\u0a34\u0003\u0194\u00ca\u0000\u0a33\u0a32"+
		"\u0001\u0000\u0000\u0000\u0a34\u0a37\u0001\u0000\u0000\u0000\u0a35\u0a33"+
		"\u0001\u0000\u0000\u0000\u0a35\u0a36\u0001\u0000\u0000\u0000\u0a36\u0a38"+
		"\u0001\u0000\u0000\u0000\u0a37\u0a35\u0001\u0000\u0000\u0000\u0a38\u0a39"+
		"\u0005\u014b\u0000\u0000\u0a39\u013f\u0001\u0000\u0000\u0000\u0a3a\u0a3b"+
		"\u0005\u0099\u0000\u0000\u0a3b\u0a3c\u0003\u0192\u00c9\u0000\u0a3c\u0a40"+
		"\u0005\u014a\u0000\u0000\u0a3d\u0a3f\u0003\u0194\u00ca\u0000\u0a3e\u0a3d"+
		"\u0001\u0000\u0000\u0000\u0a3f\u0a42\u0001\u0000\u0000\u0000\u0a40\u0a3e"+
		"\u0001\u0000\u0000\u0000\u0a40\u0a41\u0001\u0000\u0000\u0000\u0a41\u0a43"+
		"\u0001\u0000\u0000\u0000\u0a42\u0a40\u0001\u0000\u0000\u0000\u0a43\u0a44"+
		"\u0005\u014b\u0000\u0000\u0a44\u0141\u0001\u0000\u0000\u0000\u0a45\u0a46"+
		"\u0005\u009a\u0000\u0000\u0a46\u0a47\u0003\u0192\u00c9\u0000\u0a47\u0a4b"+
		"\u0005\u014a\u0000\u0000\u0a48\u0a4a\u0003\u0194\u00ca\u0000\u0a49\u0a48"+
		"\u0001\u0000\u0000\u0000\u0a4a\u0a4d\u0001\u0000\u0000\u0000\u0a4b\u0a49"+
		"\u0001\u0000\u0000\u0000\u0a4b\u0a4c\u0001\u0000\u0000\u0000\u0a4c\u0a4e"+
		"\u0001\u0000\u0000\u0000\u0a4d\u0a4b\u0001\u0000\u0000\u0000\u0a4e\u0a4f"+
		"\u0005\u014b\u0000\u0000\u0a4f\u0143\u0001\u0000\u0000\u0000\u0a50\u0a51"+
		"\u0005\u009b\u0000\u0000\u0a51\u0a52\u0003\u0192\u00c9\u0000\u0a52\u0a56"+
		"\u0005\u014a\u0000\u0000\u0a53\u0a55\u0003\u0194\u00ca\u0000\u0a54\u0a53"+
		"\u0001\u0000\u0000\u0000\u0a55\u0a58\u0001\u0000\u0000\u0000\u0a56\u0a54"+
		"\u0001\u0000\u0000\u0000\u0a56\u0a57\u0001\u0000\u0000\u0000\u0a57\u0a59"+
		"\u0001\u0000\u0000\u0000\u0a58\u0a56\u0001\u0000\u0000\u0000\u0a59\u0a5a"+
		"\u0005\u014b\u0000\u0000\u0a5a\u0145\u0001\u0000\u0000\u0000\u0a5b\u0a5c"+
		"\u0005\u009c\u0000\u0000\u0a5c\u0a5d\u0005\u009d\u0000\u0000\u0a5d\u0a5e"+
		"\u0003\u0192\u00c9\u0000\u0a5e\u0a62\u0005\u014a\u0000\u0000\u0a5f\u0a61"+
		"\u0003\u0194\u00ca\u0000\u0a60\u0a5f\u0001\u0000\u0000\u0000\u0a61\u0a64"+
		"\u0001\u0000\u0000\u0000\u0a62\u0a60\u0001\u0000\u0000\u0000\u0a62\u0a63"+
		"\u0001\u0000\u0000\u0000\u0a63\u0a65\u0001\u0000\u0000\u0000\u0a64\u0a62"+
		"\u0001\u0000\u0000\u0000\u0a65\u0a66\u0005\u014b\u0000\u0000\u0a66\u0147"+
		"\u0001\u0000\u0000\u0000\u0a67\u0a68\u0005\u009e\u0000\u0000\u0a68\u0a69"+
		"\u0005\u009c\u0000\u0000\u0a69\u0a6a\u0005\u009f\u0000\u0000\u0a6a\u0a6b"+
		"\u0003\u0192\u00c9\u0000\u0a6b\u0a6f\u0005\u014a\u0000\u0000\u0a6c\u0a6e"+
		"\u0003\u0194\u00ca\u0000\u0a6d\u0a6c\u0001\u0000\u0000\u0000\u0a6e\u0a71"+
		"\u0001\u0000\u0000\u0000\u0a6f\u0a6d\u0001\u0000\u0000\u0000\u0a6f\u0a70"+
		"\u0001\u0000\u0000\u0000\u0a70\u0a72\u0001\u0000\u0000\u0000\u0a71\u0a6f"+
		"\u0001\u0000\u0000\u0000\u0a72\u0a73\u0005\u014b\u0000\u0000\u0a73\u0149"+
		"\u0001\u0000\u0000\u0000\u0a74\u0a75\u0005\u00a0\u0000\u0000\u0a75\u0a76"+
		"\u0005\u00a1\u0000\u0000\u0a76\u0a77\u0003\u0192\u00c9\u0000\u0a77\u0a7b"+
		"\u0005\u014a\u0000\u0000\u0a78\u0a7a\u0003\u0194\u00ca\u0000\u0a79\u0a78"+
		"\u0001\u0000\u0000\u0000\u0a7a\u0a7d\u0001\u0000\u0000\u0000\u0a7b\u0a79"+
		"\u0001\u0000\u0000\u0000\u0a7b\u0a7c\u0001\u0000\u0000\u0000\u0a7c\u0a7e"+
		"\u0001\u0000\u0000\u0000\u0a7d\u0a7b\u0001\u0000\u0000\u0000\u0a7e\u0a7f"+
		"\u0005\u014b\u0000\u0000\u0a7f\u014b\u0001\u0000\u0000\u0000\u0a80\u0a81"+
		"\u0005\u00a2\u0000\u0000\u0a81\u0a82\u0003\u0192\u00c9\u0000\u0a82\u0a86"+
		"\u0005\u014a\u0000\u0000\u0a83\u0a85\u0003\u0194\u00ca\u0000\u0a84\u0a83"+
		"\u0001\u0000\u0000\u0000\u0a85\u0a88\u0001\u0000\u0000\u0000\u0a86\u0a84"+
		"\u0001\u0000\u0000\u0000\u0a86\u0a87\u0001\u0000\u0000\u0000\u0a87\u0a89"+
		"\u0001\u0000\u0000\u0000\u0a88\u0a86\u0001\u0000\u0000\u0000\u0a89\u0a8a"+
		"\u0005\u014b\u0000\u0000\u0a8a\u014d\u0001\u0000\u0000\u0000\u0a8b\u0a8c"+
		"\u0005\u00a3\u0000\u0000\u0a8c\u0a8d\u0003\u0192\u00c9\u0000\u0a8d\u0a91"+
		"\u0005\u014a\u0000\u0000\u0a8e\u0a90\u0003\u0194\u00ca\u0000\u0a8f\u0a8e"+
		"\u0001\u0000\u0000\u0000\u0a90\u0a93\u0001\u0000\u0000\u0000\u0a91\u0a8f"+
		"\u0001\u0000\u0000\u0000\u0a91\u0a92\u0001\u0000\u0000\u0000\u0a92\u0a94"+
		"\u0001\u0000\u0000\u0000\u0a93\u0a91\u0001\u0000\u0000\u0000\u0a94\u0a95"+
		"\u0005\u014b\u0000\u0000\u0a95\u014f\u0001\u0000\u0000\u0000\u0a96\u0a97"+
		"\u0005\u00a4\u0000\u0000\u0a97\u0a98\u0003\u0192\u00c9\u0000\u0a98\u0a9c"+
		"\u0005\u014a\u0000\u0000\u0a99\u0a9b\u0003\u0194\u00ca\u0000\u0a9a\u0a99"+
		"\u0001\u0000\u0000\u0000\u0a9b\u0a9e\u0001\u0000\u0000\u0000\u0a9c\u0a9a"+
		"\u0001\u0000\u0000\u0000\u0a9c\u0a9d\u0001\u0000\u0000\u0000\u0a9d\u0a9f"+
		"\u0001\u0000\u0000\u0000\u0a9e\u0a9c\u0001\u0000\u0000\u0000\u0a9f\u0aa0"+
		"\u0005\u014b\u0000\u0000\u0aa0\u0151\u0001\u0000\u0000\u0000\u0aa1\u0aa2"+
		"\u0005\u00a5\u0000\u0000\u0aa2\u0aa3\u0003\u0192\u00c9\u0000\u0aa3\u0aa7"+
		"\u0005\u014a\u0000\u0000\u0aa4\u0aa6\u0003\u0194\u00ca\u0000\u0aa5\u0aa4"+
		"\u0001\u0000\u0000\u0000\u0aa6\u0aa9\u0001\u0000\u0000\u0000\u0aa7\u0aa5"+
		"\u0001\u0000\u0000\u0000\u0aa7\u0aa8\u0001\u0000\u0000\u0000\u0aa8\u0aaa"+
		"\u0001\u0000\u0000\u0000\u0aa9\u0aa7\u0001\u0000\u0000\u0000\u0aaa\u0aab"+
		"\u0005\u014b\u0000\u0000\u0aab\u0153\u0001\u0000\u0000\u0000\u0aac\u0aad"+
		"\u0005\u00a6\u0000\u0000\u0aad\u0aae\u0003\u0192\u00c9\u0000\u0aae\u0ab2"+
		"\u0005\u014a\u0000\u0000\u0aaf\u0ab1\u0003\u0194\u00ca\u0000\u0ab0\u0aaf"+
		"\u0001\u0000\u0000\u0000\u0ab1\u0ab4\u0001\u0000\u0000\u0000\u0ab2\u0ab0"+
		"\u0001\u0000\u0000\u0000\u0ab2\u0ab3\u0001\u0000\u0000\u0000\u0ab3\u0ab5"+
		"\u0001\u0000\u0000\u0000\u0ab4\u0ab2\u0001\u0000\u0000\u0000\u0ab5\u0ab6"+
		"\u0005\u014b\u0000\u0000\u0ab6\u0155\u0001\u0000\u0000\u0000\u0ab7\u0ab8"+
		"\u0005\u00a7\u0000\u0000\u0ab8\u0ab9\u0003\u0192\u00c9\u0000\u0ab9\u0abd"+
		"\u0005\u014a\u0000\u0000\u0aba\u0abc\u0003\u0194\u00ca\u0000\u0abb\u0aba"+
		"\u0001\u0000\u0000\u0000\u0abc\u0abf\u0001\u0000\u0000\u0000\u0abd\u0abb"+
		"\u0001\u0000\u0000\u0000\u0abd\u0abe\u0001\u0000\u0000\u0000\u0abe\u0ac0"+
		"\u0001\u0000\u0000\u0000\u0abf\u0abd\u0001\u0000\u0000\u0000\u0ac0\u0ac1"+
		"\u0005\u014b\u0000\u0000\u0ac1\u0157\u0001\u0000\u0000\u0000\u0ac2\u0ac3"+
		"\u0005\u00a8\u0000\u0000\u0ac3\u0ac4\u0003\u0192\u00c9\u0000\u0ac4\u0ac8"+
		"\u0005\u014a\u0000\u0000\u0ac5\u0ac7\u0003\u0194\u00ca\u0000\u0ac6\u0ac5"+
		"\u0001\u0000\u0000\u0000\u0ac7\u0aca\u0001\u0000\u0000\u0000\u0ac8\u0ac6"+
		"\u0001\u0000\u0000\u0000\u0ac8\u0ac9\u0001\u0000\u0000\u0000\u0ac9\u0acb"+
		"\u0001\u0000\u0000\u0000\u0aca\u0ac8\u0001\u0000\u0000\u0000\u0acb\u0acc"+
		"\u0005\u014b\u0000\u0000\u0acc\u0159\u0001\u0000\u0000\u0000\u0acd\u0ace"+
		"\u0005\u00a9\u0000\u0000\u0ace\u0acf\u0003\u0192\u00c9\u0000\u0acf\u0ad3"+
		"\u0005\u014a\u0000\u0000\u0ad0\u0ad2\u0003\u0194\u00ca\u0000\u0ad1\u0ad0"+
		"\u0001\u0000\u0000\u0000\u0ad2\u0ad5\u0001\u0000\u0000\u0000\u0ad3\u0ad1"+
		"\u0001\u0000\u0000\u0000\u0ad3\u0ad4\u0001\u0000\u0000\u0000\u0ad4\u0ad6"+
		"\u0001\u0000\u0000\u0000\u0ad5\u0ad3\u0001\u0000\u0000\u0000\u0ad6\u0ad7"+
		"\u0005\u014b\u0000\u0000\u0ad7\u015b\u0001\u0000\u0000\u0000\u0ad8\u0ad9"+
		"\u0005\u00aa\u0000\u0000\u0ad9\u0ada\u0005\u00ab\u0000\u0000\u0ada\u0adb"+
		"\u0003\u0192\u00c9\u0000\u0adb\u0adf\u0005\u014a\u0000\u0000\u0adc\u0ade"+
		"\u0003\u0194\u00ca\u0000\u0add\u0adc\u0001\u0000\u0000\u0000\u0ade\u0ae1"+
		"\u0001\u0000\u0000\u0000\u0adf\u0add\u0001\u0000\u0000\u0000\u0adf\u0ae0"+
		"\u0001\u0000\u0000\u0000\u0ae0\u0ae2\u0001\u0000\u0000\u0000\u0ae1\u0adf"+
		"\u0001\u0000\u0000\u0000\u0ae2\u0ae3\u0005\u014b\u0000\u0000\u0ae3\u015d"+
		"\u0001\u0000\u0000\u0000\u0ae4\u0ae5\u0005\u00ac\u0000\u0000\u0ae5\u0ae6"+
		"\u0003\u0192\u00c9\u0000\u0ae6\u0aea\u0005\u014a\u0000\u0000\u0ae7\u0ae9"+
		"\u0003\u0194\u00ca\u0000\u0ae8\u0ae7\u0001\u0000\u0000\u0000\u0ae9\u0aec"+
		"\u0001\u0000\u0000\u0000\u0aea\u0ae8\u0001\u0000\u0000\u0000\u0aea\u0aeb"+
		"\u0001\u0000\u0000\u0000\u0aeb\u0aed\u0001\u0000\u0000\u0000\u0aec\u0aea"+
		"\u0001\u0000\u0000\u0000\u0aed\u0aee\u0005\u014b\u0000\u0000\u0aee\u015f"+
		"\u0001\u0000\u0000\u0000\u0aef\u0af0\u0005\u00ad\u0000\u0000\u0af0\u0af1"+
		"\u0003\u0192\u00c9\u0000\u0af1\u0af5\u0005\u014a\u0000\u0000\u0af2\u0af4"+
		"\u0003\u0194\u00ca\u0000\u0af3\u0af2\u0001\u0000\u0000\u0000\u0af4\u0af7"+
		"\u0001\u0000\u0000\u0000\u0af5\u0af3\u0001\u0000\u0000\u0000\u0af5\u0af6"+
		"\u0001\u0000\u0000\u0000\u0af6\u0af8\u0001\u0000\u0000\u0000\u0af7\u0af5"+
		"\u0001\u0000\u0000\u0000\u0af8\u0af9\u0005\u014b\u0000\u0000\u0af9\u0161"+
		"\u0001\u0000\u0000\u0000\u0afa\u0afb\u0005\u00ae\u0000\u0000\u0afb\u0afc"+
		"\u0003\u0192\u00c9\u0000\u0afc\u0b00\u0005\u014a\u0000\u0000\u0afd\u0aff"+
		"\u0003\u0194\u00ca\u0000\u0afe\u0afd\u0001\u0000\u0000\u0000\u0aff\u0b02"+
		"\u0001\u0000\u0000\u0000\u0b00\u0afe\u0001\u0000\u0000\u0000\u0b00\u0b01"+
		"\u0001\u0000\u0000\u0000\u0b01\u0b03\u0001\u0000\u0000\u0000\u0b02\u0b00"+
		"\u0001\u0000\u0000\u0000\u0b03\u0b04\u0005\u014b\u0000\u0000\u0b04\u0163"+
		"\u0001\u0000\u0000\u0000\u0b05\u0b06\u0005\u00af\u0000\u0000\u0b06\u0b07"+
		"\u0003\u0192\u00c9\u0000\u0b07\u0b0b\u0005\u014a\u0000\u0000\u0b08\u0b0a"+
		"\u0003\u0194\u00ca\u0000\u0b09\u0b08\u0001\u0000\u0000\u0000\u0b0a\u0b0d"+
		"\u0001\u0000\u0000\u0000\u0b0b\u0b09\u0001\u0000\u0000\u0000\u0b0b\u0b0c"+
		"\u0001\u0000\u0000\u0000\u0b0c\u0b0e\u0001\u0000\u0000\u0000\u0b0d\u0b0b"+
		"\u0001\u0000\u0000\u0000\u0b0e\u0b0f\u0005\u014b\u0000\u0000\u0b0f\u0165"+
		"\u0001\u0000\u0000\u0000\u0b10\u0b11\u0005\u00b0\u0000\u0000\u0b11\u0b12"+
		"\u0003\u0192\u00c9\u0000\u0b12\u0b16\u0005\u014a\u0000\u0000\u0b13\u0b15"+
		"\u0003\u0194\u00ca\u0000\u0b14\u0b13\u0001\u0000\u0000\u0000\u0b15\u0b18"+
		"\u0001\u0000\u0000\u0000\u0b16\u0b14\u0001\u0000\u0000\u0000\u0b16\u0b17"+
		"\u0001\u0000\u0000\u0000\u0b17\u0b19\u0001\u0000\u0000\u0000\u0b18\u0b16"+
		"\u0001\u0000\u0000\u0000\u0b19\u0b1a\u0005\u014b\u0000\u0000\u0b1a\u0167"+
		"\u0001\u0000\u0000\u0000\u0b1b\u0b1c\u0005\u00b1\u0000\u0000\u0b1c\u0b1d"+
		"\u0003\u0192\u00c9\u0000\u0b1d\u0b21\u0005\u014a\u0000\u0000\u0b1e\u0b20"+
		"\u0003\u0194\u00ca\u0000\u0b1f\u0b1e\u0001\u0000\u0000\u0000\u0b20\u0b23"+
		"\u0001\u0000\u0000\u0000\u0b21\u0b1f\u0001\u0000\u0000\u0000\u0b21\u0b22"+
		"\u0001\u0000\u0000\u0000\u0b22\u0b24\u0001\u0000\u0000\u0000\u0b23\u0b21"+
		"\u0001\u0000\u0000\u0000\u0b24\u0b25\u0005\u014b\u0000\u0000\u0b25\u0169"+
		"\u0001\u0000\u0000\u0000\u0b26\u0b27\u0005\u00b2\u0000\u0000\u0b27\u0b28"+
		"\u0003\u0192\u00c9\u0000\u0b28\u0b2c\u0005\u014a\u0000\u0000\u0b29\u0b2b"+
		"\u0003\u0194\u00ca\u0000\u0b2a\u0b29\u0001\u0000\u0000\u0000\u0b2b\u0b2e"+
		"\u0001\u0000\u0000\u0000\u0b2c\u0b2a\u0001\u0000\u0000\u0000\u0b2c\u0b2d"+
		"\u0001\u0000\u0000\u0000\u0b2d\u0b2f\u0001\u0000\u0000\u0000\u0b2e\u0b2c"+
		"\u0001\u0000\u0000\u0000\u0b2f\u0b30\u0005\u014b\u0000\u0000\u0b30\u016b"+
		"\u0001\u0000\u0000\u0000\u0b31\u0b32\u0005\u00b3\u0000\u0000\u0b32\u0b33"+
		"\u0003\u0192\u00c9\u0000\u0b33\u0b37\u0005\u014a\u0000\u0000\u0b34\u0b36"+
		"\u0003\u0194\u00ca\u0000\u0b35\u0b34\u0001\u0000\u0000\u0000\u0b36\u0b39"+
		"\u0001\u0000\u0000\u0000\u0b37\u0b35\u0001\u0000\u0000\u0000\u0b37\u0b38"+
		"\u0001\u0000\u0000\u0000\u0b38\u0b3a\u0001\u0000\u0000\u0000\u0b39\u0b37"+
		"\u0001\u0000\u0000\u0000\u0b3a\u0b3b\u0005\u014b\u0000\u0000\u0b3b\u016d"+
		"\u0001\u0000\u0000\u0000\u0b3c\u0b3d\u0005\u00b4\u0000\u0000\u0b3d\u0b3e"+
		"\u0003\u0192\u00c9\u0000\u0b3e\u0b42\u0005\u014a\u0000\u0000\u0b3f\u0b41"+
		"\u0003\u0194\u00ca\u0000\u0b40\u0b3f\u0001\u0000\u0000\u0000\u0b41\u0b44"+
		"\u0001\u0000\u0000\u0000\u0b42\u0b40\u0001\u0000\u0000\u0000\u0b42\u0b43"+
		"\u0001\u0000\u0000\u0000\u0b43\u0b45\u0001\u0000\u0000\u0000\u0b44\u0b42"+
		"\u0001\u0000\u0000\u0000\u0b45\u0b46\u0005\u014b\u0000\u0000\u0b46\u016f"+
		"\u0001\u0000\u0000\u0000\u0b47\u0b48\u0005\u00b5\u0000\u0000\u0b48\u0b49"+
		"\u0003\u0192\u00c9\u0000\u0b49\u0b4d\u0005\u014a\u0000\u0000\u0b4a\u0b4c"+
		"\u0003\u0194\u00ca\u0000\u0b4b\u0b4a\u0001\u0000\u0000\u0000\u0b4c\u0b4f"+
		"\u0001\u0000\u0000\u0000\u0b4d\u0b4b\u0001\u0000\u0000\u0000\u0b4d\u0b4e"+
		"\u0001\u0000\u0000\u0000\u0b4e\u0b50\u0001\u0000\u0000\u0000\u0b4f\u0b4d"+
		"\u0001\u0000\u0000\u0000\u0b50\u0b51\u0005\u014b\u0000\u0000\u0b51\u0171"+
		"\u0001\u0000\u0000\u0000\u0b52\u0b53\u0005\u00b6\u0000\u0000\u0b53\u0b54"+
		"\u0003\u0192\u00c9\u0000\u0b54\u0b58\u0005\u014a\u0000\u0000\u0b55\u0b57"+
		"\u0003\u0194\u00ca\u0000\u0b56\u0b55\u0001\u0000\u0000\u0000\u0b57\u0b5a"+
		"\u0001\u0000\u0000\u0000\u0b58\u0b56\u0001\u0000\u0000\u0000\u0b58\u0b59"+
		"\u0001\u0000\u0000\u0000\u0b59\u0b5b\u0001\u0000\u0000\u0000\u0b5a\u0b58"+
		"\u0001\u0000\u0000\u0000\u0b5b\u0b5c\u0005\u014b\u0000\u0000\u0b5c\u0173"+
		"\u0001\u0000\u0000\u0000\u0b5d\u0b5e\u0005\u00b7\u0000\u0000\u0b5e\u0b5f"+
		"\u0003\u0192\u00c9\u0000\u0b5f\u0b63\u0005\u014a\u0000\u0000\u0b60\u0b62"+
		"\u0003\u0194\u00ca\u0000\u0b61\u0b60\u0001\u0000\u0000\u0000\u0b62\u0b65"+
		"\u0001\u0000\u0000\u0000\u0b63\u0b61\u0001\u0000\u0000\u0000\u0b63\u0b64"+
		"\u0001\u0000\u0000\u0000\u0b64\u0b66\u0001\u0000\u0000\u0000\u0b65\u0b63"+
		"\u0001\u0000\u0000\u0000\u0b66\u0b67\u0005\u014b\u0000\u0000\u0b67\u0175"+
		"\u0001\u0000\u0000\u0000\u0b68\u0b69\u0005\u00b8\u0000\u0000\u0b69\u0b6a"+
		"\u0003\u0192\u00c9\u0000\u0b6a\u0b6e\u0005\u014a\u0000\u0000\u0b6b\u0b6d"+
		"\u0003\u0194\u00ca\u0000\u0b6c\u0b6b\u0001\u0000\u0000\u0000\u0b6d\u0b70"+
		"\u0001\u0000\u0000\u0000\u0b6e\u0b6c\u0001\u0000\u0000\u0000\u0b6e\u0b6f"+
		"\u0001\u0000\u0000\u0000\u0b6f\u0b71\u0001\u0000\u0000\u0000\u0b70\u0b6e"+
		"\u0001\u0000\u0000\u0000\u0b71\u0b72\u0005\u014b\u0000\u0000\u0b72\u0177"+
		"\u0001\u0000\u0000\u0000\u0b73\u0b74\u0005\u00b9\u0000\u0000\u0b74\u0b75"+
		"\u0003\u0192\u00c9\u0000\u0b75\u0b79\u0005\u014a\u0000\u0000\u0b76\u0b78"+
		"\u0003\u0194\u00ca\u0000\u0b77\u0b76\u0001\u0000\u0000\u0000\u0b78\u0b7b"+
		"\u0001\u0000\u0000\u0000\u0b79\u0b77\u0001\u0000\u0000\u0000\u0b79\u0b7a"+
		"\u0001\u0000\u0000\u0000\u0b7a\u0b7c\u0001\u0000\u0000\u0000\u0b7b\u0b79"+
		"\u0001\u0000\u0000\u0000\u0b7c\u0b7d\u0005\u014b\u0000\u0000\u0b7d\u0179"+
		"\u0001\u0000\u0000\u0000\u0b7e\u0b7f\u0005\u00f9\u0000\u0000\u0b7f\u0b80"+
		"\u0005\u00ba\u0000\u0000\u0b80\u0b81\u0005\u010e\u0000\u0000\u0b81\u0b82"+
		"\u0003\u0192\u00c9\u0000\u0b82\u0b86\u0005\u014a\u0000\u0000\u0b83\u0b85"+
		"\u0003\u0194\u00ca\u0000\u0b84\u0b83\u0001\u0000\u0000\u0000\u0b85\u0b88"+
		"\u0001\u0000\u0000\u0000\u0b86\u0b84\u0001\u0000\u0000\u0000\u0b86\u0b87"+
		"\u0001\u0000\u0000\u0000\u0b87\u0b89\u0001\u0000\u0000\u0000\u0b88\u0b86"+
		"\u0001\u0000\u0000\u0000\u0b89\u0b8a\u0005\u014b\u0000\u0000\u0b8a\u017b"+
		"\u0001\u0000\u0000\u0000\u0b8b\u0b8c\u0005\u00bb\u0000\u0000\u0b8c\u0b8d"+
		"\u0003\u0192\u00c9\u0000\u0b8d\u0b91\u0005\u014a\u0000\u0000\u0b8e\u0b90"+
		"\u0003\u0194\u00ca\u0000\u0b8f\u0b8e\u0001\u0000\u0000\u0000\u0b90\u0b93"+
		"\u0001\u0000\u0000\u0000\u0b91\u0b8f\u0001\u0000\u0000\u0000\u0b91\u0b92"+
		"\u0001\u0000\u0000\u0000\u0b92\u0b94\u0001\u0000\u0000\u0000\u0b93\u0b91"+
		"\u0001\u0000\u0000\u0000\u0b94\u0b95\u0005\u014b\u0000\u0000\u0b95\u017d"+
		"\u0001\u0000\u0000\u0000\u0b96\u0b97\u0005\u0114\u0000\u0000\u0b97\u0b98"+
		"\u0003\u0192\u00c9\u0000\u0b98\u0b9c\u0005\u014a\u0000\u0000\u0b99\u0b9b"+
		"\u0003\u0194\u00ca\u0000\u0b9a\u0b99\u0001\u0000\u0000\u0000\u0b9b\u0b9e"+
		"\u0001\u0000\u0000\u0000\u0b9c\u0b9a\u0001\u0000\u0000\u0000\u0b9c\u0b9d"+
		"\u0001\u0000\u0000\u0000\u0b9d\u0b9f\u0001\u0000\u0000\u0000\u0b9e\u0b9c"+
		"\u0001\u0000\u0000\u0000\u0b9f\u0ba0\u0005\u014b\u0000\u0000\u0ba0\u017f"+
		"\u0001\u0000\u0000\u0000\u0ba1\u0ba2\u0005\u00bc\u0000\u0000\u0ba2\u0ba3"+
		"\u0003\u0192\u00c9\u0000\u0ba3\u0ba7\u0005\u014a\u0000\u0000\u0ba4\u0ba6"+
		"\u0003\u0194\u00ca\u0000\u0ba5\u0ba4\u0001\u0000\u0000\u0000\u0ba6\u0ba9"+
		"\u0001\u0000\u0000\u0000\u0ba7\u0ba5\u0001\u0000\u0000\u0000\u0ba7\u0ba8"+
		"\u0001\u0000\u0000\u0000\u0ba8\u0baa\u0001\u0000\u0000\u0000\u0ba9\u0ba7"+
		"\u0001\u0000\u0000\u0000\u0baa\u0bab\u0005\u014b\u0000\u0000\u0bab\u0181"+
		"\u0001\u0000\u0000\u0000\u0bac\u0bad\u0005\u00bd\u0000\u0000\u0bad\u0bae"+
		"\u0003\u0192\u00c9\u0000\u0bae\u0bb2\u0005\u014a\u0000\u0000\u0baf\u0bb1"+
		"\u0003\u0194\u00ca\u0000\u0bb0\u0baf\u0001\u0000\u0000\u0000\u0bb1\u0bb4"+
		"\u0001\u0000\u0000\u0000\u0bb2\u0bb0\u0001\u0000\u0000\u0000\u0bb2\u0bb3"+
		"\u0001\u0000\u0000\u0000\u0bb3\u0bb5\u0001\u0000\u0000\u0000\u0bb4\u0bb2"+
		"\u0001\u0000\u0000\u0000\u0bb5\u0bb6\u0005\u014b\u0000\u0000\u0bb6\u0183"+
		"\u0001\u0000\u0000\u0000\u0bb7\u0bb8\u0005\u00be\u0000\u0000\u0bb8\u0bb9"+
		"\u0003D\"\u0000\u0bb9\u0bba\u0005\u0150\u0000\u0000\u0bba\u0185\u0001"+
		"\u0000\u0000\u0000\u0bbb\u0bbc\u0005\u00bf\u0000\u0000\u0bbc\u0bbd\u0003"+
		"D\"\u0000\u0bbd\u0bbe\u0005\u0150\u0000\u0000\u0bbe\u0187\u0001\u0000"+
		"\u0000\u0000\u0bbf\u0bc0\u0005^\u0000\u0000\u0bc0\u0bc1\u0005\u00bc\u0000"+
		"\u0000\u0bc1\u0bc2\u0003\u0192\u00c9\u0000\u0bc2\u0bc6\u0005\u014a\u0000"+
		"\u0000\u0bc3\u0bc5\u0003\u0194\u00ca\u0000\u0bc4\u0bc3\u0001\u0000\u0000"+
		"\u0000\u0bc5\u0bc8\u0001\u0000\u0000\u0000\u0bc6\u0bc4\u0001\u0000\u0000"+
		"\u0000\u0bc6\u0bc7\u0001\u0000\u0000\u0000\u0bc7\u0bc9\u0001\u0000\u0000"+
		"\u0000\u0bc8\u0bc6\u0001\u0000\u0000\u0000\u0bc9\u0bca\u0005\u014b\u0000"+
		"\u0000\u0bca\u0189\u0001\u0000\u0000\u0000\u0bcb\u0bcc\u0005_\u0000\u0000"+
		"\u0bcc\u0bcd\u0005\u00bc\u0000\u0000\u0bcd\u0bce\u0003\u0192\u00c9\u0000"+
		"\u0bce\u0bd2\u0005\u014a\u0000\u0000\u0bcf\u0bd1\u0003\u0194\u00ca\u0000"+
		"\u0bd0\u0bcf\u0001\u0000\u0000\u0000\u0bd1\u0bd4\u0001\u0000\u0000\u0000"+
		"\u0bd2\u0bd0\u0001\u0000\u0000\u0000\u0bd2\u0bd3\u0001\u0000\u0000\u0000"+
		"\u0bd3\u0bd5\u0001\u0000\u0000\u0000\u0bd4\u0bd2\u0001\u0000\u0000\u0000"+
		"\u0bd5\u0bd6\u0005\u014b\u0000\u0000\u0bd6\u018b\u0001\u0000\u0000\u0000"+
		"\u0bd7\u0bd8\u0005`\u0000\u0000\u0bd8\u0bd9\u0005\u00bc\u0000\u0000\u0bd9"+
		"\u0bda\u0003\u0192\u00c9\u0000\u0bda\u0bde\u0005\u014a\u0000\u0000\u0bdb"+
		"\u0bdd\u0003\u0194\u00ca\u0000\u0bdc\u0bdb\u0001\u0000\u0000\u0000\u0bdd"+
		"\u0be0\u0001\u0000\u0000\u0000\u0bde\u0bdc\u0001\u0000\u0000\u0000\u0bde"+
		"\u0bdf\u0001\u0000\u0000\u0000\u0bdf\u0be1\u0001\u0000\u0000\u0000\u0be0"+
		"\u0bde\u0001\u0000\u0000\u0000\u0be1\u0be2\u0005\u014b\u0000\u0000\u0be2"+
		"\u018d\u0001\u0000\u0000\u0000\u0be3\u0be5\u0005\u0177\u0000\u0000\u0be4"+
		"\u0be3\u0001\u0000\u0000\u0000\u0be5\u0be6\u0001\u0000\u0000\u0000\u0be6"+
		"\u0be4\u0001\u0000\u0000\u0000\u0be6\u0be7\u0001\u0000\u0000\u0000\u0be7"+
		"\u018f\u0001\u0000\u0000\u0000\u0be8\u0be9\u0005\u00c0\u0000\u0000\u0be9"+
		"\u0bee\u0003\u0192\u00c9\u0000\u0bea\u0beb\u0005\u014e\u0000\u0000\u0beb"+
		"\u0bed\u0003\u0192\u00c9\u0000\u0bec\u0bea\u0001\u0000\u0000\u0000\u0bed"+
		"\u0bf0\u0001\u0000\u0000\u0000\u0bee\u0bec\u0001\u0000\u0000\u0000\u0bee"+
		"\u0bef\u0001\u0000\u0000\u0000\u0bef\u0bf1\u0001\u0000\u0000\u0000\u0bf0"+
		"\u0bee\u0001\u0000\u0000\u0000\u0bf1\u0bf2\u0005\u014d\u0000\u0000\u0bf2"+
		"\u0191\u0001\u0000\u0000\u0000\u0bf3\u0bf4\u0007\u000e\u0000\u0000\u0bf4"+
		"\u0193\u0001\u0000\u0000\u0000\u0bf5\u0c05\u0003@ \u0000\u0bf6\u0c05\u0003"+
		"B!\u0000\u0bf7\u0c05\u0003\u001e\u000f\u0000\u0bf8\u0c05\u0003 \u0010"+
		"\u0000\u0bf9\u0c05\u0003\"\u0011\u0000\u0bfa\u0c05\u00036\u001b\u0000"+
		"\u0bfb\u0c05\u0003\u008aE\u0000\u0bfc\u0c05\u0003*\u0015\u0000\u0bfd\u0c05"+
		"\u0003,\u0016\u0000\u0bfe\u0c05\u0003$\u0012\u0000\u0bff\u0c05\u0003&"+
		"\u0013\u0000\u0c00\u0c05\u0003(\u0014\u0000\u0c01\u0c05\u00034\u001a\u0000"+
		"\u0c02\u0c05\u00038\u001c\u0000\u0c03\u0c05\u0003>\u001f\u0000\u0c04\u0bf5"+
		"\u0001\u0000\u0000\u0000\u0c04\u0bf6\u0001\u0000\u0000\u0000\u0c04\u0bf7"+
		"\u0001\u0000\u0000\u0000\u0c04\u0bf8\u0001\u0000\u0000\u0000\u0c04\u0bf9"+
		"\u0001\u0000\u0000\u0000\u0c04\u0bfa\u0001\u0000\u0000\u0000\u0c04\u0bfb"+
		"\u0001\u0000\u0000\u0000\u0c04\u0bfc\u0001\u0000\u0000\u0000\u0c04\u0bfd"+
		"\u0001\u0000\u0000\u0000\u0c04\u0bfe\u0001\u0000\u0000\u0000\u0c04\u0bff"+
		"\u0001\u0000\u0000\u0000\u0c04\u0c00\u0001\u0000\u0000\u0000\u0c04\u0c01"+
		"\u0001\u0000\u0000\u0000\u0c04\u0c02\u0001\u0000\u0000\u0000\u0c04\u0c03"+
		"\u0001\u0000\u0000\u0000\u0c05\u0195\u0001\u0000\u0000\u0000\u0c06\u0c0b"+
		"\u0003\u0192\u00c9\u0000\u0c07\u0c08\u0005\u014e\u0000\u0000\u0c08\u0c0a"+
		"\u0003\u0192\u00c9\u0000\u0c09\u0c07\u0001\u0000\u0000\u0000\u0c0a\u0c0d"+
		"\u0001\u0000\u0000\u0000\u0c0b\u0c09\u0001\u0000\u0000\u0000\u0c0b\u0c0c"+
		"\u0001\u0000\u0000\u0000\u0c0c\u0197\u0001\u0000\u0000\u0000\u0c0d\u0c0b"+
		"\u0001\u0000\u0000\u0000\u0c0e\u0c11\u0003\u008cF\u0000\u0c0f\u0c10\u0005"+
		"\u0151\u0000\u0000\u0c10\u0c12\u0003\u008cF\u0000\u0c11\u0c0f\u0001\u0000"+
		"\u0000\u0000\u0c11\u0c12\u0001\u0000\u0000\u0000\u0c12\u0c1b\u0001\u0000"+
		"\u0000\u0000\u0c13\u0c14\u0005\u014e\u0000\u0000\u0c14\u0c17\u0003\u008c"+
		"F\u0000\u0c15\u0c16\u0005\u0151\u0000\u0000\u0c16\u0c18\u0003\u008cF\u0000"+
		"\u0c17\u0c15\u0001\u0000\u0000\u0000\u0c17\u0c18\u0001\u0000\u0000\u0000"+
		"\u0c18\u0c1a\u0001\u0000\u0000\u0000\u0c19\u0c13\u0001\u0000\u0000\u0000"+
		"\u0c1a\u0c1d\u0001\u0000\u0000\u0000\u0c1b\u0c19\u0001\u0000\u0000\u0000"+
		"\u0c1b\u0c1c\u0001\u0000\u0000\u0000\u0c1c\u0199\u0001\u0000\u0000\u0000"+
		"\u0c1d\u0c1b\u0001\u0000\u0000\u0000\u0c1e\u0c2a\u0005\u0140\u0000\u0000"+
		"\u0c1f\u0c2a\u0005\u0141\u0000\u0000\u0c20\u0c2a\u0005\u0142\u0000\u0000"+
		"\u0c21\u0c2a\u0005\u0143\u0000\u0000\u0c22\u0c2a\u0005\u013e\u0000\u0000"+
		"\u0c23\u0c2a\u0005\u013f\u0000\u0000\u0c24\u0c2a\u0003\u019c\u00ce\u0000"+
		"\u0c25\u0c2a\u0003\u019e\u00cf\u0000\u0c26\u0c2a\u0003\u01a0\u00d0\u0000"+
		"\u0c27\u0c2a\u0003\u01a2\u00d1\u0000\u0c28\u0c2a\u0003\u01a4\u00d2\u0000"+
		"\u0c29\u0c1e\u0001\u0000\u0000\u0000\u0c29\u0c1f\u0001\u0000\u0000\u0000"+
		"\u0c29\u0c20\u0001\u0000\u0000\u0000\u0c29\u0c21\u0001\u0000\u0000\u0000"+
		"\u0c29\u0c22\u0001\u0000\u0000\u0000\u0c29\u0c23\u0001\u0000\u0000\u0000"+
		"\u0c29\u0c24\u0001\u0000\u0000\u0000\u0c29\u0c25\u0001\u0000\u0000\u0000"+
		"\u0c29\u0c26\u0001\u0000\u0000\u0000\u0c29\u0c27\u0001\u0000\u0000\u0000"+
		"\u0c29\u0c28\u0001\u0000\u0000\u0000\u0c2a\u019b\u0001\u0000\u0000\u0000"+
		"\u0c2b\u0c31\u0005\u0168\u0000\u0000\u0c2c\u0c32\u0005\u00c1\u0000\u0000"+
		"\u0c2d\u0c32\u0005\u00c2\u0000\u0000\u0c2e\u0c32\u0005\u0159\u0000\u0000"+
		"\u0c2f\u0c32\u0005\u015a\u0000\u0000\u0c30\u0c32\u0003\u0192\u00c9\u0000"+
		"\u0c31\u0c2c\u0001\u0000\u0000\u0000\u0c31\u0c2d\u0001\u0000\u0000\u0000"+
		"\u0c31\u0c2e\u0001\u0000\u0000\u0000\u0c31\u0c2f\u0001\u0000\u0000\u0000"+
		"\u0c31\u0c30\u0001\u0000\u0000\u0000\u0c32\u0c33\u0001\u0000\u0000\u0000"+
		"\u0c33\u0c34\u0005\u00c3\u0000\u0000\u0c34\u019d\u0001\u0000\u0000\u0000"+
		"\u0c35\u0c36\u0005\u00c4\u0000\u0000\u0c36\u0c37\u0005\u0148\u0000\u0000"+
		"\u0c37\u0c38\u0003\u0192\u00c9\u0000\u0c38\u0c39\u0005\u0151\u0000\u0000"+
		"\u0c39\u0c3a\u0005\u00e0\u0000\u0000\u0c3a\u0c3b\u0005\u0149\u0000\u0000"+
		"\u0c3b\u0c41\u0001\u0000\u0000\u0000\u0c3c\u0c3d\u0005\u00c5\u0000\u0000"+
		"\u0c3d\u0c3e\u0005\u0148\u0000\u0000\u0c3e\u0c3f\u0005\u00e1\u0000\u0000"+
		"\u0c3f\u0c41\u0005\u0149\u0000\u0000\u0c40\u0c35\u0001\u0000\u0000\u0000"+
		"\u0c40\u0c3c\u0001\u0000\u0000\u0000\u0c41\u019f\u0001\u0000\u0000\u0000"+
		"\u0c42\u0c43\u0005\u0134\u0000\u0000\u0c43\u0c44\u0005\u014c\u0000\u0000"+
		"\u0c44\u0c45\u0005\u0142\u0000\u0000\u0c45\u0c46\u0005\u014d\u0000\u0000"+
		"\u0c46\u01a1\u0001\u0000\u0000\u0000\u0c47\u0c48\u0005\u00c6\u0000\u0000"+
		"\u0c48\u0c49\u0005\u0142\u0000\u0000\u0c49\u01a3\u0001\u0000\u0000\u0000"+
		"\u0c4a\u0c4b\u0005A\u0000\u0000\u0c4b\u0c4c\u0005\u0142\u0000\u0000\u0c4c"+
		"\u01a5\u0001\u0000\u0000\u0000\u0c4d\u0c4e\u0005\u00c7\u0000\u0000\u0c4e"+
		"\u0c4f\u0005\u0142\u0000\u0000\u0c4f\u01a7\u0001\u0000\u0000\u0000\u0c50"+
		"\u0c51\u0005\u00c8\u0000\u0000\u0c51\u0c52\u0005\u0148\u0000\u0000\u0c52"+
		"\u0c53\u0003\u0192\u00c9\u0000\u0c53\u0c54\u0005\u0151\u0000\u0000\u0c54"+
		"\u0c55\u0003\u008cF\u0000\u0c55\u0c56\u0005\u0149\u0000\u0000\u0c56\u0c57"+
		"\u0005\u014f\u0000\u0000\u0c57\u0c58\u0003\u008cF\u0000\u0c58\u01a9\u0001"+
		"\u0000\u0000\u0000\u0c59\u0c5a\u0005\u00c9\u0000\u0000\u0c5a\u0c5b\u0005"+
		"\u0148\u0000\u0000\u0c5b\u0c5c\u0003\u0192\u00c9\u0000\u0c5c\u0c5d\u0005"+
		"\u0151\u0000\u0000\u0c5d\u0c5e\u0003\u008cF\u0000\u0c5e\u0c5f\u0005\u0149"+
		"\u0000\u0000\u0c5f\u0c60\u0005\u014f\u0000\u0000\u0c60\u0c61\u0003\u008c"+
		"F\u0000\u0c61\u01ab\u0001\u0000\u0000\u0000\u0c62\u0c63\u0005\u00ca\u0000"+
		"\u0000\u0c63\u0c64\u0005\u0148\u0000\u0000\u0c64\u0c65\u0003\u008cF\u0000"+
		"\u0c65\u0c66\u0005\u014e\u0000\u0000\u0c66\u0c67\u0003D\"\u0000\u0c67"+
		"\u0c68\u0005\u014e\u0000\u0000\u0c68\u0c69\u0003D\"\u0000\u0c69\u0c6a"+
		"\u0005\u0149\u0000\u0000\u0c6a\u01ad\u0001\u0000\u0000\u0000\u0c6b\u0c6c"+
		"\u0005\u00cb\u0000\u0000\u0c6c\u0c83\u0003\u008cF\u0000\u0c6d\u0c6e\u0005"+
		"\u00cc\u0000\u0000\u0c6e\u0c83\u0003\u008cF\u0000\u0c6f\u0c70\u0005\u00cd"+
		"\u0000\u0000\u0c70\u0c74\u0005\u014a\u0000\u0000\u0c71\u0c73\u0003\u01b0"+
		"\u00d8\u0000\u0c72\u0c71\u0001\u0000\u0000\u0000\u0c73\u0c76\u0001\u0000"+
		"\u0000\u0000\u0c74\u0c72\u0001\u0000\u0000\u0000\u0c74\u0c75\u0001\u0000"+
		"\u0000\u0000\u0c75\u0c77\u0001\u0000\u0000\u0000\u0c76\u0c74\u0001\u0000"+
		"\u0000\u0000\u0c77\u0c83\u0005\u014b\u0000\u0000\u0c78\u0c79\u0005\u00ce"+
		"\u0000\u0000\u0c79\u0c7d\u0005\u014a\u0000\u0000\u0c7a\u0c7c\u0003\u01b0"+
		"\u00d8\u0000\u0c7b\u0c7a\u0001\u0000\u0000\u0000\u0c7c\u0c7f\u0001\u0000"+
		"\u0000\u0000\u0c7d\u0c7b\u0001\u0000\u0000\u0000\u0c7d\u0c7e\u0001\u0000"+
		"\u0000\u0000\u0c7e\u0c80\u0001\u0000\u0000\u0000\u0c7f\u0c7d\u0001\u0000"+
		"\u0000\u0000\u0c80\u0c83\u0005\u014b\u0000\u0000\u0c81\u0c83\u0005\u00cf"+
		"\u0000\u0000\u0c82\u0c6b\u0001\u0000\u0000\u0000\u0c82\u0c6d\u0001\u0000"+
		"\u0000\u0000\u0c82\u0c6f\u0001\u0000\u0000\u0000\u0c82\u0c78\u0001\u0000"+
		"\u0000\u0000\u0c82\u0c81\u0001\u0000\u0000\u0000\u0c83\u01af\u0001\u0000"+
		"\u0000\u0000\u0c84\u0c85\u0003\u0192\u00c9\u0000\u0c85\u0c86\u0005\u0152"+
		"\u0000\u0000\u0c86\u0c87\u0003\u008cF\u0000\u0c87\u01b1\u0001\u0000\u0000"+
		"\u0000\u0c88\u0c9b\u0005\u00d0\u0000\u0000\u0c89\u0c8a\u0005\u00d1\u0000"+
		"\u0000\u0c8a\u0c8b\u0005\u014c\u0000\u0000\u0c8b\u0c8c\u0003D\"\u0000"+
		"\u0c8c\u0c8d\u0005\u014d\u0000\u0000\u0c8d\u0c9b\u0001\u0000\u0000\u0000"+
		"\u0c8e\u0c8f\u0005\u00d2\u0000\u0000\u0c8f\u0c90\u0005\u0161\u0000\u0000"+
		"\u0c90\u0c91\u0003\u008cF\u0000\u0c91\u0c92\u0005\u0162\u0000\u0000\u0c92"+
		"\u0c9b\u0001\u0000\u0000\u0000\u0c93\u0c94\u0005\u00d3\u0000\u0000\u0c94"+
		"\u0c95\u0005\u0161\u0000\u0000\u0c95\u0c96\u0003\u008cF\u0000\u0c96\u0c97"+
		"\u0005\u014e\u0000\u0000\u0c97\u0c98\u0003\u008cF\u0000\u0c98\u0c99\u0005"+
		"\u0162\u0000\u0000\u0c99\u0c9b\u0001\u0000\u0000\u0000\u0c9a\u0c88\u0001"+
		"\u0000\u0000\u0000\u0c9a\u0c89\u0001\u0000\u0000\u0000\u0c9a\u0c8e\u0001"+
		"\u0000\u0000\u0000\u0c9a\u0c93\u0001\u0000\u0000\u0000\u0c9b\u01b3\u0001"+
		"\u0000\u0000\u0000\u0c9c\u0c9d\u0007\u000f\u0000\u0000\u0c9d\u01b5\u0001"+
		"\u0000\u0000\u0000\u0c9e\u0c9f\u0007\u0010\u0000\u0000\u0c9f\u01b7\u0001"+
		"\u0000\u0000\u0000\u0ca0\u0ca1\u0007\u0011\u0000\u0000\u0ca1\u01b9\u0001"+
		"\u0000\u0000\u0000\u0ca2\u0ca3\u0007\u0012\u0000\u0000\u0ca3\u01bb\u0001"+
		"\u0000\u0000\u0000\u0124\u01bf\u01c5\u01c8\u023a\u0242\u0247\u024d\u0250"+
		"\u0256\u0259\u0260\u0262\u0268\u026f\u0274\u027b\u0285\u028a\u0294\u0299"+
		"\u029d\u02a2\u02a6\u02af\u02b3\u02b8\u02bc\u02c2\u02c7\u02cd\u02ea\u02ec"+
		"\u02f6\u02fe\u0302\u0307\u0313\u031e\u0329\u0334\u033b\u0340\u034b\u0356"+
		"\u035a\u0361\u036c\u0373\u0378\u0382\u038e\u0395\u039c\u03a4\u03ac\u03b4"+
		"\u03bc\u03c4\u03cc\u03d4\u03dc\u03e4\u03ec\u03f4\u03f9\u03fd\u0403\u0408"+
		"\u0413\u0416\u0429\u042d\u0434\u043c\u043f\u0447\u0454\u045e\u0461\u0470"+
		"\u0473\u0479\u047e\u0483\u0488\u0497\u049b\u04a4\u04a9\u04bc\u04bf\u04c4"+
		"\u04cb\u04cf\u04da\u04de\u04e4\u04ec\u04f1\u04f7\u04fa\u0503\u050f\u051c"+
		"\u0530\u053d\u0541\u0557\u0565\u056b\u0574\u0582\u0587\u0595\u059a\u05a3"+
		"\u05a6\u05ab\u05af\u05b7\u05bc\u05c3\u05d7\u05f8\u0605\u060d\u060f\u0628"+
		"\u0630\u0638\u0640\u0646\u064b\u0651\u0657\u065d\u0660\u0665\u066b\u0677"+
		"\u067c\u067f\u0682\u0687\u068b\u0691\u0699\u069d\u06a2\u06a7\u06ad\u06b5"+
		"\u06b8\u06bd\u06c4\u06cf\u06d4\u06d8\u06e0\u06e3\u06e9\u06ef\u06f5\u06fa"+
		"\u0705\u070a\u0710\u0716\u071b\u071f\u0724\u0728\u072f\u0739\u0744\u074f"+
		"\u0758\u0764\u076f\u077a\u0784\u0786\u0791\u079c\u07a6\u07ad\u07b8\u07c3"+
		"\u07cf\u07da\u07e5\u07f0\u07fb\u0806\u0812\u081e\u082a\u0836\u0842\u084e"+
		"\u085a\u0866\u0872\u087d\u0888\u0894\u08a0\u08ac\u08b7\u08c3\u08cf\u08db"+
		"\u08e7\u08f3\u08ff\u090b\u0917\u0923\u092f\u093b\u0947\u0953\u095f\u096e"+
		"\u097a\u0986\u0992\u099e\u09a9\u09b5\u09c1\u09cc\u09d9\u09e4\u09ef\u09fa"+
		"\u0a06\u0a12\u0a1e\u0a2a\u0a35\u0a40\u0a4b\u0a56\u0a62\u0a6f\u0a7b\u0a86"+
		"\u0a91\u0a9c\u0aa7\u0ab2\u0abd\u0ac8\u0ad3\u0adf\u0aea\u0af5\u0b00\u0b0b"+
		"\u0b16\u0b21\u0b2c\u0b37\u0b42\u0b4d\u0b58\u0b63\u0b6e\u0b79\u0b86\u0b91"+
		"\u0b9c\u0ba7\u0bb2\u0bc6\u0bd2\u0bde\u0be6\u0bee\u0c04\u0c0b\u0c11\u0c17"+
		"\u0c1b\u0c29\u0c31\u0c40\u0c74\u0c7d\u0c82\u0c9a";
	public static final String _serializedATN = Utils.join(
		new String[] {
			_serializedATNSegment0,
			_serializedATNSegment1
		},
		""
	);
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
// Generated from /home/ubuntu/Zamani/grammar/Zamani.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link ZamaniParser}.
 */
public interface ZamaniListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#program}.
	 * @param ctx the parse tree
	 */
	void enterProgram(ZamaniParser.ProgramContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#program}.
	 * @param ctx the parse tree
	 */
	void exitProgram(ZamaniParser.ProgramContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#declaration}.
	 * @param ctx the parse tree
	 */
	void enterDeclaration(ZamaniParser.DeclarationContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#declaration}.
	 * @param ctx the parse tree
	 */
	void exitDeclaration(ZamaniParser.DeclarationContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#moduleDecl}.
	 * @param ctx the parse tree
	 */
	void enterModuleDecl(ZamaniParser.ModuleDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#moduleDecl}.
	 * @param ctx the parse tree
	 */
	void exitModuleDecl(ZamaniParser.ModuleDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#importDecl}.
	 * @param ctx the parse tree
	 */
	void enterImportDecl(ZamaniParser.ImportDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#importDecl}.
	 * @param ctx the parse tree
	 */
	void exitImportDecl(ZamaniParser.ImportDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#exportDecl}.
	 * @param ctx the parse tree
	 */
	void enterExportDecl(ZamaniParser.ExportDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#exportDecl}.
	 * @param ctx the parse tree
	 */
	void exitExportDecl(ZamaniParser.ExportDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#modulePath}.
	 * @param ctx the parse tree
	 */
	void enterModulePath(ZamaniParser.ModulePathContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#modulePath}.
	 * @param ctx the parse tree
	 */
	void exitModulePath(ZamaniParser.ModulePathContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#useStmt}.
	 * @param ctx the parse tree
	 */
	void enterUseStmt(ZamaniParser.UseStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#useStmt}.
	 * @param ctx the parse tree
	 */
	void exitUseStmt(ZamaniParser.UseStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#usePath}.
	 * @param ctx the parse tree
	 */
	void enterUsePath(ZamaniParser.UsePathContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#usePath}.
	 * @param ctx the parse tree
	 */
	void exitUsePath(ZamaniParser.UsePathContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#segment}.
	 * @param ctx the parse tree
	 */
	void enterSegment(ZamaniParser.SegmentContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#segment}.
	 * @param ctx the parse tree
	 */
	void exitSegment(ZamaniParser.SegmentContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#globalUsing}.
	 * @param ctx the parse tree
	 */
	void enterGlobalUsing(ZamaniParser.GlobalUsingContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#globalUsing}.
	 * @param ctx the parse tree
	 */
	void exitGlobalUsing(ZamaniParser.GlobalUsingContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#functionDecl}.
	 * @param ctx the parse tree
	 */
	void enterFunctionDecl(ZamaniParser.FunctionDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#functionDecl}.
	 * @param ctx the parse tree
	 */
	void exitFunctionDecl(ZamaniParser.FunctionDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#params}.
	 * @param ctx the parse tree
	 */
	void enterParams(ZamaniParser.ParamsContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#params}.
	 * @param ctx the parse tree
	 */
	void exitParams(ZamaniParser.ParamsContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#param}.
	 * @param ctx the parse tree
	 */
	void enterParam(ZamaniParser.ParamContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#param}.
	 * @param ctx the parse tree
	 */
	void exitParam(ZamaniParser.ParamContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#modifiers}.
	 * @param ctx the parse tree
	 */
	void enterModifiers(ZamaniParser.ModifiersContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#modifiers}.
	 * @param ctx the parse tree
	 */
	void exitModifiers(ZamaniParser.ModifiersContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#modifier}.
	 * @param ctx the parse tree
	 */
	void enterModifier(ZamaniParser.ModifierContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#modifier}.
	 * @param ctx the parse tree
	 */
	void exitModifier(ZamaniParser.ModifierContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#returnStmt}.
	 * @param ctx the parse tree
	 */
	void enterReturnStmt(ZamaniParser.ReturnStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#returnStmt}.
	 * @param ctx the parse tree
	 */
	void exitReturnStmt(ZamaniParser.ReturnStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#breakStmt}.
	 * @param ctx the parse tree
	 */
	void enterBreakStmt(ZamaniParser.BreakStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#breakStmt}.
	 * @param ctx the parse tree
	 */
	void exitBreakStmt(ZamaniParser.BreakStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#continueStmt}.
	 * @param ctx the parse tree
	 */
	void enterContinueStmt(ZamaniParser.ContinueStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#continueStmt}.
	 * @param ctx the parse tree
	 */
	void exitContinueStmt(ZamaniParser.ContinueStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#whileStmt}.
	 * @param ctx the parse tree
	 */
	void enterWhileStmt(ZamaniParser.WhileStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#whileStmt}.
	 * @param ctx the parse tree
	 */
	void exitWhileStmt(ZamaniParser.WhileStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#forStmt}.
	 * @param ctx the parse tree
	 */
	void enterForStmt(ZamaniParser.ForStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#forStmt}.
	 * @param ctx the parse tree
	 */
	void exitForStmt(ZamaniParser.ForStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#loopExpr}.
	 * @param ctx the parse tree
	 */
	void enterLoopExpr(ZamaniParser.LoopExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#loopExpr}.
	 * @param ctx the parse tree
	 */
	void exitLoopExpr(ZamaniParser.LoopExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#ifExpr}.
	 * @param ctx the parse tree
	 */
	void enterIfExpr(ZamaniParser.IfExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#ifExpr}.
	 * @param ctx the parse tree
	 */
	void exitIfExpr(ZamaniParser.IfExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#matchStmt}.
	 * @param ctx the parse tree
	 */
	void enterMatchStmt(ZamaniParser.MatchStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#matchStmt}.
	 * @param ctx the parse tree
	 */
	void exitMatchStmt(ZamaniParser.MatchStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#matchExpr}.
	 * @param ctx the parse tree
	 */
	void enterMatchExpr(ZamaniParser.MatchExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#matchExpr}.
	 * @param ctx the parse tree
	 */
	void exitMatchExpr(ZamaniParser.MatchExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#matchCase}.
	 * @param ctx the parse tree
	 */
	void enterMatchCase(ZamaniParser.MatchCaseContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#matchCase}.
	 * @param ctx the parse tree
	 */
	void exitMatchCase(ZamaniParser.MatchCaseContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#pattern}.
	 * @param ctx the parse tree
	 */
	void enterPattern(ZamaniParser.PatternContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#pattern}.
	 * @param ctx the parse tree
	 */
	void exitPattern(ZamaniParser.PatternContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#unsafeBlock}.
	 * @param ctx the parse tree
	 */
	void enterUnsafeBlock(ZamaniParser.UnsafeBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#unsafeBlock}.
	 * @param ctx the parse tree
	 */
	void exitUnsafeBlock(ZamaniParser.UnsafeBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#throwStmt}.
	 * @param ctx the parse tree
	 */
	void enterThrowStmt(ZamaniParser.ThrowStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#throwStmt}.
	 * @param ctx the parse tree
	 */
	void exitThrowStmt(ZamaniParser.ThrowStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#tryCatchStmt}.
	 * @param ctx the parse tree
	 */
	void enterTryCatchStmt(ZamaniParser.TryCatchStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#tryCatchStmt}.
	 * @param ctx the parse tree
	 */
	void exitTryCatchStmt(ZamaniParser.TryCatchStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#catchClause}.
	 * @param ctx the parse tree
	 */
	void enterCatchClause(ZamaniParser.CatchClauseContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#catchClause}.
	 * @param ctx the parse tree
	 */
	void exitCatchClause(ZamaniParser.CatchClauseContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#finallyClause}.
	 * @param ctx the parse tree
	 */
	void enterFinallyClause(ZamaniParser.FinallyClauseContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#finallyClause}.
	 * @param ctx the parse tree
	 */
	void exitFinallyClause(ZamaniParser.FinallyClauseContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#blockExpr}.
	 * @param ctx the parse tree
	 */
	void enterBlockExpr(ZamaniParser.BlockExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#blockExpr}.
	 * @param ctx the parse tree
	 */
	void exitBlockExpr(ZamaniParser.BlockExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#letStmt}.
	 * @param ctx the parse tree
	 */
	void enterLetStmt(ZamaniParser.LetStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#letStmt}.
	 * @param ctx the parse tree
	 */
	void exitLetStmt(ZamaniParser.LetStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#constStmt}.
	 * @param ctx the parse tree
	 */
	void enterConstStmt(ZamaniParser.ConstStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#constStmt}.
	 * @param ctx the parse tree
	 */
	void exitConstStmt(ZamaniParser.ConstStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#expression}.
	 * @param ctx the parse tree
	 */
	void enterExpression(ZamaniParser.ExpressionContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#expression}.
	 * @param ctx the parse tree
	 */
	void exitExpression(ZamaniParser.ExpressionContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#assignmentExpr}.
	 * @param ctx the parse tree
	 */
	void enterAssignmentExpr(ZamaniParser.AssignmentExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#assignmentExpr}.
	 * @param ctx the parse tree
	 */
	void exitAssignmentExpr(ZamaniParser.AssignmentExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#assignOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignOp(ZamaniParser.AssignOpContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#assignOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignOp(ZamaniParser.AssignOpContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#rangeExpr}.
	 * @param ctx the parse tree
	 */
	void enterRangeExpr(ZamaniParser.RangeExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#rangeExpr}.
	 * @param ctx the parse tree
	 */
	void exitRangeExpr(ZamaniParser.RangeExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#logicalOrExpr}.
	 * @param ctx the parse tree
	 */
	void enterLogicalOrExpr(ZamaniParser.LogicalOrExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#logicalOrExpr}.
	 * @param ctx the parse tree
	 */
	void exitLogicalOrExpr(ZamaniParser.LogicalOrExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#logicalAndExpr}.
	 * @param ctx the parse tree
	 */
	void enterLogicalAndExpr(ZamaniParser.LogicalAndExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#logicalAndExpr}.
	 * @param ctx the parse tree
	 */
	void exitLogicalAndExpr(ZamaniParser.LogicalAndExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#bitOrExpr}.
	 * @param ctx the parse tree
	 */
	void enterBitOrExpr(ZamaniParser.BitOrExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#bitOrExpr}.
	 * @param ctx the parse tree
	 */
	void exitBitOrExpr(ZamaniParser.BitOrExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#bitXorExpr}.
	 * @param ctx the parse tree
	 */
	void enterBitXorExpr(ZamaniParser.BitXorExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#bitXorExpr}.
	 * @param ctx the parse tree
	 */
	void exitBitXorExpr(ZamaniParser.BitXorExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#bitAndExpr}.
	 * @param ctx the parse tree
	 */
	void enterBitAndExpr(ZamaniParser.BitAndExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#bitAndExpr}.
	 * @param ctx the parse tree
	 */
	void exitBitAndExpr(ZamaniParser.BitAndExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#equalityExpr}.
	 * @param ctx the parse tree
	 */
	void enterEqualityExpr(ZamaniParser.EqualityExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#equalityExpr}.
	 * @param ctx the parse tree
	 */
	void exitEqualityExpr(ZamaniParser.EqualityExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#comparisonExpr}.
	 * @param ctx the parse tree
	 */
	void enterComparisonExpr(ZamaniParser.ComparisonExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#comparisonExpr}.
	 * @param ctx the parse tree
	 */
	void exitComparisonExpr(ZamaniParser.ComparisonExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#shiftExpr}.
	 * @param ctx the parse tree
	 */
	void enterShiftExpr(ZamaniParser.ShiftExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#shiftExpr}.
	 * @param ctx the parse tree
	 */
	void exitShiftExpr(ZamaniParser.ShiftExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sumExpr}.
	 * @param ctx the parse tree
	 */
	void enterSumExpr(ZamaniParser.SumExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sumExpr}.
	 * @param ctx the parse tree
	 */
	void exitSumExpr(ZamaniParser.SumExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#productExpr}.
	 * @param ctx the parse tree
	 */
	void enterProductExpr(ZamaniParser.ProductExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#productExpr}.
	 * @param ctx the parse tree
	 */
	void exitProductExpr(ZamaniParser.ProductExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#castExpr}.
	 * @param ctx the parse tree
	 */
	void enterCastExpr(ZamaniParser.CastExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#castExpr}.
	 * @param ctx the parse tree
	 */
	void exitCastExpr(ZamaniParser.CastExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#prefixExpr}.
	 * @param ctx the parse tree
	 */
	void enterPrefixExpr(ZamaniParser.PrefixExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#prefixExpr}.
	 * @param ctx the parse tree
	 */
	void exitPrefixExpr(ZamaniParser.PrefixExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#postfixExpr}.
	 * @param ctx the parse tree
	 */
	void enterPostfixExpr(ZamaniParser.PostfixExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#postfixExpr}.
	 * @param ctx the parse tree
	 */
	void exitPostfixExpr(ZamaniParser.PostfixExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code callOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterCallOp(ZamaniParser.CallOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code callOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitCallOp(ZamaniParser.CallOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code indexOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterIndexOp(ZamaniParser.IndexOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code indexOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitIndexOp(ZamaniParser.IndexOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code memberOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterMemberOp(ZamaniParser.MemberOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code memberOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitMemberOp(ZamaniParser.MemberOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code tryPropagateOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterTryPropagateOp(ZamaniParser.TryPropagateOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code tryPropagateOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitTryPropagateOp(ZamaniParser.TryPropagateOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code postIncOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterPostIncOp(ZamaniParser.PostIncOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code postIncOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitPostIncOp(ZamaniParser.PostIncOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code postDecOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterPostDecOp(ZamaniParser.PostDecOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code postDecOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitPostDecOp(ZamaniParser.PostDecOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code withEffectOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterWithEffectOp(ZamaniParser.WithEffectOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code withEffectOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitWithEffectOp(ZamaniParser.WithEffectOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code withBlockOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void enterWithBlockOp(ZamaniParser.WithBlockOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code withBlockOp}
	 * labeled alternative in {@link ZamaniParser#postfixOp}.
	 * @param ctx the parse tree
	 */
	void exitWithBlockOp(ZamaniParser.WithBlockOpContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#args}.
	 * @param ctx the parse tree
	 */
	void enterArgs(ZamaniParser.ArgsContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#args}.
	 * @param ctx the parse tree
	 */
	void exitArgs(ZamaniParser.ArgsContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#namedArgument}.
	 * @param ctx the parse tree
	 */
	void enterNamedArgument(ZamaniParser.NamedArgumentContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#namedArgument}.
	 * @param ctx the parse tree
	 */
	void exitNamedArgument(ZamaniParser.NamedArgumentContext ctx);
	/**
	 * Enter a parse tree produced by the {@code identExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterIdentExpr(ZamaniParser.IdentExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code identExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitIdentExpr(ZamaniParser.IdentExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code literalExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterLiteralExpr(ZamaniParser.LiteralExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code literalExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitLiteralExpr(ZamaniParser.LiteralExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code parenExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterParenExpr(ZamaniParser.ParenExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code parenExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitParenExpr(ZamaniParser.ParenExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code tupleExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterTupleExpr(ZamaniParser.TupleExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code tupleExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitTupleExpr(ZamaniParser.TupleExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code arrayExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterArrayExpr(ZamaniParser.ArrayExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code arrayExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitArrayExpr(ZamaniParser.ArrayExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code mapExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterMapExpr(ZamaniParser.MapExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code mapExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitMapExpr(ZamaniParser.MapExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code blockValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterBlockValExpr(ZamaniParser.BlockValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code blockValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitBlockValExpr(ZamaniParser.BlockValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code lambdaExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterLambdaExpr(ZamaniParser.LambdaExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code lambdaExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitLambdaExpr(ZamaniParser.LambdaExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code anonFnExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterAnonFnExpr(ZamaniParser.AnonFnExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code anonFnExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitAnonFnExpr(ZamaniParser.AnonFnExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ifValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterIfValExpr(ZamaniParser.IfValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ifValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitIfValExpr(ZamaniParser.IfValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code matchValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterMatchValExpr(ZamaniParser.MatchValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code matchValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitMatchValExpr(ZamaniParser.MatchValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code loopValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterLoopValExpr(ZamaniParser.LoopValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code loopValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitLoopValExpr(ZamaniParser.LoopValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code asyncExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterAsyncExpr(ZamaniParser.AsyncExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code asyncExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitAsyncExpr(ZamaniParser.AsyncExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code awaitExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterAwaitExpr(ZamaniParser.AwaitExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code awaitExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitAwaitExpr(ZamaniParser.AwaitExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code spawnValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterSpawnValExpr(ZamaniParser.SpawnValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code spawnValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitSpawnValExpr(ZamaniParser.SpawnValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code newExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterNewExpr(ZamaniParser.NewExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code newExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitNewExpr(ZamaniParser.NewExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code tryCatchExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterTryCatchExpr(ZamaniParser.TryCatchExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code tryCatchExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitTryCatchExpr(ZamaniParser.TryCatchExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code yieldExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterYieldExpr(ZamaniParser.YieldExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code yieldExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitYieldExpr(ZamaniParser.YieldExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code recallValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterRecallValExpr(ZamaniParser.RecallValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code recallValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitRecallValExpr(ZamaniParser.RecallValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code learnValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterLearnValExpr(ZamaniParser.LearnValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code learnValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitLearnValExpr(ZamaniParser.LearnValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code performValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterPerformValExpr(ZamaniParser.PerformValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code performValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitPerformValExpr(ZamaniParser.PerformValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code zamaniValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterZamaniValExpr(ZamaniParser.ZamaniValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code zamaniValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitZamaniValExpr(ZamaniParser.ZamaniValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code sasaValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterSasaValExpr(ZamaniParser.SasaValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code sasaValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitSasaValExpr(ZamaniParser.SasaValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code quantumOpValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterQuantumOpValExpr(ZamaniParser.QuantumOpValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code quantumOpValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitQuantumOpValExpr(ZamaniParser.QuantumOpValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code nanoValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterNanoValExpr(ZamaniParser.NanoValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code nanoValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitNanoValExpr(ZamaniParser.NanoValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code mtsValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterMtsValExpr(ZamaniParser.MtsValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code mtsValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitMtsValExpr(ZamaniParser.MtsValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code consensusValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterConsensusValExpr(ZamaniParser.ConsensusValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code consensusValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitConsensusValExpr(ZamaniParser.ConsensusValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ancestorValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterAncestorValExpr(ZamaniParser.AncestorValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ancestorValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitAncestorValExpr(ZamaniParser.AncestorValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code mopValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterMopValExpr(ZamaniParser.MopValExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code mopValExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitMopValExpr(ZamaniParser.MopValExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code macroCallExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterMacroCallExpr(ZamaniParser.MacroCallExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code macroCallExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitMacroCallExpr(ZamaniParser.MacroCallExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code superExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterSuperExpr(ZamaniParser.SuperExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code superExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitSuperExpr(ZamaniParser.SuperExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code thisExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterThisExpr(ZamaniParser.ThisExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code thisExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitThisExpr(ZamaniParser.ThisExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code selfExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterSelfExpr(ZamaniParser.SelfExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code selfExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitSelfExpr(ZamaniParser.SelfExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code interpStringExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void enterInterpStringExpr(ZamaniParser.InterpStringExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code interpStringExpr}
	 * labeled alternative in {@link ZamaniParser#primaryExpr}.
	 * @param ctx the parse tree
	 */
	void exitInterpStringExpr(ZamaniParser.InterpStringExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#structLiteralTail}.
	 * @param ctx the parse tree
	 */
	void enterStructLiteralTail(ZamaniParser.StructLiteralTailContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#structLiteralTail}.
	 * @param ctx the parse tree
	 */
	void exitStructLiteralTail(ZamaniParser.StructLiteralTailContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#recallExpr}.
	 * @param ctx the parse tree
	 */
	void enterRecallExpr(ZamaniParser.RecallExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#recallExpr}.
	 * @param ctx the parse tree
	 */
	void exitRecallExpr(ZamaniParser.RecallExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#learnExpr}.
	 * @param ctx the parse tree
	 */
	void enterLearnExpr(ZamaniParser.LearnExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#learnExpr}.
	 * @param ctx the parse tree
	 */
	void exitLearnExpr(ZamaniParser.LearnExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#performExpr}.
	 * @param ctx the parse tree
	 */
	void enterPerformExpr(ZamaniParser.PerformExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#performExpr}.
	 * @param ctx the parse tree
	 */
	void exitPerformExpr(ZamaniParser.PerformExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#zamaniExpr}.
	 * @param ctx the parse tree
	 */
	void enterZamaniExpr(ZamaniParser.ZamaniExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#zamaniExpr}.
	 * @param ctx the parse tree
	 */
	void exitZamaniExpr(ZamaniParser.ZamaniExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sasaExpr}.
	 * @param ctx the parse tree
	 */
	void enterSasaExpr(ZamaniParser.SasaExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sasaExpr}.
	 * @param ctx the parse tree
	 */
	void exitSasaExpr(ZamaniParser.SasaExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#quantumOpExpr}.
	 * @param ctx the parse tree
	 */
	void enterQuantumOpExpr(ZamaniParser.QuantumOpExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#quantumOpExpr}.
	 * @param ctx the parse tree
	 */
	void exitQuantumOpExpr(ZamaniParser.QuantumOpExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#nanoExpr}.
	 * @param ctx the parse tree
	 */
	void enterNanoExpr(ZamaniParser.NanoExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#nanoExpr}.
	 * @param ctx the parse tree
	 */
	void exitNanoExpr(ZamaniParser.NanoExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#mtsExpr}.
	 * @param ctx the parse tree
	 */
	void enterMtsExpr(ZamaniParser.MtsExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#mtsExpr}.
	 * @param ctx the parse tree
	 */
	void exitMtsExpr(ZamaniParser.MtsExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#consensusExpr}.
	 * @param ctx the parse tree
	 */
	void enterConsensusExpr(ZamaniParser.ConsensusExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#consensusExpr}.
	 * @param ctx the parse tree
	 */
	void exitConsensusExpr(ZamaniParser.ConsensusExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#ancestorCall}.
	 * @param ctx the parse tree
	 */
	void enterAncestorCall(ZamaniParser.AncestorCallContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#ancestorCall}.
	 * @param ctx the parse tree
	 */
	void exitAncestorCall(ZamaniParser.AncestorCallContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#mopExpr}.
	 * @param ctx the parse tree
	 */
	void enterMopExpr(ZamaniParser.MopExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#mopExpr}.
	 * @param ctx the parse tree
	 */
	void exitMopExpr(ZamaniParser.MopExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#macroCall}.
	 * @param ctx the parse tree
	 */
	void enterMacroCall(ZamaniParser.MacroCallContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#macroCall}.
	 * @param ctx the parse tree
	 */
	void exitMacroCall(ZamaniParser.MacroCallContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#exprList}.
	 * @param ctx the parse tree
	 */
	void enterExprList(ZamaniParser.ExprListContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#exprList}.
	 * @param ctx the parse tree
	 */
	void exitExprList(ZamaniParser.ExprListContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#expressionStmt}.
	 * @param ctx the parse tree
	 */
	void enterExpressionStmt(ZamaniParser.ExpressionStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#expressionStmt}.
	 * @param ctx the parse tree
	 */
	void exitExpressionStmt(ZamaniParser.ExpressionStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeExpr}.
	 * @param ctx the parse tree
	 */
	void enterTypeExpr(ZamaniParser.TypeExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeExpr}.
	 * @param ctx the parse tree
	 */
	void exitTypeExpr(ZamaniParser.TypeExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#baseType}.
	 * @param ctx the parse tree
	 */
	void enterBaseType(ZamaniParser.BaseTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#baseType}.
	 * @param ctx the parse tree
	 */
	void exitBaseType(ZamaniParser.BaseTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeParams}.
	 * @param ctx the parse tree
	 */
	void enterTypeParams(ZamaniParser.TypeParamsContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeParams}.
	 * @param ctx the parse tree
	 */
	void exitTypeParams(ZamaniParser.TypeParamsContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeParam}.
	 * @param ctx the parse tree
	 */
	void enterTypeParam(ZamaniParser.TypeParamContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeParam}.
	 * @param ctx the parse tree
	 */
	void exitTypeParam(ZamaniParser.TypeParamContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeArgs}.
	 * @param ctx the parse tree
	 */
	void enterTypeArgs(ZamaniParser.TypeArgsContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeArgs}.
	 * @param ctx the parse tree
	 */
	void exitTypeArgs(ZamaniParser.TypeArgsContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#structDecl}.
	 * @param ctx the parse tree
	 */
	void enterStructDecl(ZamaniParser.StructDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#structDecl}.
	 * @param ctx the parse tree
	 */
	void exitStructDecl(ZamaniParser.StructDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#structField}.
	 * @param ctx the parse tree
	 */
	void enterStructField(ZamaniParser.StructFieldContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#structField}.
	 * @param ctx the parse tree
	 */
	void exitStructField(ZamaniParser.StructFieldContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#enumDecl}.
	 * @param ctx the parse tree
	 */
	void enterEnumDecl(ZamaniParser.EnumDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#enumDecl}.
	 * @param ctx the parse tree
	 */
	void exitEnumDecl(ZamaniParser.EnumDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#enumVariant}.
	 * @param ctx the parse tree
	 */
	void enterEnumVariant(ZamaniParser.EnumVariantContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#enumVariant}.
	 * @param ctx the parse tree
	 */
	void exitEnumVariant(ZamaniParser.EnumVariantContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#traitDecl}.
	 * @param ctx the parse tree
	 */
	void enterTraitDecl(ZamaniParser.TraitDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#traitDecl}.
	 * @param ctx the parse tree
	 */
	void exitTraitDecl(ZamaniParser.TraitDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#traitItem}.
	 * @param ctx the parse tree
	 */
	void enterTraitItem(ZamaniParser.TraitItemContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#traitItem}.
	 * @param ctx the parse tree
	 */
	void exitTraitItem(ZamaniParser.TraitItemContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#implDecl}.
	 * @param ctx the parse tree
	 */
	void enterImplDecl(ZamaniParser.ImplDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#implDecl}.
	 * @param ctx the parse tree
	 */
	void exitImplDecl(ZamaniParser.ImplDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#implItem}.
	 * @param ctx the parse tree
	 */
	void enterImplItem(ZamaniParser.ImplItemContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#implItem}.
	 * @param ctx the parse tree
	 */
	void exitImplItem(ZamaniParser.ImplItemContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeAliasDecl}.
	 * @param ctx the parse tree
	 */
	void enterTypeAliasDecl(ZamaniParser.TypeAliasDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeAliasDecl}.
	 * @param ctx the parse tree
	 */
	void exitTypeAliasDecl(ZamaniParser.TypeAliasDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#constDecl}.
	 * @param ctx the parse tree
	 */
	void enterConstDecl(ZamaniParser.ConstDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#constDecl}.
	 * @param ctx the parse tree
	 */
	void exitConstDecl(ZamaniParser.ConstDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#classDecl}.
	 * @param ctx the parse tree
	 */
	void enterClassDecl(ZamaniParser.ClassDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#classDecl}.
	 * @param ctx the parse tree
	 */
	void exitClassDecl(ZamaniParser.ClassDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#classItem}.
	 * @param ctx the parse tree
	 */
	void enterClassItem(ZamaniParser.ClassItemContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#classItem}.
	 * @param ctx the parse tree
	 */
	void exitClassItem(ZamaniParser.ClassItemContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#constructorDecl}.
	 * @param ctx the parse tree
	 */
	void enterConstructorDecl(ZamaniParser.ConstructorDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#constructorDecl}.
	 * @param ctx the parse tree
	 */
	void exitConstructorDecl(ZamaniParser.ConstructorDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#destructorDecl}.
	 * @param ctx the parse tree
	 */
	void enterDestructorDecl(ZamaniParser.DestructorDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#destructorDecl}.
	 * @param ctx the parse tree
	 */
	void exitDestructorDecl(ZamaniParser.DestructorDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#interfaceDecl}.
	 * @param ctx the parse tree
	 */
	void enterInterfaceDecl(ZamaniParser.InterfaceDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#interfaceDecl}.
	 * @param ctx the parse tree
	 */
	void exitInterfaceDecl(ZamaniParser.InterfaceDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#recordDecl}.
	 * @param ctx the parse tree
	 */
	void enterRecordDecl(ZamaniParser.RecordDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#recordDecl}.
	 * @param ctx the parse tree
	 */
	void exitRecordDecl(ZamaniParser.RecordDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#quantumCircuitDecl}.
	 * @param ctx the parse tree
	 */
	void enterQuantumCircuitDecl(ZamaniParser.QuantumCircuitDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#quantumCircuitDecl}.
	 * @param ctx the parse tree
	 */
	void exitQuantumCircuitDecl(ZamaniParser.QuantumCircuitDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#nanoAgentDecl}.
	 * @param ctx the parse tree
	 */
	void enterNanoAgentDecl(ZamaniParser.NanoAgentDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#nanoAgentDecl}.
	 * @param ctx the parse tree
	 */
	void exitNanoAgentDecl(ZamaniParser.NanoAgentDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#languageDecl}.
	 * @param ctx the parse tree
	 */
	void enterLanguageDecl(ZamaniParser.LanguageDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#languageDecl}.
	 * @param ctx the parse tree
	 */
	void exitLanguageDecl(ZamaniParser.LanguageDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#effectDecl}.
	 * @param ctx the parse tree
	 */
	void enterEffectDecl(ZamaniParser.EffectDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#effectDecl}.
	 * @param ctx the parse tree
	 */
	void exitEffectDecl(ZamaniParser.EffectDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#effectOp}.
	 * @param ctx the parse tree
	 */
	void enterEffectOp(ZamaniParser.EffectOpContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#effectOp}.
	 * @param ctx the parse tree
	 */
	void exitEffectOp(ZamaniParser.EffectOpContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#effectList}.
	 * @param ctx the parse tree
	 */
	void enterEffectList(ZamaniParser.EffectListContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#effectList}.
	 * @param ctx the parse tree
	 */
	void exitEffectList(ZamaniParser.EffectListContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#effectName}.
	 * @param ctx the parse tree
	 */
	void enterEffectName(ZamaniParser.EffectNameContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#effectName}.
	 * @param ctx the parse tree
	 */
	void exitEffectName(ZamaniParser.EffectNameContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#mtsDecl}.
	 * @param ctx the parse tree
	 */
	void enterMtsDecl(ZamaniParser.MtsDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#mtsDecl}.
	 * @param ctx the parse tree
	 */
	void exitMtsDecl(ZamaniParser.MtsDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sankofaDecl}.
	 * @param ctx the parse tree
	 */
	void enterSankofaDecl(ZamaniParser.SankofaDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sankofaDecl}.
	 * @param ctx the parse tree
	 */
	void exitSankofaDecl(ZamaniParser.SankofaDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#agentDecl}.
	 * @param ctx the parse tree
	 */
	void enterAgentDecl(ZamaniParser.AgentDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#agentDecl}.
	 * @param ctx the parse tree
	 */
	void exitAgentDecl(ZamaniParser.AgentDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#cognitiveBlock}.
	 * @param ctx the parse tree
	 */
	void enterCognitiveBlock(ZamaniParser.CognitiveBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#cognitiveBlock}.
	 * @param ctx the parse tree
	 */
	void exitCognitiveBlock(ZamaniParser.CognitiveBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#metaBlock}.
	 * @param ctx the parse tree
	 */
	void enterMetaBlock(ZamaniParser.MetaBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#metaBlock}.
	 * @param ctx the parse tree
	 */
	void exitMetaBlock(ZamaniParser.MetaBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#hdlModuleDecl}.
	 * @param ctx the parse tree
	 */
	void enterHdlModuleDecl(ZamaniParser.HdlModuleDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#hdlModuleDecl}.
	 * @param ctx the parse tree
	 */
	void exitHdlModuleDecl(ZamaniParser.HdlModuleDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#cloudDecl}.
	 * @param ctx the parse tree
	 */
	void enterCloudDecl(ZamaniParser.CloudDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#cloudDecl}.
	 * @param ctx the parse tree
	 */
	void exitCloudDecl(ZamaniParser.CloudDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#distributedDecl}.
	 * @param ctx the parse tree
	 */
	void enterDistributedDecl(ZamaniParser.DistributedDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#distributedDecl}.
	 * @param ctx the parse tree
	 */
	void exitDistributedDecl(ZamaniParser.DistributedDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#onDeviceAgentDecl}.
	 * @param ctx the parse tree
	 */
	void enterOnDeviceAgentDecl(ZamaniParser.OnDeviceAgentDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#onDeviceAgentDecl}.
	 * @param ctx the parse tree
	 */
	void exitOnDeviceAgentDecl(ZamaniParser.OnDeviceAgentDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#selfEvolveDecl}.
	 * @param ctx the parse tree
	 */
	void enterSelfEvolveDecl(ZamaniParser.SelfEvolveDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#selfEvolveDecl}.
	 * @param ctx the parse tree
	 */
	void exitSelfEvolveDecl(ZamaniParser.SelfEvolveDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#optPassDecl}.
	 * @param ctx the parse tree
	 */
	void enterOptPassDecl(ZamaniParser.OptPassDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#optPassDecl}.
	 * @param ctx the parse tree
	 */
	void exitOptPassDecl(ZamaniParser.OptPassDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#targetPlatform}.
	 * @param ctx the parse tree
	 */
	void enterTargetPlatform(ZamaniParser.TargetPlatformContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#targetPlatform}.
	 * @param ctx the parse tree
	 */
	void exitTargetPlatform(ZamaniParser.TargetPlatformContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#runtimeDecl}.
	 * @param ctx the parse tree
	 */
	void enterRuntimeDecl(ZamaniParser.RuntimeDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#runtimeDecl}.
	 * @param ctx the parse tree
	 */
	void exitRuntimeDecl(ZamaniParser.RuntimeDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#actorDecl}.
	 * @param ctx the parse tree
	 */
	void enterActorDecl(ZamaniParser.ActorDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#actorDecl}.
	 * @param ctx the parse tree
	 */
	void exitActorDecl(ZamaniParser.ActorDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#aiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void enterAiSystemDecl(ZamaniParser.AiSystemDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#aiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void exitAiSystemDecl(ZamaniParser.AiSystemDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#agiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void enterAgiSystemDecl(ZamaniParser.AgiSystemDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#agiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void exitAgiSystemDecl(ZamaniParser.AgiSystemDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#asiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void enterAsiSystemDecl(ZamaniParser.AsiSystemDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#asiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void exitAsiSystemDecl(ZamaniParser.AsiSystemDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#aesiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void enterAesiSystemDecl(ZamaniParser.AesiSystemDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#aesiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void exitAesiSystemDecl(ZamaniParser.AesiSystemDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#asesiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void enterAsesiSystemDecl(ZamaniParser.AsesiSystemDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#asesiSystemDecl}.
	 * @param ctx the parse tree
	 */
	void exitAsesiSystemDecl(ZamaniParser.AsesiSystemDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#adminInterfaceDecl}.
	 * @param ctx the parse tree
	 */
	void enterAdminInterfaceDecl(ZamaniParser.AdminInterfaceDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#adminInterfaceDecl}.
	 * @param ctx the parse tree
	 */
	void exitAdminInterfaceDecl(ZamaniParser.AdminInterfaceDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#paymentGatewayDecl}.
	 * @param ctx the parse tree
	 */
	void enterPaymentGatewayDecl(ZamaniParser.PaymentGatewayDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#paymentGatewayDecl}.
	 * @param ctx the parse tree
	 */
	void exitPaymentGatewayDecl(ZamaniParser.PaymentGatewayDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#userFeedbackDecl}.
	 * @param ctx the parse tree
	 */
	void enterUserFeedbackDecl(ZamaniParser.UserFeedbackDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#userFeedbackDecl}.
	 * @param ctx the parse tree
	 */
	void exitUserFeedbackDecl(ZamaniParser.UserFeedbackDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#copyrightNoticeDecl}.
	 * @param ctx the parse tree
	 */
	void enterCopyrightNoticeDecl(ZamaniParser.CopyrightNoticeDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#copyrightNoticeDecl}.
	 * @param ctx the parse tree
	 */
	void exitCopyrightNoticeDecl(ZamaniParser.CopyrightNoticeDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#tailorMadeFeatureDecl}.
	 * @param ctx the parse tree
	 */
	void enterTailorMadeFeatureDecl(ZamaniParser.TailorMadeFeatureDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#tailorMadeFeatureDecl}.
	 * @param ctx the parse tree
	 */
	void exitTailorMadeFeatureDecl(ZamaniParser.TailorMadeFeatureDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#programOnceDecl}.
	 * @param ctx the parse tree
	 */
	void enterProgramOnceDecl(ZamaniParser.ProgramOnceDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#programOnceDecl}.
	 * @param ctx the parse tree
	 */
	void exitProgramOnceDecl(ZamaniParser.ProgramOnceDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#maliciousIdeaDetection}.
	 * @param ctx the parse tree
	 */
	void enterMaliciousIdeaDetection(ZamaniParser.MaliciousIdeaDetectionContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#maliciousIdeaDetection}.
	 * @param ctx the parse tree
	 */
	void exitMaliciousIdeaDetection(ZamaniParser.MaliciousIdeaDetectionContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#userBlockingDecl}.
	 * @param ctx the parse tree
	 */
	void enterUserBlockingDecl(ZamaniParser.UserBlockingDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#userBlockingDecl}.
	 * @param ctx the parse tree
	 */
	void exitUserBlockingDecl(ZamaniParser.UserBlockingDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#legalActionDecl}.
	 * @param ctx the parse tree
	 */
	void enterLegalActionDecl(ZamaniParser.LegalActionDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#legalActionDecl}.
	 * @param ctx the parse tree
	 */
	void exitLegalActionDecl(ZamaniParser.LegalActionDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sandboxDecl}.
	 * @param ctx the parse tree
	 */
	void enterSandboxDecl(ZamaniParser.SandboxDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sandboxDecl}.
	 * @param ctx the parse tree
	 */
	void exitSandboxDecl(ZamaniParser.SandboxDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalSimulationDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalSimulationDecl(ZamaniParser.OmniversalSimulationDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalSimulationDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalSimulationDecl(ZamaniParser.OmniversalSimulationDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalCodeSynthDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalCodeSynthDecl(ZamaniParser.OmniversalCodeSynthDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalCodeSynthDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalCodeSynthDecl(ZamaniParser.OmniversalCodeSynthDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalDeployDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalDeployDecl(ZamaniParser.OmniversalDeployDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalDeployDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalDeployDecl(ZamaniParser.OmniversalDeployDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalAlignmentDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalAlignmentDecl(ZamaniParser.OmniversalAlignmentDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalAlignmentDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalAlignmentDecl(ZamaniParser.OmniversalAlignmentDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalContainmentDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalContainmentDecl(ZamaniParser.OmniversalContainmentDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalContainmentDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalContainmentDecl(ZamaniParser.OmniversalContainmentDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalTrustDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalTrustDecl(ZamaniParser.OmniversalTrustDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalTrustDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalTrustDecl(ZamaniParser.OmniversalTrustDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalKnowledgeDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalKnowledgeDecl(ZamaniParser.OmniversalKnowledgeDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalKnowledgeDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalKnowledgeDecl(ZamaniParser.OmniversalKnowledgeDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalGenerativeDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalGenerativeDecl(ZamaniParser.OmniversalGenerativeDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalGenerativeDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalGenerativeDecl(ZamaniParser.OmniversalGenerativeDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalSovereigntyDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalSovereigntyDecl(ZamaniParser.OmniversalSovereigntyDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalSovereigntyDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalSovereigntyDecl(ZamaniParser.OmniversalSovereigntyDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalGoalDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalGoalDecl(ZamaniParser.OmniversalGoalDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalGoalDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalGoalDecl(ZamaniParser.OmniversalGoalDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalBioNanoDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalBioNanoDecl(ZamaniParser.OmniversalBioNanoDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalBioNanoDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalBioNanoDecl(ZamaniParser.OmniversalBioNanoDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalRealityDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalRealityDecl(ZamaniParser.OmniversalRealityDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalRealityDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalRealityDecl(ZamaniParser.OmniversalRealityDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#omniversalNlpDecl}.
	 * @param ctx the parse tree
	 */
	void enterOmniversalNlpDecl(ZamaniParser.OmniversalNlpDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#omniversalNlpDecl}.
	 * @param ctx the parse tree
	 */
	void exitOmniversalNlpDecl(ZamaniParser.OmniversalNlpDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#chatArchitectDecl}.
	 * @param ctx the parse tree
	 */
	void enterChatArchitectDecl(ZamaniParser.ChatArchitectDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#chatArchitectDecl}.
	 * @param ctx the parse tree
	 */
	void exitChatArchitectDecl(ZamaniParser.ChatArchitectDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#greenComputingAttr}.
	 * @param ctx the parse tree
	 */
	void enterGreenComputingAttr(ZamaniParser.GreenComputingAttrContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#greenComputingAttr}.
	 * @param ctx the parse tree
	 */
	void exitGreenComputingAttr(ZamaniParser.GreenComputingAttrContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#thermalOptDecl}.
	 * @param ctx the parse tree
	 */
	void enterThermalOptDecl(ZamaniParser.ThermalOptDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#thermalOptDecl}.
	 * @param ctx the parse tree
	 */
	void exitThermalOptDecl(ZamaniParser.ThermalOptDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#resourceConserveDecl}.
	 * @param ctx the parse tree
	 */
	void enterResourceConserveDecl(ZamaniParser.ResourceConserveDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#resourceConserveDecl}.
	 * @param ctx the parse tree
	 */
	void exitResourceConserveDecl(ZamaniParser.ResourceConserveDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#selfDiscoverDecl}.
	 * @param ctx the parse tree
	 */
	void enterSelfDiscoverDecl(ZamaniParser.SelfDiscoverDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#selfDiscoverDecl}.
	 * @param ctx the parse tree
	 */
	void exitSelfDiscoverDecl(ZamaniParser.SelfDiscoverDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#developerAnalyticsDecl}.
	 * @param ctx the parse tree
	 */
	void enterDeveloperAnalyticsDecl(ZamaniParser.DeveloperAnalyticsDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#developerAnalyticsDecl}.
	 * @param ctx the parse tree
	 */
	void exitDeveloperAnalyticsDecl(ZamaniParser.DeveloperAnalyticsDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#licenseTrackingDecl}.
	 * @param ctx the parse tree
	 */
	void enterLicenseTrackingDecl(ZamaniParser.LicenseTrackingDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#licenseTrackingDecl}.
	 * @param ctx the parse tree
	 */
	void exitLicenseTrackingDecl(ZamaniParser.LicenseTrackingDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#deploymentDecl}.
	 * @param ctx the parse tree
	 */
	void enterDeploymentDecl(ZamaniParser.DeploymentDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#deploymentDecl}.
	 * @param ctx the parse tree
	 */
	void exitDeploymentDecl(ZamaniParser.DeploymentDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#versionReleaseDecl}.
	 * @param ctx the parse tree
	 */
	void enterVersionReleaseDecl(ZamaniParser.VersionReleaseDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#versionReleaseDecl}.
	 * @param ctx the parse tree
	 */
	void exitVersionReleaseDecl(ZamaniParser.VersionReleaseDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#lspServerDecl}.
	 * @param ctx the parse tree
	 */
	void enterLspServerDecl(ZamaniParser.LspServerDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#lspServerDecl}.
	 * @param ctx the parse tree
	 */
	void exitLspServerDecl(ZamaniParser.LspServerDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeClassDecl}.
	 * @param ctx the parse tree
	 */
	void enterTypeClassDecl(ZamaniParser.TypeClassDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeClassDecl}.
	 * @param ctx the parse tree
	 */
	void exitTypeClassDecl(ZamaniParser.TypeClassDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeClassInstance}.
	 * @param ctx the parse tree
	 */
	void enterTypeClassInstance(ZamaniParser.TypeClassInstanceContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeClassInstance}.
	 * @param ctx the parse tree
	 */
	void exitTypeClassInstance(ZamaniParser.TypeClassInstanceContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#higherKindedTypeDecl}.
	 * @param ctx the parse tree
	 */
	void enterHigherKindedTypeDecl(ZamaniParser.HigherKindedTypeDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#higherKindedTypeDecl}.
	 * @param ctx the parse tree
	 */
	void exitHigherKindedTypeDecl(ZamaniParser.HigherKindedTypeDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#selfAdjustDecl}.
	 * @param ctx the parse tree
	 */
	void enterSelfAdjustDecl(ZamaniParser.SelfAdjustDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#selfAdjustDecl}.
	 * @param ctx the parse tree
	 */
	void exitSelfAdjustDecl(ZamaniParser.SelfAdjustDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#selfVersioningDecl}.
	 * @param ctx the parse tree
	 */
	void enterSelfVersioningDecl(ZamaniParser.SelfVersioningDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#selfVersioningDecl}.
	 * @param ctx the parse tree
	 */
	void exitSelfVersioningDecl(ZamaniParser.SelfVersioningDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#extensionMethodDecl}.
	 * @param ctx the parse tree
	 */
	void enterExtensionMethodDecl(ZamaniParser.ExtensionMethodDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#extensionMethodDecl}.
	 * @param ctx the parse tree
	 */
	void exitExtensionMethodDecl(ZamaniParser.ExtensionMethodDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#extensionPropertyDecl}.
	 * @param ctx the parse tree
	 */
	void enterExtensionPropertyDecl(ZamaniParser.ExtensionPropertyDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#extensionPropertyDecl}.
	 * @param ctx the parse tree
	 */
	void exitExtensionPropertyDecl(ZamaniParser.ExtensionPropertyDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#extensionIndexerDecl}.
	 * @param ctx the parse tree
	 */
	void enterExtensionIndexerDecl(ZamaniParser.ExtensionIndexerDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#extensionIndexerDecl}.
	 * @param ctx the parse tree
	 */
	void exitExtensionIndexerDecl(ZamaniParser.ExtensionIndexerDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#extensionOperatorDecl}.
	 * @param ctx the parse tree
	 */
	void enterExtensionOperatorDecl(ZamaniParser.ExtensionOperatorDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#extensionOperatorDecl}.
	 * @param ctx the parse tree
	 */
	void exitExtensionOperatorDecl(ZamaniParser.ExtensionOperatorDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#macroDecl}.
	 * @param ctx the parse tree
	 */
	void enterMacroDecl(ZamaniParser.MacroDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#macroDecl}.
	 * @param ctx the parse tree
	 */
	void exitMacroDecl(ZamaniParser.MacroDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#domainSpecificLanguageDecl}.
	 * @param ctx the parse tree
	 */
	void enterDomainSpecificLanguageDecl(ZamaniParser.DomainSpecificLanguageDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#domainSpecificLanguageDecl}.
	 * @param ctx the parse tree
	 */
	void exitDomainSpecificLanguageDecl(ZamaniParser.DomainSpecificLanguageDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#aspectDecl}.
	 * @param ctx the parse tree
	 */
	void enterAspectDecl(ZamaniParser.AspectDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#aspectDecl}.
	 * @param ctx the parse tree
	 */
	void exitAspectDecl(ZamaniParser.AspectDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#typeProviderDecl}.
	 * @param ctx the parse tree
	 */
	void enterTypeProviderDecl(ZamaniParser.TypeProviderDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#typeProviderDecl}.
	 * @param ctx the parse tree
	 */
	void exitTypeProviderDecl(ZamaniParser.TypeProviderDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#dataParallelismDecl}.
	 * @param ctx the parse tree
	 */
	void enterDataParallelismDecl(ZamaniParser.DataParallelismDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#dataParallelismDecl}.
	 * @param ctx the parse tree
	 */
	void exitDataParallelismDecl(ZamaniParser.DataParallelismDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#concurrentDataStructureDecl}.
	 * @param ctx the parse tree
	 */
	void enterConcurrentDataStructureDecl(ZamaniParser.ConcurrentDataStructureDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#concurrentDataStructureDecl}.
	 * @param ctx the parse tree
	 */
	void exitConcurrentDataStructureDecl(ZamaniParser.ConcurrentDataStructureDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#messageHandlerDecl}.
	 * @param ctx the parse tree
	 */
	void enterMessageHandlerDecl(ZamaniParser.MessageHandlerDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#messageHandlerDecl}.
	 * @param ctx the parse tree
	 */
	void exitMessageHandlerDecl(ZamaniParser.MessageHandlerDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#musicDecl}.
	 * @param ctx the parse tree
	 */
	void enterMusicDecl(ZamaniParser.MusicDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#musicDecl}.
	 * @param ctx the parse tree
	 */
	void exitMusicDecl(ZamaniParser.MusicDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#roboticsDecl}.
	 * @param ctx the parse tree
	 */
	void enterRoboticsDecl(ZamaniParser.RoboticsDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#roboticsDecl}.
	 * @param ctx the parse tree
	 */
	void exitRoboticsDecl(ZamaniParser.RoboticsDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#deepLearningDecl}.
	 * @param ctx the parse tree
	 */
	void enterDeepLearningDecl(ZamaniParser.DeepLearningDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#deepLearningDecl}.
	 * @param ctx the parse tree
	 */
	void exitDeepLearningDecl(ZamaniParser.DeepLearningDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#graphicsDecl}.
	 * @param ctx the parse tree
	 */
	void enterGraphicsDecl(ZamaniParser.GraphicsDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#graphicsDecl}.
	 * @param ctx the parse tree
	 */
	void exitGraphicsDecl(ZamaniParser.GraphicsDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#videoDecl}.
	 * @param ctx the parse tree
	 */
	void enterVideoDecl(ZamaniParser.VideoDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#videoDecl}.
	 * @param ctx the parse tree
	 */
	void exitVideoDecl(ZamaniParser.VideoDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#tensorDecl}.
	 * @param ctx the parse tree
	 */
	void enterTensorDecl(ZamaniParser.TensorDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#tensorDecl}.
	 * @param ctx the parse tree
	 */
	void exitTensorDecl(ZamaniParser.TensorDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#matrixDecl}.
	 * @param ctx the parse tree
	 */
	void enterMatrixDecl(ZamaniParser.MatrixDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#matrixDecl}.
	 * @param ctx the parse tree
	 */
	void exitMatrixDecl(ZamaniParser.MatrixDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#vectorDecl}.
	 * @param ctx the parse tree
	 */
	void enterVectorDecl(ZamaniParser.VectorDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#vectorDecl}.
	 * @param ctx the parse tree
	 */
	void exitVectorDecl(ZamaniParser.VectorDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#mlModelDecl}.
	 * @param ctx the parse tree
	 */
	void enterMlModelDecl(ZamaniParser.MlModelDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#mlModelDecl}.
	 * @param ctx the parse tree
	 */
	void exitMlModelDecl(ZamaniParser.MlModelDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#quantumMlBlock}.
	 * @param ctx the parse tree
	 */
	void enterQuantumMlBlock(ZamaniParser.QuantumMlBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#quantumMlBlock}.
	 * @param ctx the parse tree
	 */
	void exitQuantumMlBlock(ZamaniParser.QuantumMlBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#explainableRlBlock}.
	 * @param ctx the parse tree
	 */
	void enterExplainableRlBlock(ZamaniParser.ExplainableRlBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#explainableRlBlock}.
	 * @param ctx the parse tree
	 */
	void exitExplainableRlBlock(ZamaniParser.ExplainableRlBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#explainableDeepLearningBlock}.
	 * @param ctx the parse tree
	 */
	void enterExplainableDeepLearningBlock(ZamaniParser.ExplainableDeepLearningBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#explainableDeepLearningBlock}.
	 * @param ctx the parse tree
	 */
	void exitExplainableDeepLearningBlock(ZamaniParser.ExplainableDeepLearningBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#knowledgeGraphBlock}.
	 * @param ctx the parse tree
	 */
	void enterKnowledgeGraphBlock(ZamaniParser.KnowledgeGraphBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#knowledgeGraphBlock}.
	 * @param ctx the parse tree
	 */
	void exitKnowledgeGraphBlock(ZamaniParser.KnowledgeGraphBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#probabilisticGraphicalModelBlock}.
	 * @param ctx the parse tree
	 */
	void enterProbabilisticGraphicalModelBlock(ZamaniParser.ProbabilisticGraphicalModelBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#probabilisticGraphicalModelBlock}.
	 * @param ctx the parse tree
	 */
	void exitProbabilisticGraphicalModelBlock(ZamaniParser.ProbabilisticGraphicalModelBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#transferLearningBlock}.
	 * @param ctx the parse tree
	 */
	void enterTransferLearningBlock(ZamaniParser.TransferLearningBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#transferLearningBlock}.
	 * @param ctx the parse tree
	 */
	void exitTransferLearningBlock(ZamaniParser.TransferLearningBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#multiAgentBlock}.
	 * @param ctx the parse tree
	 */
	void enterMultiAgentBlock(ZamaniParser.MultiAgentBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#multiAgentBlock}.
	 * @param ctx the parse tree
	 */
	void exitMultiAgentBlock(ZamaniParser.MultiAgentBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#autonomousSystemBlock}.
	 * @param ctx the parse tree
	 */
	void enterAutonomousSystemBlock(ZamaniParser.AutonomousSystemBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#autonomousSystemBlock}.
	 * @param ctx the parse tree
	 */
	void exitAutonomousSystemBlock(ZamaniParser.AutonomousSystemBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#graphModelingBlock}.
	 * @param ctx the parse tree
	 */
	void enterGraphModelingBlock(ZamaniParser.GraphModelingBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#graphModelingBlock}.
	 * @param ctx the parse tree
	 */
	void exitGraphModelingBlock(ZamaniParser.GraphModelingBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#advancedNlpBlock}.
	 * @param ctx the parse tree
	 */
	void enterAdvancedNlpBlock(ZamaniParser.AdvancedNlpBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#advancedNlpBlock}.
	 * @param ctx the parse tree
	 */
	void exitAdvancedNlpBlock(ZamaniParser.AdvancedNlpBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#cognitiveArchitectureBlock}.
	 * @param ctx the parse tree
	 */
	void enterCognitiveArchitectureBlock(ZamaniParser.CognitiveArchitectureBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#cognitiveArchitectureBlock}.
	 * @param ctx the parse tree
	 */
	void exitCognitiveArchitectureBlock(ZamaniParser.CognitiveArchitectureBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#aiForBusinessBlock}.
	 * @param ctx the parse tree
	 */
	void enterAiForBusinessBlock(ZamaniParser.AiForBusinessBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#aiForBusinessBlock}.
	 * @param ctx the parse tree
	 */
	void exitAiForBusinessBlock(ZamaniParser.AiForBusinessBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#vrArInteractionBlock}.
	 * @param ctx the parse tree
	 */
	void enterVrArInteractionBlock(ZamaniParser.VrArInteractionBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#vrArInteractionBlock}.
	 * @param ctx the parse tree
	 */
	void exitVrArInteractionBlock(ZamaniParser.VrArInteractionBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#imageVideoAnalysisBlock}.
	 * @param ctx the parse tree
	 */
	void enterImageVideoAnalysisBlock(ZamaniParser.ImageVideoAnalysisBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#imageVideoAnalysisBlock}.
	 * @param ctx the parse tree
	 */
	void exitImageVideoAnalysisBlock(ZamaniParser.ImageVideoAnalysisBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#fileScopedType}.
	 * @param ctx the parse tree
	 */
	void enterFileScopedType(ZamaniParser.FileScopedTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#fileScopedType}.
	 * @param ctx the parse tree
	 */
	void exitFileScopedType(ZamaniParser.FileScopedTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#hybridDef}.
	 * @param ctx the parse tree
	 */
	void enterHybridDef(ZamaniParser.HybridDefContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#hybridDef}.
	 * @param ctx the parse tree
	 */
	void exitHybridDef(ZamaniParser.HybridDefContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#interfaceDef}.
	 * @param ctx the parse tree
	 */
	void enterInterfaceDef(ZamaniParser.InterfaceDefContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#interfaceDef}.
	 * @param ctx the parse tree
	 */
	void exitInterfaceDef(ZamaniParser.InterfaceDefContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#agentCapability}.
	 * @param ctx the parse tree
	 */
	void enterAgentCapability(ZamaniParser.AgentCapabilityContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#agentCapability}.
	 * @param ctx the parse tree
	 */
	void exitAgentCapability(ZamaniParser.AgentCapabilityContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#agentBehavior}.
	 * @param ctx the parse tree
	 */
	void enterAgentBehavior(ZamaniParser.AgentBehaviorContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#agentBehavior}.
	 * @param ctx the parse tree
	 */
	void exitAgentBehavior(ZamaniParser.AgentBehaviorContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#explainStmt}.
	 * @param ctx the parse tree
	 */
	void enterExplainStmt(ZamaniParser.ExplainStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#explainStmt}.
	 * @param ctx the parse tree
	 */
	void exitExplainStmt(ZamaniParser.ExplainStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#transparentStmt}.
	 * @param ctx the parse tree
	 */
	void enterTransparentStmt(ZamaniParser.TransparentStmtContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#transparentStmt}.
	 * @param ctx the parse tree
	 */
	void exitTransparentStmt(ZamaniParser.TransparentStmtContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#asiCapabilityDef}.
	 * @param ctx the parse tree
	 */
	void enterAsiCapabilityDef(ZamaniParser.AsiCapabilityDefContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#asiCapabilityDef}.
	 * @param ctx the parse tree
	 */
	void exitAsiCapabilityDef(ZamaniParser.AsiCapabilityDefContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#aesiCapabilityDef}.
	 * @param ctx the parse tree
	 */
	void enterAesiCapabilityDef(ZamaniParser.AesiCapabilityDefContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#aesiCapabilityDef}.
	 * @param ctx the parse tree
	 */
	void exitAesiCapabilityDef(ZamaniParser.AesiCapabilityDefContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#asesiCapabilityDef}.
	 * @param ctx the parse tree
	 */
	void enterAsesiCapabilityDef(ZamaniParser.AsesiCapabilityDefContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#asesiCapabilityDef}.
	 * @param ctx the parse tree
	 */
	void exitAsesiCapabilityDef(ZamaniParser.AsesiCapabilityDefContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#docComment}.
	 * @param ctx the parse tree
	 */
	void enterDocComment(ZamaniParser.DocCommentContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#docComment}.
	 * @param ctx the parse tree
	 */
	void exitDocComment(ZamaniParser.DocCommentContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#attributeDecl}.
	 * @param ctx the parse tree
	 */
	void enterAttributeDecl(ZamaniParser.AttributeDeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#attributeDecl}.
	 * @param ctx the parse tree
	 */
	void exitAttributeDecl(ZamaniParser.AttributeDeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#ident}.
	 * @param ctx the parse tree
	 */
	void enterIdent(ZamaniParser.IdentContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#ident}.
	 * @param ctx the parse tree
	 */
	void exitIdent(ZamaniParser.IdentContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#statement}.
	 * @param ctx the parse tree
	 */
	void enterStatement(ZamaniParser.StatementContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#statement}.
	 * @param ctx the parse tree
	 */
	void exitStatement(ZamaniParser.StatementContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#identList}.
	 * @param ctx the parse tree
	 */
	void enterIdentList(ZamaniParser.IdentListContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#identList}.
	 * @param ctx the parse tree
	 */
	void exitIdentList(ZamaniParser.IdentListContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#whereClause}.
	 * @param ctx the parse tree
	 */
	void enterWhereClause(ZamaniParser.WhereClauseContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#whereClause}.
	 * @param ctx the parse tree
	 */
	void exitWhereClause(ZamaniParser.WhereClauseContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteral(ZamaniParser.LiteralContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteral(ZamaniParser.LiteralContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#quantumLit}.
	 * @param ctx the parse tree
	 */
	void enterQuantumLit(ZamaniParser.QuantumLitContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#quantumLit}.
	 * @param ctx the parse tree
	 */
	void exitQuantumLit(ZamaniParser.QuantumLitContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#nanoLit}.
	 * @param ctx the parse tree
	 */
	void enterNanoLit(ZamaniParser.NanoLitContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#nanoLit}.
	 * @param ctx the parse tree
	 */
	void exitNanoLit(ZamaniParser.NanoLitContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#mtsLit}.
	 * @param ctx the parse tree
	 */
	void enterMtsLit(ZamaniParser.MtsLitContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#mtsLit}.
	 * @param ctx the parse tree
	 */
	void exitMtsLit(ZamaniParser.MtsLitContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#rawStringLit}.
	 * @param ctx the parse tree
	 */
	void enterRawStringLit(ZamaniParser.RawStringLitContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#rawStringLit}.
	 * @param ctx the parse tree
	 */
	void exitRawStringLit(ZamaniParser.RawStringLitContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#utf8StringLit}.
	 * @param ctx the parse tree
	 */
	void enterUtf8StringLit(ZamaniParser.Utf8StringLitContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#utf8StringLit}.
	 * @param ctx the parse tree
	 */
	void exitUtf8StringLit(ZamaniParser.Utf8StringLitContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#interpolatedString}.
	 * @param ctx the parse tree
	 */
	void enterInterpolatedString(ZamaniParser.InterpolatedStringContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#interpolatedString}.
	 * @param ctx the parse tree
	 */
	void exitInterpolatedString(ZamaniParser.InterpolatedStringContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#piType}.
	 * @param ctx the parse tree
	 */
	void enterPiType(ZamaniParser.PiTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#piType}.
	 * @param ctx the parse tree
	 */
	void exitPiType(ZamaniParser.PiTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sigmaType}.
	 * @param ctx the parse tree
	 */
	void enterSigmaType(ZamaniParser.SigmaTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sigmaType}.
	 * @param ctx the parse tree
	 */
	void exitSigmaType(ZamaniParser.SigmaTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#identityType}.
	 * @param ctx the parse tree
	 */
	void enterIdentityType(ZamaniParser.IdentityTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#identityType}.
	 * @param ctx the parse tree
	 */
	void exitIdentityType(ZamaniParser.IdentityTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sessionOp}.
	 * @param ctx the parse tree
	 */
	void enterSessionOp(ZamaniParser.SessionOpContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sessionOp}.
	 * @param ctx the parse tree
	 */
	void exitSessionOp(ZamaniParser.SessionOpContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sessionBranch}.
	 * @param ctx the parse tree
	 */
	void enterSessionBranch(ZamaniParser.SessionBranchContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sessionBranch}.
	 * @param ctx the parse tree
	 */
	void exitSessionBranch(ZamaniParser.SessionBranchContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#quantumType}.
	 * @param ctx the parse tree
	 */
	void enterQuantumType(ZamaniParser.QuantumTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#quantumType}.
	 * @param ctx the parse tree
	 */
	void exitQuantumType(ZamaniParser.QuantumTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#nanoType}.
	 * @param ctx the parse tree
	 */
	void enterNanoType(ZamaniParser.NanoTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#nanoType}.
	 * @param ctx the parse tree
	 */
	void exitNanoType(ZamaniParser.NanoTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#mtsType}.
	 * @param ctx the parse tree
	 */
	void enterMtsType(ZamaniParser.MtsTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#mtsType}.
	 * @param ctx the parse tree
	 */
	void exitMtsType(ZamaniParser.MtsTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#sankofaType}.
	 * @param ctx the parse tree
	 */
	void enterSankofaType(ZamaniParser.SankofaTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#sankofaType}.
	 * @param ctx the parse tree
	 */
	void exitSankofaType(ZamaniParser.SankofaTypeContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZamaniParser#cognitiveType}.
	 * @param ctx the parse tree
	 */
	void enterCognitiveType(ZamaniParser.CognitiveTypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZamaniParser#cognitiveType}.
	 * @param ctx the parse tree
	 */
	void exitCognitiveType(ZamaniParser.CognitiveTypeContext ctx);
}
//! Zenith Parser
//!
//! This module implements the parser for the Zenith programming language.
//! It takes a stream of tokens from the lexer and constructs an Abstract Syntax Tree (AST).
//! The parser is responsible for enforcing the grammatical rules of the language.

use crate::lexer::{Lexer, LexerError};
use crate::tokens::{Token, TokenType}; // Keep TokenType for pattern matching within parser
use crate::source_map::Span; // Corrected Span import
use crate::ast;

/// Represents a parsing error.
#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
}

/// The main parser structure.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance.
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        // Initialize current and peek tokens
        parser.next_token();
        parser.next_token();
        parser
    }

    /// Advances the parser's token stream, setting the peek_token as current_token.
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
    }

    /// Consumes the current token if its type matches the expected type, otherwise records an error.
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    /// Checks if the current token's type matches the given type.
    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.as_ref().map_or(false, |t| t.token_type == token_type)
    }

    /// Checks if the peek token's type matches the given type.
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.as_ref().map_or(false, |t| t.token_type == token_type)
    }

    /// Records a parsing error for an unexpected peek token.
    fn peek_error(&mut self, expected_type: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            expected_type,
            self.peek_token.as_ref().map(|t| t.token_type)
        );
        let span = self.peek_token.as_ref().map_or(Span::dummy(), |t| t.span);
        self.errors.push(ParserError { message: msg, span });
    }

    /// Records a parsing error for an unexpected current token.
    fn current_error(&mut self, expected_type: TokenType) {
        let msg = format!(
            "Expected current token to be {:?}, got {:?} instead",
            expected_type,
            self.current_token.as_ref().map(|t| t.token_type)
        );
        let span = self.current_token.as_ref().map_or(Span::dummy(), |t| t.span);
        self.errors.push(ParserError { message: msg, span });
    }

    /// Main parsing entry point.
    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<ParserError>> {
        let mut statements = Vec::new();
        let program_start_span = self.current_token.as_ref().map_or(Span::dummy(), |t| t.span);

        while !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // If parse_statement returns None, it means an error occurred and was recorded.
                // We need to advance to prevent infinite loops, but carefully.
                // For now, simple token advance.
                self.next_token();
            }
        }

        if !self.errors.is_empty() || !self.lexer.get_errors().is_empty() {
            let mut all_errors = self.errors.clone();
            for lex_err in self.lexer.get_errors() {
                all_errors.push(ParserError { message: lex_err.message.clone(), span: lex_err.span });
            }
            Err(all_errors)
        } else {
            let program_end_span = self.current_token.as_ref().map_or(program_start_span, |t| t.span);
            Ok(ast::Program {
                statements,
                span: Span::new(program_start_span.file, program_start_span.start, program_end_span.end, program_start_span.line, program_start_span.column),
            })
        }
    }

    /// Parses a single statement.
    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.as_ref()?.token_type {
            TokenType::KeywordLet => self.parse_let_statement(),
            TokenType::KeywordReturn => self.parse_return_statement(),
            TokenType::KeywordFn => self.parse_function_statement(),
            TokenType::KeywordQuantum => self.parse_quantum_circuit_statement(),
            TokenType::KeywordNano => self.parse_nano_agent_statement(),
            TokenType::KeywordRemember => self.parse_sankofa_memory_statement(),
            TokenType::KeywordType => self.parse_type_declaration_statement(),
            TokenType::KeywordEffect => self.parse_effect_declaration_statement(),
            TokenType::KeywordLanguage => self.parse_language_declaration_statement(),
            TokenType::KeywordWhile => self.parse_while_statement(),
            TokenType::KeywordFor => self.parse_for_statement(),
            TokenType::KeywordBreak => self.parse_break_statement(),
            TokenType::KeywordContinue => self.parse_continue_statement(),
            TokenType::KeywordMatch => self.parse_match_statement(),
            TokenType::KeywordUnsafe => self.parse_unsafe_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Parses a `let` statement.
    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'let'

        let name = self.current_token.as_ref()?.literal.clone();
        let name_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::Identifier)?; // Expect identifier
        self.next_token();

        let mut type_expr = None;
        if self.current_token_is(TokenType::Colon) {
            self.next_token(); // Consume ':'
            type_expr = Some(self.parse_type_expression()?);
        }

        self.expect_peek(TokenType::Assign)?; // Expect '='
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        let end_span = expr.get_span(); // Assuming get_span() method on Expression
        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::Let(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            type_expr,
            expr,
        ))
    }

    /// Parses a `return` statement.
    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'return'

        let expr = self.parse_expression(Precedence::Lowest)?;

        let end_span = expr.get_span();
        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::Return(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            expr,
        ))
    }

    /// Parses a function declaration.
    fn parse_function_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'fn'

        let name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Function name
        self.next_token();

        let params = self.parse_function_parameters()?;

        let mut return_type_expr = None;
        if self.peek_token_is(TokenType::Colon) {
            self.next_token(); // Consume ':'
            self.next_token(); // Consume return type token
            return_type_expr = Some(self.parse_type_expression()?);
        }

        let body = self.parse_block_expression()?;
        let end_span = body.get_span();

        Some(ast::Statement::Function(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            params,
            return_type_expr,
            Box::new(body),
        ))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<ast::Parameter>> {
        let mut parameters = Vec::new();
        self.expect_peek(TokenType::LParen)?; // Expect '('
        self.next_token();

        if !self.current_token_is(TokenType::RParen) {
            loop {
                let param_span_start = self.current_token.as_ref()?.span;
                let name = self.current_token.as_ref()?.literal.clone();
                self.expect_peek(TokenType::Identifier)?; // Parameter name
                self.next_token();

                self.expect_peek(TokenType::Colon)?; // Expect ':'
                self.next_token();

                let param_type = self.parse_type_expression()?;
                let param_span_end = self.current_token.as_ref().map_or(param_span_start, |t| t.span);

                // For now, linear/affine are handled at the TypeExpr level, not directly on Parameter
                parameters.push(ast::Parameter {
                    span: Span::new(param_span_start.file, param_span_start.start, param_span_end.end, param_span_start.line, param_span_start.column),
                    name,
                    param_type,
                    is_linear: false,
                    is_affine: false,
                });

                if self.peek_token_is(TokenType::Comma) {
                    self.next_token(); // Consume ','
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();

        Some(parameters)
    }

    /// Parses a quantum circuit declaration.
    fn parse_quantum_circuit_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'quantum'

        self.expect_peek(TokenType::KeywordCircuit)?; // Expect 'circuit'
        self.next_token();

        let name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Circuit name
        self.next_token();

        let body = self.parse_block_expression()?;
        let end_span = body.get_span();

        Some(ast::Statement::QuantumCircuit(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            Box::new(body),
        ))
    }

    /// Parses a nano-agent declaration.
    fn parse_nano_agent_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'nano'

        self.expect_peek(TokenType::KeywordAgent)?; // Expect 'agent'
        self.next_token();

        let name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Agent name
        self.next_token();

        let body = self.parse_block_expression()?;
        let end_span = body.get_span();

        Some(ast::Statement::NanoAgent(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            Box::new(body),
        ))
    }

    /// Parses a Sankofa memory declaration.
    fn parse_sankofa_memory_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'remember'

        let name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Memory key name
        self.next_token();

        self.expect_peek(TokenType::Assign)?; // Expect '='
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        let end_span = expr.get_span();
        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::SankofaMemory(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            expr,
        ))
    }

    /// Parses a type declaration.
    fn parse_type_declaration_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'type'

        let name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Type name
        self.next_token();

        self.expect_peek(TokenType::Assign)?; // Expect '='
        self.next_token();

        let type_expr = self.parse_type_expression()?;
        let end_span = type_expr.get_span(); // Assuming get_span() on TypeExpr

        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::TypeDeclaration(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            type_expr,
        ))
    }

    /// Parses an effect declaration.
    fn parse_effect_declaration_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'effect'

        let name = self.current_token.as_ref()?.literal.clone();
        let end_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::Identifier)?; // Effect name
        self.next_token();

        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::EffectDeclaration(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, end_span.column),
            name,
        ))
    }

    /// Parses a language declaration.
    fn parse_language_declaration_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'language'

        let name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Language name
        self.next_token();

        self.expect_peek(TokenType::KeywordGrammar)?; // Expect 'grammar'
        self.next_token();

        let grammar_expr = self.parse_expression(Precedence::Lowest)?;
        let end_span = grammar_expr.get_span();

        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::LanguageDeclaration(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            name,
            grammar_expr,
        ))
    }

    /// Parses a `while` statement.
    fn parse_while_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'while'

        let condition = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expression()?;
        let end_span = body.get_span();

        Some(ast::Statement::While(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            Box::new(condition),
            Box::new(body),
        ))
    }

    /// Parses a `for` statement.
    fn parse_for_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'for'

        let iterator_name = self.current_token.as_ref()?.literal.clone();
        let iterator_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::Identifier)?; // Iterator variable name
        self.next_token();

        self.expect_peek(TokenType::KeywordIn)?; // Expect 'in'
        self.next_token();

        let iterable = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expression()?;
        let end_span = body.get_span();

        Some(ast::Statement::For(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            ast::Identifier(iterator_name, iterator_span),
            Box::new(iterable),
            Box::new(body),
        ))
    }

    /// Parses a `break` statement.
    fn parse_break_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        let end_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'break'

        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::Break(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column)
        ))
    }

    /// Parses a `continue` statement.
    fn parse_continue_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        let end_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'continue'

        self.expect_peek(TokenType::Semicolon)?; // Expect ';'
        self.next_token();

        Some(ast::Statement::Continue(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column)
        ))
    }

    /// Parses a `match` statement.
    fn parse_match_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'match'

        let matched_expr = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(TokenType::LBrace)?; // Expect '{'
        self.next_token();

        let mut cases = Vec::new();
        while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
            if let Some(case) = self.parse_match_case() {
                cases.push(case);
            } else {
                // Error recovery: skip until next '}' or EOF
                while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
                    self.next_token();
                }
                break;
            }
        }

        let end_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::RBrace)?; // Expect '}'
        self.next_token();

        Some(ast::Statement::Match(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            Box::new(matched_expr),
            cases,
        ))
    }

    fn parse_match_case(&mut self) -> Option<ast::MatchCase> {
        let start_span = self.current_token.as_ref()?.span;
        let pattern = self.parse_expression(Precedence::Lowest)?; // Pattern can be an expression or a literal

        self.expect_peek(TokenType::DoubleArrow)?; // Expect '=>'
        self.next_token();

        let body = self.parse_expression(Precedence::Lowest)?;
        let end_span = body.get_span();

        // Optional comma separator for match cases
        if self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
        }

        Some(ast::MatchCase {
            span: Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            pattern,
            body,
        })
    }

    /// Parses an `unsafe` statement.
    fn parse_unsafe_statement(&mut self) -> Option<ast::Statement> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'unsafe'

        let mut proof_string: Option<String> = None;
        if self.current_token_is(TokenType::Bang) {
            self.next_token(); // Consume '!'
            if self.current_token_is(TokenType::LParen) {
                self.next_token(); // Consume '('
                // Expect 'evas' identifier
                if self.current_token_is(TokenType::Identifier) && self.current_token.as_ref()?.literal == "evas" {
                    self.next_token(); // Consume 'evas'
                    self.expect_peek(TokenType::Colon)?; // Expect ':'
                    self.next_token();
                    if self.current_token_is(TokenType::LBrace) {
                        self.next_token(); // Consume '{'
                        if self.current_token_is(TokenType::String) {
                            proof_string = Some(self.current_token.as_ref()?.literal.clone());
                            self.next_token(); // Consume string literal
                        }
                        self.expect_peek(TokenType::RBrace)?; // Expect '}'
                        self.next_token();
                    } else {
                        self.errors.push(ParserError { message: "Expected '{' after 'evas:' in unsafe proof.".to_string(), span: self.current_token.as_ref()?.span });
                    }
                } else {
                    self.errors.push(ParserError { message: "Expected 'evas' identifier in unsafe proof.".to_string(), span: self.current_token.as_ref()?.span });
                }
                self.expect_peek(TokenType::RParen)?; // Expect ')'
                self.next_token();
            }
        }

        let body = self.parse_block_expression()?;
        let end_span = body.get_span();

        Some(ast::Statement::Unsafe(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            proof_string,
            Box::new(body),
        ))
    }

    /// Parses an expression used as a statement (e.g., function call).
    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        let end_span = expr.get_span();

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token(); // Consume ';'
            self.next_token();
        }

        Some(ast::Statement::Expression(expr))
    }

    /// Parses a block expression (e.g., `{ let x = 5; x + 1 }`).
    fn parse_block_expression(&mut self) -> Option<ast::Expression> {
        let start_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::LBrace)?; // Expect '{'
        self.next_token();

        let mut statements = Vec::new();
        while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // Error recovery: skip tokens until next '}' or EOF
                while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
                    self.next_token();
                }
                break;
            }
        }

        let end_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::RBrace)?; // Expect '}'
        self.next_token();

        Some(ast::Expression::Block(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            statements,
        ))
    }

    /// Parses any expression based on precedence climbing.
    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        let mut left_expr = self.parse_prefix_expression()?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            self.next_token(); // Advance to infix operator
            left_expr = self.parse_infix_expression(left_expr)?;
        }

        Some(left_expr)
    }

    /// Parses a prefix expression (e.g., `-5`, `!true`).
    fn parse_prefix_expression(&mut self) -> Option<ast::Expression> {
        let start_span = self.current_token.as_ref()?.span;
        let current_token_type = self.current_token.as_ref()?.token_type;

        match current_token_type {
            TokenType::Identifier => self.parse_identifier_expression(),
            TokenType::Integer | TokenType::Float | TokenType::String | TokenType::Char | TokenType::QuantumLiteral | TokenType::MTSLiteral => self.parse_literal_expression(),
            TokenType::KeywordTrue | TokenType::KeywordFalse => self.parse_boolean_literal(),
            TokenType::Bang | TokenType::Minus => {
                self.next_token(); // Consume operator
                let right = self.parse_expression(Precedence::Prefix)?; // Parse operand with higher precedence
                let end_span = right.get_span();
                Some(ast::Expression::Prefix(
                    Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
                    current_token_type,
                    Box::new(right),
                ))
            }
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::LBrace => self.parse_block_expression(), // Blocks can also be expressions
            _ => {
                self.errors.push(ParserError { message: format!("Unexpected token {:?} for prefix expression.", current_token_type), span: start_span });
                None
            }
        }
    }

    /// Parses an infix expression (e.g., `a + b`, `x == y`).
    fn parse_infix_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let current_token = self.current_token.clone()?;
        let current_token_type = current_token.token_type;
        let precedence = self.current_precedence();
        self.next_token(); // Consume operator

        let right = self.parse_expression(precedence)?;
        let end_span = right.get_span();

        Some(ast::Expression::Infix(
            Span::new(left.get_span().file, left.get_span().start, end_span.end, left.get_span().line, left.get_span().column),
            Box::new(left),
            current_token_type,
            Box::new(right),
        ))
    }

    /// Parses an identifier expression.
    fn parse_identifier_expression(&mut self) -> Option<ast::Expression> {
        let ident = ast::Identifier(
            self.current_token.as_ref()?.literal.clone(),
            self.current_token.as_ref()?.span,
        );
        self.next_token();

        // Handle function calls if next token is '('
        if self.current_token_is(TokenType::LParen) {
            return self.parse_call_expression(ast::Expression::Identifier(ident));
        }
        // Handle indexing if next token is '['
        if self.current_token_is(TokenType::LBracket) {
            return self.parse_index_expression(ast::Expression::Identifier(ident));
        }
        // Handle member access if next token is '.'
        if self.current_token_is(TokenType::Dot) {
            return self.parse_member_access_expression(ast::Expression::Identifier(ident));
        }

        Some(ast::Expression::Identifier(ident))
    }

    /// Parses a literal expression.
    fn parse_literal_expression(&mut self) -> Option<ast::Expression> {
        let token = self.current_token.clone()?;
        let span = token.span;
        let literal_val = match token.token_type {
            TokenType::Integer => ast::Literal::Integer(token.literal, span),
            TokenType::Float => ast::Literal::Float(token.literal, span),
            TokenType::String => ast::Literal::String(token.literal, span),
            TokenType::Char => ast::Literal::Char(token.literal, span),
            TokenType::QuantumLiteral => ast::Literal::Quantum(token.literal, span),
            TokenType::MTSLiteral => ast::Literal::MTS(token.literal, span),
            _ => {
                self.errors.push(ParserError { message: format!("Unexpected token {:?} for literal expression.", token.token_type), span });
                return None;
            }
        };
        self.next_token();
        Some(ast::Expression::Literal(literal_val))
    }

    /// Parses a boolean literal (true/false).
    fn parse_boolean_literal(&mut self) -> Option<ast::Expression> {
        let token = self.current_token.clone()?;
        let span = token.span;
        let value = token.token_type == TokenType::KeywordTrue;
        self.next_token();
        Some(ast::Expression::Literal(ast::Literal::Boolean(value, span)))
    }

    /// Parses a grouped expression (e.g., `(1 + 2)`).
    fn parse_grouped_expression(&mut self) -> Option<ast::Expression> {
        self.next_token(); // Consume '('
        let expr = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();
        Some(expr)
    }

    /// Parses a function call expression.
    fn parse_call_expression(&mut self, function: ast::Expression) -> Option<ast::Expression> {
        let start_span = function.get_span();
        self.next_token(); // Consume '('

        let mut args = Vec::new();
        if !self.current_token_is(TokenType::RParen) {
            loop {
                args.push(self.parse_expression(Precedence::Lowest)?);
                if self.peek_token_is(TokenType::Comma) {
                    self.next_token(); // Consume ','
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        let end_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();

        Some(ast::Expression::Call(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            Box::new(function),
            args,
        ))
    }

    /// Parses an index expression (e.g., `array[index]`).
    fn parse_index_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let start_span = left.get_span();
        self.next_token(); // Consume '['

        let index_expr = self.parse_expression(Precedence::Lowest)?;

        let end_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::RBracket)?; // Expect ']'
        self.next_token();

        Some(ast::Expression::Index(
            Span::new(start_span.file, start_span.start, end_span.end, start_span.line, start_span.column),
            Box::new(left),
            Box::new(index_expr),
        ))
    }

    /// Parses a member access expression (e.g., `object.member`).
    fn parse_member_access_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let start_span = left.get_span();
        self.next_token(); // Consume '.'

        let member_name = self.current_token.as_ref()?.literal.clone();
        let member_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::Identifier)?; // Member name
        self.next_token();

        Some(ast::Expression::MemberAccess(
            Span::new(start_span.file, start_span.start, member_span.end, start_span.line, start_span.column),
            Box::new(left),
            ast::Identifier(member_name, member_span),
        ))
    }

    /// Parses a type expression.
    fn parse_type_expression(&mut self) -> Option<ast::TypeExpr> {
        let start_span = self.current_token.as_ref()?.span;
        let current_token_type = self.current_token.as_ref()?.token_type;
        let current_token_literal = self.current_token.as_ref()?.literal.clone();

        match current_token_type {
            TokenType::Identifier | TokenType::KeywordInt | TokenType::KeywordFloat | TokenType::KeywordBool | TokenType::KeywordChar | TokenType::KeywordString | TokenType::KeywordQubit | TokenType::KeywordNanoAgent | TokenType::KeywordHistory | TokenType::KeywordConsensusTrue | TokenType::KeywordInterMemory | TokenType::KeywordSuperposition | TokenType::KeywordEntangled | TokenType::KeywordQMeasured | TokenType::KeywordArchaeve => {
                let ident = ast::Identifier(current_token_literal, start_span);
                self.next_token(); // Consume identifier/keyword

                // Handle generics: Type<Arg1, Arg2>
                if self.current_token_is(TokenType::LT) {
                    // NOTE: This will consume '<', which might be a problem if it's actually a comparison operator.
                    // Proper parsing would require lookahead or a more sophisticated precedence check.
                    self.next_token(); // Consume '<'
                    let mut generic_args = Vec::new();
                    while !self.current_token_is(TokenType::GT) && !self.current_token_is(TokenType::EOF) {
                        generic_args.push(self.parse_type_expression()?);
                        if self.peek_token_is(TokenType::Comma) {
                            self.next_token(); // Consume ','
                            self.next_token();
                        } else {
                            break;
                        }
                    }
                    self.expect_peek(TokenType::GT)?; // Expect '>'
                    self.next_token();
                    return Some(ast::TypeExpr::Generic(ident, generic_args));
                }

                // Handle Array/QReg: Type[size]
                if self.current_token_is(TokenType::LBracket) {
                    self.next_token(); // Consume '['
                    let size_literal = self.current_token.as_ref()?.literal.clone(); // Expect integer literal for size
                    self.expect_peek(TokenType::Integer)?; 
                    self.next_token(); // Consume integer
                    self.expect_peek(TokenType::RBracket)?; // Expect ']'
                    self.next_token();
                    // Differentiate between generic array and specific QReg
                    if ident.0 == "QReg" || ident.0 == "Qubit" { // 'QReg' is not in keywords yet, but if it were an alias for Qubit[]
                        return Some(ast::TypeExpr::QuantumReg(Box::new(ast::TypeExpr::Base(ast::Identifier("Qubit".to_string(), Span::dummy()))), size_literal));
                    } else if ident.0 == "MtsSlice" {
                         return Some(ast::TypeExpr::MtsSlice(Box::new(ast::TypeExpr::Base(ast::Identifier("Any".to_string(), Span::dummy()))), Some(size_literal)));
                    } else if ident.0 == "History" {
                        return Some(ast::TypeExpr::HistoryType(Box::new(ast::TypeExpr::Base(ast::Identifier("Any".to_string(), Span::dummy()))), Some(size_literal))); // Size for years
                    }
                    return Some(ast::TypeExpr::Array(Box::new(ast::TypeExpr::Base(ident)), Some(size_literal)));
                }

                // Handle Linear/Affine/Effectful prefixes (if they were part of type expression parsing, currently keywords)
                // For now, these are keywords that would modify the *subsequent* type, rather than being part of the base type name.
                // The AST's `TypeExpr::Linear` etc. would be constructed by the parser if it saw `linear` then a type. 
                // This match arm handles the BASE type itself.
                Some(ast::TypeExpr::Base(ident))
            }
            TokenType::KeywordFn => self.parse_function_type_expression(),
            TokenType::LParen => self.parse_tuple_type_expression(),
            TokenType::KeywordLinear => {
                let linear_span = start_span;
                self.next_token(); // Consume 'linear'
                let inner_type = self.parse_type_expression()?;
                let end_span = inner_type.get_span();
                Some(ast::TypeExpr::Linear(Box::new(inner_type)))
            }
            TokenType::KeywordAffine => {
                let affine_span = start_span;
                self.next_token(); // Consume 'affine'
                let inner_type = self.parse_type_expression()?;
                let end_span = inner_type.get_span();
                Some(ast::TypeExpr::Affine(Box::new(inner_type)))
            }
            TokenType::PiSymbol => self.parse_pi_type_expression(),
            TokenType::SigmaSymbol => self.parse_sigma_type_expression(),
            _ => {
                self.errors.push(ParserError { message: format!("Unexpected token {:?} for type expression.", current_token_type), span: start_span });
                None
            }
        }
    }

    /// Parses a function type expression (e.g., `fn(int, bool) -> float`).
    fn parse_function_type_expression(&mut self) -> Option<ast::TypeExpr> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'fn'

        self.expect_peek(TokenType::LParen)?; // Expect '('
        self.next_token();

        let mut param_types = Vec::new();
        if !self.current_token_is(TokenType::RParen) {
            loop {
                param_types.push(self.parse_type_expression()?);
                if self.peek_token_is(TokenType::Comma) {
                    self.next_token(); // Consume ','
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();

        self.expect_peek(TokenType::Arrow)?; // Expect '->' (assuming a new TokenType::Arrow)
        self.next_token();

        let return_type = self.parse_type_expression()?;
        let end_span = return_type.get_span();

        Some(ast::TypeExpr::FunctionType(param_types, Box::new(return_type)))
    }

    /// Parses a tuple type expression (e.g., `(int, bool)`).
    fn parse_tuple_type_expression(&mut self) -> Option<ast::TypeExpr> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume '('

        let mut types = Vec::new();
        if !self.current_token_is(TokenType::RParen) {
            loop {
                types.push(self.parse_type_expression()?);
                if self.peek_token_is(TokenType::Comma) {
                    self.next_token(); // Consume ','
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        let end_span = self.current_token.as_ref()?.span;
        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();

        Some(ast::TypeExpr::Tuple(types))
    }

    /// Parses a Pi-type expression (e.g., `Π(x: int) -> bool`).
    fn parse_pi_type_expression(&mut self) -> Option<ast::TypeExpr> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'Π'

        self.expect_peek(TokenType::LParen)?; // Expect '('
        self.next_token();

        let binder_name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Binder name
        self.next_token();

        self.expect_peek(TokenType::Colon)?; // Expect ':'
        self.next_token();

        let binder_type = self.parse_type_expression()?;

        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();

        self.expect_peek(TokenType::Arrow)?; // Expect '->'
        self.next_token();

        let return_type = self.parse_type_expression()?;
        let end_span = return_type.get_span();

        Some(ast::TypeExpr::PiType(
            binder_name,
            Box::new(binder_type),
            Box::new(return_type),
        ))
    }

    /// Parses a Sigma-type expression (e.g., `Σ(x: int) x bool`).
    fn parse_sigma_type_expression(&mut self) -> Option<ast::TypeExpr> {
        let start_span = self.current_token.as_ref()?.span;
        self.next_token(); // Consume 'Σ'

        self.expect_peek(TokenType::LParen)?; // Expect '('
        self.next_token();

        let binder_name = self.current_token.as_ref()?.literal.clone();
        self.expect_peek(TokenType::Identifier)?; // Binder name
        self.next_token();

        self.expect_peek(TokenType::Colon)?; // Expect ':'
        self.next_token();

        let first_type = self.parse_type_expression()?;

        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token();

        // Expect 'x' or similar separator for dependent pair
        self.expect_peek(TokenType::Star)?; // Using Star for 'x'
        self.next_token();

        let second_type = self.parse_type_expression()?;
        let end_span = second_type.get_span();

        Some(ast::TypeExpr::SigmaType(
            binder_name,
            Box::new(first_type),
            Box::new(second_type),
        ))
    }

    // --- Helper functions for precedence parsing ---

    /// Defines operator precedence levels.
    fn current_precedence(&self) -> Precedence {
        self.current_token.as_ref().map_or(Precedence::Lowest, |t| t.token_type.into())
    }

    fn peek_precedence(&self) -> Precedence {
        self.peek_token.as_ref().map_or(Precedence::Lowest, |t| t.token_type.into())
    }

    pub fn get_errors(&self) -> &[ParserError] {
        &self.errors
    }
}

/// Operator precedence levels.
#[derive(PartialEq, PartialOrd, Debug)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // + or -
    Product,     // * or /
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // myArray[X]
    Member,      // myObject.myMember
}

impl From<TokenType> for Precedence {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Equals | TokenType::NotEquals => Precedence::Equals,
            TokenType::LT | TokenType::GT | TokenType::LTE | TokenType::GTE => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Star | TokenType::Slash => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            TokenType::LBracket => Precedence::Index,
            TokenType::Dot => Precedence::Member,
            _ => Precedence::Lowest,
        }
    }
}

// --- Trait to help get span from Expressions/TypeExprs ---
// This is a common utility that might live in a separate `util` or `ast_utils` module
pub trait GetSpan {
    fn get_span(&self) -> Span;
}

impl GetSpan for ast::Expression {
    fn get_span(&self) -> Span {
        match self {
            ast::Expression::Literal(_, span) => *span,
            ast::Expression::Identifier(ident) => ident.1,
            ast::Expression::Prefix(span, _, _) => *span,
            ast::Expression::Infix(span, _, _, _) => *span,
            ast::Expression::If(span, _, _, _) => *span,
            ast::Expression::Block(span, _) => *span,
            ast::Expression::Call(span, _, _) => *span,
            ast::Expression::Index(span, _, _) => *span,
            ast::Expression::MemberAccess(span, _, _) => *span,
            ast::Expression::QuantumGateApplication(span, _, _) => *span,
            ast::Expression::NanoAction(span, _, _) => *span,
            ast::Expression::MtsOperation(span, _, _) => *span,
            ast::Expression::PerformEffect(span, _, _) => *span,
        }
    }
}

impl GetSpan for ast::TypeExpr {
    fn get_span(&self) -> Span {
        match self {
            ast::TypeExpr::Base(ident) => ident.1,
            ast::TypeExpr::Array(inner, _) => inner.get_span(),
            ast::TypeExpr::FunctionType(params, ret) => {
                // Span from first param to return type
                let start = params.first().map_or(Span::dummy(), |t| t.get_span()).start;
                let end = ret.get_span().end;
                Span::new(ret.get_span().file, start, end, ret.get_span().line, ret.get_span().column)
            }
            ast::TypeExpr::Tuple(types) => {
                let start = types.first().map_or(Span::dummy(), |t| t.get_span()).start;
                let end = types.last().map_or(Span::dummy(), |t| t.get_span()).end;
                Span::new(types.first().map_or(Span::dummy(), |t| t.get_span()).file, start, end, types.first().map_or(Span::dummy(), |t| t.get_span()).line, types.first().map_or(Span::dummy(), |t| t.get_span()).column)
            }
            ast::TypeExpr::Generic(ident, _) => ident.1,
            ast::TypeExpr::Linear(inner) => inner.get_span(),
            ast::TypeExpr::Affine(inner) => inner.get_span(),
            ast::TypeExpr::Effectful(inner, _) => inner.get_span(),
            ast::TypeExpr::Dependent(inner, _) => inner.get_span(),
            ast::TypeExpr::PiType(_, _, ret) => ret.get_span(),
            ast::TypeExpr::SigmaType(_, _, second) => second.get_span(),
            ast::TypeExpr::Proof(inner, _) => inner.get_span(),
            ast::TypeExpr::TypeFamily(ident, _) => ident.1,
            ast::TypeExpr::QuantumReg(base, _) => base.get_span(),
            ast::TypeExpr::Superposition(inner) => inner.get_span(),
            ast::TypeExpr::Entangled(types) => types.first().map_or(Span::dummy(), |t| t.get_span()),
            ast::TypeExpr::QMeasured(inner) => inner.get_span(),
            ast::TypeExpr::NanoAgentType(inner) => inner.get_span(),
            ast::TypeExpr::ArchaeveType(inner) => inner.get_span(),
            ast::TypeExpr::MtsSlice(inner, _) => inner.get_span(),
            ast::TypeExpr::HistoryType(inner, _) => inner.get_span(),
            ast::TypeExpr::ConsensusTrueType(inner) => inner.get_span(),
            ast::TypeExpr::InterMemoryType(_, inner) => inner.get_span(),
            ast::TypeExpr::Error(span) => *span,
        }
    }
}

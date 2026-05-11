
//! Zenith Universal Meta-Compiler (UMC) Parser
//!
//! This module implements the parsing phase of the Zenith compiler. It takes a
//! stream of tokens from the lexer and constructs an Abstract Syntax Tree (AST)
//! representing the program's structure. The parser is responsible for enforcing
//! Zenith's grammar rules and reporting syntax errors.

use crate::ast::{
    Program, Statement, Expression, Literal, Identifier, Parameter, MatchCase,
    AccessModifier, ClassMember, InterfaceMember, TypeExpr, MethodModifier
}; // Import MethodModifier
use crate::lexer::{Lexer, Token, TokenType, LexerError};
use crate::source_map::{Span, FileId, BytePos};
// use crate::compiler_types::TypeExpr; // No longer needed here if TypeExpr is in AST

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

// Precedence values for infix operators
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // + -
    Product,     // * /
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // myArray[X]
    Member,      // object.member or object.method()
    New,         // new Class()
}

fn token_precedence(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Equals | TokenType::NotEquals => Precedence::Equals,
        TokenType::LessThan | TokenType::GreaterThan | TokenType::LessThanEqual | TokenType::GreaterThanEqual => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Star | TokenType::Slash | TokenType::Modulo => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        TokenType::Dot => Precedence::Member,
        // TokenType::KeywordNew => Precedence::New, // Handled as prefix in parse_expression
        _ => Precedence::Lowest,
    }
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        // Propagate lexer errors
        self.errors.extend(self.lexer.get_errors().iter().cloned().map(|e| ParserError { message: e.message, span: e.span }));
        // Clear lexer errors after propagating
        self.lexer.errors.clear();
    }

    fn current_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_current(&mut self, token_type: TokenType) -> Option<Token> {
        if self.current_is(token_type.clone()) {
            let token = self.current_token.clone();
            self.next_token();
            Some(token)
        } else {
            self.add_error(format!("Expected {:?}, found {:?}", token_type, self.current_token.token_type), self.current_token.span.clone());
            None
        }
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_is(token_type.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn add_error(&mut self, message: String, span: Span) {
        self.errors.push(ParserError { message, span });
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!("Expected next token to be {:?}, got {:?} instead",
                          token_type,
                          self.peek_token.token_type);
        self.errors.push(ParserError { message: msg, span: self.peek_token.span.clone() });
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.current_token.token_type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            // Ensure progress, avoid infinite loops on invalid statements
            if self.current_token.token_type != TokenType::EOF && 
               self.current_token.token_type != TokenType::Semicolon && 
               self.current_token.token_type != TokenType::RBrace &&
               !self.lexer.get_errors().is_empty() // If lexer produced errors, we might be stuck
            {
                // Try to synchronize by skipping to the next likely statement start or semicolon
                while self.current_token.token_type != TokenType::Semicolon &&
                      self.current_token.token_type != TokenType::RBrace &&
                      self.current_token.token_type != TokenType::EOF &&
                      !self.current_is(TokenType::KeywordFn) &&
                      !self.current_is(TokenType::KeywordLet) &&
                      !self.current_is(TokenType::KeywordClass) &&
                      !self.current_is(TokenType::KeywordInterface) // New sync points
                {
                    self.next_token();
                }
            }
            if self.current_is(TokenType::Semicolon) { self.next_token(); } // Consume trailing semicolon
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::KeywordLet => self.parse_let_statement(),
            TokenType::KeywordReturn => self.parse_return_statement(),
            TokenType::KeywordFn => self.parse_function_declaration(),
            TokenType::KeywordQuantum => self.parse_quantum_circuit_declaration(),
            TokenType::KeywordNano => self.parse_nano_agent_declaration(),
            TokenType::KeywordRemember => self.parse_sankofa_memory_declaration(),
            TokenType::KeywordType => self.parse_type_definition_statement(),
            TokenType::KeywordEffect => self.parse_effect_declaration(),
            TokenType::KeywordLanguage => self.parse_language_declaration(),
            TokenType::KeywordWhile => self.parse_while_statement(),
            TokenType::KeywordFor => self.parse_for_statement(),
            TokenType::KeywordBreak => self.parse_break_statement(),
            TokenType::KeywordContinue => self.parse_continue_statement(),
            TokenType::KeywordMatch => self.parse_match_statement(),
            TokenType::KeywordUnsafe => self.parse_unsafe_statement(),
            TokenType::KeywordHandle => self.parse_handle_statement(),
            
            // --- OOP Additions ---
            TokenType::KeywordClass => self.parse_class_declaration(),
            TokenType::KeywordInterface => self.parse_interface_declaration(),

            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordLet)?; // Consume 'let'
        let name = self.parse_identifier()?;
        let typ = if self.peek_is(TokenType::Colon) {
            self.next_token(); // Consume ':'
            self.next_token(); // Move to type
            Some(self.parse_type_expression()?)
        } else { None };
        self.expect_peek(TokenType::Assign)?; // Expect '=', consume if present
        self.next_token(); // Consume '='
        let expression = self.parse_expression(Precedence::Lowest)?; // Parse the expression
        let span_end = expression.span();
        if self.peek_is(TokenType::Semicolon) { self.next_token(); } // Consume optional semicolon
        Some(Statement::Let(span_start.merge(&span_end), name.0, typ, expression))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.next_token(); // Consume 'return'
        let expression = self.parse_expression(Precedence::Lowest)?; // Parse the expression
        let span_end = expression.span();
        if self.peek_is(TokenType::Semicolon) { self.next_token(); } // Consume optional semicolon
        Some(Statement::Return(span_start.merge(&span_end), expression))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?; // Parse the expression
        let span_start = expression.span();
        // Semicolon is optional for the last expression in a block or top-level
        if self.peek_is(TokenType::Semicolon) { self.next_token(); } 
        Some(Statement::Expression(expression))
    }

    fn parse_block_expression(&mut self) -> Option<Expression> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::LBrace)?; // Consume '{'
        let mut statements = Vec::new();
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            if self.current_is(TokenType::Semicolon) { self.next_token(); } // Consume optional semicolon
        }
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RBrace)?; // Consume '}'
        Some(Expression::Block(span_start.merge(&span_end), statements))
    }

    fn parse_function_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordFn)?; // Consume 'fn'
        let name = self.parse_identifier()?;
        self.expect_current(TokenType::LParen)?; // Consume '('
        let parameters = self.parse_function_parameters()?;
        self.expect_current(TokenType::RParen)?; // Consume ')'
        let return_type = if self.peek_is(TokenType::ThinArrow) {
            self.next_token(); // Consume '->'
            self.next_token(); // Move to type
            Some(self.parse_type_expression()?)
        } else { None };
        let effects = if self.current_is(TokenType::KeywordWith) {
            self.next_token(); // consume 'with'
            self.expect_current(TokenType::KeywordEffects)?; // consume 'effects'
            self.expect_current(TokenType::LBrace)?; // consume '{'
            let mut effect_ids = Vec::new();
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                effect_ids.push(self.parse_identifier()?);
                if self.current_is(TokenType::Comma) { self.next_token(); }
            }
            self.expect_current(TokenType::RBrace)?; // consume '}'
            effect_ids
        } else { Vec::new() };
        let body = self.parse_block_expression()?;
        let span_end = body.span();
        Some(Statement::Function(span_start.merge(&span_end), name.0, parameters, return_type, body))
    }

    fn parse_function_parameters(&mut self) -> Vec<Parameter> {
        let mut parameters = Vec::new();
        if self.peek_is(TokenType::RParen) { // No parameters
            return parameters;
        }
        self.next_token(); // Move to first parameter

        loop {
            let name = self.parse_identifier()?;
            self.expect_current(TokenType::Colon)?; // Consume ':'
            let typ = self.parse_type_expression()?; // Parse type annotation
            parameters.push(Parameter { name, typ: Some(typ) });

            if self.peek_is(TokenType::Comma) {
                self.next_token(); // Consume ','
                self.next_token(); // Move to next parameter name
            } else {
                break;
            }
        }
        parameters
    }

    fn parse_quantum_circuit_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordQuantum)?; // Consume 'quantum'
        self.expect_current(TokenType::KeywordCircuit)?; // Consume 'circuit'
        let name = self.parse_identifier()?;
        let body = self.parse_block_expression()?;
        let span_end = body.span();
        Some(Statement::QuantumCircuit(span_start.merge(&span_end), name.0, body))
    }

    fn parse_nano_agent_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordNano)?; // Consume 'nano'
        self.expect_current(TokenType::KeywordAgent)?; // Consume 'agent'
        let name = self.parse_identifier()?;
        let body = self.parse_block_expression()?;
        let span_end = body.span();
        Some(Statement::NanoAgent(span_start.merge(&span_end), name.0, body))
    }

    fn parse_sankofa_memory_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordRemember)?; // Consume 'remember'
        let name = self.parse_identifier()?;
        self.expect_peek(TokenType::Assign)?; // Expect '=', consume if present
        self.next_token(); // Consume '='
        let expression = self.parse_expression(Precedence::Lowest)?; // Parse the expression
        let span_end = expression.span();
        if self.peek_is(TokenType::Semicolon) { self.next_token(); } // Consume optional semicolon
        Some(Statement::SankofaMemory(span_start.merge(&span_end), name.0, expression))
    }

    fn parse_type_definition_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordType)?; // Consume 'type'
        let name = self.parse_identifier()?;
        self.expect_current(TokenType::Assign)?; // Expect '=', consume
        let type_expr = self.parse_type_expression()?;
        let span_end = type_expr.span();
        self.expect_current(TokenType::Semicolon)?; // Expect semicolon
        Some(Statement::TypeDeclaration(span_start.merge(&span_end), name.0, type_expr))
    }

    fn parse_effect_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordEffect)?; // Consume 'effect'
        let name = self.parse_identifier()?;
        let span_end = name.1.clone();
        self.expect_current(TokenType::Semicolon)?; // Expect semicolon
        Some(Statement::EffectDeclaration(span_start.merge(&span_end), name))
    }

    fn parse_language_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordLanguage)?; // Consume 'language'
        let paradigm = self.parse_string_literal()?.0;
        let version = self.parse_string_literal()?.0;
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::Semicolon)?; // Expect semicolon
        Some(Statement::LanguageDeclaration(span_start.merge(&span_end), paradigm, version))
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordWhile)?; // Consume 'while'
        self.expect_current(TokenType::LParen)?; // Consume '('
        let condition = self.parse_expression(Precedence::Lowest)?; // Parse condition
        self.expect_current(TokenType::RParen)?; // Consume ')'
        let body = self.parse_block_expression()?; // Parse body block
        let span_end = body.span();
        Some(Statement::While(span_start.merge(&span_end), condition, body))
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordFor)?; // Consume 'for'
        let iterator_var = self.parse_identifier()?;
        self.expect_current(TokenType::KeywordIn)?; // Consume 'in'
        let iterable_expr = self.parse_expression(Precedence::Lowest)?; // Parse iterable
        let body = self.parse_block_expression()?; // Parse body block
        let span_end = body.span();
        Some(Statement::For(span_start.merge(&span_end), iterator_var, iterable_expr, body))
    }

    fn parse_break_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordBreak)?; // Consume 'break'
        let span_end = self.current_token.span.clone();
        if self.peek_is(TokenType::Semicolon) { self.next_token(); } // Consume optional semicolon
        Some(Statement::Break(span_start.merge(&span_end)))
    }

    fn parse_continue_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordContinue)?; // Consume 'continue'
        let span_end = self.current_token.span.clone();
        if self.peek_is(TokenType::Semicolon) { self.next_token(); } // Consume optional semicolon
        Some(Statement::Continue(span_start.merge(&span_end)))
    }

    fn parse_match_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordMatch)?; // Consume 'match'
        let expression = self.parse_expression(Precedence::Lowest)?; // Parse expression to match
        self.expect_current(TokenType::LBrace)?; // Consume '{'
        let mut cases = Vec::new();
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            // parse match case
            self.expect_current(TokenType::KeywordCase)?; // Consume 'case'
            let pattern = self.parse_expression(Precedence::Lowest)?; // Parse pattern
            self.expect_current(TokenType::ThinArrow)?; // Consume '->'
            let body = self.parse_block_expression()?; // Parse case body
            cases.push(MatchCase { pattern, body });
        }
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RBrace)?; // Consume '}'
        Some(Statement::Match(span_start.merge(&span_end), expression, cases))
    }

    fn parse_unsafe_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordUnsafe)?; // Consume 'unsafe'
        let proof_id = if self.current_is(TokenType::LParen) {
            self.next_token(); // Consume '('
            self.expect_current(TokenType::Identifier)?; // Expect 'evas'
            self.expect_current(TokenType::Colon)?; // Expect ':'
            let id = self.parse_identifier()?;
            self.expect_current(TokenType::RParen)?; // Consume ')'
            Some(id)
        } else { None };
        let body = self.parse_block_expression()?;
        let span_end = body.span();
        Some(Statement::Unsafe(span_start.merge(&span_end), proof_id, body))
    }

    fn parse_handle_statement(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordHandle)?; // Consume 'handle'
        let effect_id = self.parse_identifier()?;
        let body = self.parse_block_expression()?;
        self.expect_current(TokenType::KeywordWith)?; // Consume 'with'
        self.expect_current(TokenType::LBrace)?; // Consume '{'
        let handler_body = self.parse_expression(Precedence::Lowest)?; // Parse the handler block
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RBrace)?; // Consume '}'
        Some(Statement::Handle(span_start.merge(&span_end), effect_id, body, handler_body))
    }

    // --- Expression Parsing ---
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expr = self.parse_prefix_expression()?;
        while !self.peek_is(TokenType::Semicolon) && precedence < token_precedence(&self.peek_token.token_type) {
            self.next_token(); // Advance to the infix operator
            left_expr = self.parse_infix_expression(left_expr.clone())?;
        }
        Some(left_expr)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        match self.current_token.token_type {
            TokenType::Identifier => self.parse_identifier_expression(),
            TokenType::Integer => self.parse_integer_literal(),
            TokenType::Float => self.parse_float_literal(),
            TokenType::String => self.parse_string_literal_expression(),
            TokenType::Boolean => self.parse_boolean_literal(),
            TokenType::Char => self.parse_char_literal_expression(),
            TokenType::QuantumLiteral => self.parse_quantum_literal_expression(),
            TokenType::NanoAnnotation => self.parse_nano_literal_expression(),
            TokenType::MTSLiteral => self.parse_mts_literal_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::LBrace => self.parse_block_expression(), // Blocks are expressions
            TokenType::Bang => self.parse_prefix_operator(),
            TokenType::Minus => self.parse_prefix_operator(),
            
            // --- OOP Additions ---
            TokenType::KeywordNew => self.parse_new_expression(),
            TokenType::KeywordThis => self.parse_this_expression(),
            TokenType::KeywordSuper => self.parse_super_expression(),

            _ => {
                self.add_error(format!("No prefix parse function for {:?}", self.current_token.token_type), self.current_token.span.clone());
                None
            }
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let precedence = token_precedence(&self.current_token.token_type);
        let token = self.current_token.clone();
        self.next_token();

        match token.token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash | TokenType::Modulo | 
            TokenType::Equals | TokenType::NotEquals | TokenType::LessThan | TokenType::GreaterThan | 
            TokenType::LessThanEqual | TokenType::GreaterThanEqual | TokenType::LogicalAnd | 
            TokenType::LogicalOr | TokenType::BitAnd | TokenType::BitOr | TokenType::Caret | 
            TokenType::LeftShift | TokenType::RightShift => Some(Expression::Infix(
                token.span.merge(&self.current_token.span),
                Box::new(left),
                token.token_type,
                Box::new(self.parse_expression(precedence)?),
            )),
            TokenType::LParen => self.parse_call_expression(left),
            TokenType::LBracket => self.parse_index_expression(left),
            TokenType::Dot => self.parse_member_access_expression(left), // Handles both field and method calls
            _ => {
                self.add_error(format!("No infix parse function for {:?}", token.token_type), token.span);
                None
            }
        }
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        if let TokenType::Identifier = self.current_token.token_type {
            let ident = Identifier(self.current_token.literal.clone(), self.current_token.span.clone());
            self.next_token();
            Some(ident)
        } else {
            self.add_error(format!("Expected an identifier, got {:?}", self.current_token.token_type), self.current_token.span.clone());
            None
        }
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        let ident = self.parse_identifier()?;
        Some(Expression::Identifier(ident))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.clone();
        let lit = Literal::Integer(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_float_literal(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.clone();
        let lit = Literal::Float(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_string_literal_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.clone();
        let lit = Literal::String(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_string_literal(&mut self) -> Option<(String, Span)> { // Helper for string literals
        if let TokenType::String = self.current_token.token_type {
            let lit_val = self.current_token.literal.clone();
            let span = self.current_token.span.clone();
            self.next_token();
            Some((lit_val, span))
        } else {
            self.add_error(format!("Expected a string literal, got {:?}", self.current_token.token_type), self.current_token.span.clone());
            None
        }
    }

    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let value = self.current_is(TokenType::Boolean) && self.current_token.literal == "true";
        let lit = Literal::Boolean(value, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_char_literal_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.chars().next().unwrap_or(' '); // Get first char
        let lit = Literal::Char(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_quantum_literal_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.clone();
        let lit = Literal::Quantum(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_nano_literal_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.clone();
        let lit = Literal::Nano(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_mts_literal_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        let literal_val = self.current_token.literal.clone();
        let lit = Literal::MTS(literal_val, span);
        self.next_token();
        Some(Expression::Literal(lit))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token(); // Consume '('
        let expr = self.parse_expression(Precedence::Lowest)?; // Parse inner expression
        self.expect_peek(TokenType::RParen)?; // Expect ')'
        self.next_token(); // Consume ')'
        Some(expr)
    }

    fn parse_expression_list(&mut self, end_token: TokenType) -> Option<Vec<Expression>> {
        let mut list = Vec::new();
        if self.current_is(end_token.clone()) { // Empty list
            return Some(list);
        }
        list.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_is(TokenType::Comma) {
            self.next_token(); // Consume ','
            self.next_token(); // Move to next expression
            list.push(self.parse_expression(Precedence::Lowest)?);
        }
        Some(list)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let span_start = function.span();
        self.next_token(); // Consume '('
        let arguments = self.parse_expression_list(TokenType::RParen)?; // Parse arguments
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RParen)?; // Consume ')'
        Some(Expression::Call(span_start.merge(&span_end), Box::new(function), arguments))
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        let span_start = left.span();
        self.next_token(); // Consume '['
        let index = self.parse_expression(Precedence::Lowest)?; // Parse index expression
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RBracket)?; // Consume ']'
        Some(Expression::Index(span_start.merge(&span_end), Box::new(left), Box::new(index)))
    }

    fn parse_prefix_operator(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?; // Parse operand with higher precedence
        Some(Expression::Prefix(token.span.merge(&right.span()), token.token_type, Box::new(right)))
    }

    // --- OOP Additions --- 
    fn parse_class_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordClass)?; // Consume 'class'
        let class_name = self.parse_identifier()?;
        
        let mut parents = Vec::new();
        if self.current_is(TokenType::KeywordExtends) {
            self.next_token(); // consume 'extends'
            parents.push(self.parse_identifier()?);
        }
        if self.current_is(TokenType::KeywordImplements) {
            self.next_token(); // consume 'implements'
            loop {
                parents.push(self.parse_identifier()?);
                if !self.peek_is(TokenType::Comma) {
                    break;
                }
                self.next_token(); // consume ','
                self.next_token(); // consume next identifier
            }
        }

        self.expect_current(TokenType::LBrace)?; // '{'
        let mut members = Vec::new();
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            if let Some(member) = self.parse_class_member() {
                members.push(member);
            }
            // Ensure progress, prevent infinite loop on unparseable members
            if self.current_token.span == span_start && !self.current_is(TokenType::RBrace) {
                 self.add_error(format!("Expected class member or '}}', found {:?}", self.current_token.token_type), self.current_token.span.clone());
                 self.next_token(); // Try to recover
            }
            if self.current_is(TokenType::Semicolon) { self.next_token(); } // Consume trailing semicolon
        }
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RBrace)?; // '}'

        Some(Statement::Class(span_start.merge(&span_end), class_name, parents, members))
    }

    fn parse_interface_declaration(&mut self) -> Option<Statement> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordInterface)?; // Consume 'interface'
        let interface_name = self.parse_identifier()?;
        
        let mut parents = Vec::new();
        if self.current_is(TokenType::KeywordExtends) { // Interfaces can extend other interfaces
            self.next_token(); // consume 'extends'
            loop {
                parents.push(self.parse_identifier()?);
                if !self.peek_is(TokenType::Comma) {
                    break;
                }
                self.next_token(); // consume ','
                self.next_token(); // consume next identifier
            }
        }

        self.expect_current(TokenType::LBrace)?; // '{'
        let mut members = Vec::new();
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            if let Some(member) = self.parse_interface_member() {
                members.push(member);
            }
            if self.current_is(TokenType::Semicolon) { self.next_token(); } // Consume trailing semicolon
        }
        let span_end = self.current_token.span.clone();
        self.expect_current(TokenType::RBrace)?; // '}'

        Some(Statement::Interface(span_start.merge(&span_end), interface_name, parents, members))
    }

    fn parse_access_modifier(&mut self) -> Option<AccessModifier> {
        match self.current_token.token_type {
            TokenType::KeywordPublic => { self.next_token(); Some(AccessModifier::Public) },
            TokenType::KeywordPrivate => { self.next_token(); Some(AccessModifier::Private) },
            TokenType::KeywordProtected => { self.next_token(); Some(AccessModifier::Protected) },
            _ => None, // No explicit modifier, default will be handled by semantic analysis
        }
    }

    fn parse_method_modifier(&mut self) -> Option<MethodModifier> {
        match self.current_token.token_type {
            TokenType::KeywordOverride => { self.next_token(); Some(MethodModifier::Override) },
            TokenType::KeywordVirtual => { self.next_token(); Some(MethodModifier::Virtual) },
            TokenType::KeywordAbstract => { self.next_token(); Some(MethodModifier::Abstract) },
            _ => None,
        }
    }

    fn parse_class_member(&mut self) -> Option<ClassMember> {
        let span_start = self.current_token.span.clone();
        let access_modifier = self.parse_access_modifier().unwrap_or(AccessModifier::Private); // Default to Private
        let method_modifier = self.parse_method_modifier();

        // Check for 'fn' for method, 'let' for field
        if self.current_is(TokenType::KeywordFn) {
            // Parse method
            self.expect_current(TokenType::KeywordFn)?; // Consume 'fn'
            let method_name = self.parse_identifier()?;
            self.expect_current(TokenType::LParen)?; // Consume '('
            let parameters = self.parse_function_parameters()?;
            self.expect_current(TokenType::RParen)?; // Consume ')'
            let return_type = if self.current_is(TokenType::ThinArrow) {
                self.next_token(); // consume '->'
                Some(self.parse_type_expression()?)
            } else { None };

            let effects = if self.current_is(TokenType::KeywordWith) {
                self.next_token(); // consume 'with'
                self.expect_current(TokenType::KeywordEffects)?; // consume 'effects'
                self.expect_current(TokenType::LBrace)?; // consume '{'
                let mut effect_ids = Vec::new();
                while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                    effect_ids.push(self.parse_identifier()?);
                    if self.current_is(TokenType::Comma) { self.next_token(); }
                }
                self.expect_current(TokenType::RBrace)?; // consume '}'
                effect_ids
            } else { Vec::new() };

            // Abstract methods do not have a body
            let body = if method_modifier == Some(MethodModifier::Abstract) {
                // No body, just expect a semicolon
                self.expect_current(TokenType::Semicolon)?; 
                Expression::Block(span_start.clone(), Vec::new()) // Dummy empty block
            } else {
                self.parse_block_expression()? // Parse method body
            };
            
            let span_end = body.span();
            Some(ClassMember::Method(span_start.merge(&span_end), access_modifier, method_modifier, method_name, parameters, return_type, body, effects))
        } else if self.current_is(TokenType::KeywordLet) {
            // Parse field
            self.expect_current(TokenType::KeywordLet)?; // Consume 'let'
            let field_name = self.parse_identifier()?;
            self.expect_current(TokenType::Colon)?; // Consume ':'
            let field_type = self.parse_type_expression()?;
            let initializer = if self.current_is(TokenType::Assign) {
                self.next_token(); // consume '='
                Some(self.parse_expression(Precedence::Lowest)?) // Parse expression with lowest precedence
            } else { None };
            let span_end = initializer.as_ref().map_or(field_type.span(), |e| e.span());
            self.expect_current(TokenType::Semicolon)?; // Expect semicolon
            Some(ClassMember::Field(span_start.merge(&span_end), access_modifier, field_name, field_type, initializer))
        } else {
            self.add_error(format!("Expected 'fn' or 'let' for class member, found {:?}", self.current_token.token_type), self.current_token.span.clone());
            None
        }
    }

    fn parse_interface_member(&mut self) -> Option<InterfaceMember> {
        let span_start = self.current_token.span.clone();
        // Interface members are always public implicitly, but could be explicitly stated
        if self.current_is(TokenType::KeywordPublic) { self.next_token(); } // Consume 'public' if present

        self.expect_current(TokenType::KeywordFn)?; // Consume 'fn'
        let method_name = self.parse_identifier()?;
        self.expect_current(TokenType::LParen)?; // Consume '('
        let parameters = self.parse_function_parameters()?;
        self.expect_current(TokenType::RParen)?; // Consume ')'
        let return_type = if self.current_is(TokenType::ThinArrow) {
            self.next_token(); // consume '->'
            Some(self.parse_type_expression()?)
        } else { None };

        let effects = if self.current_is(TokenType::KeywordWith) {
            self.next_token(); // consume 'with'
            self.expect_current(TokenType::KeywordEffects)?; // consume 'effects'
            self.expect_current(TokenType::LBrace)?; // consume '{'
            let mut effect_ids = Vec::new();
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                effect_ids.push(self.parse_identifier()?);
                if self.current_is(TokenType::Comma) { self.next_token(); }
            }
            self.expect_current(TokenType::RBrace)?; // consume '}'
            effect_ids
        } else { Vec::new() };
        self.expect_current(TokenType::Semicolon)?; // Interface methods have no body

        let span_end = self.current_token.span.clone();
        Some(InterfaceMember::MethodSignature(span_start.merge(&span_end), method_name, parameters, return_type, effects))
    }

    fn parse_type_expression(&mut self) -> Option<TypeExpr> {
        let span_start = self.current_token.span.clone();
        let identifier = self.parse_identifier()?;
        let mut type_expr = TypeExpr::Identifier(identifier);

        // Conceptual: For generics, e.g., List<int>
        // if self.current_is(TokenType::LessThan) {
        //     self.next_token(); // consume '<'
        //     let generic_arg = self.parse_type_expression()?;
        //     self.expect_current(TokenType::GreaterThan)?; // consume '>'
        //     type_expr = TypeExpr::Generic(Box::new(type_expr), Box::new(generic_arg));
        // }

        Some(type_expr)
    }

    fn parse_new_expression(&mut self) -> Option<Expression> {
        let span_start = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordNew)?; // Consume 'new'
        let class_name = self.parse_identifier()?;
        self.expect_current(TokenType::LParen)?; // Consume '('
        let arguments = self.parse_expression_list(TokenType::RParen)?; // Parse arguments
        let span_end = self.current_token.span.clone(); // After RParen
        self.expect_current(TokenType::RParen)?; // Consume ')'
        Some(Expression::NewInstance(span_start.merge(&span_end), class_name, arguments))
    }

    fn parse_this_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordThis)?; // Consume 'this'
        Some(Expression::This(span))
    }

    fn parse_super_expression(&mut self) -> Option<Expression> {
        let span = self.current_token.span.clone();
        self.expect_current(TokenType::KeywordSuper)?; // Consume 'super'
        Some(Expression::Super(span))
    }

    // Extend parse_member_access (or create new method) to handle MethodCall/FieldAccess
    fn parse_member_access_expression(&mut self, left: Expression) -> Option<Expression> {
        // Assume 'left' is already parsed (e.g., object_instance)
        let span_start = left.span();
        self.expect_current(TokenType::Dot)?; // Consume '.'
        let member_name = self.parse_identifier()?;
        
        let span_end = member_name.1.clone();

        if self.current_is(TokenType::LParen) { // It's a method call
            self.next_token(); // consume '('
            let arguments = self.parse_expression_list(TokenType::RParen)?; // Parse arguments
            let final_span_end = self.current_token.span.clone(); // After RParen
            self.expect_current(TokenType::RParen)?; // Consume ')'
            Some(Expression::MethodCall(span_start.merge(&final_span_end), Box::new(left), member_name, arguments))
        } else { // It's a field access
            Some(Expression::FieldAccess(span_start.merge(&span_end), Box::new(left), member_name))
        }
    }
}

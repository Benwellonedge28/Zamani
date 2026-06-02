//! Zenith UMC Parser
//!
//! Converts a token stream from the Lexer into a typed AST.
//! Supports the full Zenith grammar: classical, quantum, nano, MTS, Sankofa, OOP.

use crate::ast::{
    AccessModifier, ClassMember, Expression, Identifier, InterfaceMember, Literal, MatchCase,
    MethodModifier, Parameter, Program, Statement, TypeExpr,
};
use crate::compiler_types::AccessModifier as CtAccessModifier;
use crate::lexer::{Lexer, Token, TokenType};
use crate::source_map::{BytePos, FileId, Span};

// ─── Parser error ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
}

// ─── Operator precedence ─────────────────────────────────────────────────────

#[derive(PartialOrd, PartialEq, Clone, Copy)]
enum Precedence {
    Lowest,
    Assign,
    LogicalOr,
    LogicalAnd,
    Equals,
    Compare,
    Sum,
    Product,
    Unary,
    Call,
    Index,
    Member,
}

fn token_precedence(t: &TokenType) -> Precedence {
    match t {
        TokenType::Assign => Precedence::Assign,
        TokenType::LogicalOr => Precedence::LogicalOr,
        TokenType::LogicalAnd => Precedence::LogicalAnd,
        TokenType::Equals | TokenType::NotEquals => Precedence::Equals,
        TokenType::LessThan
        | TokenType::GreaterThan
        | TokenType::LessThanEqual
        | TokenType::GreaterThanEqual => Precedence::Compare,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Star | TokenType::Slash | TokenType::Modulo => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        TokenType::Dot => Precedence::Member,
        _ => Precedence::Lowest,
    }
}

// ─── Parser ───────────────────────────────────────────────────────────────────

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Parser {
            lexer,
            current,
            peek,
            errors: vec![],
        }
    }

    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    /// Parse a full Zenith program.
    pub fn parse_program(&mut self) -> Program {
        let mut stmts = vec![];
        while self.current.token_type != TokenType::EOF {
            if let Some(s) = self.parse_statement() {
                stmts.push(s);
            } else {
                self.advance(); // error recovery
            }
        }
        Program { statements: stmts }
    }

    // ── Navigation ────────────────────────────────────────────────────────────

    fn advance(&mut self) -> Token {
        let prev = self.current.clone();
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token();
        // propagate lexer errors
        for e in self.lexer.get_errors() {
            self.errors.push(ParserError {
                message: e.message.clone(),
                span: e.span.clone(),
            });
        }
        self.lexer.errors.clear();
        prev
    }

    fn current_is(&self, t: TokenType) -> bool {
        self.current.token_type == t
    }
    fn peek_is(&self, t: TokenType) -> bool {
        self.peek.token_type == t
    }

    fn expect(&mut self, t: TokenType) -> Option<Token> {
        if self.current_is(t.clone()) {
            Some(self.advance())
        } else {
            let msg = format!("Expected {:?}, found {:?}", t, self.current.token_type);
            self.errors.push(ParserError {
                message: msg,
                span: self.current.span.clone(),
            });
            None
        }
    }

    fn dummy_span(&self) -> Span {
        self.current.span.clone()
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current.token_type {
            TokenType::KeywordLet => self.parse_let(),
            TokenType::KeywordReturn => self.parse_return(),
            TokenType::KeywordFn => self.parse_function(),
            TokenType::KeywordIf => {
                let expr = self.parse_if_expression()?;
                Some(Statement::Expression(expr))
            }
            TokenType::KeywordWhile => self.parse_while(),
            TokenType::KeywordFor => self.parse_for(),
            TokenType::KeywordBreak => {
                let span = self.current.span.clone();
                self.advance();
                Some(Statement::Break(span))
            }
            TokenType::KeywordContinue => {
                let span = self.current.span.clone();
                self.advance();
                Some(Statement::Continue(span))
            }
            TokenType::KeywordMatch => self.parse_match(),
            TokenType::KeywordQuantum => self.parse_quantum_circuit(),
            TokenType::KeywordAgent => self.parse_nano_agent(),
            TokenType::KeywordRemember => self.parse_sankofa_memory(),
            _ => {
                let expr = self.parse_expression(Precedence::Lowest)?;
                if self.current_is(TokenType::Semicolon) {
                    self.advance();
                }
                Some(Statement::Expression(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance(); // consume `let`
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let typ = if self.current_is(TokenType::Colon) {
            self.advance();
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Let(span, name, typ, val))
    }

    fn parse_return(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance();
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Return(span, val))
    }

    fn parse_function(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance(); // consume `fn`
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        self.expect(TokenType::LParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenType::RParen)?;
        let ret = if self.current_is(TokenType::ThinArrow) {
            self.advance();
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        let body = self.parse_block_expression()?;
        Some(Statement::Function(span, name, params, ret, body))
    }

    fn parse_while(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance();
        let cond = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expression()?;
        Some(Statement::While(span, cond, body))
    }

    fn parse_for(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance();
        let name = self.current.literal.clone();
        let id_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        self.expect(TokenType::KeywordIn)?;
        let iter = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expression()?;
        Some(Statement::For(span, Identifier(name, id_span), iter, body))
    }

    fn parse_match(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance();
        let subject = self.parse_expression(Precedence::Lowest)?;
        self.expect(TokenType::LBrace)?;
        let mut cases = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            let cs = self.current.span.clone();
            let pat = self.parse_expression(Precedence::Lowest)?;
            self.expect(TokenType::ThinArrow)?;
            let body = self.parse_expression(Precedence::Lowest)?;
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
            cases.push(MatchCase {
                pattern: pat,
                body,
                span: cs,
            });
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Match(span, subject, cases))
    }

    fn parse_quantum_circuit(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance(); // quantum
        if self.current_is(TokenType::KeywordCircuit) {
            self.advance();
        }
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let body = self.parse_block_expression()?;
        Some(Statement::QuantumCircuit(span, name, body))
    }

    fn parse_nano_agent(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance(); // agent
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let body = self.parse_block_expression()?;
        Some(Statement::NanoAgent(span, name, body))
    }

    fn parse_sankofa_memory(&mut self) -> Option<Statement> {
        let span = self.current.span.clone();
        self.advance(); // remember
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::SankofaMemory(span, name, val))
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    fn parse_expression(&mut self, prec: Precedence) -> Option<Expression> {
        let mut left = self.parse_prefix()?;
        while token_precedence(&self.current.token_type) > prec {
            left = self.parse_infix(left)?;
        }
        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        match self.current.token_type.clone() {
            TokenType::Identifier => {
                let name = self.current.literal.clone();
                let span = self.current.span.clone();
                self.advance();
                Some(Expression::Identifier(Identifier(name, span)))
            }
            TokenType::Integer => {
                let val: i64 = self.current.literal.parse().unwrap_or(0);
                let span = self.current.span.clone();
                self.advance();
                Some(Expression::Literal(Literal::Integer(val, span)))
            }
            TokenType::Float => {
                let val: f64 = self.current.literal.parse().unwrap_or(0.0);
                let span = self.current.span.clone();
                self.advance();
                Some(Expression::Literal(Literal::Float(val, span)))
            }
            TokenType::String => {
                let val = self.current.literal.clone();
                let span = self.current.span.clone();
                self.advance();
                Some(Expression::Literal(Literal::String(val, span)))
            }
            TokenType::Boolean => {
                let val = self.current.literal == "true";
                let span = self.current.span.clone();
                self.advance();
                Some(Expression::Literal(Literal::Boolean(val, span)))
            }
            TokenType::Char => {
                let ch = self.current.literal.chars().next().unwrap_or('\0');
                let span = self.current.span.clone();
                self.advance();
                Some(Expression::Literal(Literal::Char(ch, span)))
            }
            TokenType::Not | TokenType::Minus => {
                let span = self.current.span.clone();
                let op = self.current.token_type.clone();
                self.advance();
                let right = self.parse_expression(Precedence::Unary)?;
                Some(Expression::Prefix(span, op, Box::new(right)))
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression(Precedence::Lowest)?;
                self.expect(TokenType::RParen)?;
                Some(expr)
            }
            TokenType::LBrace => self.parse_block_expression(),
            TokenType::KeywordIf => self.parse_if_expression(),
            TokenType::KeywordRecall => {
                let span = self.current.span.clone();
                self.advance();
                let key = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Recall(span, Box::new(key)))
            }
            _ => {
                let msg = format!(
                    "Unexpected token in expression: {:?}",
                    self.current.token_type
                );
                self.errors.push(ParserError {
                    message: msg,
                    span: self.current.span.clone(),
                });
                None
            }
        }
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        match self.current.token_type.clone() {
            TokenType::LParen => {
                let span = self.current.span.clone();
                self.advance();
                let mut args = vec![];
                while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                    args.push(self.parse_expression(Precedence::Lowest)?);
                    if self.current_is(TokenType::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenType::RParen)?;
                Some(Expression::Call(span, Box::new(left), args))
            }
            TokenType::LBracket => {
                let span = self.current.span.clone();
                self.advance();
                let index = self.parse_expression(Precedence::Lowest)?;
                self.expect(TokenType::RBracket)?;
                Some(Expression::Index(span, Box::new(left), Box::new(index)))
            }
            TokenType::Dot => {
                let span = self.current.span.clone();
                self.advance();
                let member_name = self.current.literal.clone();
                let member_span = self.current.span.clone();
                self.expect(TokenType::Identifier)?;
                Some(Expression::MemberAccess(
                    span,
                    Box::new(left),
                    Identifier(member_name, member_span),
                ))
            }
            op => {
                let span = self.current.span.clone();
                let prec = token_precedence(&op);
                self.advance();
                let right = self.parse_expression(prec)?;
                Some(Expression::Infix(span, Box::new(left), op, Box::new(right)))
            }
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let span = self.current.span.clone();
        self.advance(); // consume `if`
        let cond = self.parse_expression(Precedence::Lowest)?;
        let then = self.parse_block_expression()?;
        let else_branch = if self.current_is(TokenType::KeywordElse) {
            self.advance();
            if self.current_is(TokenType::KeywordIf) {
                Some(Box::new(self.parse_if_expression()?))
            } else {
                Some(Box::new(self.parse_block_expression()?))
            }
        } else {
            None
        };
        Some(Expression::If(
            span,
            Box::new(cond),
            Box::new(then),
            else_branch,
        ))
    }

    fn parse_block_expression(&mut self) -> Option<Expression> {
        let span = self.current.span.clone();
        self.expect(TokenType::LBrace)?;
        let mut stmts = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            if let Some(s) = self.parse_statement() {
                stmts.push(s);
            } else {
                self.advance();
            }
        }
        self.expect(TokenType::RBrace)?;
        Some(Expression::Block(span, stmts))
    }

    // ── Parameters & Types ────────────────────────────────────────────────────

    fn parse_parameters(&mut self) -> Option<Vec<Parameter>> {
        let mut params = vec![];
        while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
            let name_str = self.current.literal.clone();
            let name_span = self.current.span.clone();
            self.expect(TokenType::Identifier)?;
            let typ = if self.current_is(TokenType::Colon) {
                self.advance();
                Some(self.parse_type_expr()?)
            } else {
                None
            };
            params.push(Parameter {
                name: Identifier(name_str, name_span),
                typ,
                default: None,
            });
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
        }
        Some(params)
    }

    fn parse_type_expr(&mut self) -> Option<TypeExpr> {
        let name = self.current.literal.clone();
        let span = self.current.span.clone();
        if self.current_is(TokenType::Identifier) {
            self.advance();
            Some(TypeExpr::Identifier(Identifier(name, span)))
        } else {
            self.errors.push(ParserError {
                message: format!("Expected type, found {:?}", self.current.token_type),
                span: self.current.span.clone(),
            });
            None
        }
    }
}

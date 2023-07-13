use crate::data::{Value, ValueType};
use crate::errors::{Errors, Result};

pub struct Context<'json> {
    json: &'json str,
    cursor: usize,
}

pub struct Parser<'json> {
    context: Context<'json>,
}

impl<'json> Context<'json> {
    pub fn new<S>(json: &S) -> Context
    where
        S: AsRef<str> + ?Sized,
    {
        Context {
            json: json.as_ref(),
            cursor: 0,
        }
    }

    pub fn cur(&self) -> Option<char> {
        self.json.chars().nth(self.cursor)
    }

    pub fn advance(&mut self) {
        if self.cursor < self.json.len() {
            self.cursor += 1
        }
    }

    pub fn advance_n(&mut self, steps: usize) {
        if self.cursor < self.json.len() {
            self.cursor += steps
        }
    }

    // TODO: maybe rename to peek?
    pub fn peek(&self, n: usize) -> &str {
        if self.cursor + n > self.json.len() {
            return "";
        }
        &self.json[self.cursor..self.cursor + n]
    }
}

impl Parser<'_> {
    pub fn new<S>(json: &S) -> Parser
    where
        S: AsRef<str> + ?Sized,
    {
        Parser {
            context: Context::new(json.as_ref()),
        }
    }

    fn parse_whitespace(&mut self) {
        loop {
            match self.context.cur() {
                Some(c) => match c {
                    ' ' | '\t' | '\n' | '\r' => self.context.advance(),
                    _ => break,
                },
                None => break,
            }
        }
    }

    fn parse_literal<S>(&mut self, literal: S, v_type: ValueType) -> Result<Value>
    where
        S: AsRef<str>,
    {
        let literal = literal.as_ref();
        return match self.context.peek(literal.len()) == literal {
            true => {
                self.context.advance_n(literal.len());
                Ok(Value { v_type })
            }
            false => Err(Errors::InvalidValue),
        };
    }

    fn parse_value(&mut self) -> Result<Value> {
        return match self.context.cur() {
            None => Err(Errors::ReachEOF),
            Some(c) => match c {
                't' => self.parse_literal("true", ValueType::Bool(true)),
                'f' => self.parse_literal("false", ValueType::Bool(false)),
                'n' => self.parse_literal("null", ValueType::Null),
                _ => Err(Errors::InvalidValue),
            },
        };
    }

    pub fn parse<S>(json: &S) -> Result<Value>
    where
        S: AsRef<str> + ?Sized,
    {
        let mut parser = Parser::new(json);
        parser.parse_whitespace();
        return parser.parse_value();
    }
}

use crate::data::Value;
use crate::errors::{Errors, Result};
use std::collections::HashMap;

pub struct Context<'json> {
    json: &'json str,
    cursor: usize,
}

pub struct Parser<'json> {
    context: Context<'json>,
}

pub struct Iter<'json> {
    partial_json: &'json str,
    cursor: usize,
}

impl Iter<'_> {
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

impl Iterator for Iter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.partial_json.chars().nth(self.cursor);
        self.cursor += 1;
        return ret;
    }
}

impl Iter<'_> {
    pub fn peek(&mut self) -> Option<char> {
        self.partial_json.chars().nth(self.cursor)
    }

    /// str which have been looked by this iter
    pub fn looked(&mut self) -> &str {
        &self.partial_json[..self.cursor]
    }

    fn consume_digits(&mut self, min: usize) -> Result<()> {
        let mut cnt: usize = 0;
        /* consume at least n digits */
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.next();
                cnt += 1;
            } else {
                if cnt < min {
                    return Err(Errors::InvalidValue);
                }
                break;
            }
        }

        return if cnt >= min {
            Ok(())
        } else {
            Err(Errors::InvalidValue)
        };
    }
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

    pub fn iter(&self) -> Iter {
        Iter {
            partial_json: &self.json[self.cursor..],
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

    pub fn next(&mut self) -> Option<char> {
        let ret = self.cur();
        self.advance();
        ret
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

    fn parse_literal<S>(&mut self, literal: S, value: Value) -> Result<Value>
    where
        S: AsRef<str>,
    {
        let literal = literal.as_ref();
        return match self.context.peek(literal.len()) == literal {
            true => {
                self.context.advance_n(literal.len());
                Ok(value)
            }
            false => Err(Errors::InvalidValue),
        };
    }

    fn parse_number(&mut self) -> Result<Value> {
        let mut iter = self.context.iter();

        /* match minus sign */
        if iter.peek() == Some('-') {
            iter.next();
        }

        /* match the digits before decimal point */
        if let Some(c) = iter.peek() {
            match c {
                '0' => {
                    iter.next();
                }
                c => match c.is_ascii_digit() {
                    true => iter.consume_digits(1)?,
                    false => return Err(Errors::InvalidValue),
                },
            }
        }

        /* match decimal point */
        if iter.peek() == Some('.') {
            iter.next(); /* skip the decimal point */
            iter.consume_digits(1)?
        }

        /* match exponential field */
        match iter.peek() {
            /* match e | E */
            Some('e') | Some('E') => {
                iter.next();
                /* match + | - */
                match iter.peek() {
                    Some('+') | Some('-') => {
                        iter.next();
                    }
                    _ => { /* continue */ }
                }
                /* + | - is optional, if not appear, can only follow digits */
                /* match one or more digits */
                iter.consume_digits(1)?;
            }
            _ => {}
        }
        // F**king Painful! I will definitely use regex in the future :)
        // Regex Ver: r"(?:^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)"

        let num = iter.looked();
        let steps = num.len();
        let num = match num.parse::<f64>().map_err(|_| Errors::InvalidValue)? {
            num if num == f64::INFINITY => return Err(Errors::NumberTooBig),
            num => num,
        };

        self.context.advance_n(steps);

        Ok(Value::Number(num))
    }

    fn parse_raw_string(&mut self) -> Result<String> {
        assert_eq!(self.context.next(), Some('\"'));

        let mut chars = Vec::<char>::new();

        while let Some(c) = self.context.next() {
            match c {
                /* reach the end of string */
                '\"' => {
                    return Ok(chars.into_iter().collect::<String>());
                }
                /* escape sequence */
                '\\' => match self.context.next() {
                    Some('\"') => chars.push('\"'),
                    Some('\\') => chars.push('\\'),
                    Some('/') => chars.push('/'),
                    Some('b') => chars.push('\x08'),
                    Some('f') => chars.push('\x0C'),
                    Some('n') => chars.push('\n'),
                    Some('r') => chars.push('\r'),
                    Some('t') => chars.push('\t'),
                    Some(_) => return Err(Errors::InvalidStringEscape),
                    None => return Err(Errors::MissingQuotationMark),
                },
                /* TODO: Unicode is not considered */
                c if (c as u8) < 0x20 => return Err(Errors::InvalidStringChar),
                _ => chars.push(c),
            }
        }
        return Err(Errors::MissingQuotationMark);
    }

    fn parse_string(&mut self) -> Result<Value> {
        Ok(Value::String(self.parse_raw_string()?))
    }

    fn parse_array(&mut self) -> Result<Value> {
        assert_eq!(self.context.next(), Some('['));
        let mut array = Vec::<Value>::new();
        loop {
            self.parse_whitespace();
            if let Some(c) = self.context.cur() {
                match c {
                    ',' => {
                        self.context.next(); /* just continue parsing */
                    }
                    ']' => {
                        self.context.next();
                        return Ok(Value::Array(array));
                    }
                    ' ' => {
                        unreachable!()
                    }
                    _ => array.push(self.parse_value()?),
                }
            } else {
                return Err(Errors::MissingCommaOrClosingBracket);
            }
            self.parse_whitespace();
        }
    }

    fn parse_pair(&mut self) -> Result<(String, Value)> {
        assert_eq!(self.context.cur(), Some('\"'));
        let key = self.parse_raw_string()?;
        self.parse_whitespace();
        match self.context.cur() {
            Some(':') => {
                self.context.next();
                self.parse_whitespace();
            }
            _ => {
                return Err(Errors::MissingSemicolon);
            }
        }

        let value = self.parse_value()?;
        Ok((key, value))
    }

    fn parse_object(&mut self) -> Result<Value> {
        assert_eq!(self.context.next(), Some('{'));
        let mut object = HashMap::<String, Value>::new();
        loop {
            self.parse_whitespace();
            match self.context.cur() {
                Some('\"') => {
                    let (key, val) = self.parse_pair()?;
                    object.insert(key, val);
                }
                /* empty object */
                Some('}') => {
                    self.context.next();
                    return Ok(Value::Object(object));
                }
                _ => {
                    return Err(Errors::MissingKey);
                }
            }
            self.parse_whitespace();
            match self.context.cur() {
                Some(',') => {
                    self.context.next();
                }
                Some('}') => {
                    self.context.next();
                    return Ok(Value::Object(object));
                }
                _ => {
                    return Err(Errors::MissingCommaOrClosingCurlyBracket);
                }
            }
        }
    }

    fn parse_value(&mut self) -> Result<Value> {
        return match self.context.cur() {
            None => Err(Errors::ReachEOF),
            Some(c) => match c {
                't' => self.parse_literal("true", Value::Bool(true)),
                'f' => self.parse_literal("false", Value::Bool(false)),
                'n' => self.parse_literal("null", Value::Null),
                '[' => self.parse_array(),
                '{' => self.parse_object(),
                '\"' => self.parse_string(),
                _ => self.parse_number(),
            },
        };
    }

    pub fn parse<S>(json: &S) -> Result<Value>
    where
        S: AsRef<str> + ?Sized,
    {
        let mut parser = Parser::new(json);
        parser.parse_whitespace();
        let ret = parser.parse_value()?;
        parser.parse_whitespace();
        return match parser.context.next() {
            None => Ok(ret),
            Some(_) => Err(Errors::RootNotSingular),
        };
    }
}

use crate::data::{Value, ValueType};
use crate::errors::{Errors, Result};

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

    fn consume_digits(iter: &mut Iter, min: usize) -> Result<()> {
        let mut cnt: usize = 0;
        /* consume at least n digits */
        while let Some(c) = iter.peek() {
            if c.is_ascii_digit() {
                iter.next();
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

    fn parse_number(&mut self) -> Result<Value> {
        let mut iter = self.context.iter();

        /* match minus sign */
        if iter.peek() == Some('-') {
            iter.next();
        }

        /* match the digits before decimal point */
        match iter.next() {
            Some('0') => { /* continue */ }
            Some(c) => match c.is_ascii_digit() {
                true => Parser::consume_digits(&mut iter, 0)?,
                false => return Err(Errors::InvalidValue),
            },
            None => return Err(Errors::InvalidValue),
        }

        /* match decimal point */
        match iter.peek() {
            None => { /* no fraction part, continue */ }
            Some(c) => {
                match c {
                    '.' => {
                        iter.next(); /* skip the decimal point */
                        Parser::consume_digits(&mut iter, 1)?
                    }
                    'e' | 'E' => { /* exp field, continue */ }
                    _ => {
                        return Err(Errors::InvalidValue);
                    }
                }
            }
        }

        /* match exponential field */
        match iter.peek() {
            None => { /* no exp field, continue */ }
            Some(c) => {
                /* match e | E */
                match c {
                    'e' | 'E' => {
                        iter.next(); /* continue parsing */
                    }
                    _ => {
                        return Err(Errors::InvalidValue);
                    }
                }

                /* match + | - */
                match iter.peek() {
                    None => {
                        return Err(Errors::InvalidValue);
                    }
                    Some(c) => {
                        match c {
                            '+' | '-' => {
                                iter.next();
                            }
                            /* + | - is optional, if not appear, can only follow digits */
                            _ => {
                                if !c.is_ascii_digit() {
                                    return Err(Errors::InvalidValue);
                                }
                            }
                        }
                    }
                }

                /* match one or more digits */
                Parser::consume_digits(&mut iter, 1)?;
            }
        }
        // F**king Painful! I will definitely use regex in the future :)
        // Regex Ver: r"(?:^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)"

        let num = iter.looked();
        let steps = num.len();
        let num = num.parse::<f64>().map_err(|_| Errors::InvalidValue)?;
        self.context.advance_n(steps);

        Ok(Value {
            v_type: ValueType::Number(num),
        })
    }

    fn parse_value(&mut self) -> Result<Value> {
        return match self.context.cur() {
            None => Err(Errors::ReachEOF),
            Some(c) => match c {
                't' => self.parse_literal("true", ValueType::Bool(true)),
                'f' => self.parse_literal("false", ValueType::Bool(false)),
                'n' => self.parse_literal("null", ValueType::Null),
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
        return parser.parse_value();
    }
}

// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{iter::Peekable, rc::Rc};

use cotowali_chars::*;
use cotowali_source::{Loc, Source};
use cotowali_token::{Token, TokenKind, TokenReader};
use cotowali_utils::must;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    loc: Loc,
    loc_head: Loc,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a Rc<Source>) -> Self {
        Self {
            chars: s.code.chars().peekable(),
            loc: Loc::head(Some(s)),
            loc_head: Loc::head(Some(s)),
        }
    }

    fn peek_char(&mut self) -> Option<Char<'_>> {
        self.chars.peek().copied()
    }

    fn eof(&mut self) -> bool {
        self.peek_char().is_none()
    }

    fn consume(&mut self) -> Option<Char<'_>> {
        if self.eof() {
            panic!("Unexpected EOF")
        }

        let c = self.chars.next()?;

        self.loc_head.index += c.len();
        self.loc_head.column += 1;
        self.loc.len += c.len();

        if c == "\r" && self.peek_char().map_or(false, |c| c == "\n") {
            self.consume();
            self.loc_head.line -= 1; // rollback incremented line.
        }

        if is_newline(c) {
            self.loc_head.line += 1;
            self.loc_head.column = 1;
        }

        Some(c)
    }

    fn consume_if(&mut self, mut f: impl FnMut(Char<'_>) -> bool) -> Option<Char<'_>> {
        if f(self.peek_char()?) {
            Some(self.consume()?)
        } else {
            None
        }
    }

    fn consume_while(&mut self, mut f: impl FnMut(Char<'_>) -> bool) {
        while self.consume_if(|c| f(c)).is_some() {}
    }

    fn skip_whitespaces(&mut self) {
        self.consume_while(is_whitespace);
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token {
            loc: self.loc.clone(),
            kind,
        }
    }
}

impl<'a> Lexer<'a> {
    fn read(&mut self) -> Option<Token> {
        self.skip_whitespaces();
        self.loc = self.loc_head.clone();
        self.loc.len = 0;

        if self.eof() {
            return None;
        }

        if is_decimal_digit(self.peek_char()?) {
            Some(self.read_number())
        } else {
            self.consume_while(|c| !is_decimal_digit(c));
            Some(self.new_token(TokenKind::Error("err".to_string())))
        }
    }

    fn read_number(&mut self) -> Token {
        let mut value = 0;
        while let Some(c) = self.consume_if(is_decimal_digit) {
            value *= 10;
            value += must!(c.char().to_digit(10));
        }
        self.new_token(TokenKind::IntLiteral(value.into()))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}

pub fn tokenize(s: &Rc<Source>) -> TokenReader<'_> {
    TokenReader::new(Lexer::new(s))
}

#[cfg(test)]
mod tests {
    use crate::*;
    use cotowali_asserts::*;
    use cotowali_source::{loc, Source};
    use cotowali_token::token;

    macro_rules! test {
        (
            $code:expr => [
                $( { $v:expr, { $begin:expr,$end:expr;$line:expr,$col:expr } }),* $(,)?
            ] $(,)?
        ) => {
            let __s = Rc::new(Source::inline($code));
            assert_iter_eq!(
                tokenize(&__s),
                [
                    $(
                        token!($v, loc! {$begin,$end;$line,$col})
                    ),*
                ]
            )
        };
    }

    #[test]
    fn test_newline() {
        test!(
            "1\n2\r3\r\n4\n\r6\n\n8\r\r10\n\r\n\r\n13" => [
                {1,  {0,1  ; 1,1}},
                {2,  {2,3  ; 2,1}},
                {3,  {4,5  ; 3,1}},
                {4,  {7,8  ; 4,1}},
                {6,  {10,11; 6,1}},
                {8,  {13,14; 8,1}},
                {10, {16,18; 10,1}},
                {13, {23,25; 13,1}},
            ],
        );
    }

    #[test]
    fn test_number() {
        test!(
            "1 16 256 2048" => [
                {1,    {0,1 ; 1,1}},
                {16,   {2,4 ; 1,3}},
                {256,  {5,8 ; 1,6}},
                {2048, {9,13; 1,10}},
            ],
        );
    }
}

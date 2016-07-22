use std::str::Chars;
use std::iter::Peekable;
use std::iter::Enumerate;

use ::syntax::lexer::{Token, Lexer};
use ::syntax::next_is;


pub struct RustSyntax;

impl Lexer for RustSyntax {
    fn handle_char(&self, ch: char, mut iter: &mut Peekable<Enumerate<Chars>>, y_pos: usize, idx: usize) -> Option<Token> {
        match ch {
            '#' => {
                let st = idx;
                let mut end = idx;
                if next_is(&mut iter, '!') || next_is(&mut iter, '[') {
                    let mut s = String::from("#");
                    while let Some(&(e, c)) = iter.peek() {
                        if c == '\n' { break }
                        end = e;
                        s.push(iter.next().unwrap().1)
                    }
                    return Some(Token::Attribute(s));
                }
                return Some(Token::Hash)
            }

            '/' => {
                let st = idx;
                let mut end = idx;
                if next_is(&mut iter, '/') {
                    let mut s = String::from("/");
                    s.push(iter.next().unwrap().1);

                    let mut doc_comment = false;
                    if next_is(&mut iter, '/') || next_is(&mut iter, '!') {
                        doc_comment = true;
                    }

                    while let Some(&(e, c)) = iter.peek() {
                        if c == '\n' { break }
                        end = e;
                        s.push(iter.next().unwrap().1)
                    }

                    return Some(if doc_comment{ Token::DocComment(s) } else { Token::SingleLineComment(s) });
                }
                return Some(Token::ForwardSlash);
            }

            '"' => {
                let st = idx;
                let mut end = idx;
                let mut s = String::from("\"");
                while let Some(&(e, c)) = iter.peek() {
                    end = e;
                    s.push(iter.next().unwrap().1);
                    if c == '\\' {
                        if let Some(&(e, c_)) = iter.peek() {
                            // this is handling escaped single quotes...
                            if c_ == '"' {
                                s.push(iter.next().unwrap().1);
                                continue;
                            }
                        }
                    }
                    if c == '"' {
                        break;
                    }
                }
                return Some(Token::String(s));
            }

            '\'' => {
                let st = idx;
                let mut end = idx;
                let mut s = String::from("\'");
                while let Some(&(e, c)) = iter.peek() {
                    end = e;
                    s.push(iter.next().unwrap().1);
                    if c == '\\' {
                        if let Some(&(e, c_)) = iter.peek() {
                            // this is handling escaped single quotes...
                            if c_ == '\'' {
                                s.push(iter.next().unwrap().1);
                                continue;
                            }
                        }
                    }
                    if c == '\'' {
                        break;
                    }
                }
                return Some(Token::Special(s));
            }

            _ => None,
        }

    }

    fn handle_ident(&self, ch: char, mut iter: &mut Peekable<Enumerate<Chars>>, y_pos: usize) -> Token {
        let mut ident = String::new();
        ident.push(ch);
        let mut start = 0;

        if let Some(&(x, c)) = iter.peek() {
            start = x - 1;
        }

        while self.is_ident(iter.peek()) {
            ident.push(iter.next().unwrap().1)
        }

        let end = ident.len() - 1;
        let token;

        if next_is(&mut iter, '(') {
            // function calls or definitions
            token = Token::FunctionCallDef(ident);
        } else if next_is(&mut iter, '!') {
            // macro calls
            ident.push(iter.next().unwrap().1);
            token = Token::Special(ident);
        } else {
            // regular idents
            token = Token::Ident(ident);
        }

        token
    }
}


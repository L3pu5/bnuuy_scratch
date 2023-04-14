//
//   lexer.
//       By L3pu5, L3pu5_Hare
//
//References: https://rustwasm.github.io/book/game-of-life/hello-world.html


use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use core::fmt;

#[wasm_bindgen]
pub fn build_tokens(input_string: JsValue) -> JsValue{
    //Is the input a string?
    if !input_string.is_string(){
        println!("Expected a string, got {}", input_string.as_string().expect("Could not parse Javascript object as string"));
        return JsValue::NULL;
    }
    //Try to parse the string into a Rust String
    let parse_input : String = input_string.as_string()
                                .expect("Could not parse input as std::String");
    //Instantiate the lexer
    let mut lex: Lexer = Lexer::create_from_string(&parse_input);
    //Create the TokenSequence
    let mut output: String = String::from("[");
    while !lex.eof() {
        let t = lex.read_next_token();
        if t.is_some() {
            output.push_str(&t.unwrap().to_json());    
            output.push(',');
        }
    }
    output.remove(output.len()-1);
    output += "]";

    return JsValue::from_str(&output);
}

  
    //TokenType enum
    pub enum TokenKind {
        //Grammar
        OpenCurly,
        CloseCurly,
        OpenParen,
        CloseParen,
        OpenBrack,
        CloseBrack,
    
        Semi,
        Colon,
        Dot,
        Comma,
        //All other grammar 
        NewLine,
        Grammar,
        Space,
        //Literals
        Number,         //All numbers are numbers, including 0x/0b
        String,         //All String-type literals "", '', [[]]
        //Symbols
        Symbol,         //All word types are symbols.
        Invalid,        //Else
    }
    
    impl fmt::Display for TokenKind{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Grammar      => {return write!(f, "Grammar")}
                Self::Dot          => {return write!(f, "Dot")}
                Self::Space        => {return write!(f, "Space")}
                Self::Comma        => {return write!(f, "Comma")}
                Self::Semi         => {return write!(f, "Semi")}
                Self::Colon        => {return write!(f, "Colon")}
                Self::OpenCurly    => {return write!(f, "OpenCurly")}
                Self::CloseCurly   => {return write!(f, "CloseCurly")}
                Self::OpenParen    => {return write!(f, "OpenParen")}
                Self::CloseParen   => {return write!(f, "CloseParen")}
                Self::OpenBrack    => {return write!(f, "OpenBrack")}
                Self::CloseBrack   => {return write!(f, "CloseBrack")}
                Self::Symbol       => {return write!(f, "SYMBOL")}
                Self::Number       => {return write!(f, "NUMBER")}
                Self::Invalid      => {return write!(f, "Invalid")}
                Self::NewLine      => {return write!(f, "NEWLINE")}
                Self::String       => {return write!(f, "STRING")}
                _                  => {return write!(f, "UNIMPLEMENTED")}
            }
        }
    }
    
    
    //Tokens
    pub struct Token<'a> {
        text:       &'a [char],
        pub kind:       TokenKind,
    }
    
    impl fmt::Display for Token<'_>{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            return write!(f, "Token: '{}' of kind '{}'", String::from_iter(self.text), self.kind);
        }
    }

    impl Token<'_> {
        fn to_json(self: &Self) -> String {
            match self.kind {
                TokenKind::String   => {return format!("{{ \"text\": {},  \"kind\": \"{}\"  }}", String::from_iter(self.text), self.kind)}
                TokenKind::NewLine  => {return format!("{{ \"text\": \"NEWLINE\",  \"kind\": \"{}\"  }}", self.kind)}
                _ => {return format!("{{ \"text\": \"{}\",  \"kind\": \"{}\"  }}", String::from_iter(self.text), self.kind)} 
            }
            
        }
    }
    
    impl<'a> Token<'a>{
        fn new(text: &'a [char], kind: TokenKind) -> Token {
            Token {
                text: text,
                kind: kind
            }
        }

        fn copy(t: Token<'a>) -> Token {
            Token {
                text: t.text,
                kind: t.kind
            }
        }
    }
    
    pub struct Lexer {
        cursor:     usize,
        src:        Vec<char>,
        line:       i16,
        col:        i16,
    }
    
    impl Lexer {
        pub fn create_from_string(string_input: &String) -> Lexer {
            Lexer {
                cursor: 0,
                src: string_input.chars().collect(),
                line: 0,
                col: 0,
            }
        }
    
        pub fn eof(self: &Self) -> bool {
            return self.cursor >= self.src.len();
        }
    
        fn current(self: &Self) -> char {
            return self.src[self.cursor-1];
        }
    
        fn advance(self: &mut Self){
            self.cursor += 1;
            self.col += 1;
        }

        fn new_line(self: &mut Self){
            self.col = 0;
            self.line += 1;
        }
    
        fn peek(self: &Self) -> Option<char>{
            if self.cursor <= self.src.len()-1{
                return Some(self.src[self.cursor]);
            }
            return None;
        }
    
        fn skip_white_space(self: &mut Self) -> Option<usize>{
            let start_index = self.cursor-1;
            if !self.current().is_whitespace() && self.current() != '\n'{
                return None;
            }
    
            while self.current().is_whitespace() && self.current() != '\n'{
                //If the character after us is NOT a white space, do not advance, we will already be advanced on.
                if self.peek().is_some() && !self.peek().unwrap().is_whitespace(){
                    if start_index == self.cursor-1{
                        return None;
                    }
                    return Some( start_index)
                }
                self.advance();
                if self.peek().is_none() {
                    if start_index == self.cursor-1{
                        return None;
                    }
                    return Some( start_index)
                }
            }
            return None;
        }

        //Continue consuming characters until we reach the character we started with.
        fn advance_through_string_literal(self: &mut Self, starter: char){
            while self.peek().is_some() {
                let character: char = self.peek().unwrap();
                if character != starter { 
                    self.advance();
                } else {
                    self.advance();
                    break;
                }
            }
            return;
        }
    
        pub fn read_next_token(self: &mut Self) -> Option<Token>{
            if self.eof(){
                return None;
            }

            //Increment the cursor
            self.advance();
            //Skip white space
            match self.skip_white_space(){
                Some( start ) => {return Some( Token::new(&self.src[start..self.cursor], TokenKind::Space))}
                _ => {} //Continue;
            }
    
            let start_index: usize = self.cursor-1;
            let current: char = self.current();
    
            //Symbol
            if current.is_alphabetic(){
                while self.peek().is_some() {
                    let character: char = self.peek().unwrap();
                    if character.is_alphanumeric(){ 
                        self.advance();
                    } else {
                        break;
                    }
                }
                return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Symbol)); 
            }
            
            //Number
            if current.is_numeric(){
                while self.peek().is_some() {
                    let character: char = self.peek().unwrap();
                    if character.is_alphanumeric(){ 
                        self.advance();
                    } else {
                        break;
                    }
                }
                return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Number)); 
            }
            
            //match on char
            match  current {
                ':' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Colon))}
                ';' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Semi))}
                ',' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Comma))}
                '.' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Dot))}
                '{' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::OpenCurly))}
                '}' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::CloseCurly))}
                '[' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::OpenBrack))}
                ']' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::CloseBrack))}
                '(' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::OpenParen))}
                ')' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::CloseParen))}
                '$' | '#' | '<' | '>' | '=' | '!' => {return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::Grammar))}
                '\'' | '\"' => { self.advance_through_string_literal(current); return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::String))}
                '\n' => {self.new_line(); return Some( Token::new(&self.src[start_index..self.cursor], TokenKind::NewLine))}
                '\0'|'\r' => {} //Ignore EOF
                ' ' => {return Some(Token::new(&self.src[start_index..self.cursor], TokenKind::Space))}
                _ => { return Some(Token::new(&self.src[start_index..self.cursor], TokenKind::Grammar))} // panic!("Unimplemented character exception.")}
            }
            
            return None;

            //
            //
        }
    
    }


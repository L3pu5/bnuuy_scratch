//
//   lexer.
//       By L3pu5, L3pu5_Hare
//

fn main(){
    let mut input_string: String = String::new();
    std::io::stdin().read_line(&mut input_string)
        .expect("Could not read input line");
    let mut lex: Lexer = Lexer::create_from_source_string(&input_string);
    while !lex.eof(){
        let t: Token = lex.read_next_token();
        match t.token_type{
            TokenType::NewLine => {}
            _ => {println!("{}", t)}
        }
    }
}

//Token Enum:
enum TokenType{
    //Boundings
    OpenPara,
    ClosePara,
    OpenCurly,
    CloseCurly,
    OpenBrack,
    CloseBrack,
    //Grammar
    Semi,
    Colon,
    Comma,
    Dot,
    // 
    Number,
    Symbol,
    ID,
    NewLine,
    EOF,
    Invalid,
}

//Implement to_string for TokenType
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            Self::OpenPara  => { return write!(f, "OPENPARA");}
            Self::ClosePara => { return write!(f, "CLOSEPARA");}
            Self::OpenBrack  => { return write!(f, "OPENBRACK");}
            Self::CloseBrack => { return write!(f, "CLOSEBRACK");}
            Self::OpenCurly  => { return write!(f, "OPENCURLY");}
            Self::CloseCurly => { return write!(f, "CLOSECURLY");}
            Self::ID        => { return write!(f, "ID");}
            Self::NewLine   => { return write!(f, "NEWLINE");}
            Self::Dot       => { return write!(f, "Dot");}
            Self::Comma     => { return write!(f, "Comma");}
            Self::Semi      => { return write!(f, "Semi");}
            Self::Colon     => { return write!(f, "Colon");}
            Self::Symbol    => { return write!(f, "SYMBOL");}
            Self::Number    => { return write!(f, "NUMBER");}
            Self::EOF       => { return write!(f, "EOF");}
            Self::Invalid   => { return write!(f, "INVALID");}
            _               => { return write!(f, "UNIMPLEMENTED");}
        }
    }
} 

//Token Struct:
struct Token<'a>{
    token_type:     TokenType,
    text:           &'a [char],
    depth_para:     u8,
    depth_curly:    u8,
    depth_brack:    u8,
}

//Create Token
impl<'a> Token<'_>{
    fn new(t_type: TokenType, text: &[char]) -> Token{
        return Token { token_type: t_type, text: text, depth_brack: 0, depth_curly: 0, depth_para: 0};
    }

    fn new_from_Lexer(t_type: TokenType, text: &'a [char], lex: &Lexer) -> Token<'a>{
        return Token { token_type: t_type, text: text, depth_brack: lex.depth_brack, depth_curly: lex.depth_curly, depth_para: lex.depth_para};
    }
}

//Implement to_string for token
impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "'{}' with type '{}'", String::from_iter(self.text), self.token_type);
    }
} 

//Lexer Struct:
// Cursor USize is index of currently considererd character
// Src is byte array &[u8] slice and is lifetime of Lexer.
struct Lexer {
    cursor:     usize,
    current:    char,
    src:        Vec<char>,
    eof:        bool,
    depth_para:     u8,
    depth_curly:    u8,
    depth_brack:    u8,
}

//fmt::Display Lexer<'_> returns the lossy interpretation of src.
impl std::fmt::Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", String::from_iter(self.src.iter()));
    }
} 

impl Lexer {
    fn create_from_source_string(source_code: & String) -> Lexer{
        let mut lex : Lexer = Lexer {
            cursor: 0,
            src:    source_code.chars().collect(),
            current: '\0',
            eof: false,
            depth_para:     0,
            depth_curly:    0,
            depth_brack:    0,
        };
        lex.prime();
        return lex;
    }

    fn advance(self: &mut Self){
        self.cursor += 1;
        if self.cursor == self.src.len(){
            self.eof = true;
        }
        self.current = self.src[self.cursor]
    }

    fn prime(self: &mut Self){
        self.current = self.src[self.cursor];
    }

    fn eof(self: &Self) -> bool {
        return self.eof || self.cursor == self.src.len()-1;
    }

    fn para_depth(self: &mut Self, amount: i8){
        let i: i8 = self.depth_para as i8 + amount;
        self.depth_para = std::cmp::min(self.depth_para, 0) as u8;
    }

    fn curly_depth(self: &mut Self, amount: i8){
        let i: i8 = self.depth_curly as i8 + amount;
        self.depth_curly = std::cmp::min(self.depth_curly, 0) as u8;
    }

    fn brack_depth(self: &mut Self, amount: i8){
        let i: i8 = self.depth_brack as i8 + amount;
        self.depth_brack = std::cmp::min(self.depth_brack, 0) as u8;
    }

    fn read_next_token(self: &mut Self) -> Token {
        //Skip white space
        while self.current == ' ' || self.current == '\t' || self.current == '\n' {
            self.advance();
        }
        let start_cursor: usize = self.cursor;
        //Advance the cursor, as we consume this character.
        if self.current.is_alphabetic() {
            // We are a character.
            self.advance();
            while self.current.is_alphanumeric(){
                self.advance();
            }
            //Push this token
            return Token::new_from_Lexer(TokenType::Symbol, &self.src[start_cursor..self.cursor], &self);
        }

        if self.current.is_numeric() {
            //We are a number
            self.advance();
            while self.current.is_alphanumeric(){
                self.advance();
            }
            return Token::new_from_Lexer(TokenType::Number, &self.src[start_cursor..self.cursor], &self);
        }

        match self.current {
            '\0'    => { 
                            self.eof = true;
                            return Token::new(TokenType::EOF, &self.src[start_cursor..self.cursor])
                        }
            '\n'    => { self.advance(); return Token::new_from_Lexer(TokenType::NewLine, &self.src[start_cursor..self.cursor], &self)}
            '.'     => { self.advance(); return Token::new_from_Lexer(TokenType::Dot, &self.src[start_cursor..self.cursor], &self)}
            ','     => { self.advance(); return Token::new_from_Lexer(TokenType::Comma, &self.src[start_cursor..self.cursor], &self)}
            ';'     => { self.advance(); return Token::new_from_Lexer(TokenType::Semi, &self.src[start_cursor..self.cursor], &self)}
            ':'     => { self.advance(); return Token::new_from_Lexer(TokenType::Colon, &self.src[start_cursor..self.cursor], &self)}
            '{'     =>  { self.curly_depth(1); self.advance(); return Token::new_from_Lexer(TokenType::OpenCurly, &self.src[start_cursor..self.cursor], &self)}
            '}'     =>  { self.curly_depth(-1); self.advance(); return Token::new_from_Lexer(TokenType::CloseCurly, &self.src[start_cursor..self.cursor], &self)}
            '['     =>  { self.brack_depth(1); self.advance(); return Token::new_from_Lexer(TokenType::OpenBrack, &self.src[start_cursor..self.cursor], &self)}
            ']'     =>  { self.brack_depth(-1); self.advance(); return Token::new_from_Lexer(TokenType::CloseBrack, &self.src[start_cursor..self.cursor], &self)}
            ')'     => { self.para_depth(1);  self.advance(); return Token::new_from_Lexer(TokenType::ClosePara, &self.src[start_cursor..self.cursor], &self)} 
            '('     => { self.para_depth(-1); self.advance(); return Token::new_from_Lexer(TokenType::OpenPara, &self.src[start_cursor..self.cursor], &self)} 
            _       => { self.advance(); return Token::new_from_Lexer(TokenType::Invalid, &self.src[start_cursor..self.cursor], &self)}
        }
        
    }
}
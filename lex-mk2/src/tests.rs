#[cfg(test)]
mod tests {
    use crate::lexer::parser::{Lexer, TokenKind};

    #[test]
    fn sequence_of_symbols(){
        let test_case: String = String::from("This is a sequence of symbols");
        let mut count : i8 = 0;
        let mut lexer : Lexer = Lexer::create_from_string(&test_case);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
                if matches!(token.kind, TokenKind::Symbol) {
                    count += 1;
                }
            }
        }
        assert!(count == 6, "There should be six symbols for this text. Found {count}");
    }

    #[test]
    fn sequence_of_numbers(){
        let test_case: String = String::from("123 456 798 159 357");
        let mut count : i8 = 0;
        let mut lexer : Lexer = Lexer::create_from_string(&test_case);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
                if matches!(token.kind, TokenKind::Number) {
                    count += 1;
                }
            }
        }
        assert!(count == 5, "There should be five number for this text. Found {count}");
    }

    #[test]
    fn all_the_braces(){
        let test_case: String = String::from("{{{{}}}}[]()");
        let mut count : i8 = 0;
        let mut lexer : Lexer = Lexer::create_from_string(&test_case);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
                count += 1;
            }
        }
        assert!(count == 12, "There should be 12 [](){{}} dot for this text. Found {count}");
    }

    #[test]
    fn asd_and_123(){
        let test_case: String = String::from("asd123 asd \"123\"");
        let mut count : i8 = 0;
        let mut lexer : Lexer = Lexer::create_from_string(&test_case);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
                count += 1;
            }
        }
        assert!(count == 3, "There should be 3 symbols. Found {count}");
    }
    
    #[test]
    fn dot_dot_dot(){
        let test_case: String = String::from("... thisclass.object.member");
        let mut count : i8 = 0;
        let mut lexer : Lexer = Lexer::create_from_string(&test_case);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
                if matches!(token.kind, TokenKind::Dot) {
                    count += 1;
                }
            }
        }
        assert!(count == 5, "There should be five dot for this text. Found {count}");
    }
}
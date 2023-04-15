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
                if !matches!(token.kind, TokenKind::Space) {
                    count += 1;
                }
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

    #[test]
    fn welcome_text(){
        let welcome_text = String::from("Welcome to bunny_scratch by L3Pu5 Hare!\n|  Everything here has been written from scratch using React/JS and 🦀Rust/Webpack🕸️.\n|  Tokenisation is done with rust, and this simple text editor is written in React/JS.\n\n<-- FileSystem| (Editor) |Settings -->\nFileSytem\n| This is your temporary in-browser (filesystem).\n| Do not save anything important in (/temp), it will be purged when the browser tab dies. VERY temporary.\nEditor\n| This is the centre panel. Excepting this splash screen, you can edit files in here!.\nSettings\n| This area lets you tweak the syntax highlighting and settings such as the tabwidth, etc.\n| You can also use the export feature to save to your local disk.");
        let mut lexer : Lexer = Lexer::create_from_string(&welcome_text);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
            }
        }
    }

    #[test]
    fn starting_with_spaces(){
        let test = String::from("      \nThereareSixBehindMe\n    ");
        let mut lexer : Lexer = Lexer::create_from_string(&test);
        let mut spaces = 0;
        let mut newlines = 0;
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                match token.kind {
                    TokenKind::NewLine => {newlines += 1;}
                    TokenKind::Space =>   {spaces += 1;}
                    _ => {}
                }
                println!("{}", &token);
            }
        }
        assert!(spaces == 2, "Spaces were not 2");
        assert!(newlines == 2, "newlines were not 2");
    }

    #[test]
    fn string_lit(){
        let welcome_text = String::from(" \"don't mind me I am a string literal \"");
        let mut lexer : Lexer = Lexer::create_from_string(&welcome_text);
        while !lexer.eof() {
            let t = lexer.read_next_token();
            if t.is_some() {
                let token = t.unwrap();
                println!("{}", &token);
            }
        }
    }
}
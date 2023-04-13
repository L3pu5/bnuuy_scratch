//
//   Lexer in Rust, second attempt.
//       By L3pu5, L3pu5_Hare
//

// This is a second attempt at a lexer, after I merged the main project without thinking into the wasm build. 
// This one will use a +1 index for the cursor as I continue to learn Rust.

mod lexer;
use lexer::parser;

mod tests;

fn main(){
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Could not parse input from terminal");

    let mut lexer : parser::Lexer = parser::Lexer::create_from_string(&input);
    while !lexer.eof() {
        let t = lexer.read_next_token();
        if t.is_some() {
            println!("{}", t.unwrap())
        }
    }
    println!("Done!");
}

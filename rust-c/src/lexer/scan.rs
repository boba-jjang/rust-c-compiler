use std::fs;
// use super::token::Token;
/*
super → go up to lexer
token → go into lexer::token
Token → the Token enum defined there
*/

pub fn scan(input: &str) { // -> Vec<Token>
   let contents : String = fs::read_to_string(input).expect("Should have been able to read the file");
   println!("Within contents\n{contents}");
    // for line in input {
    //     println!("{line}");
    // }
    // let mut tokens = Vec::new();

    // for word in input.split_whitespace() {
    //     tokens.push(Token::Word(word.to_string()));
    // }

    // tokens
}
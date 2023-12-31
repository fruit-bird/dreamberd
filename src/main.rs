mod lexer;
mod parser;

use logos::Logos;

use crate::lexer::GoofyToken;

fn main() {
    let input = "union do_stuff(arg1: Bruh) { return 25001 }";
    let goofy_lexer = GoofyToken::lexer(input);

    for token in goofy_lexer {
        println!("{:?}", token);
    }
}

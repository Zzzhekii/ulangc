use ulangc::parser::{Parser, Lexer};

use env_logger;

fn main() {
    // Initialize the logger.
    env_logger::init();

    let mut lexer = Lexer::from_str(
"static main = fn do
    std::println ('Hello, World!')
end");

    let _ast = match Parser::parse(&mut lexer) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{}", err);
            return
        }
    }; 
}

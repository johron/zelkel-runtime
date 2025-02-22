mod parser;
mod lexer;
mod evaluator;

fn main() {
    let code = String::from(r#"
.entry:
    psh 14
    psh "Hello, world!\n"
    psh 1
    psh 1
    sys
"#);
    let tokens = lexer::lex(code).expect("Failed to lex");

    let parsed = parser::parse(tokens).expect("Failed to parse");

    let evaluated = evaluator::evaluate(parsed).expect("Failed to evaluate");
    let stack = evaluated.0;
    let code = evaluated.1;

    println!("{:?}", stack);

    std::process::exit(code);
}

#[cfg(test)]
mod tests;

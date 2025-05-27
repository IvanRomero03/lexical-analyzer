use lexical_analyzer::lexer::{process_text, Token};

fn main() {
    let mut input = String::new();

    println!("Enter a string to tokenize:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    let result_tokens = process_text(&input);
    println!("Tokens found:");

    for token in result_tokens.iter() {
        let mut out = "<TOKEN: ".to_string();
        out.push_str(&format!("{:?}", token.token));
        out.push_str(" ");
        out.push_str(&format!("value: {:?}", token.value));
        if token.token == Token::String(token.value.clone()) {
            out.push_str(" (String): ");
            out.push_str(token.value.as_str());
        } else if matches!(token.token, Token::Int(_)) {
            out.push_str(" (Int): ");
            out.push_str(token.value.as_str());
        } else if matches!(token.token, Token::Float(_)) {
            out.push_str(" (Float): ");
            out.push_str(token.value.as_str());
        }
        out.push_str(">");
        println!("{}", out);
    }
}

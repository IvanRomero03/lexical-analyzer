use plex::lexer;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Float(f32),
    String(String),
    Int(i32),
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Divide,             // /
    If,                 // if
    Else,               // else
    While,              // while
    PlusEquals,         // +=
    MultiplyEquals,     // *=
    MinusEquals,        // -=
    DivideEquals,       // /=
    NotEquals,          // !=
    GreaterThan,        // >
    LessThan,           // <
    GreaterThanOrEqual, // >=
    LessThanOrEqual,    // <=
    Whitespace,         // Whitespace (ignored)
}

lexer! {
    pub(crate) fn take_token(text: 'a) -> Token;
    r#"-?[0-9]*\.[0-9]*"# => Token::Float(text.parse().unwrap()),
    r#"-?[0-9]+"# => Token::Int(text.parse().unwrap()),
    r#"if"# => Token::If,
    r#"else"# => Token::Else,
    r#"while"# => Token::While,
    r#"[a-zA-Z][a-zA-Z0-9_]*"# => {
        let txt = text.to_string();
        Token::String(txt)
    },
    r#"\+"# => Token::Plus,
    r#"-"# => Token::Minus,
    r#"\*"# => Token::Multiply,
    r#"/"# => Token::Divide,
    r#"\+="# => Token::PlusEquals,
    r#"\*="# => Token::MultiplyEquals,
    r#"-="# => Token::MinusEquals,
    r#"/="# => Token::DivideEquals,
    r#"\!="# => Token::NotEquals,
    r#">="# => Token::GreaterThanOrEqual,
    r#"<"# => Token::LessThan,
    r#">"# => Token::GreaterThan,
    r#"<="# => Token::LessThanOrEqual,
    //x 78 8.4 -55 size55 54RR if <= while x += *= /= -= !=
}

pub struct ResultingToken {
    pub token: Token,
    pub value: String,
}

pub fn process_text(text: &str) -> Vec<ResultingToken> {
    let mut result_tokens: Vec<ResultingToken> = Vec::new();

    let tokens = text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for token in tokens.iter() {
        let output_tokens: Vec<_> = take_token(token.as_str()).into_iter().collect();
        // if more than one token then throw error
        if output_tokens.len() > 1 {
            println!("Error: More than one token found for input '{}'", token);
            continue;
        }
        if output_tokens.len() == 0 {
            println!("Error: No token found for input '{}'", token);
            continue;
        }
        if output_tokens[0].1.len() > 0 {
            println!("Error: No token found for input '{}'", token);
            continue;
        }
        for tok in output_tokens.iter() {
            result_tokens.push(ResultingToken {
                token: tok.0.clone(),
                value: token.to_string(),
            });
        }
    }

    return result_tokens;
}

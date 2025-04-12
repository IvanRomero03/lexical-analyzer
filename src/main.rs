use plex::lexer;

#[derive(Clone, Debug, PartialEq)]
enum Token {
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
}

lexer! {
    fn take_token(text: 'a) -> Token;
    r#"-?[0-9]+\.[0-9]+"# => Token::Float(text.parse().unwrap()),
    r#"-?[0-9]+"# => Token::Int(text.parse().unwrap()),
    r#"if"# => Token::If,
    r#"else"# => Token::Else,
    r#"while"# => Token::While,
    r#"[a-z|A-Z]+[(a-z)|(A-Z)|(0-9)|(_)]*"# => {
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

struct ResultingToken {
    token: Token,
    value: String,
}

fn process_text(text: &str) -> Vec<ResultingToken> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "x 78 8.4 -55 size55 54RR if <= while x += *= /= -= !=";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 14);
        assert_eq!(result_tokens[0].token, Token::String("x".to_string()));
        assert_eq!(result_tokens[1].token, Token::Int(78));
        assert_eq!(result_tokens[2].token, Token::Float(8.4));
        assert_eq!(result_tokens[3].token, Token::Int(-55));
        assert_eq!(result_tokens[4].token, Token::String("size55".to_string()));
        assert_eq!(result_tokens[5].token, Token::If);
        assert_eq!(result_tokens[6].token, Token::LessThanOrEqual);
        assert_eq!(result_tokens[7].token, Token::While);
        assert_eq!(result_tokens[8].token, Token::String("x".to_string()));
        assert_eq!(result_tokens[9].token, Token::PlusEquals);
        assert_eq!(result_tokens[10].token, Token::MultiplyEquals);
        assert_eq!(result_tokens[11].token, Token::DivideEquals);
        assert_eq!(result_tokens[12].token, Token::MinusEquals);
        assert_eq!(result_tokens[13].token, Token::NotEquals);
    }

    #[test]
    fn test_invalid_token() {
        let input = "1invalid_token";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 0);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 0);
    }

    #[test]
    fn test_invalid_float() {
        let input = "3.14.15";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 0);
    }

    #[test]
    fn test_invalid_int() {
        let input = "123abc";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 0);
    }

    #[test]
    fn valid_int() {
        let input = "123 -123";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 2);
        assert_eq!(result_tokens[0].token, Token::Int(123));
        assert_eq!(result_tokens[1].token, Token::Int(-123));
    }
    #[test]
    fn valid_float() {
        let input = "123.45 -123.5";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 2);
        assert_eq!(result_tokens[0].token, Token::Float(123.45));
        assert_eq!(result_tokens[1].token, Token::Float(-123.5));
    }

    #[test]
    fn valid_string() {
        let input = "hello world";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 2);
        assert_eq!(result_tokens[0].token, Token::String("hello".to_string()));
        assert_eq!(result_tokens[1].token, Token::String("world".to_string()));
    }

    #[test]
    fn valid_operators() {
        let input = "+ - * / += -= *= /=";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 8);
        assert_eq!(result_tokens[0].token, Token::Plus);
        assert_eq!(result_tokens[1].token, Token::Minus);
        assert_eq!(result_tokens[2].token, Token::Multiply);
        assert_eq!(result_tokens[3].token, Token::Divide);
        assert_eq!(result_tokens[4].token, Token::PlusEquals);
        assert_eq!(result_tokens[5].token, Token::MinusEquals);
        assert_eq!(result_tokens[6].token, Token::MultiplyEquals);
        assert_eq!(result_tokens[7].token, Token::DivideEquals);
    }
    #[test]
    fn valid_comparisons() {
        let input = "!= > < >= <=";
        let result_tokens = process_text(input);
        assert_eq!(result_tokens.len(), 5);
        assert_eq!(result_tokens[0].token, Token::NotEquals);
        assert_eq!(result_tokens[1].token, Token::GreaterThan);
        assert_eq!(result_tokens[2].token, Token::LessThan);
        assert_eq!(result_tokens[3].token, Token::GreaterThanOrEqual);
        assert_eq!(result_tokens[4].token, Token::LessThanOrEqual);
    }
}

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

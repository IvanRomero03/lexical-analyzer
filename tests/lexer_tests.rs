use lexical_analyzer::lexer::{process_text, Token};

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

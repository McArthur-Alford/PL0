#[derive(Debug, Clone, PartialEq)]
enum Token {
    PERIOD,                        // .
    SEMICOLON,                     // ;
    COMMA,                         // ,
    ASSIGN,                        // :=
    LPAREN,                        // (
    RPAREN,                        // )
    PLUS,                          // +
    MINUS,                         // -
    TIMES,                         // *
    SLASH,                         // /
    EQUAL,                         // =
    NEQUAL,                        // <>
    LESS,                          // <
    LEQUAL,                        // <=
    GREATER,                       // >
    GEQUAL,                        // >=
    WHILE,                         // while
    THEN,                          // then
    ELSE,                          // else
    END,                           // end
    IF,                            // if
    IDENTIFIER(String),            // [a-zA-Z][a-zA-Z0-9]*
    NUMBER(i32),                   // [0-9]+
    INVALID(String, usize, usize), // Anything Else
}

impl Token {
    fn read_token(text: &String, a: usize, b: usize) -> (Token, usize) {
        let l = b;
        let t = match &text[a..b] {
            ":=" => Token::ASSIGN,
            "<>" => Token::NEQUAL,
            "<=" => Token::LEQUAL,
            ">=" => Token::GEQUAL,
            "." => Token::PERIOD,
            ";" => Token::SEMICOLON,
            "," => Token::COMMA,
            "(" => Token::LPAREN,
            ")" => Token::RPAREN,
            "+" => Token::PLUS,
            "-" => Token::MINUS,
            "*" => Token::TIMES,
            "/" => Token::SLASH,
            "=" => Token::EQUAL,
            "<" => Token::LESS,
            ">" => Token::GREATER,
            "while"  => Token::WHILE,
            "then"   => Token::THEN,
            "else"   => Token::ELSE,
            "end"    => Token::END,
            "if"     => Token::IF,
            s if s.chars().all(|c| c.is_alphanumeric())
                && s.chars().nth(0).unwrap_or_else(|| '-').is_alphabetic()
                => Token::IDENTIFIER(s.to_string()),
            s if s.chars().all(|c| c.is_numeric())
                => {
                    match s.parse::<i32>() {
                        Ok(i) => Token::NUMBER(i),
                        Err(_)      => Token::INVALID(s.to_string(), a, b)
                    }
                }
            _ => Token::INVALID(text[a..b].to_string(), a, b),
        };

        if matches!(t, Token::INVALID(_, _, _)) 
            || b >= text.len()
            || text[a..b].contains(char::is_whitespace) {
            // Base case
            // stop if we hit whitespace, exceed text length, 
            // or hit something unmatchable
            return (t, l);
        }

        // the token was a valid match, lets see if we can make it one longer
        let (ft, fl) = Token::read_token(text, a, b + 1);
        match matches!(ft, Token::INVALID(_, _, _)) {
            true => (t, l),
            false => (ft, fl)
        }
    }
}

fn read(text: String) -> Vec<Token> {
    let mut n = 0;
    let mut tokens = Vec::new();
    while n < text.len() {
        let (token, l) = Token::read_token(&text, n, n);
        tokens.push(token);
        n += l;
    }
    tokens
}


#[cfg(test)]
mod Tests {
    use super::*;

    #[test]
    fn test_read_token() {
        let text = "Magic".to_string();
        let (t, l) = Token::read_token(&text, 0, 1);
        assert_eq!(text.len(), l);
        assert_eq!(Token::IDENTIFIER(text.clone()), t);
    }

    #[test]
    fn test_read_program() {
        let text = "
            if x == 1 then x:=y+2 else x :=0 end
            x := x + 1;
        ";

        let tokens = read(text.to_string());
        assert_eq!(
            vec![
                Token::IF,
                Token::IDENTIFIER("x".to_string()),
                Token::EQUAL,
                Token::NUMBER(1),
                Token::THEN,
                Token::IDENTIFIER("x".to_string()),
                Token::ASSIGN,
                Token::IDENTIFIER("y".to_string()),
                Token::PLUS,
                Token::NUMBER(2),
                Token::ELSE,
                Token::IDENTIFIER("x".to_string()),
                Token::ASSIGN,
                Token::NUMBER(0),
                Token::END,
                Token::IDENTIFIER("x".to_string()),
                Token::ASSIGN,
                Token::IDENTIFIER("x".to_string()),
                Token::PLUS,
                Token::NUMBER(1),
                Token::SEMICOLON,
            ],
            tokens
        )
    }
}
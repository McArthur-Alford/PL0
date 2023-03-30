// fn traverse_string(sentence: String, n: usize) -> String {
//     sentence.chars().skip(n).collect()
// }

// trait Symbol where Self: Sized {
//     /*  read
//         @param sentence: &mut String
//         @return (Option<Self>, usize)
//         lhs Returns None if the sentence is not parseable.
//         lhs Returns Some(Self) if the sentence is parseable.
//         rhs Returns the number of characters read from the sentence.
//      */
//     fn read(sentence: &mut String) -> (Option<Self>, usize);
// }

// #[derive(Debug, Clone, PartialEq)]
// enum Token {
//     PERIOD,             // .
//     SEMICOLON,          // ;
//     COMMA,              // ,
//     ASSIGN,             // :=
//     LPAREN,             // (
//     RPAREN,             // )
//     PLUS,               // +
//     MINUS,              // -
//     TIMES,              // *
//     SLASH,              // /
//     EQUAL,              // =
//     NEQUAL,             // <>
//     LESS,               // <
//     LEQUAL,             // <=
//     GREATER,            // >
//     GEQUAL,             // >=
//     IDENTIFIER(String), // [a-zA-Z][a-zA-Z0-9]*
//     NUMBER(i32),        // [0-9]+
// }

// #[derive(Debug, Clone, PartialEq)]
// struct ParseError {
//     message: String,
//     location: usize,
// }

// impl Token {
//     fn parse_word(sentence: String) -> Result<Self, String> {
//         /*  parse_word
//             @param sentence: String
//             @return Result<Self, String>
//             returns the Token that corresponds to the next word.
//             returns Err(String) if the word is not a valid token. 
//                 String is the word that was not a valid token.
//          */
//         loop {
//             let x = match sentence.as_str() {
//                 s if s.starts_with(":=") => Ok(Self::ASSIGN),
//                 s if s.starts_with("<>") => Ok(Self::NEQUAL),
//                 s if s.starts_with("<=") => Ok(Self::LEQUAL),
//                 s if s.starts_with(">=") => Ok(Self::GEQUAL),
//                 s if s.starts_with(".")  => Ok(Self::PERIOD),
//                 s if s.starts_with(";")  => Ok(Self::SEMICOLON),
//                 s if s.starts_with(",")  => Ok(Self::COMMA),
//                 s if s.starts_with("(")  => Ok(Self::LPAREN),
//                 s if s.starts_with(")")  => Ok(Self::RPAREN),
//                 s if s.starts_with("+")  => Ok(Self::PLUS),
//                 s if s.starts_with("-")  => Ok(Self::MINUS),
//                 s if s.starts_with("*")  => Ok(Self::TIMES),
//                 s if s.starts_with("/")  => Ok(Self::SLASH),
//                 s if s.starts_with("=")  => Ok(Self::EQUAL),
//                 s if s.starts_with("<")  => Ok(Self::LESS),
//                 s if s.starts_with(">")  => Ok(Self::GREATER),
//                 s if s.chars().all(|c| c.is_numeric()) 
//                     => Ok(Self::NUMBER(s.parse().unwrap())),
//                 s if s.chars().all(|c| c.is_alphanumeric())
//                     && s.chars().next().unwrap().is_alphabetic()
//                     => Ok(Self::IDENTIFIER(s.to_string())),
//                 s => Err(s),
//             };
//             match x {
//                 Ok(token) => return Ok(token),
//                 Err(s) => {
//                     let n = s.chars().next().unwrap().len_utf8();
//                     sentence = traverse_string(sentence, n);
//                 }
//             }
//         }
//     }

//     fn parse_sentence(sentence: String) -> Result<Vec<Self>, Vec<ParseError>> {
//         /* parse_sentence
//             @param sentence: String
//             @return Result<Vec<Self>, Vec<ParseError>>
//             Converts a string into a vector of tokens.
//             returns Err(Vec<ParseError>) if the sentence is not a valid token.
//                 Vec<ParseError> contains the error message for each invalid token.
//          */

        
//     }
// }

// /*
//     program = block "." ;
//     program = block PERIOD SEMICOLON
// */
// struct ProgramNode {
//     block: BlockNode,
// }

// impl Symbol for ProgramNode {
//     fn read(sentence: &mut String) -> (Option<Self>, usize) {
//         match BlockNode::read(sentence) {
//             (Some(block), n) => {
//                 let mut sentence = traverse_string(sentence, n);
//                 match Terminal::read(&mut sentence) {
//                     (Some(Terminal::PERIOD), n) => (Some(Self { block }), n),
//                     (None, n) => (None, n)
//                 }
//             },
//             },
//             (None, n) => (None, n)
//         }
//     }
// }

// struct BlockNode {}

// impl Symbol for BlockNode {
//     fn read(sentence: &str) -> (Option<Self>, usize) {
//         todo!();
//     }
// }

// struct StatementNode {}

// struct ConditionNode {}

// struct ExpressionNode {}

// struct TermNode {}

// struct FactorNode {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_traverse_string() {
//         let sentence = "Hello, World!".to_string();
//         let n = 7;
//         let expected = "World!".to_string();
//         let actual = traverse_string(sentence, n);
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     fn test_parse_word() {
//         let word = "Hello".to_string();
//         let expected = Ok(Token::IDENTIFIER(word.clone()));
//         let actual = Token::parse_word(word);
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     fn test_parse_sentence() {
//         let sentence = "Hello, World!".to_string();
//         let expected = Ok(vec![
//             Token::IDENTIFIER("Hello".to_string()),
//             Token::COMMA,
//             Token::IDENTIFIER("World".to_string()),
//             Token::PERIOD,
//         ]);
//         let actual = Token::parse_sentence(sentence);
//         assert_eq!(expected, actual);
//     }
// }
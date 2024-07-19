#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                 x + y;
            };

            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";

        struct Test {
            expected_type: TokenType,
            expected_litteral: String,
        }

        impl Test {
            fn new<T>(tt: TokenType, litt: T) -> Test
            where
                T: Into<String>,
            {
                Test {
                    expected_type: tt,
                    expected_litteral: litt.into(),
                }
            }
        }

        let tests = [
            Test::new(TokenType::LET, "let"),
            Test::new(TokenType::IDENT, "five"),
            Test::new(TokenType::ASSIGN, "="),
            Test::new(TokenType::INT, "5"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::LET, "let"),
            Test::new(TokenType::IDENT, "ten"),
            Test::new(TokenType::ASSIGN, "="),
            Test::new(TokenType::INT, "10"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::LET, "let"),
            Test::new(TokenType::IDENT, "add"),
            Test::new(TokenType::ASSIGN, "="),
            Test::new(TokenType::FUNCTION, "fn"),
            Test::new(TokenType::LPAREN, "("),
            Test::new(TokenType::IDENT, "x"),
            Test::new(TokenType::COMMA, ","),
            Test::new(TokenType::IDENT, "y"),
            Test::new(TokenType::RPAREN, ")"),
            Test::new(TokenType::LBRACE, "{"),
            Test::new(TokenType::IDENT, "x"),
            Test::new(TokenType::PLUS, "+"),
            Test::new(TokenType::IDENT, "y"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::RBRACE, "}"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::LET, "let"),
            Test::new(TokenType::IDENT, "result"),
            Test::new(TokenType::ASSIGN, "="),
            Test::new(TokenType::IDENT, "add"),
            Test::new(TokenType::LPAREN, "("),
            Test::new(TokenType::IDENT, "five"),
            Test::new(TokenType::COMMA, ","),
            Test::new(TokenType::IDENT, "ten"),
            Test::new(TokenType::RPAREN, ")"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::BANG, "!"),
            Test::new(TokenType::MINUS, "-"),
            Test::new(TokenType::SLASH, "/"),
            Test::new(TokenType::ASTERISK, "*"),
            Test::new(TokenType::INT, "5"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::INT, "5"),
            Test::new(TokenType::LT, "<"),
            Test::new(TokenType::INT, "10"),
            Test::new(TokenType::GT, ">"),
            Test::new(TokenType::INT, "5"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::IF, "if"),
            Test::new(TokenType::LPAREN, "("),
            Test::new(TokenType::INT, "5"),
            Test::new(TokenType::LT, "<"),
            Test::new(TokenType::INT, "10"),
            Test::new(TokenType::RPAREN, ")"),
            Test::new(TokenType::LBRACE, "{"),
            Test::new(TokenType::RETURN, "return"),
            Test::new(TokenType::TRUE, "true"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::RBRACE, "}"),
            Test::new(TokenType::ELSE, "else"),
            Test::new(TokenType::LBRACE, "{"),
            Test::new(TokenType::RETURN, "return"),
            Test::new(TokenType::FALSE, "false"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::RBRACE, "}"),
            Test::new(TokenType::INT, "10"),
            Test::new(TokenType::EQ, "=="),
            Test::new(TokenType::INT, "10"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::INT, "10"),
            Test::new(TokenType::NOTEq, "!="),
            Test::new(TokenType::INT, "9"),
            Test::new(TokenType::SEMICOLON, ";"),
            Test::new(TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        tests.map(|t| {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, t.expected_type, "Wrong type");
            assert_eq!(tok.litteral, t.expected_litteral, "Wrong Litteral");
        });
    }
}

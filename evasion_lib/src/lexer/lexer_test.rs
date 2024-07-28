#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenTypes};

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
            expected_type: TokenTypes,
            expected_litteral: String,
        }

        impl Test {
            fn new<T>(tt: TokenTypes, litt: T) -> Test
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
            Test::new(TokenTypes::LET, "let"),
            Test::new(TokenTypes::IDENT, "five"),
            Test::new(TokenTypes::ASSIGN, "="),
            Test::new(TokenTypes::INT, "5"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::LET, "let"),
            Test::new(TokenTypes::IDENT, "ten"),
            Test::new(TokenTypes::ASSIGN, "="),
            Test::new(TokenTypes::INT, "10"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::LET, "let"),
            Test::new(TokenTypes::IDENT, "add"),
            Test::new(TokenTypes::ASSIGN, "="),
            Test::new(TokenTypes::FUNCTION, "fn"),
            Test::new(TokenTypes::LPAREN, "("),
            Test::new(TokenTypes::IDENT, "x"),
            Test::new(TokenTypes::COMMA, ","),
            Test::new(TokenTypes::IDENT, "y"),
            Test::new(TokenTypes::RPAREN, ")"),
            Test::new(TokenTypes::LBRACE, "{"),
            Test::new(TokenTypes::IDENT, "x"),
            Test::new(TokenTypes::PLUS, "+"),
            Test::new(TokenTypes::IDENT, "y"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::RBRACE, "}"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::LET, "let"),
            Test::new(TokenTypes::IDENT, "result"),
            Test::new(TokenTypes::ASSIGN, "="),
            Test::new(TokenTypes::IDENT, "add"),
            Test::new(TokenTypes::LPAREN, "("),
            Test::new(TokenTypes::IDENT, "five"),
            Test::new(TokenTypes::COMMA, ","),
            Test::new(TokenTypes::IDENT, "ten"),
            Test::new(TokenTypes::RPAREN, ")"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::BANG, "!"),
            Test::new(TokenTypes::MINUS, "-"),
            Test::new(TokenTypes::SLASH, "/"),
            Test::new(TokenTypes::ASTERISK, "*"),
            Test::new(TokenTypes::INT, "5"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::INT, "5"),
            Test::new(TokenTypes::LT, "<"),
            Test::new(TokenTypes::INT, "10"),
            Test::new(TokenTypes::GT, ">"),
            Test::new(TokenTypes::INT, "5"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::IF, "if"),
            Test::new(TokenTypes::LPAREN, "("),
            Test::new(TokenTypes::INT, "5"),
            Test::new(TokenTypes::LT, "<"),
            Test::new(TokenTypes::INT, "10"),
            Test::new(TokenTypes::RPAREN, ")"),
            Test::new(TokenTypes::LBRACE, "{"),
            Test::new(TokenTypes::RETURN, "return"),
            Test::new(TokenTypes::TRUE, "true"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::RBRACE, "}"),
            Test::new(TokenTypes::ELSE, "else"),
            Test::new(TokenTypes::LBRACE, "{"),
            Test::new(TokenTypes::RETURN, "return"),
            Test::new(TokenTypes::FALSE, "false"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::RBRACE, "}"),
            Test::new(TokenTypes::INT, "10"),
            Test::new(TokenTypes::EQ, "=="),
            Test::new(TokenTypes::INT, "10"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::INT, "10"),
            Test::new(TokenTypes::NOTEq, "!="),
            Test::new(TokenTypes::INT, "9"),
            Test::new(TokenTypes::SEMICOLON, ";"),
            Test::new(TokenTypes::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        tests.map(|t| {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, t.expected_type, "Wrong type");
            assert_eq!(tok.litteral, t.expected_litteral, "Wrong Litteral");
        });
    }
}

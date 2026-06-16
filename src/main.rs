enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    StringToken(String),
    NumberToken(f64),
    True,
    False,
    Null,
}

enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    Str(String),
    Number(f64),
    Bool(bool),
    Null,
}

struct Lexer {
    input: Vec<u8>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input.as_bytes().to_vec(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn peek(&self) -> Option<u8> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

    fn read_keyword(&mut self, keyword: &str) {
        for expected in keyword.as_bytes() {
            match self.current() {
                Some(c) if c == *expected => self.advance(),
                Some(c) => panic!(
                    "Unexpected character '{}' while reading keyword '{}'",
                    c as char, keyword
                ),
                None => panic!(
                    "Unexpected end of input while reading keyword '{}'",
                    keyword
                ),
            }
        }
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.current() {
            None => break,

            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                lexer.advance();
            }

            Some(b'{') => {
                tokens.push(Token::LeftBrace);
                lexer.advance();
            }
            Some(b'}') => {
                tokens.push(Token::RightBrace);
                lexer.advance();
            }
            Some(b'[') => {
                tokens.push(Token::LeftBracket);
                lexer.advance();
            }
            Some(b']') => {
                tokens.push(Token::RightBracket);
                lexer.advance();
            }
            Some(b':') => {
                tokens.push(Token::Colon);
                lexer.advance();
            }
            Some(b',') => {
                tokens.push(Token::Comma);
                lexer.advance();
            }

            Some(b'"') => {
                let s = lexer.read_string();
                tokens.push(Token::StringToken(s));
            }

            Some(b't') => {
                lexer.read_keyword("true");
                tokens.push(Token::True);
            }
            Some(b'f') => {
                lexer.read_keyword("false");
                tokens.push(Token::False);
            }
            Some(b'n') => {
                lexer.read_keyword("null");
                tokens.push(Token::Null);
            }

            Some(b'-') | Some(b'0'..=b'9') => {
                let n = lexer.read_number();
                tokens.push(Token::NumberToken(n));
            }

            Some(c) => {
                panic!("Unexpected character: {}", c as char);
            }
        }
    }

    tokens
}
fn parse(tokens: &[Token]) -> JsonValue {
    todo!()
}

fn main() {
    let input = r#"{"name": "alice", "age": 30, "active": true}"#;
    let tokens = tokenize(input);
    let value = parse(&tokens);
}

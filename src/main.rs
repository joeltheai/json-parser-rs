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

fn tokenize(input: &str) -> Vec<Token> {
    todo!()
}

fn parse(tokens: &[Token]) -> JsonValue {
    todo!()
}

fn main() {
    let input = r#"{"name": "alice", "age": 30, "active": true}"#;
    let tokens = tokenize(input);
    let value = parse(&tokens);
}

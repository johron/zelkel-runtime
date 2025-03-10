use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Identifier(String),
    Label(String),
    Function(String),
    Integer(i32),
    Float(f32),
    String(String),
    Punctuation(char),
    Buffer(String),
    Variable(String),
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenValue::Integer(i) => write!(f, "{}", i),
            TokenValue::Float(fl) => write!(f, "{}", fl),
            TokenValue::String(s) => write!(f, "{}", s),
            TokenValue::Identifier(id) => write!(f, "{}", id),
            TokenValue::Label(l) => write!(f, "{}", l),
            TokenValue::Punctuation(p) => write!(f, "{}", p),
            TokenValue::Function(fn_name) => write!(f, "{}", fn_name),
            TokenValue::Buffer(b) => write!(f, "{}", b),
            TokenValue::Variable(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: &'static str,
    pub value: TokenValue,
}

fn until<F>(chars: &Vec<char>, start: usize, check: F) -> (String, usize)
where
    F: Fn(char) -> bool
{
    let mut cur = start;
    let mut value = String::new();
    while cur < chars.len() && check(chars[cur]) {
        if chars[cur] == '\\' && cur + 1 < chars.len() {
            cur += 1;
            match chars[cur] {
                'n' => value.push('\n'),
                't' => value.push('\t'),
                '\\' => value.push('\\'),
                '"' => value.push('"'),
                _ => value.push(chars[cur]),
            }
            cur += 1;
        } else {
            value.push(chars[cur]);
            cur += 1;
        }
    }

    (value, cur)
}

fn could_be(c: char, s: &str) -> bool {
    s.chars().any(|x| x == c)
}

pub fn lex(input: String) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = vec![];
    let mut cur = 0;

    while cur < chars.len() {
        let c = chars[cur];
        if c.is_alphabetic() {
            let value = until(&chars, cur, |c| c.is_alphanumeric() || c == '_');
            tokens.push(Token { kind: "identifier", value: TokenValue::Identifier(value.0) });
            cur = value.1;
        } else if c == '.' && cur + 1 < chars.len() && chars[cur + 1].is_alphabetic() {
            cur += 1;
            let value = until(&chars, cur, |c| c.is_alphanumeric() || c == '_');
            tokens.push(Token { kind: "label", value: TokenValue::Label(".".to_owned() + &*value.0) });
            cur = value.1;
        } else if c == '@' {
            let value = until(&chars, cur + 1, |c| c.is_alphanumeric() || c == '_');
            tokens.push(Token { kind: "function", value: TokenValue::Function("@".to_owned() + &*value.0) });
            cur = value.1;
        } else if c == '*' {
            let value = until(&chars, cur + 1, |c| c.is_alphanumeric() || c == '_');
            tokens.push(Token { kind: "buffer", value: TokenValue::Buffer("*".to_owned() + &*value.0) });
            cur = value.1;
        } else if c == '$' {
            let value = until(&chars, cur + 1, |c| c.is_alphanumeric() || c == '_');
            tokens.push(Token { kind: "variable", value: TokenValue::Variable("$".to_owned() + &*value.0) });
            cur = value.1;
        } else if c.is_digit(10) || c == '.' {
            let value = until(&chars, cur, |c| c.is_digit(10) || c == '.');
            if value.0.contains('.') {
                let float_value: f32 = value.0.parse().map_err(|_| format!("Invalid float: '{}'", value.0))?;
                tokens.push(Token { kind: "float", value: TokenValue::Float(float_value) });
            } else {
                let integer_value: i32 = value.0.parse().map_err(|_| format!("Invalid integer: '{}'", value.0))?;
                tokens.push(Token { kind: "integer", value: TokenValue::Integer(integer_value) });
            }
            cur = value.1;
        } else if c == '"' {
            let value = until(&chars, cur + 1, |c| c != '"');
            let string_value = value.0;
            cur = value.1 + 1;

            tokens.push(Token { kind: "string", value: TokenValue::String(string_value) });
        } else if could_be(c, ":,") {
            tokens.push(Token { kind: "punctuation", value: TokenValue::Punctuation(c) });
            cur += 1;
        } else if c.is_whitespace() {
            cur += 1;
        } else {
            Err(format!("Unexpected character: '{}'", c))?;
        }
    }

    Ok(tokens)
}
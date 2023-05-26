use std::{env, error::Error, fmt, fs, io, path::Path};

#[derive(Debug)]
enum TokenType {
    LEFT_PAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    IDENTIFIER,
    STRING,
    NUMBER,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }

    fn to_string(&self) -> String {
        format!("{} {} ", self.token_type, self.lexeme)
    }
}

#[derive(Debug)]
struct Scanner {
    source: String,
    token_list: Vec<Token>,
}

impl Scanner {
    fn new(source: String) -> Self {
        let lines = source.split("\n").collect::<Vec<&str>>();
        let token_list = scan_token(&lines);
        Self { source, token_list }
    }
}

fn scan_token(lines: &Vec<&str>) -> Vec<Token> {
    let mut token: Vec<Token> = Vec::new();
    for (k, v) in lines.into_iter().enumerate() {
        for i in v.split(" ").collect::<Vec<_>>().into_iter() {
            match i {
                "(" => token.push(Token::new(TokenType::LEFT_PAREN, i.to_string(), k)),
                ")" => token.push(Token::new(TokenType::RIGHTPAREN, i.to_string(), k)),
                "{" => token.push(Token::new(TokenType::LEFTBRACE, i.to_string(), k)),
                "}" => token.push(Token::new(TokenType::RIGHTBRACE, i.to_string(), k)),
                "," => token.push(Token::new(TokenType::COMMA, i.to_string(), k)),
                "." => token.push(Token::new(TokenType::DOT, i.to_string(), k)),
                "-" => token.push(Token::new(TokenType::MINUS, i.to_string(), k)),
                "+" => token.push(Token::new(TokenType::PLUS, i.to_string(), k)),
                ";" => token.push(Token::new(TokenType::SEMICOLON, i.to_string(), k)),
                "*" => token.push(Token::new(TokenType::STAR, i.to_string(), k)),
                " " => token.push(Token::new(TokenType::EOF, i.to_string(), k)),
                "!" => token.push(Token::new(TokenType::BANG, i.to_string(), k)),
                "!=" => token.push(Token::new(TokenType::BANGEQUAL, i.to_string(), k)),
                "=" => token.push(Token::new(TokenType::EQUAL, i.to_string(), k)),
                "==" => token.push(Token::new(TokenType::EQUALEQUAL, i.to_string(), k)),
                "<" => token.push(Token::new(TokenType::LESS, i.to_string(), k)),
                "<=" => token.push(Token::new(TokenType::LESSEQUAL, i.to_string(), k)),
                ">" => token.push(Token::new(TokenType::GREATER, i.to_string(), k)),
                ">=" => token.push(Token::new(TokenType::GREATEREQUAL, i.to_string(), k)),
                _ => {
                    eprintln!("{}", i.to_string());
                    panic!("ERROR: unexpected character, lexer failed")
                }
            }
        }
    }
    token
}

fn main() {
    let mut args = env::args();
    args.next();

    for x in args {
        match x.as_str() {
            "zen" => match read_prompt() {
                Ok(read_prompt) => read_prompt,
                Err(e) => eprintln!("ERROR:{e}"),
            },
            _ => match read_from_file(x) {
                Ok(read_from_file) => read_from_file,
                Err(e) => eprintln!("ERROR:{e}"),
            },
        }
    }
}

fn read_prompt() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("{buffer}");
    Ok(())
}

fn read_from_file(x: String) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(x.as_str());
    let file_content = fs::read_to_string(file_path)?;
    let tokens = Scanner::new(file_content);
    println!("{tokens:?}");
    Ok(())
}

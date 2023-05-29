use std::{env, error::Error, fmt, fs, io, path::Path};

#[derive(Debug)]
enum TokenType {
    LEFTPAREN,    //
    RIGHTPAREN,   //
    LEFTBRACE,    //
    RIGHTBRACE,   //
    COMMA,        //
    DOT,          //
    MINUS,        //
    PLUS,         //
    SEMICOLON,    //
    STAR,         //
    BANG,         //
    BANGEQUAL,    //
    EQUAL,        //
    EQUALEQUAL,   //
    GREATER,      //
    GREATEREQUAL, //
    LESS,         //
    LESSEQUAL,    //
    EOF,          //
    SLASH,
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
        let mut token_list = scan_token(&lines);
        token_list.push(Token::new(TokenType::EOF, " ".to_string(), lines.len()));
        Self { source, token_list }
    }
}

fn scan_token(lines: &Vec<&str>) -> Vec<Token> {
    let mut token: Vec<Token> = Vec::new();
    for (k, v) in lines.into_iter().enumerate() {
        let char_array: Vec<_> = v.chars().collect();
        let mut current: usize = 0;
        while char_array.len() > current {
            match char_array[current].to_string().as_str() {
                "(" => token.push(Token::new(
                    TokenType::LEFTPAREN,
                    char_array[current].to_string(),
                    k + 1,
                )),
                ")" => token.push(Token::new(
                    TokenType::RIGHTPAREN,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "{" => token.push(Token::new(
                    TokenType::LEFTBRACE,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "}" => token.push(Token::new(
                    TokenType::RIGHTBRACE,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "," => token.push(Token::new(
                    TokenType::COMMA,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "." => token.push(Token::new(
                    TokenType::DOT,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "-" => token.push(Token::new(
                    TokenType::MINUS,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "+" => token.push(Token::new(
                    TokenType::PLUS,
                    char_array[current].to_string(),
                    k + 1,
                )),
                ";" => token.push(Token::new(
                    TokenType::SEMICOLON,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "*" => token.push(Token::new(
                    TokenType::STAR,
                    char_array[current].to_string(),
                    k + 1,
                )),
                "!" => {
                    current += 1;
                    if char_array[current].to_string() == "=".to_string() {
                        token.push(Token::new(
                            TokenType::BANGEQUAL,
                            format!(
                                "{}{}",
                                char_array[current - 1].to_string(),
                                char_array[current].to_string()
                            ),
                            k + 1,
                        ))
                    }
                    token.push(Token::new(
                        TokenType::BANG,
                        char_array[current].to_string(),
                        k + 1,
                    ))
                }
                "=" => {
                    current += 1;
                    if char_array[current].to_string() == "=".to_string() {
                        token.push(Token::new(
                            TokenType::EQUALEQUAL,
                            format!(
                                "{}{}",
                                char_array[current - 1].to_string(),
                                char_array[current].to_string()
                            ),
                            k + 1,
                        ))
                    }
                    token.push(Token::new(
                        TokenType::EQUAL,
                        char_array[current].to_string(),
                        k + 1,
                    ))
                }
                "<" => {
                    current += 1;
                    if char_array[current].to_string() == "=".to_string() {
                        token.push(Token::new(
                            TokenType::LESSEQUAL,
                            format!(
                                "{}{}",
                                char_array[current - 1].to_string(),
                                char_array[current].to_string()
                            ),
                            k + 1,
                        ))
                    }
                    token.push(Token::new(
                        TokenType::EQUAL,
                        char_array[current].to_string(),
                        k + 1,
                    ))
                }

                ">" => {
                    current += 1;
                    if char_array[current].to_string() == "=".to_string() {
                        token.push(Token::new(
                            TokenType::GREATEREQUAL,
                            format!(
                                "{}{}",
                                char_array[current - 1].to_string(),
                                char_array[current].to_string()
                            ),
                            k + 1,
                        ))
                    }
                    token.push(Token::new(
                        TokenType::GREATER,
                        char_array[current].to_string(),
                        k + 1,
                    ))
                }

                "/" => {
                    current += 1;
                    if char_array[current].to_string() == "/".to_string() {
                        while char_array[current].to_string() != "\n".to_string() {
                            current += 1;
                            println!("{}", char_array[current]);
                        }
                    } else {
                        token.push(Token::new(
                            TokenType::SLASH,
                            char_array[current].to_string(),
                            k + 1,
                        ))
                    }
                }

                _ => {
                    eprintln!(
                        "ERROR: unexpected character {}",
                        char_array[current].to_string(),
                    );
                }
            }
            current += 1;
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
    println!("{tokens:#?}");
    Ok(())
}

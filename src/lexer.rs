#[derive(Debug)]
#[derive(PartialEq)]

// Define an enumeration to represent different token kinds.
pub enum TokenKind {
    DATA,
    INPUT,
    PROCESS,
    OUTPUT,
    END,
    ID,
    NUM,
    TRUE,
    FALSE,
    READ,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    VECTOR,
    NUMBER,
    REGRESSIONA,
    REGRESSIONB,
    MEAN,
    STDDEV,
    CORRELATION,
    STRING,
}

#[derive(Debug)]
#[derive(PartialEq)]

// Define and implement a struct to represent a token with lexeme.
pub struct Token {
    pub token: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(token: TokenKind, lexeme: String) -> Self {
        // Constructor to create a new Token instance.
        Self { token, lexeme }
    }
}

#[derive(Debug)]
// Define a Lexer struct to perform analysis.
pub struct Lexer {
    // Characters to analyzed
    contents: Vec<char>,
    // Current character index
    index: usize,
    // List to store the extracted tokens and made it public.
    pub token_list: Vec<Token>,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        // Constructor to create a new Lexer instance.
        Self {
            contents: contents.chars().collect(),
            index: 0,
            token_list: Vec::<Token>::new(),
        }
    }

    pub fn lexical_analysis(&mut self) {
        // Perform lexical analysis on the file given in the prompt.
        while self.index < self.contents.len() {
            let c = self.get_char();

            match c {
                ':' => {
                    self.token_list.push(Token::new(TokenKind::COLON, ":".to_string()));
                    self.index += 1;
                }

                ',' => {
                    self.token_list.push(Token::new(TokenKind::COMMA, ",".to_string()));
                    self.index += 1;
                }

                '.' => {
                    self.token_list.push(Token::new(TokenKind::PERIOD, ".".to_string()));
                    self.index += 1;
                }

                '(' => {
                    self.token_list.push(Token::new(TokenKind::LPAREN, "(".to_string()));
                    self.index += 1;
                }

                ')' => {
                    self.token_list.push(Token::new(TokenKind::RPAREN, ")".to_string()));
                    self.index += 1;
                }

                '=' => {
                    self.token_list.push(Token::new(TokenKind::ASSIGN, "=".to_string()));
                    self.index += 1;
                }

                '\"' => {
                    self.index += 1;

                    let scanner: String = self
                        .contents
                        .iter()
                        .skip(self.index)
                        .take_while(|ch| **ch != '\"')
                        .collect();

                    self.index += scanner.len() + 1;

                    self.token_list.push(Token::new(TokenKind::STRING, scanner));
                }

                _ if c.is_numeric() => {
                    let scanner: String = self
                        .contents
                        .iter()
                        .skip(self.index)
                        .take_while(|ch| ch.is_numeric())
                        .collect();
                    self.index += scanner.len();
                    self.token_list.push(Token::new(TokenKind::NUM, scanner));
                }

                _ if (c.is_alphabetic() && c.is_lowercase()) => {
                    let scanner: String = self
                        .contents
                        .iter()
                        .skip(self.index)
                        .take_while(|ch| ch.is_alphabetic() && ch.is_lowercase())
                        .collect();
                    self.index += scanner.len();

                    let _kind = match scanner.as_str() {
                        "data" => self.token_list.push(Token::new(TokenKind::DATA, scanner)),
                        "input" => self.token_list.push(Token::new(TokenKind::INPUT, scanner)),
                        "process" => self.token_list.push(Token::new(TokenKind::PROCESS, scanner)),
                        "output" => self.token_list.push(Token::new(TokenKind::OUTPUT, scanner)),
                        "end" => self.token_list.push(Token::new(TokenKind::END, scanner)),
                        "true" => self.token_list.push(Token::new(TokenKind::TRUE, scanner)),
                        "false" => self.token_list.push(Token::new(TokenKind::FALSE, scanner)),
                        "read" => self.token_list.push(Token::new(TokenKind::READ, scanner)),
                        "vector" => self.token_list.push(Token::new(TokenKind::VECTOR, scanner)),
                        "number" => self.token_list.push(Token::new(TokenKind::NUMBER, scanner)),
                        "regressiona" => {
                            self.token_list.push(Token::new(TokenKind::REGRESSIONA, scanner))
                        }
                        "regressionb" => {
                            self.token_list.push(Token::new(TokenKind::REGRESSIONB, scanner))
                        }
                        "mean" => self.token_list.push(Token::new(TokenKind::MEAN, scanner)),
                        "stddev" => self.token_list.push(Token::new(TokenKind::STDDEV, scanner)),
                        "correlation" => {
                            self.token_list.push(Token::new(TokenKind::CORRELATION, scanner))
                        }

                        // Add more patterns for other keywords or identifiers
                        _ => self.token_list.push(Token::new(TokenKind::ID, scanner)),
                        // Default to treating it as an identifier
                    };
                }

                _ if c.is_uppercase() => {
                    // Explode if we see an uppercase letter.
                    panic!("Lexical Error!!!");
                }

                _ => {
                    self.index += 1
                } // Ignore space.
            }
        }

        println!("{:?}", self.token_list);
    }
    //get cuurnt character
    pub fn get_char(&self) -> char {
        // Error handling.
        let c = match self.contents.get(self.index) {
            Some(path) => path,
            None => panic!("Expected a character"),
        };
        return *c;
    }
}

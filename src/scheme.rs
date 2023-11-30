use std::env;

use crate::lexer::{Lexer, Token, TokenKind};

/// A struct representing a Scheme translator.
pub struct Scheme {
    lexer: Lexer,
    token_index: usize,
}

impl Scheme {
    /// Creates a new instance of `Scheme`.
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            token_index: 0,
        }
    }

    /// Translates the input tokens to Scheme code.
    pub fn translate_scheme(&mut self) {
        // Find the position of the first 'INPUT' token in the token list
        let input_token: &Token = self
            .lexer
            .token_list
            .iter()
            .find(|&x| x.token == TokenKind::INPUT)
            .unwrap();
        self.token_index = self
            .lexer
            .token_list
            .iter()
            .position(|x| x == input_token)
            .unwrap();

        let mut scheme_output = String::new();
        let file_name = env::args().nth(1);
        let flag = env::args().nth(2);

        // Construct Scheme output preamble
        scheme_output.push_str(&format!("; Processing input file {} {}\n", file_name.unwrap(), flag.unwrap()));
        scheme_output.push_str("; Lexical and Syntax analysis passed\n");

        // Main loop for translating tokens to Scheme code
        loop {
            let token = self.get_token();
            match token.token {
                TokenKind::INPUT => {
                    // Skip 'INPUT' and related tokens, and generate Scheme 'read-csv' statements
                    self.token_index += 2;
                    loop {
                        scheme_output.push_str(&format!("(define {} (read-csv ", self.get_token().lexeme));
                        self.token_index += 4;
                        scheme_output.push_str(&format!("\"{}\" #", self.get_token().lexeme));
                        self.token_index += 2;
                        let boolean = if self.get_token().token == TokenKind::TRUE {"t"} else {"f"};
                        scheme_output.push_str(&format!("{} ", boolean));
                        self.token_index += 2;
                        scheme_output.push_str(&format!("{}))", self.get_token().lexeme));
                        self.token_index += 2;
                        if self.get_token().token != TokenKind::COMMA {
                            scheme_output.push_str("\n");
                            break;
                        } else {
                            scheme_output.push_str("\n");
                            self.token_index += 1;
                        }
                    }
                }
                TokenKind::PROCESS => {
                    // Skip 'PROCESS' and related tokens, and generate Scheme processing statements
                    self.token_index += 2;
                    loop {
                        scheme_output.push_str(&format!("(define {} (", self.get_token().lexeme));
                        self.token_index += 2;
                        scheme_output.push_str(&format!("{} ", self.get_token().lexeme));
                        match self.get_token().token {
                            TokenKind::REGRESSIONA | TokenKind::REGRESSIONB | TokenKind::CORRELATION => {
                                self.token_index += 2;
                                scheme_output.push_str(&format!("{} ", self.get_token().lexeme));
                                self.token_index += 2;
                                scheme_output.push_str(&format!("{}))", self.get_token().lexeme));
                                self.token_index += 2;
                                if self.get_token().token != TokenKind::COMMA {
                                    scheme_output.push_str("\n");
                                    break;
                                } else {
                                    scheme_output.push_str("\n");
                                    self.token_index += 1;
                                }
                            }
                            TokenKind::MEAN | TokenKind::STDDEV => {
                                self.token_index += 2;
                                scheme_output.push_str(&format!("{}))", self.get_token().lexeme));
                                self.token_index += 2;
                                if self.get_token().token != TokenKind::COMMA {
                                    scheme_output.push_str("\n");
                                    break;
                                } else {
                                    scheme_output.push_str("\n");
                                    self.token_index += 1;
                                }
                            }
                            _ => (),
                        }
                    }
                }
                TokenKind::OUTPUT => {
                    // Skip 'OUTPUT' and related tokens, and generate Scheme output statements
                    self.token_index += 2;
                    loop {
                        match self.get_token().token {
                            TokenKind::STRING => {
                                scheme_output.push_str(&format!("(define \"{}\")\n(newline)\n", self.get_token().lexeme));
                                self.token_index += 1;
                            }
                            TokenKind::ID => {
                                scheme_output.push_str(&format!("(define {})\n(newline)\n", self.get_token().lexeme));
                                self.token_index += 1;
                            }
                            _ => (),
                        }
                        if self.get_token().token != TokenKind::COMMA {
                            break;
                        } else {
                            self.token_index += 1;
                        }
                    }
                }
                TokenKind::END => {
                    // Remove the trailing character and break from the loop
                    scheme_output.pop();
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        // Print the generated Scheme output
        println!("{}", scheme_output);
    }

    // Helper function to retrieve the current token
    pub fn get_token(&self) -> &Token {
        let token = self.lexer.token_list.get(self.token_index);
        if self.token_index < self.lexer.token_list.len() {
            match token {
                Some(_path) => (),
                None => panic!("Error: Expected a token here!"),
            }
        } else {
            panic!("Error: Out of bound token_list access!");
        }
        return token.unwrap();
    }
}

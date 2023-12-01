use std::env;

use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenKind;

// Scheme struct to handle Scheme translation
pub struct Scheme {
    lexer_scheme: Lexer,
    token_index: usize,
}

impl Scheme {
    // Constructor for Scheme struct
    pub fn new(lexer_scheme: Lexer) -> Self {
        Self { lexer_scheme, token_index: 0 }
    }

    // Method to translate Scheme code
    pub fn translate_scheme(&mut self) {
        // Finding the INPUT token in the token list
        let input_token: &Token = self
            .lexer_scheme
            .token_list
            .iter()
            .find(|&x| x.token == TokenKind::INPUT)
            .unwrap();

        // Setting the token index to the position of the INPUT token
        self.token_index = self
            .lexer_scheme
            .token_list
            .iter()
            .position(|x| x == input_token)
            .unwrap();

        // Initializing a string to store the translated Scheme code
        let mut output_print = String::new();

        // Getting the file name and flag from command-line arguments
        let file_name = env::args().nth(1);
        let flag = env::args().nth(2);

        // Appending comments to the translated code
        output_print.push_str(&format!("; processing input file {} {}\n", file_name.unwrap(), flag.unwrap()));
        output_print.push_str("; Lexical and Syntax analysis passed\n");

        // Loop for processing Scheme code based on input tokens
        loop {
            let token = self.get_token();
            match token.token {
                TokenKind::INPUT => {
                    // Processing the INPUT section
                    self.token_index += 2;
                    loop {
                        output_print.push_str(&format!("(define {} (read-csv ", self.get_token().lexeme));
                        self.token_index += 4;
                        output_print.push_str(&format!("\"{}\" #", self.get_token().lexeme));
                        self.token_index += 2;
                        let boolean = if self.get_token().token == TokenKind::TRUE {"t"} else {"f"};
                        output_print.push_str(&format!("{} ", boolean));
                        self.token_index += 2;
                        output_print.push_str(&format!("{}))", self.get_token().lexeme));
                        self.token_index += 2;
                        if self.get_token().token != TokenKind::COMMA {
                            output_print.push_str(&format!("\n"));
                            break;
                        } else {
                            output_print.push_str(&format!("\n"));
                            self.token_index += 1;
                        }
                    }
                }
                TokenKind::PROCESS => {
                    // Processing the PROCESS section
                    self.token_index += 2;
                    loop {
                        output_print.push_str(&format!("(define {} (", self.get_token().lexeme));
                        self.token_index += 2;
                        output_print.push_str(&format!("{} ", self.get_token().lexeme));
                        match self.get_token().token {
                            TokenKind::REGRESSIONA | TokenKind::REGRESSIONB | TokenKind::CORRELATION => {
                                self.token_index += 2;
                                output_print.push_str(&format!("{} ", self.get_token().lexeme));
                                self.token_index += 2;
                                output_print.push_str(&format!("{}))", self.get_token().lexeme));
                                self.token_index += 2;
                                if self.get_token().token != TokenKind::COMMA {
                                    output_print.push_str(&format!("\n"));
                                    break;
                                } else {
                                    output_print.push_str(&format!("\n"));
                                    self.token_index += 1;
                                }
                            }
                            TokenKind::MEAN | TokenKind::STDDEV => {
                                self.token_index += 2;
                                output_print.push_str(&format!("{}))", self.get_token().lexeme));
                                self.token_index += 2;
                                if self.get_token().token != TokenKind::COMMA {
                                    output_print.push_str(&format!("\n"));
                                    break;
                                } else {
                                    output_print.push_str(&format!("\n"));
                                    self.token_index += 1;
                                }
                            }
                            _ => (),
                        }
                    }
                }
                TokenKind::OUTPUT => {
                    // Processing the OUTPUT section
                    self.token_index += 2;
                    loop {
                        match self.get_token().token {
                            TokenKind::STRING => {
                                output_print.push_str(&format!("(define \"{}\")\n(newline)\n", self.get_token().lexeme));
                                self.token_index += 1;
                            }
                            TokenKind::ID => { 
                                output_print.push_str(&format!("(define {})\n(newline)\n", self.get_token().lexeme));
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
                    // Processing the END section
                    output_print.pop();
                    break;
                }
                // Handling other token kinds (unimplemented)
                _ => unimplemented!(),
            }
        }

        // Printing the translated Scheme code
        println!("{}", output_print);
    }

    // Method to get the current token
    pub fn get_token(&self) -> &Token {
        let token = self.lexer_scheme.token_list.get(self.token_index);
        if self.token_index < self.lexer_scheme.token_list.len() {
            match token {
                Some(_path) => (),
                None => panic!("Expected a token here!"),
            }
        } else {
            panic!("Out of bound token_list access!");
        }
        return token.unwrap();
    }
}

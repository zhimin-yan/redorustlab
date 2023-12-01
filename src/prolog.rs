use std::env;

use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenKind;

// Prolog struct to handle Prolog translation
pub struct Prolog {
    lexer_prolog: Lexer,
    token_index: usize,
}

impl Prolog {
    // Constructor for Prolog struct
    pub fn new(lexer_prolog: Lexer) -> Self {
        Self { lexer_prolog, token_index: 0 }
    }

    // Method to translate Prolog code
    pub fn translate_prolog(&mut self) {
        // Finding the INPUT token in the token list
        let input_token: &Token = self
            .lexer_prolog
            .token_list
            .iter()
            .find(|&x| x.token == TokenKind::INPUT)
            .unwrap();

        // Setting the token index to the position of the INPUT token
        self.token_index = self
            .lexer_prolog
            .token_list
            .iter()
            .position(|x| x == input_token)
            .unwrap();

        // Initializing a string to store the translated Prolog code
        let mut output_print = String::new();

        // Getting the file name from command-line arguments
        let file_name = env::args().nth(1);

        // Appending comments to the translated code
        output_print.push_str(&format!("/* processing input file {}\n", file_name.unwrap()));
        output_print.push_str("   Lexical and Syntax analysis passed */\n\nmain :-\n");

        // Loop for processing Prolog code based on input tokens
        loop {
            let token = self.get_token();
            match token.token {
                TokenKind::INPUT => {
                    // Processing the INPUT section
                    self.token_index += 2;
                    loop {
                        self.token_index += 4;
                        output_print.push_str(&format!("   load_data_column(\'{}\', ", self.get_token().lexeme));
                        self.token_index += 2;
                        output_print.push_str(&format!("{}, Data{}),\n", self.get_token().lexeme, self.get_token().lexeme));
                        self.token_index += 2;
                        if self.get_token().token != TokenKind::COMMA {
                            break;
                        } else {
                            self.token_index += 1;
                        }
                    }
                }
                TokenKind::PROCESS => {
                    // Processing the PROCESS section
                    self.token_index += 2;
                    loop {
                        self.token_index += 2;
                        match self.get_token().token {
                            TokenKind::REGRESSIONA | TokenKind::REGRESSIONB | TokenKind::CORRELATION => {
                                output_print.push_str(&format!("   {}(Data0, Data1, ", self.get_token().lexeme));
                                self.token_index -= 2;
                                output_print.push_str(&format!("{}),\n", self.get_token().lexeme.to_ascii_uppercase()));
                                self.token_index += 8;
                                if self.get_token().token != TokenKind::COMMA {
                                    break;
                                } else {
                                    self.token_index += 1;
                                }
                            }
                            TokenKind::MEAN | TokenKind::STDDEV => {
                                output_print.push_str(&format!("   {}(Data0, Data1, ", self.get_token().lexeme));
                                self.token_index -= 2;
                                output_print.push_str(&format!("{}),\n", self.get_token().lexeme.to_ascii_uppercase()));
                                self.token_index += 6;
                                if self.get_token().token != TokenKind::COMMA {
                                    break;
                                } else {
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
                                output_print.push_str(&format!("   writeln(\"{}\"),\n", self.get_token().lexeme));
                                self.token_index += 1;
                            }
                            TokenKind::ID => {
                                output_print.push_str(&format!("   writeln({}),\n", self.get_token().lexeme));
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
                    output_print.pop();
                    output_print.push('.');
                    output_print.push('\n');
                    break;
                }
                _ => {
                    // Breaking loop for other token kinds
                    break;
                }
            }
        }

        // Printing the translated Prolog code
        println!("{}", output_print);
    }

    // Method to get the current token
    pub fn get_token(&self) -> &Token {
        let token = self.lexer_prolog.token_list.get(self.token_index);
        if self.token_index < self.lexer_prolog.token_list.len() {
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

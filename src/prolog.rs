use std::env;

use crate::lexer::{Lexer, Token, TokenKind};

// Prolog struct for translating tokens to Prolog statements
pub struct Prolog {
    lexer: Lexer,
    token_index: usize,
}

impl Prolog {
    // Constructor to create a new Prolog instance
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            token_index: 0,
        }
    }

    // Method to translate tokens to Prolog statements
    pub fn translate_prolog(&mut self) {
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

        let mut prolog_output = String::new();
        let file_name = env::args().nth(1);

        // Construct Prolog output preamble
        prolog_output.push_str(&format!(
            "/* Processing input file {}\n   Lexical and Syntax analysis passed */\n\nmain :-\n",
            file_name.unwrap()
        ));

        // Main loop for translating tokens to Prolog statements
        loop {
            let token = self.get_token();
            match token.token {
                TokenKind::INPUT => {
                    // Skip 'INPUT' and related tokens, and generate Prolog load_data_column statements
                    self.token_index += 2;
                    loop {
                        self.token_index += 4;
                        prolog_output.push_str(&format!(
                            "   load_data_column(\'{}\', {}, Data{}),\n",
                            self.get_token().lexeme,
                            self.get_token().lexeme,
                            self.get_token().lexeme
                        ));
                        self.token_index += 2;
                        if self.get_token().token != TokenKind::COMMA {
                            break;
                        } else {
                            self.token_index += 1;
                        }
                    }
                }
                TokenKind::PROCESS => {
                    // Skip 'PROCESS' and related tokens, and generate Prolog processing statements
                    self.token_index += 2;
                    loop {
                        self.token_index += 2;
                        match self.get_token().token {
                            TokenKind::REGRESSIONA | TokenKind::REGRESSIONB | TokenKind::CORRELATION => {
                                prolog_output.push_str(&format!(
                                    "   {}(Data0, Data1, {}),\n",
                                    self.get_token().lexeme,
                                    self.get_token().lexeme.to_ascii_uppercase()
                                ));
                                self.token_index -= 2;
                                self.token_index += 8;
                                if self.get_token().token != TokenKind::COMMA {
                                    break;
                                } else {
                                    self.token_index += 1;
                                }
                            }
                            TokenKind::MEAN | TokenKind::STDDEV => {
                                prolog_output.push_str(&format!(
                                    "   {}(Data0, Data1, {}),\n",
                                    self.get_token().lexeme,
                                    self.get_token().lexeme.to_ascii_uppercase()
                                ));
                                self.token_index -= 2;
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
                    // Skip 'OUTPUT' and related tokens, and generate Prolog output statements
                    self.token_index += 2;
                    loop {
                        match self.get_token().token {
                            TokenKind::STRING => {
                                prolog_output.push_str(&format!(
                                    "   writeln(\"{}\"),\n",
                                    self.get_token().lexeme
                                ));
                                self.token_index += 1;
                            }
                            TokenKind::ID => {
                                prolog_output.push_str(&format!(
                                    "   writeln({}),\n",
                                    self.get_token().lexeme
                                ));
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
                    // Construct the ending statement and break from the loop
                    prolog_output.pop();
                    prolog_output.pop();
                    prolog_output.push('.');
                    prolog_output.push('\n');
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        // Print the generated Prolog output
        println!("{}", prolog_output);
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

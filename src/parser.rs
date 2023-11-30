use crate::lexer::{Lexer, Token, TokenKind};

// Custom parser struct for syntax analysis
#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a Lexer,
    token_index: usize,
}

impl<'a> Parser<'a> {
    // Constructor to create a new Parser instance
    pub fn new(lexer: &'a Lexer) -> Self {
        Self { lexer, token_index: 0 }
    }

    // Method to perform syntaxical analysis
    pub fn perform_analysis(&mut self) {
        // Flags to track the presence of sections
        let mut data_section = false;
        let mut input_section = false;
        let mut process_section = false;
        let mut output_section = false;

        // Loop through tokens
        while self.token_index < self.lexer.token_list.len() {
            let token = self.retrieve_token();
            match token.token {
                TokenKind::DATA => {
                    // Check for valid section order
                    if data_section | input_section | process_section | output_section {
                        panic!("Error: 'data' must be the starting point");
                    }
                    data_section = true;
                    self.token_index += 1;

                    // Check for colon after 'data'
                    let _colon = if TokenKind::COLON == self.retrieve_token().token {
                        self.token_index += 1;
                    } else {
                        panic!("Error: Expected a colon");
                    };

                    // Loop through data section
                    loop {
                        // Check for identifier
                        let _id = if TokenKind::ID == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected an identifier");
                        };

                        // Check for colon after identifier
                        let _colon = if TokenKind::COLON == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected a colon");
                        };

                        // Check for VECTOR or NUMBER
                        match self.retrieve_token().token {
                            TokenKind::VECTOR | TokenKind::NUMBER => {
                                self.token_index += 1;
                            }
                            _ => {
                                panic!("Error: Expected either a vector or a number");
                            }
                        }

                        // Check for comma or exit loop
                        let _comma = if TokenKind::COMMA == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }

                TokenKind::INPUT => {
                    // Check for valid section order
                    if !data_section | input_section | process_section | output_section {
                        panic!("Error: 'input' must come after 'data' and before 'process'");
                    }
                    input_section = true;
                    self.token_index += 1;

                    // Check for colon after 'input'
                    let _colon = if TokenKind::COLON == self.retrieve_token().token {
                        self.token_index += 1;
                    } else {
                        panic!("Error: Expected a colon");
                    };

                    // Loop through input section
                    loop {
                        // Check for identifier
                        let _id = if TokenKind::ID == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected an identifier");
                        };

                        // Check for assignment
                        let _assign = if TokenKind::ASSIGN == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected an assignment");
                        };

                        // Check for 'read'
                        let _read = if TokenKind::READ == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected a read");
                        };

                        // Check for left parenthesis
                        let _left_parenthesis =
                            if TokenKind::LPAREN == self.retrieve_token().token {
                                self.token_index += 1;
                            } else {
                                panic!("Error: Expected a left parenthesis");
                            };

                        // Check for STRING
                        let _string = if TokenKind::STRING == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected a string");
                        };

                        // Check for comma or exit loop
                        let _comma = if TokenKind::COMMA == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };

                        // Check for TRUE or FALSE
                        match self.retrieve_token().token {
                            TokenKind::TRUE | TokenKind::FALSE => {
                                self.token_index += 1;
                            }
                            _ => {
                                panic!("Error: Expected either a true or a false");
                            }
                        }

                        // Check for comma or exit loop
                        let _comma = if TokenKind::COMMA == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }

                TokenKind::PROCESS => {
                    // Check for valid section order
                    if !data_section | !input_section | process_section | output_section {
                        panic!("Error: 'process' must come after 'input' and before 'output'");
                    }
                    process_section = true;
                    self.token_index += 1;

                    // Check for colon after 'process'
                    let _colon = if TokenKind::COLON == self.retrieve_token().token {
                        self.token_index += 1;
                    } else {
                        panic!("Error: Expected a colon");
                    };

                    // Loop through process section
                    loop {
                        // Check for identifier
                        let _id = if TokenKind::ID == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected an identifier");
                        };

                        // Check for assignment
                        let _assign = if TokenKind::ASSIGN == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            panic!("Error: Expected an assignment");
                        };

                        // Check for REGRESSIONA, REGRESSIONB, or CORRELATION
                        match self.retrieve_token().token {
                            TokenKind::REGRESSIONA | TokenKind::REGRESSIONB | TokenKind::CORRELATION => {
                                self.token_index += 1;

                                // Check for left parenthesis
                                let _left_parenthesis =
                                    if TokenKind::LPAREN == self.retrieve_token().token {
                                        self.token_index += 1;
                                    } else {
                                        panic!("Error: Expected a left parenthesis");
                                    };

                                // Check for identifier
                                let _id = if TokenKind::ID == self.retrieve_token().token {
                                    self.token_index += 1;
                                } else {
                                    panic!("Error: Expected an identifier");
                                };

                                // Check for comma
                                let _comma = if TokenKind::COMMA == self.retrieve_token().token {
                                    self.token_index += 1;
                                } else {
                                    panic!("Error: Expected a comma");
                                };

                                // Check for identifier
                                let _id = if TokenKind::ID == self.retrieve_token().token {
                                    self.token_index += 1;
                                } else {
                                    panic!("Error: Expected an identifier");
                                };

                                // Check for right parenthesis
                                let _right_parenthesis =
                                    if TokenKind::RPAREN == self.retrieve_token().token {
                                        self.token_index += 1;
                                    } else {
                                        panic!("Error: Expected a right parenthesis");
                                    };
                            }

                            TokenKind::MEAN | TokenKind::STDDEV => {
                                self.token_index += 1;

                                // Check for left parenthesis
                                let _left_parenthesis =
                                    if TokenKind::LPAREN == self.retrieve_token().token {
                                        self.token_index += 1;
                                    } else {
                                        panic!("Error: Expected a left parenthesis");
                                    };

                                // Check for identifier
                                let _id = if TokenKind::ID == self.retrieve_token().token {
                                    self.token_index += 1;
                                } else {
                                    panic!("Error: Expected an identifier");
                                };

                                // Check for right parenthesis
                                let _right_parenthesis =
                                    if TokenKind::RPAREN == self.retrieve_token().token {
                                        self.token_index += 1;
                                    } else {
                                        panic!("Error: Expected a right parenthesis");
                                    };
                            }
                            _ => {
                                panic!("Error: Expected either a REGRESSIONA, REGRESSIONB, CORRELATION, MEAN, or STDDEV");
                            }
                        }

                        // Check for comma or exit loop
                        let _comma = if TokenKind::COMMA == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }

                TokenKind::OUTPUT => {
                    // Check for valid section order
                    if !data_section | !input_section | !process_section | output_section {
                        panic!("Error: 'output' must come after 'process' and before 'end'");
                    }
                    output_section = true;
                    self.token_index += 1;

                    // Check for colon after 'output'
                    let _colon = if TokenKind::COLON == self.retrieve_token().token {
                        self.token_index += 1;
                    } else {
                        panic!("Error: Expected a colon");
                    };

                    // Loop through output section
                    loop {
                        // Check for STRING or ID
                        match self.retrieve_token().token {
                            TokenKind::STRING | TokenKind::ID => {
                                self.token_index += 1;
                            }
                            _ => {
                                panic!("Error: Expected either a string or an identifier");
                            }
                        }

                        // Check for comma or exit loop
                        let _comma = if TokenKind::COMMA == self.retrieve_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }

                TokenKind::END => {
                    // Check for valid section order
                    if !data_section | !input_section | !process_section | !output_section {
                        panic!("Error: 'end' must come after 'output'");
                    }

                    // Check for 'end'
                    let _end = if TokenKind::END == self.retrieve_token().token {
                        self.token_index += 1;
                    } else {
                        panic!("Error: Expected an end");
                    };

                    // Check for period
                    let _period = if TokenKind::PERIOD == self.retrieve_token().token {
                        self.token_index += 1;
                    } else {
                        panic!("Error: Expected a period");
                    };
                }

                _ => unimplemented!(),
            }
        }
    }

    // Method to retrieve the current token
    pub fn retrieve_token(&self) -> &Token {
        let token = self.lexer.token_list.get(self.token_index);

        // Check for token existence or panic if out of bounds
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

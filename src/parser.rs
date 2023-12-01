// Importing necessary modules
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenKind;
// Defining the Parser
#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a Lexer,
    token_index: usize,
}
// Initializing a new Parser instance with a reference to a Lexer
impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        Self { lexer, token_index: 0 }
    }
    // Performing syntactical analysis on the input tokens
    pub fn syntaxical_analysis(&mut self) {
        // Flags to track the presence of different sections
        let mut data = false;
        let mut input = false;
        let mut process = false;
        let mut output = false;
        // Looping through the tokens
        while self.token_index < self.lexer.token_list.len() {
            // Getting the current token
            let token = self.get_token();
            // Handling different token kinds
            match token.token  {
                TokenKind::DATA => {
                    // Checking for proper sequence of sections
                    if data | input | process | output {panic!("Need to start with 'data'")};
                    data = true;
                    // Outputting debug information
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    self.token_index += 1;
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    // Checking for the presence of a colon
                    let _colon = if TokenKind::COLON == self.get_token().token {self.token_index += 1} else {panic!("Expected a colon")};
                    // Loop for processing the data section
                    loop {
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _id = if TokenKind::ID == self.get_token().token {self.token_index += 1} else {panic!("Expected an identifier")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _colon = if TokenKind::COLON == self.get_token().token {self.token_index += 1} else {panic!("Expected a colon")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        match self.get_token().token {
                            TokenKind::VECTOR | TokenKind::NUMBER => {self.token_index += 1}
                            _ => {panic!("Expected either a vector or a number")}
                        }
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _comma = if TokenKind::COMMA == self.get_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }
                // Handling the INPUT
                TokenKind::INPUT => {
                    // Checking for proper sequence of sections
                    if !data | input | process | output {panic!("Need to start after 'data' and before 'process'")};
                    input = true;
                    self.token_index += 1;
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    let _colon = if TokenKind::COLON == self.get_token().token {self.token_index += 1} else {panic!("Expected a colon")};
                    // Loop for processing the input section
                    loop {
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _id = if TokenKind::ID == self.get_token().token {self.token_index += 1} else {panic!("Expected an identifier")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _assign = if TokenKind::ASSIGN == self.get_token().token {self.token_index += 1} else {panic!("Expected an assignment")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _read = if TokenKind::READ == self.get_token().token {self.token_index += 1} else {panic!("Expected a read")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _left_parenthesis = if TokenKind::LPAREN == self.get_token().token {self.token_index += 1} else {panic!("Expected a left parenthesis")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _string = if TokenKind::STRING == self.get_token().token {self.token_index += 1} else {panic!("Expected a string")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _comma = if TokenKind::COMMA == self.get_token().token {self.token_index += 1} else {panic!("Expected a comma")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        match self.get_token().token {
                            TokenKind::TRUE | TokenKind::FALSE => {self.token_index += 1}
                            _ => {panic!("Expected either a true or a false")}
                        }
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _comma = if TokenKind::COMMA == self.get_token().token {self.token_index += 1} else {panic!("Expected a comma")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _num = if TokenKind::NUM == self.get_token().token {self.token_index += 1} else {panic!("Expected a num")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _right_parethesis = if TokenKind::RPAREN == self.get_token().token {self.token_index += 1} else {panic!("Expected a right parenthesis")};
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _comma = if TokenKind::COMMA == self.get_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }
                // Handling the PROCESS
                TokenKind::PROCESS => {
                    // Checking for proper sequence of sections
                    if !data | !input | process | output {panic!("Need to start after 'input' and before 'output'")};
                    process = true;
                    self.token_index += 1;
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    let _colon = if TokenKind::COLON == self.get_token().token {self.token_index += 1} else {panic!("Expected a colon")};
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    // Loop for processing the process section
                    loop {
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    let _id = if TokenKind::ID == self.get_token().token {self.token_index += 1} else {panic!("Expected an identifier")};
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    let _assign = if TokenKind::ASSIGN == self.get_token().token {self.token_index += 1} else {panic!("Expected an assignment")};
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    match self.get_token().token {
                        TokenKind::REGRESSIONA | TokenKind::REGRESSIONB | TokenKind::CORRELATION => {
                            self.token_index += 1;
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _left_parenthesis = if TokenKind::LPAREN == self.get_token().token {self.token_index += 1} else {panic!("Expected a left parenthesis")};
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _id = if TokenKind::ID == self.get_token().token {self.token_index += 1} else {panic!("Expected an identifier")};
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _comma = if TokenKind::COMMA == self.get_token().token {self.token_index += 1} else {panic!("Expected a comma")};
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _id = if TokenKind::ID == self.get_token().token {self.token_index += 1} else {panic!("Expected an identifier")};
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _right_parethesis = if TokenKind::RPAREN == self.get_token().token {self.token_index += 1} else {panic!("Expected a right parenthesis")};
                        }
                        TokenKind::MEAN | TokenKind::STDDEV => {
                            self.token_index += 1;
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _left_parenthesis = if TokenKind::LPAREN == self.get_token().token {self.token_index += 1} else {panic!("Expected a left parenthesis")};
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _id = if TokenKind::ID == self.get_token().token {self.token_index += 1} else {panic!("Expected an identifier")};
                            println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                            let _right_parethesis = if TokenKind::RPAREN == self.get_token().token {self.token_index += 1} else {panic!("Expected a right parenthesis")};
                        }
                        _ => {panic!("Expected either a REGRESSIONA, REGRESSIONB, CORRELATION, MEAN, or STDDEV")}
                    }
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _comma = if TokenKind::COMMA == self.get_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }
                // Handling the OUTPUT
                TokenKind::OUTPUT => {
                    // Checking for proper sequence of sections
                    if !data | !input | !process | output {panic!("Need to start after 'process' and before 'end'")};
                    output = true;
                    self.token_index += 1;
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    let _colon = if TokenKind::COLON == self.get_token().token {self.token_index += 1} else {panic!("Expected a colon")};
                    // Loop for processing the output
                    loop {
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        match self.get_token().token {
                            TokenKind::STRING | TokenKind::ID => {self.token_index += 1}
                            _ => {panic!("Expected either a true or a false")}
                        }
                        println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                        let _comma = if TokenKind::COMMA == self.get_token().token {
                            self.token_index += 1;
                        } else {
                            break;
                        };
                    }
                }
                // Handling the END
                TokenKind::END => {
                    // Checking for proper sequence of sections
                    if !data | !input | !process | !output {panic!("Need to start after 'output'")};
                    let _end = if TokenKind::END == self.get_token().token {self.token_index += 1} else {panic!("Expected an end")};
                    println!("{}. {:?}", self.token_index + 1, self.get_token().token);
                    let _period = if TokenKind::PERIOD == self.get_token().token {self.token_index += 1} else {panic!("Expected a period")};
                }
                // Handling other token kinds
                _ => unimplemented!(),
            }
        }
    }

    // Method to get the current token
    pub fn get_token(&self) -> &Token {
        // Accessing the current token
        let token = self.lexer.token_list.get(self.token_index);
        // Checking for valid token access
        if self.token_index < self.lexer.token_list.len() {
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
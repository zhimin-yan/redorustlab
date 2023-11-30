// Import the standard file system module and environment module.
use std::fs; 
use std::env;

// Import the modules.
mod lexer;
use lexer::Lexer;
mod parser;
use parser::Parser;
mod scheme;
use scheme::Scheme;
mod prolog;
use prolog::Prolog;

fn main() {
    // Get the first argument from the command line (file name).
    let file_name = env::args().nth(1);
    let file_name = match file_name { 
        Some(path) => path,
        None => panic!("Expected a file name"),
    };

    // Read the contents of the file with the provided file name.
    let contents = fs::read_to_string(file_name);
    let contents = match contents { 
        Ok(content) => content,
        Err(e) => panic!("Cannot read the content of the file: {:?}", e),
    };

    // Get the second argument from the command line (flag). Use _ prefix to avoid warning.
    let _flag = env::args().nth(2);
    let _flag = match _flag { 
        Some(path) => path,
        None => panic!("Expected an additional flag"),
    };

    // Lexer and Parser processes.
    let mut lexer = Lexer::new(contents);
    lexer.lexical_analysis();
    let mut parser = Parser::new(&lexer);
    parser.syntaxical_analysis();
    
    // Check the flag option, do error handling, and print the language output or kill the program.
    match _flag.as_str() {
        "-s" => {
            // Translate to Scheme language.
            let mut _scheme_output = Scheme::new(lexer);
            _scheme_output.translate_scheme();
        }
        "-p" => {
            // Translate to Prolog language.
            let mut _prolog_output = Prolog::new(lexer);
            _prolog_output.translate_prolog();
        }
        _ => {
            // Invalid flag option.
            panic!("Invalid flag option!");
        }
    };
}


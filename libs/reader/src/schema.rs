use std::{fs::File, io::{BufRead, BufReader}};

enum Keywords {
    // Keywords
    Function, IF, Else, Return, While,
    Var, Type,

    // Boolean values
    False, True,

    // Operators
    AndBool, AndInt, Divide, Equal, GreaterThan,
    LessThan, Minus, Modulo, Mult,
    Not, OrBool, OrInt, Plus,
    // LEFT_SHIFT, RIGHT_SHIFT, OP_START = AND_BOOL, OP_END = RIGHT_SHIFT,

    // miscelanious stuff
    LeftParen, LeftBrace, LeftBracket,
    RightParen, RightBrace, RightBracket,
    Comma, Colon, Period,
    Comment, MlComment,
    Identifier, StringLiteral, Number,

    // EOF
    Eof
}


struct Lexer {
    
}


trait Tokenizer {
    fn tokenize(&self, path_to_file: &str);

    fn do_thing2(&self);
}


impl Tokenizer for Lexer {
    fn tokenize(&self, path_to_file: &str) {
        let f = File::open(path_to_file).expect("The file provided in the path does not exist");

        let mut reader = BufReader::new(f);

        let tokens: Vec<Keywords> = vec![];
        let mut line = String::new();

        loop {
            // read_line returns the number of bytes read from start until NL character is reached
            let len = reader.read_line(&mut line);
        }
    }

    fn do_thing2(&self) {
        todo!();
    }
}



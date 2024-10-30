use std::{fs::File, io::{BufRead, BufReader}, path::Path};
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


pub struct Lexer {
    // itr fields
    reader: BufReader<String, File>,

    // produces
    tokens: Vec<Keywords>,
}


pub trait Tokenizer {
    fn new(src: &Path) -> Self;

    fn tokenize(&self);
}


impl Tokenizer for Lexer {
    fn new(src: &Path) -> Lexer {
        let f = File::open(src).expect("The file provided in the path does not exist");
        let buf_reader = BufReader::new(f);

        Lexer{reader: buf_reader, tokens: vec![]}
    }

    fn tokenize(&self) {
        let mut line = String::new();
        loop {
            // read_line returns the number of bytes read from start until NL character is reached
            let len = self.reader.read_line(&mut line);
        }
    }
}

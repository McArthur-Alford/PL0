fn crop_string(sentence: String, n: usize) -> String {
    sentence.chars().skip(n).collect()
}

trait Symbol {
    /*  read
        @param sentence: &str
        @return (Option<Self>, usize)
        lhs Returns None if the sentence is not parseable.
        lhs Returns Some(Self) if the sentence is parseable.
        rhs Returns the number of characters read from the sentence.
     */
    fn read(sentence: &str) -> (Option<self>, usize);
}

enum Terminal {
    PERIOD,    // .
    SEMICOLON, // ;
    COMMA,     // ,
}

/*
    program = block "." ;
    program = block PERIOD SEMICOLON
*/
struct ProgramNode {
    block: Block,
}

impl Symbol for Program {
    fn read(sentence: &str) -> (Option<Self>, usize) {
        let block = Block::read(sentence);
        Some(Self { block })
    }
}

struct Block {}

struct Statement {}

struct Condition {}

struct Expression {}

struct Term {}

struct Factor {}

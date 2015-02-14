use std::num;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Comma,
    OCB,
    CCB,
    OB,
    CB,
    Colon,
    Number(f64),
    StringVal(String),
    Bool(bool),
    Null
}

#[derive(Debug)]
enum LexState {
    Normal,
    String1,
    Num1,
    Symbol1
}

enum LexErrorType {
    NumError,
    InvalidSymbol
}

struct LexError {
    err: LexErrorType,
    lineno: u64
}

fn const_to_token(c: char) -> Token {
    match c {
        ',' => Token::Comma,
        '{' => Token::OCB,
        '}' => Token::CCB,
        '[' => Token::OB,
        ']' => Token::CB,
        ':' => Token::Colon,
        _ => panic!()
    }
}

/// lexes the input string into a "stack" (really a vector) of tokens
pub fn lex(input: String) -> Result<Vec<Token>, LexError> {
    use lexer::LexState::*;
    use lexer::Token::*;
    let mut lexed = vec![];
    let mut state = LexState::Normal;
    let mut cur_string = String::new();
    let mut line: u64 = 0;
    for c in input.chars() {
        match state {
            Normal =>
                match c {
                    ',' | '{' | '}' | '[' | ']' | ':' =>
                        lexed.push(const_to_token(c)),
                    '"' => { cur_string.clear(); state = String1; },
                    '-' | '+' | '0'...'9' => { cur_string.clear(); cur_string.push(c); state = Num1 },
                    '\n' => line += 1,
                    'a'...'z' => { cur_string.clear(); cur_string.push(c); state = Symbol1 },
                    ' ' => (),
                    _ => return Err(LexError { err: LexErrorType::InvalidSymbol, lineno: line })
                },
            String1 =>
                match c {
                    '"' =>
                    {
                        lexed.push(StringVal(cur_string.clone()));
                        state = Normal;
                    },
                    _ => cur_string.push(c)
                },
            Num1 =>
                match c {
                    ',' | ']' | '}' =>
                        match num::from_str_radix(cur_string.as_slice(), 10) {
                            Ok(num) => { lexed.push(Number(num)); lexed.push(const_to_token(c)); state = Normal; },
                            Err(_) => return Err(LexError { err: LexErrorType::NumError, lineno: line })
                        },
                    ' ' | '\n' =>
                        match num::from_str_radix(cur_string.as_slice(), 10) {
                            Ok(num) => { lexed.push(Number(num));state = Normal; },
                            Err(_) => return Err(LexError { err: LexErrorType::NumError, lineno: line })
                        },
                    '0'...'9' | 'e' | 'E' | '.' | '+' | '-' => cur_string.push(c),
                    _ => return Err(LexError { err: LexErrorType::NumError, lineno: line })
                },
            Symbol1 =>
                match c {
                    ',' | ']' | '}' | ':' =>
                    {
                        match cur_string.as_slice() {
                            "true" => lexed.push(Bool(true)), 
                            "false" => lexed.push(Bool(false)),
                            "null" => lexed.push(Null),
                            _ => return Err(LexError { err: LexErrorType::InvalidSymbol, lineno: line })
                        }
                        lexed.push(const_to_token(c));
                        state = Normal;
                    }
                    ' ' | '\n' =>
                    {
                        match cur_string.as_slice() {
                            "true" => lexed.push(Bool(true)), 
                            "false" => lexed.push(Bool(false)),
                            "null" => lexed.push(Null),
                            _ => return Err(LexError { err: LexErrorType::InvalidSymbol, lineno: line })
                        }
                        state = Normal;
                    }
                    'a'...'z' => cur_string.push(c),
                    _ => return Err(LexError { err: LexErrorType::InvalidSymbol, lineno: line })
                }
        }

    }
    Ok(lexed)
}

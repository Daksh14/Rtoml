use std::string::ToString;

#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
    Literal(String),
    // equals (=)
    Eq,
    // square brackets open
    Sbo,
    // curly brackets open
    Cbo,
    // square brakcets close
    Sbc,
    // curly brackets close
    Cbc,
    Hash,
    DoubleQuote,
    SingleQuote,
    LineBreak,
    TripleDoubleQuotes,
    Comma,
}

impl ToString for Tokens {
    fn to_string(&self) -> String {
        let x = match self {
            Tokens::Literal(x) => x.as_str(),
            Tokens::Eq => ("="),
            Tokens::Sbo => ("["),
            Tokens::Sbc => ("]"),
            Tokens::Cbo => ("{"),
            Tokens::Cbc => ("}"),
            Tokens::Hash => ("#"),
            Tokens::Comma => ",",
            Tokens::TripleDoubleQuotes => r#""""#,
            Tokens::DoubleQuote => (r#"""#),
            Tokens::SingleQuote => "'",
            Tokens::LineBreak => "\n",
        };
        String::from(x)
    }
}

pub struct Lexer;

#[macro_use]
macro_rules! push {
    ( $lexemes : expr, $context : expr, $new_token : expr ) => {
        if !$context.is_empty() {
            $lexemes.push(crate::lexer::Tokens::Literal($context));
            $context = String::new();
        }
        $lexemes.push($new_token);
    };
}

impl Lexer {
    pub fn lex(data: Vec<u8>) -> Vec<Tokens> {
        let mut lexemes: Vec<Tokens> = Vec::new();
        let mut peekable = data.into_iter().peekable();
        let mut literal_context = String::new();
        let mut lexing_literal = false;
        while let Some(val) = peekable.next() {
            match val {
                // brackets
                b'[' => {
                    push!(lexemes, literal_context, Tokens::Sbo);
                }
                b']' => {
                    push!(lexemes, literal_context, Tokens::Sbc);
                }
                b'{' => {
                    push!(lexemes, literal_context, Tokens::Cbo);
                }
                b'}' => {
                    push!(lexemes, literal_context, Tokens::Cbc);
                }
                // symbols
                b'=' => {
                    push!(lexemes, literal_context, Tokens::Eq);
                }
                b'#' => {
                    push!(lexemes, literal_context, Tokens::Hash);
                }
                b'"' => {
                    if let Some(b'"') = peekable.peek() {
                        peekable.next();
                        if let Some(b'"') = peekable.peek() {
                            peekable.next();
                            push!(lexemes, literal_context, Tokens::TripleDoubleQuotes);
                        } else {
                            push!(lexemes, literal_context, Tokens::DoubleQuote);
                        }
                    } else {
                        push!(lexemes, literal_context, Tokens::DoubleQuote);
                    }
                    if lexing_literal {
                        lexing_literal = false
                    } else {
                        lexing_literal = true;
                    }
                }
                b'\'' => {
                    push!(lexemes, literal_context, Tokens::SingleQuote);
                }
                b'\\' => {
                    if lexing_literal {
                        if let Some(x) = peekable.next() {
                            literal_context.push(x as char)
                        } else {
                            literal_context.push(val as char);
                        }
                    } else {
                        literal_context.push(val as char);
                    }
                }
                b'\n' => {
                    push!(lexemes, literal_context, Tokens::LineBreak);
                }
                b',' => {
                    push!(lexemes, literal_context, Tokens::Comma);
                }
                b' ' => {
                    if lexing_literal {
                        literal_context.push(val as char)
                    }
                }
                _ => {
                    literal_context.push(val as char);
                }
            };
        }
        lexemes
    }
}

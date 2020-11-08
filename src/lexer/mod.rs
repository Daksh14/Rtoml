use crate::lexer::context::Context;
use std::string::ToString;

pub mod context;

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
    TripleSingleQuotes,
    BackSlash,
    Comma,
}

impl Tokens {
    pub fn as_char(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}

impl ToString for Tokens {
    fn to_string(&self) -> String {
        String::from(match self {
            Tokens::Literal(x) => x.as_str(),
            Tokens::Eq => "=",
            Tokens::Sbo => "[",
            Tokens::Sbc => "]",
            Tokens::Cbo => "{",
            Tokens::Cbc => "}",
            Tokens::Hash => "#",
            Tokens::Comma => ",",
            Tokens::TripleDoubleQuotes => r#""""#,
            Tokens::DoubleQuote => r#""""#,
            Tokens::SingleQuote => "'",
            Tokens::LineBreak => "\\n",
            Tokens::TripleSingleQuotes => "'''",
            Tokens::BackSlash => "\\",
        })
    }
}

pub struct Lexer;

#[macro_use]
macro_rules! push {
    ( $lexemes : expr, $context : expr, $new_token : expr ) => {
        if !$context.is_empty() {
            $lexemes.push($context.get_literal());
        }
        $context.all_false();
        $lexemes.push($new_token);
    };
    ( $lexemes : expr, $context : expr, $new_token : expr, $peekable : expr ) => {
        if !$context.is_empty() {
            $lexemes.push($context.get_literal());
        }
        $context.all_false();
        $lexemes.push($new_token);
        $peekable.next();
    };
}

macro_rules! quote {
    ( $lexemes : expr, $context : expr, $quote : expr ) => {
        $context.literal_context ^= true;
        if !$context.is_empty() {
            $lexemes.push($context.get_literal());
        }
        $lexemes.push($quote);
    };
}

#[macro_use]
macro_rules! literal_check {
    ( $context : expr, $char: expr, $peekable : expr, $body : expr) => {{
        if !$context.is_literal_context() {
            $body
        } else {
            $context.push($char);
            $peekable.next();
        }
    }};
}

impl Lexer {
    pub fn lex(data: Vec<u8>) -> Vec<Tokens> {
        let mut lexemes: Vec<Tokens> = Vec::new();
        let mut peekable = data.into_iter().peekable();
        let mut context = Context::new();
        while peekable.peek().is_some() {
            let val = peekable.peek().unwrap();
            match val {
                // brackets
                b'[' => literal_check!(context, Tokens::Sbo.as_char(), peekable, {
                    push!(lexemes, context, Tokens::Sbo, peekable);
                }),
                b']' => literal_check!(context, Tokens::Sbc.as_char(), peekable, {
                    push!(lexemes, context, Tokens::Sbc, peekable);
                }),
                b'{' => {
                    context.inline_table_context = true;
                    literal_check!(context, Tokens::Cbo.as_char(), peekable, {
                        push!(lexemes, context, Tokens::Cbo, peekable);
                    })
                }
                b'}' => {
                    context.inline_table_context = false;
                    literal_check!(context, Tokens::Cbc.as_char(), peekable, {
                        push!(lexemes, context, Tokens::Cbc, peekable);
                    })
                }
                // symbols
                b'=' => literal_check!(context, Tokens::Eq.as_char(), peekable, {
                    push!(lexemes, context, Tokens::Eq, peekable);
                }),
                b'#' => literal_check!(context, Tokens::Hash.as_char(), peekable, {
                    push!(lexemes, context, Tokens::Hash, peekable);
                }),
                b',' => literal_check!(context, Tokens::Comma.as_char(), peekable, {
                    push!(lexemes, context, Tokens::Comma, peekable);
                }),
                b'"' => {
                    peekable.next();
                    if let Some(b'"') = peekable.peek() {
                        peekable.next();
                        if let Some(b'"') = peekable.peek() {
                            quote!(lexemes, context, Tokens::TripleDoubleQuotes);
                        } else {
                            quote!(lexemes, context, Tokens::DoubleQuote);
                        }
                        peekable.next();
                    } else {
                        quote!(lexemes, context, Tokens::DoubleQuote);
                    }
                }
                b'\'' => {
                    peekable.next();
                    if let Some(b'\'') = peekable.peek() {
                        peekable.next();
                        if let Some(b'\'') = peekable.peek() {
                            quote!(lexemes, context, Tokens::TripleSingleQuotes);
                        } else {
                            quote!(lexemes, context, Tokens::SingleQuote);
                        }
                        peekable.next();
                    } else {
                        quote!(lexemes, context, Tokens::SingleQuote);
                    }
                }
                b'\n' => {
                    push!(lexemes, context, Tokens::LineBreak, peekable);
                }
                b'\\' => {
                    push!(lexemes, context, Tokens::BackSlash, peekable);
                    if let Some(x) = peekable.peek() {
                        lexemes.push(Tokens::Literal((*x as char).to_string()));
                        peekable.next();
                    }
                }
                b' ' => {
                    if (context.is_literal_context() || context.is_int_context())
                        && !context.is_inline_table_context()
                        && context.is_int_context()
                        || context.is_literal_context()
                    {
                        context.push(' ');
                    }
                    peekable.next();
                }
                _ => {
                    if (*val as char).is_numeric() {
                        context.int_context = true;
                    }
                    context.push(peekable.next().unwrap() as char);
                }
            };
        }
        if !context.is_empty() {
            lexemes.push(context.get_literal());
        }
        println!("{:?}", lexemes);
        lexemes
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn basic_lexing() {
        let string = b"value = 1";
        assert_eq!(
            vec![
                Tokens::Literal("value".to_string()),
                Tokens::Eq,
                Tokens::Literal("1".to_string())
            ],
            Lexer::lex(string.to_vec())
        );
    }

    #[test]
    pub fn spaces_after_nums() {
        let string = b"{ value = 1 }";
        assert_eq!(
            vec![
                Tokens::Cbo,
                Tokens::Literal("value".to_string()),
                Tokens::Eq,
                Tokens::Literal("1".to_string()),
                Tokens::Cbc
            ],
            Lexer::lex(string.to_vec())
        );
    }

    #[test]
    pub fn preseve_spaces_in_quotes() {
        let string = br#"value = "hello, world""#;
        assert_eq!(
            vec![
                Tokens::Literal("value".to_string()),
                Tokens::Eq,
                Tokens::DoubleQuote,
                Tokens::Literal("hello, world".to_string()),
                Tokens::DoubleQuote,
            ],
            Lexer::lex(string.to_vec())
        );
    }

    #[test]
    pub fn preseve_spaces_for_ints() {
        let string = br#"value = 1 234 5"#;
        let lexed = Lexer::lex(string.to_vec());
        assert_eq!(
            vec![
                Tokens::Literal("value".to_string()),
                Tokens::Eq,
                Tokens::Literal("1 234 5".to_string())
            ],
            lexed
        );
    }
}

use crate::bulletins::handle_value;
use crate::lexer::Tokens;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    pub table_name: Option<String>,
    pub key_value: HashMap<String, Node>,
}

impl Context {
    pub fn get_table(&self) -> Option<&String> {
        self.table_name.as_ref()
    }
    pub fn new(table_name: Option<String>) -> Self {
        Context {
            table_name,
            key_value: HashMap::new(),
        }
    }
    pub fn set_value(&mut self, key: &String, value: Node) {
        self.key_value.insert(key.to_owned(), value);
    }
    pub fn has_context(&self) -> bool {
        if self.table_name.is_some() {
            if self.key_value.len() > 0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn change_context(&mut self, table_name: String) {
        self.table_name = Some(table_name);
        self.key_value = HashMap::new();
    }
}

#[derive(Debug, Clone)]
pub enum TomlValue {
    Int(i64),
    Floating(f64),
    String(String),
    Array(Vec<TomlValue>),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum Node {
    Value(TomlValue),
    Table(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeadOption<T> {
    Some(T),
    None,
    Head,
}

impl<T> HeadOption<T> {
    pub fn is_some(&self) -> bool {
        if let HeadOption::None = self {
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub child: Vec<Ast>,
    pub item: HeadOption<Context>,
}

#[macro_export]
macro_rules! comp_err {
    ( $expected : expr , $found : expr) => {
        panic!(
            "Compiling error, expected {:?}, found {:?}",
            $expected, $found
        );
    };
    ( $message : expr) => {
        panic!($message);
    };
}

#[macro_export]
macro_rules! assert_toml {
    ( $left : expr , $right : expr) => {
        if let Some(x) = $left {
            if x == $right {
                ()
            } else {
                comp_err!($right, x);
            }
        }
    };
}

impl Node {
    pub fn get_value(self) -> TomlValue {
        if let Node::Value(x) = self {
            x
        } else {
            comp_err!("Expected value, found Segment")
        }
    }
    pub fn get_segment(self) -> String {
        if let Node::Table(x) = self {
            x
        } else {
            comp_err!("Expected table segment, found value")
        }
    }
}

impl Ast {
    pub fn make(lexemes: Vec<Tokens>) -> Self {
        println!("{:?}", lexemes);
        let mut peekable = lexemes.into_iter().peekable();
        let mut head = Ast::new_head();
        let mut context = Context::new(None);
        while peekable.peek().is_some() {
            // Some declaration
            if let Some(Tokens::Literal(_)) = peekable.peek() {
                let identifier = peekable.next().unwrap().to_string();
                if let Some(x) = identifier.chars().nth(0) {
                    if x.is_numeric() {
                        comp_err!("identifier names should not start with numbers");
                    }
                } else {
                    comp_err!("Invalid identifier name");
                }
                // Validate for equal to sign
                assert_toml!(peekable.next(), Tokens::Eq);
                let value = handle_value(&mut peekable);
                context.set_value(&identifier, value)
            }
            // comment
            if let Some(Tokens::Hash) = peekable.peek() {
                peekable.next();
                while peekable.peek().is_some() {
                    if let Some(Tokens::LineBreak) = peekable.next() {
                        break;
                    }
                }
            }
            // table declaration
            if let Some(Tokens::Sbo) = peekable.next() {
                if let Some(Tokens::Literal(x)) = peekable.next() {
                    if context.has_context() {
                        head.child.push(Ast::new_context(context.clone()));
                    }
                    context.change_context(x);
                }
            }
        }
        head.child.push(Ast::new_context(context.clone()));
        head
    }
    pub fn new_head() -> Self {
        Ast {
            child: Vec::new(),
            item: HeadOption::Head,
        }
    }
    pub fn new_segment(segment: &String) -> Self {
        let context = Context::new(Some(segment.to_string()));
        Ast {
            child: Vec::new(),
            item: HeadOption::Some(context),
        }
    }
    pub fn new_context(context: Context) -> Self {
        Ast {
            child: Vec::new(),
            item: HeadOption::Some(context),
        }
    }
    pub fn has_token(&self) -> bool {
        self.item.is_some()
    }
}

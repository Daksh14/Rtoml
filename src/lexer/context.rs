use crate::lexer::Tokens;

pub struct Context {
    pub int_context: bool,
    pub literal_context: bool,
    pub literal: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            int_context: false,
            literal_context: false,
            literal: String::new(),
        }
    }
    pub fn is_int_context(&self) -> bool {
        self.int_context
    }
    pub fn all_false(&mut self) {
        self.int_context = false;
        self.literal_context = false;
    }
    pub fn is_literal_context(&self) -> bool {
        self.literal_context
    }
    pub fn push(&mut self, unit: char) {
        self.literal.push(unit);
    }
    pub fn get_literal(&mut self) -> Tokens {
        let val = Tokens::Literal(self.literal.to_string());
        self.literal.clear();
        val
    }
    pub fn is_empty(&self) -> bool {
        self.literal.len() == 0
    }
}

use crate::TomlValue;
use std::collections::HashMap;
use std::iter::Iterator;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Context {
    pub table_name: Option<String>,
    pub key_value: HashMap<String, TomlValue>,
}

type ArrTables = HashMap<String, HashMap<String, Vec<TomlValue>>>;

#[derive(Debug, Clone)]
pub struct TableArrayContext {
    // <tablename, <key, value>>
    pub tables: ArrTables,
    pub context_is_valid: bool,
}

impl TableArrayContext {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            context_is_valid: false,
        }
    }
    pub fn has_table(&self, table_name: &str) -> bool {
        self.iter().any(|e| *e.0 == table_name)
    }
    pub fn set_value(&mut self, key: &str, value: TomlValue, for_table: String) {
        if let Some(x) = self.tables.get_mut(&for_table) {
            if let Some(y) = x.get_mut(key) {
                y.push(value);
            } else {
                x.insert(key.to_string(), vec![value]);
            }
        } else {
            let mut map = HashMap::new();
            map.insert(key.to_string(), vec![value]);
            self.tables.insert(for_table, map);
        }
    }
    pub fn has_some(&self) -> bool {
        self.tables.len() != 1
    }
    pub fn make_context_valid(&mut self) {
        self.context_is_valid = true
    }
    pub fn make_context_invalid(&mut self) {
        self.context_is_valid = false
    }
    pub fn context_is_valid(&self) -> bool {
        self.context_is_valid
    }
}

impl Deref for TableArrayContext {
    type Target = ArrTables;
    fn deref(&self) -> &Self::Target {
        &self.tables
    }
}

impl Context {
    pub fn get_table(&self) -> Option<&String> {
        self.table_name.as_ref()
    }
    pub fn new(table_name: Option<String>) -> Self {
        Self {
            table_name,
            key_value: HashMap::new(),
        }
    }
    pub fn set_value(&mut self, key: &str, value: TomlValue) {
        self.key_value.insert(key.to_owned(), value);
    }
    pub fn has_context(&self) -> bool {
        if self.table_name.is_some() {
            !self.key_value.is_empty()
        } else {
            false
        }
    }
    pub fn change_context(&mut self, table_name: String) {
        self.table_name = Some(table_name);
        self.key_value = HashMap::new();
    }
}

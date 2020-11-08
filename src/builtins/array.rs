use crate::assert_toml;
use crate::builtins::{ParsingError, TomlValue, Types};
use crate::close_or_comma;
use crate::lexer::Tokens;
use std::iter::Peekable;

pub struct TomlArray {
    arr_type: Option<Types>,
}

impl TomlArray {
    pub fn handle<S: Iterator<Item = Tokens>>(
        peekable: &mut Peekable<S>,
    ) -> Result<TomlValue, ParsingError> {
        let mut vec = Vec::new();
        let mut arr = Self { arr_type: None };
        while peekable.peek().is_some() {
            match peekable.peek() {
                Some(Tokens::Sbc) => break,
                Some(Tokens::Sbo) => {
                    peekable.next();
                    let value = TomlArray::handle(peekable)?;
                    vec.push(value);
                    close_or_comma!(peekable, Tokens::Sbc);
                }
                Some(x) => {
                    if arr.has_type() {
                        let types = Types::get_type(&x.to_string())?;
                        if Some(types) == arr.arr_type {
                            let value = Types::handle_value(peekable)?;
                            vec.push(value);
                        } else {
                            return Err(ParsingError::TypeError(
                                types.to_string(),
                                arr.arr_type.unwrap().to_string(),
                            ));
                        }
                    } else {
                        arr.set_type(Types::get_type(&x.to_string())?);
                        let value = Types::handle_value(peekable)?;
                        vec.push(value);
                    }
                    close_or_comma!(peekable, Tokens::Sbc);
                }
                None => break,
            }
        }
        assert_toml!(peekable.next(), Tokens::Sbc);
        Ok(TomlValue::Array(vec))
    }

    pub fn has_type(&self) -> bool {
        self.arr_type.is_some()
    }
    pub fn set_type(&mut self, ele_type: Types) {
        self.arr_type = Some(ele_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::TomlValue;
    use std::collections::HashMap;

    #[test]
    pub fn int_arr() {
        let string = "[1,2,3,4]";
        let lexial = Lexer::lex(string.as_bytes().to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::Int(1),
                TomlValue::Int(2),
                TomlValue::Int(3),
                TomlValue::Int(4)
            ])
        );
    }

    #[test]
    pub fn floating_arr() {
        let string = b"[1.1,2.2,3.3,4.4]";
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::Floating(1.1),
                TomlValue::Floating(2.2),
                TomlValue::Floating(3.3),
                TomlValue::Floating(4.4)
            ])
        );
    }

    #[test]
    pub fn string_arr() {
        let string = br#"["1","2","3","4"]"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::String("1".to_string()),
                TomlValue::String("2".to_string()),
                TomlValue::String("3".to_string()),
                TomlValue::String("4".to_string())
            ])
        );
    }

    #[test]
    pub fn bool_arr() {
        let string = br#"[true, false, true, false]"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::Boolean(true),
                TomlValue::Boolean(false),
                TomlValue::Boolean(true),
                TomlValue::Boolean(false)
            ])
        );
    }

    #[test]
    pub fn inline_table_arr() {
        let string =
            br#"[ { value = "string", arr = [1,2,3] }, { value = "string2", arr = [3,2,1] } ]"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        map1.insert(
            String::from("value"),
            TomlValue::String(String::from("string")),
        );
        map1.insert(
            String::from("arr"),
            TomlValue::Array(vec![
                TomlValue::Int(1),
                TomlValue::Int(2),
                TomlValue::Int(3),
            ]),
        );
        map2.insert(
            String::from("value"),
            TomlValue::String(String::from("string2")),
        );
        map2.insert(
            String::from("arr"),
            TomlValue::Array(vec![
                TomlValue::Int(3),
                TomlValue::Int(2),
                TomlValue::Int(1),
            ]),
        );
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::InlineTable(map1),
                TomlValue::InlineTable(map2)
            ])
        );
    }

    #[test]
    pub fn nested_arr() {
        let string = br#"[[1,2,3], [4,5,6], [7,8,9]]"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::Array(vec![
                    TomlValue::Int(1),
                    TomlValue::Int(2),
                    TomlValue::Int(3)
                ]),
                TomlValue::Array(vec![
                    TomlValue::Int(4),
                    TomlValue::Int(5),
                    TomlValue::Int(6)
                ]),
                TomlValue::Array(vec![
                    TomlValue::Int(7),
                    TomlValue::Int(8),
                    TomlValue::Int(9)
                ]),
            ])
        );
    }

    #[test]
    pub fn nested_deep_arr() {
        let string = br#"[[[1],[2],[3]], [[4],[5],[6]], [[7],[8],[9]]]"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable).unwrap();
        assert_eq!(
            toml_value,
            TomlValue::Array(vec![
                TomlValue::Array(vec![
                    TomlValue::Array(vec![TomlValue::Int(1)]),
                    TomlValue::Array(vec![TomlValue::Int(2)]),
                    TomlValue::Array(vec![TomlValue::Int(3)])
                ]),
                TomlValue::Array(vec![
                    TomlValue::Array(vec![TomlValue::Int(4)]),
                    TomlValue::Array(vec![TomlValue::Int(5)]),
                    TomlValue::Array(vec![TomlValue::Int(6)])
                ]),
                TomlValue::Array(vec![
                    TomlValue::Array(vec![TomlValue::Int(7)]),
                    TomlValue::Array(vec![TomlValue::Int(8)]),
                    TomlValue::Array(vec![TomlValue::Int(9)])
                ]),
            ])
        );
    }

    #[test]
    #[should_panic]
    pub fn no_closing_square_bracket() {
        let string = b"[1,2,3,4";
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable);
        toml_value.ok().unwrap();
    }

    #[test]
    #[should_panic]
    pub fn no_closing_comma() {
        let string = b"[1 2 3, 4]";
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable);
        toml_value.ok().unwrap();
    }

    #[test]
    #[should_panic]
    pub fn no_value_after_comma() {
        let string = b"[1, 2, 3,,]";
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let toml_value = Types::handle_value(&mut peekable);
        toml_value.ok().unwrap();
    }
}

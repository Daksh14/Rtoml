use crate::builtins::{ParsingError, Types};
use crate::comp_err;
use crate::TomlValue;

pub struct TomlNum;

impl TomlNum {
    pub fn handle(num_str: String) -> TomlValue {
        let mut final_num = String::new();
        let mut peekable = num_str.chars().peekable();
        let mut dot_count: u8 = 0;
        while peekable.peek().is_some() {
            if let Some(chars) = peekable.next() {
                match chars {
                    '_' => {
                        if let Some(num) = peekable.peek() {
                            if !num.is_numeric() {
                                comp_err!("Expected num after _");
                            }
                        } else {
                            comp_err!("Expected num after _");
                        }
                    }
                    '.' => {
                        dot_count += 1;
                        if dot_count > 1 {
                            comp_err!("Unexpected dot `.` while parsing a floating point, put the value in quotes if its not a floating point")
                        }
                        final_num.push('.')
                    }
                    _ => {
                        if chars.is_numeric() {
                            final_num.push(chars)
                        } else {
                            comp_err!("Unexpected character when parsing int");
                        }
                    }
                }
            }
        }
        if let Ok(bit64_int) = final_num.parse::<i64>() {
            TomlValue::Int(bit64_int)
        } else if dot_count > 0 {
            if let Ok(floating) = final_num.parse::<f64>() {
                TomlValue::Floating(floating)
            } else {
                comp_err!("Error in parsing integer");
            }
        } else {
            comp_err!("Error in parsing integer");
        }
    }

    pub fn check_num(num_str: &str) -> Result<Types, ParsingError> {
        let mut num_type: Option<Types> = None;
        let mut peekable = num_str.chars().peekable();
        while peekable.peek().is_some() {
            let chars = peekable.peek().unwrap();
            match chars {
                '_' => {
                    peekable.next();
                }
                '.' => {
                    peekable.next();
                    // check if there's a number after the `.` dot
                    if peekable.peek().is_some() {
                        let num = peekable.next().unwrap();
                        if num.is_numeric() {
                            num_type = Some(Types::Floating);
                        } else {
                            return Err(ParsingError::Expected(
                                "num".to_string(),
                                "NaN".to_string(),
                            ));
                        }
                    } else {
                        return Err(ParsingError::Expected(
                            "num".to_string(),
                            "None".to_string(),
                        ));
                    }
                }
                _ => {
                    if chars.is_numeric() {
                        if num_type.is_none() {
                            num_type = Some(Types::Int);
                        }
                    } else {
                        num_type = Some(Types::Void)
                    }
                    peekable.next();
                }
            }
        }
        if let Some(x) = num_type {
            Ok(x)
        } else {
            comp_err!("Error in parsing num");
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn basic_ints() {
        let num = TomlNum::handle("1".to_string());
        assert_eq!(num, TomlValue::Int(1));
    }

    #[test]
    pub fn basic_floating() {
        let num = TomlNum::handle("1.1".to_string());
        assert_eq!(num, TomlValue::Floating(1.1));
    }

    #[test]
    pub fn undersoce_seperated() {
        let num = TomlNum::handle("1_1_111_22".to_string());
        assert_eq!(num, TomlValue::Int(1111122));
    }
    #[test]
    pub fn undersoce_seperated_floating() {
        let num = TomlNum::handle("1_1_11.1_22".to_string());
        assert_eq!(num, TomlValue::Floating(1111.122));
    }
}

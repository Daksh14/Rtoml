use crate::ast::TomlValue;
use crate::bulletins::Types;
use crate::comp_err;

pub fn handle_num(num_str: String) -> TomlValue {
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
    } else {
        if dot_count > 0 {
            if let Ok(floating) = final_num.parse::<f64>() {
                TomlValue::Floating(floating)
            } else {
                comp_err!("Error in parsing integer, this a likely a bug");
            }
        } else {
            comp_err!("Error in parsing integer, this a likely a bug");
        }
    }
}

pub fn check_num(num_str: &String) -> Types {
    let mut num_type: Option<Types> = None;
    let mut peekable = num_str.chars().peekable();
    while peekable.peek().is_some() {
        if let Some(chars) = peekable.next() {
            match chars {
                '_' => (),
                '.' => {
                    // check if there's a number after the `.` dot
                    if let Some(num) = peekable.next() {
                        if num.is_numeric() {
                            num_type = Some(Types::Floating);
                        } else {
                            comp_err!("Expected num after dot for a valid floating num");
                        }
                    } else {
                        comp_err!("Expected num after dot for a valid floating num");
                    }
                }
                _ => {
                    if chars.is_numeric() {
                        if let None = num_type {
                            num_type = Some(Types::Int);
                        }
                    } else {
                        num_type = Some(Types::Void)
                    }
                }
            }
        }
    }
    if let Some(x) = num_type {
        x
    } else {
        comp_err!("Error in parsing num");
    }
}

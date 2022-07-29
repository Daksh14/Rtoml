use crate::error::ErrLocation;
use crate::parser::r_slice::RSlice;
use crate::{DateTime, TomlError, TomlValue};

use crate::parser::r_iter::RIter;
use crate::parser::ParsedValue;
use speedate;

#[derive(PartialEq)]
pub enum Hint {
    Inf,
    Nan,
    Number,
    Negative,
    Positive,
}

pub fn parse_num_or_date<'a>(
    literal: &str,
    hint: Hint,
    slice: RSlice<'a>,
) -> Result<ParsedValue<'a>, TomlError<'a>> {
    let literal = literal.replace('_', "");

    let value = match hint {
        Hint::Inf => TomlValue::Float(f64::INFINITY),
        Hint::Nan => TomlValue::Float(f64::NAN),
        Hint::Number => {
            if let Ok(integer) = literal.parse() {
                TomlValue::Int(integer)
            } else if let Ok(float) = literal.parse() {
                TomlValue::Float(float)
            } else if let Ok(parsed_date_time) = speedate::DateTime::parse_str(&literal) {
                TomlValue::DateTime(DateTime::DateTime(parsed_date_time))
            } else if let Ok(parsed_date) = speedate::Date::parse_str(&literal) {
                TomlValue::DateTime(DateTime::Date(parsed_date))
            } else if let Ok(time) = speedate::Time::parse_str(&literal) {
                TomlValue::DateTime(DateTime::Time(time))
            } else {
                return Err(TomlError::CannotParseValue(ErrLocation::new(RIter::from(
                    slice,
                ))));
            }
        }
        Hint::Positive => {
            let check = check_if_nan_or_inf(&literal[1..]);

            if Some(Hint::Inf) == check {
                TomlValue::Float(f64::INFINITY)
            } else if Some(Hint::Nan) == check {
                TomlValue::Float(f64::NAN)
            } else if let Ok(integer) = literal.parse() {
                TomlValue::Int(integer)
            } else if let Ok(number) = literal.parse() {
                TomlValue::Float(number)
            } else {
                return Err(TomlError::CannotParseValue(ErrLocation::new(RIter::from(
                    slice,
                ))));
            }
        }
        Hint::Negative => {
            let check = check_if_nan_or_inf(&literal[1..]);

            if Some(Hint::Inf) == check {
                TomlValue::Float(f64::NEG_INFINITY)
            } else if Some(Hint::Nan) == check {
                TomlValue::Float(f64::NAN)
            } else if let Ok(integer) = literal.parse() {
                TomlValue::Int(integer)
            } else if let Ok(number) = literal.parse() {
                TomlValue::Float(number)
            } else {
                return Err(TomlError::CannotParseValue(ErrLocation::new(RIter::from(
                    slice,
                ))));
            }
        }
    };

    Ok(ParsedValue::new(value, RIter::from(slice)))
}

pub fn check_if_nan_or_inf(literal: &str) -> Option<Hint> {
    match literal {
        "inf" => Some(Hint::Inf),
        "nan" => Some(Hint::Nan),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::builtins::{parse_value, tests::get_tokens_from_literal};
    use crate::DateTime;
    use crate::TomlValue;
    use speedate;

    #[test]
    pub fn floats() {
        // floats with underscore
        assert_eq!(
            parse_value(get_tokens_from_literal("1_2_3_4.1_2_3_"))
                .unwrap()
                .value,
            TomlValue::Float(1234.123)
        );
        // positive floats
        assert_eq!(
            parse_value(get_tokens_from_literal("+1.102"))
                .unwrap()
                .value,
            TomlValue::Float(1.102)
        );
        // positive floats without positive sign
        assert_eq!(
            parse_value(get_tokens_from_literal("1.102")).unwrap().value,
            TomlValue::Float(1.102)
        );
        // negative floats
        assert_eq!(
            parse_value(get_tokens_from_literal("-1.102"))
                .unwrap()
                .value,
            TomlValue::Float(-1.102)
        );
        // nan positive
        assert!(parse_value(get_tokens_from_literal("+nan"))
            .unwrap()
            .value
            .as_floating()
            .unwrap()
            .is_nan());
        // nan positive without positive sign
        assert!(parse_value(get_tokens_from_literal("nan"))
            .unwrap()
            .value
            .as_floating()
            .unwrap()
            .is_nan());
        // nan negative
        assert!(parse_value(get_tokens_from_literal("-nan"))
            .unwrap()
            .value
            .as_floating()
            .unwrap()
            .is_nan());
        // inf positive
        assert_eq!(
            parse_value(get_tokens_from_literal("+inf")).unwrap().value,
            TomlValue::Float(f64::INFINITY)
        );
        // inf positive without positive sign
        assert_eq!(
            parse_value(get_tokens_from_literal("inf")).unwrap().value,
            TomlValue::Float(f64::INFINITY)
        );
        // inf negative
        assert_eq!(
            parse_value(get_tokens_from_literal("-inf")).unwrap().value,
            TomlValue::Float(f64::NEG_INFINITY)
        );
    }

    #[test]
    pub fn datetime() {
        //  RFC 3339
        let first_date = "1979-05-27T07:32:00Z";
        let second_date = "1979-05-27T00:32:00-07:00";
        let third_date = "1979-05-27T00:32:00.999999-07:00";
        let forth_date = "1979-05-27 07:32:00Z";
        let fifth_date = "1979-05-27T07:32:00";
        let six_date = "1979-05-27T00:32:00.999999";
        // Date
        let seventh_date = "1979-05-27";
        // Time
        let first_time = "07:32:00";
        let second_time = "00:32:00.999999";

        assert_eq!(
            parse_value(get_tokens_from_literal(first_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::DateTime(
                speedate::DateTime::parse_str(first_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(second_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::DateTime(
                speedate::DateTime::parse_str(second_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(third_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::DateTime(
                speedate::DateTime::parse_str(third_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(forth_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::DateTime(
                speedate::DateTime::parse_str(forth_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(fifth_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::DateTime(
                speedate::DateTime::parse_str(fifth_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(six_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::DateTime(
                speedate::DateTime::parse_str(six_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(seventh_date))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::Date(
                speedate::Date::parse_str(seventh_date).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(first_time))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::Time(
                speedate::Time::parse_str(first_time).unwrap()
            ))
        );
        assert_eq!(
            parse_value(get_tokens_from_literal(second_time))
                .unwrap()
                .value,
            TomlValue::DateTime(DateTime::Time(
                speedate::Time::parse_str(second_time).unwrap()
            ))
        );
    }

    #[test]
    pub fn integers() {
        // numbers with underscore
        assert_eq!(
            parse_value(get_tokens_from_literal("1_2_3_4"))
                .unwrap()
                .value,
            TomlValue::Int(1234)
        );
        // numbers with underscore without positive sign
        assert_eq!(
            parse_value(get_tokens_from_literal("+1_2_3_4"))
                .unwrap()
                .value,
            TomlValue::Int(1234)
        );
        // positive numbers
        assert_eq!(
            parse_value(get_tokens_from_literal("+1")).unwrap().value,
            TomlValue::Int(1)
        );
        // positive numbers without positive sign
        assert_eq!(
            parse_value(get_tokens_from_literal("1")).unwrap().value,
            TomlValue::Int(1)
        );
        // negative numbers
        assert_eq!(
            parse_value(get_tokens_from_literal("-1")).unwrap().value,
            TomlValue::Int(-1)
        );
    }
}

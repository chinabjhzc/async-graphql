use crate::validators::InputValueValidator;
use graphql_parser::query::Value;

/// List minimum length validator
pub struct ListMinLength {
    /// Must be greater than or equal to this value.
    pub length: usize,
}

impl InputValueValidator for ListMinLength {
    fn is_valid(&self, value: &Value) -> Option<String> {
        if let Value::List(values) = value {
            if values.len() < self.length {
                Some(format!(
                    "the value length is {}, but the length must be greater than or equal to {}",
                    values.len(),
                    self.length
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// List maximum length validator
pub struct ListMaxLength {
    /// Must be less than or equal to this value.
    pub length: usize,
}

impl InputValueValidator for ListMaxLength {
    fn is_valid(&self, value: &Value) -> Option<String> {
        if let Value::List(values) = value {
            if values.len() > self.length {
                Some(format!(
                    "the value length is {}, but the length must be less than or equal to {}",
                    values.len(),
                    self.length
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

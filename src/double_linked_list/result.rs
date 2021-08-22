pub type DeLLResult<T> = Result<T, DeLLError>;

pub enum DeLLError {
    Index {
        max_possible_value: u16,
        provided_value: u16,
    },
    Empty,
    Unexpected(Option<String>),
}

impl DeLLError {
    fn message(&self) -> (String, String) {
        match self {
            DeLLError::Index {
                max_possible_value,
                provided_value,
            } => (
                "Index out of Bounds".to_string(),
                format!(
                    "Provided Index [ {} ] was not in the range [ 0..{} ]",
                    provided_value, max_possible_value
                ),
            ),
            DeLLError::Empty => (
                "Operation on Empty List".to_string(),
                "List is Empty ( head and tail are NONE )".to_string(),
            ),
            DeLLError::Unexpected(optional_message) => (
                format!("Unexpected Error Occurred"),
                optional_message
                    .clone()
                    .unwrap_or("Debug Information not available".to_string()),
            ),
        }
    }
}

use std::fmt;

impl fmt::Display for DeLLError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message().0)
    }
}

impl fmt::Debug for DeLLError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let message = self.message();
        write!(formatter, "{} : {}", message.0, message.1)
    }
}

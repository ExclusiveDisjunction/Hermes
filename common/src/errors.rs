use std::fmt::{Debug, Display};

pub enum Error {
    ArgumentError(String, String), //Name, Value
    NullError(String), //name
    FormatError(String, String), //string, reason
    RangeError(String, String, Option<String>, Option<String>), //variable, value, range min, range max
    NotFoundError(String), //identifyer
    PermissionError,
    OperatorError(String, String, Option<String>), //operator, operand1, [operand2] (for math)
    UnexpectedError(String), //Reason
    OperationError(String, String), //Action, Reason (for activities)
    ConversionError(String), //Reason
    IOError(std::io::Error)
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentError(name, value) => write!(f, "argument '{}' held an invalid value of '{}'", name, value),
            Self::NullError(name) => write!(f, "'{}' is null", name),
            Self::FormatError(string, reason) => write!(f, "the value '{}' held an invalid format because of '{}'", string, reason),
            Self::RangeError(variable, value, range_min, range_max) => {
                match (range_min, range_max) {
                    (Some(min), Some(max)) => write!(f, "the value '{value}' in variable '{variable}' is out of range ({min} to {max})"),
                    (_, _) => write!(f, "the value '{value}' in variable '{variable}' is out of range")
                }
            },
            Self::NotFoundError(value) => write!(f, "the value '{}' was not found", value),
            Self::PermissionError => write!(f, "invalid permissions"),
            Self::OperatorError(operator, operand1, operand2 ) => {
                match operand2 {
                    None => write!(f, "operator '{operator}' cannot be applied to '{operand1}'"),
                    Some(s) => write!(f, "operator '{operator}' cannot be applied to '{operand1}' and '{s}'")
                }
            },
            Self::ConversionError(s) => write!(f, "converson failed because of '{s}'"),
            Self::UnexpectedError(s) => write!(f, "unexpected error: '{s}'"),
            Self::OperationError(action, reason) => write!(f, "operation '{action}' is not permitted because of '{reason}'"),
            Self::IOError(e) => (e as &dyn Debug).fmt(f)
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

#[macro_export]
macro_rules! argument_error {
    // name, value
    ($name: expr, $value: expr) => { // name, value
        {
            Error::ArgumentError($name.to_string(), format!("{:?}", &$value))
        }
    };
    ($name: expr, $fmt_str: expr, $($v: expr), *) => {
        {
            Error::ArgumentError($name.to_string(), format!($fmt_str, $(&$v)* ))
        }
    }
}
/// Returns Error of NullError, containing a name passed
/// ```
/// assert_eq!(null_error!("arg0"), Error::NullError("arg0".to_string()))
/// ```
#[macro_export]
macro_rules! null_error {
    
    ($name: expr) => {
        {
            $crate::core::errors::Error::NullError($name.to_string())
        }
    }
}
#[macro_export]
macro_rules! format_error {
    
    ($content: expr, $reason_str: expr, $($v: expr), *) => {
        {
            // content, reason formatting string, values...
            $crate::core::errors::Error::FormatError($content.to_string(), format!($reason_str, $(&$v)*))
        }
    };
    ($content: expr, $reason: expr) => {
        {
            $crate::core::errors::Error::FormatError($content.to_string(), $reason.to_string())
        }
    }
}
#[macro_export]
macro_rules! range_error {
    // 
    ($variable: expr, $value: expr) => {
        {
            $crate::core::errors::Error::RangeError($variable.to_string(), format!("{:?}", &$value), None, None)
        }
    };
    ($variable: expr, $value: expr, $min: expr, $max: expr) => {
        {
            $crate::core::errors::Error::RangeError($variable.to_string(), format!("{:?}", &$value), Some(format!("{:?}", $min)), Some(format!("{:?}", $max)))
        }
    }
}
#[macro_export]
macro_rules! not_found_error {
    ($identifyer: expr) => {
        {
            $crate::core::errors::Error::NotFoundError($identifyer.to_string())
        }
    }
}
#[macro_export]
macro_rules! permission_error {
    () => {
        {
            $crate::core::errors::Error::PermissionError()
        }
    }
}
#[macro_export]
macro_rules! operator_error{
    ($operator: expr, $operand1: expr) => {
        {
            $crate::core::errors::Error::OperatorError($operator.to_string(), format!("{:?}", &$operand1), None)
        }
    };
    ($operator: expr, $operand1: expr, $operand2: expr) => {
        {
            $crate::core::errors::Error::OperatorError($operator.to_string(), format!("{:?}", &$operand1), Some( format!("{:?}", &$operand2) ))
        }
    }
}
#[macro_export]
macro_rules! conversion_error {
    ($reason: expr) => {
        {
            $crate::core::errors::Error::ConversionError($reason.to_string())
        }
    };
    ($reason_fmt: expr, $($v: expr), *) => {
        {
            $crate::core::errors::Error::ConversionError(format!($reason_fmt, $(&$v), *))
        }
    }
}
#[macro_export]
macro_rules! unexpected_error {
    ($fmt_str: expr, $( $v: expr), *) => {
        {
            $crate::core::errors::Error::UnexpectedError(format!($fmt_str, $(&$v)*))
        }
    };
    ($reason: expr) => {
        {
            $crate::core::errors::Error::UnexpectedError($reason.to_string())
        }
    }
}
#[macro_export]
macro_rules! operation_error {
    ($action: expr, $fmt_str: expr, $( $v: expr), *) => {
        {
            $crate::core::errors::Error::OperationError($action.to_string(), format!($fmt_str, $(&$v)*))
        }
    };
    ($action: expr, $reason: expr) => {
        {
            $crate::core::errors::Error::OperationError($action.to_string(), $reason.to_string())
        }
    }
}
#[macro_export]
macro_rules! io_error {
    ($kind: expr, $fmt_str: expr, $( $v: expr), *) => {
        {
            $crate::core::errors::Error::IOError(std::io::Error($kind, format!($fmt_str, $(&$v)*)))
        }
    };
    ($kind: expr, $reason: expr) => {
        {
            $crate::core::errors::Error::IOError(std::io::Error($kind, $reason.to_string()))
        }
    };
    ($io_error: expr) => {
        {
            $crate::core::errors::Error::IOError($io_error)
        }
    }
}

/*
pub fn argument_error(name: &str, value: &impl Debug) -> Error {
    Error::ArgumentError(name.to_string(), format!("{value:?}"))
}
pub fn null_error(name: &str) -> Error {
    Error::NullError(name.to_string())
}
pub fn format_error(string: &str, reason: &str) -> Error {
    Error::FormatError(string.to_string(), reason.to_string())
}
pub fn range_error(variable: &str, value: &impl Debug, min_value: impl Debug, max_value: impl Debug) -> Error {
    Error::RangeError(variable.to_string(), format!("{value:?}"), format!("{min_value:?}"), format!("{max_value:?}"))
}
pub fn not_found_error(identifyer: &str) -> Error {
    Error::NotFoundError(identifyer.to_string())
}
pub fn permission_error() -> Error {
    Error::PermissionError
}
pub fn operator_error(operator: &impl Debug, operand1: &impl Debug, operand2: Option<&impl Debug>) -> Error {
    match operand2 {
        None => Error::OperatorError(format!("{operator:?}"), format!("{operand1:?}"), None),
        Some(o2) => Error::OperatorError(format!("{operator:?}"), format!("{operand1:?}"), Some(format!("{o2:?}")))
    }
}
pub fn conversion_error(reason: &str) -> Error {
    Error::ConversionError(reason.to_string())
}
pub fn unexpected_error(reason: &str) -> Error {
    Error::UnexpectedError(reason.to_string())
}
pub fn operation_error(action: &str, reason: &str) -> Error {
    Error::OperationError(action.to_string(), reason.to_string())
}
pub fn io_error(error: std::io::Error) -> Error {
    Error::IOError(error)
}
*/
pub use String;

use crate::{JsAny, JsValue};

impl JsAny for String {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::String(self)
    }

    fn to_number(&self) -> super::Number {
        match self.parse() {
            Ok(parsed) => parsed,
            Err(_) => super::Number::NAN,
        }
    }
}

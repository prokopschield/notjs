use crate::{JsAny, JsValue};

pub type Boolean = bool;

impl JsAny for Boolean {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Boolean(self)
    }

    fn to_number(&self) -> super::Number {
        if *self {
            1 as super::Number
        } else {
            0 as super::Number
        }
    }
}

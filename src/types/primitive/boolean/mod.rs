use crate::{JsAny, JsValue};

pub type Boolean = bool;

impl JsAny for Boolean {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Boolean(self)
    }
}

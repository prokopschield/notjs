use crate::{JsAny, JsValue};

pub type Number = f64;

impl JsAny for Number {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Number(self)
    }

    fn to_number(&self) -> Number {
        *self
    }
}

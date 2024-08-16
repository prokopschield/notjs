use crate::{JsAny, JsValue};

pub type BigInt = i128;

impl JsAny for BigInt {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::BigInt(self)
    }
}

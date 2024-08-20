use std::f64::NAN;

use crate::JsValue;

use super::Number;

pub trait JsAny {
    fn into_value(self) -> JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Unknown(Box::from(self))
    }

    fn to_number(&self) -> Number {
        NAN
    }
}

use crate::{JsAny, JsValue};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Undefined {}

impl JsAny for Undefined {
    fn as_value(self) -> JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Undefined(self)
    }
}

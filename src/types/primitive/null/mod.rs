use crate::{JsAny, JsValue};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null {}

impl JsAny for Null {
    fn into_value(self) -> JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Null(self)
    }
}

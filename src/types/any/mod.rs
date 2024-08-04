use crate::JsValue;

pub trait JsAny {
    fn as_value(self) -> JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Unknown(Box::from(self))
    }
}

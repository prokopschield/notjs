use crate::JsValue;

pub trait JsAny {
    fn into_value(self) -> JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Unknown(Box::from(self))
    }
}

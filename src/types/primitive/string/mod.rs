pub use String;

use crate::{JsAny, JsValue};

impl JsAny for String {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::String(self)
    }
}

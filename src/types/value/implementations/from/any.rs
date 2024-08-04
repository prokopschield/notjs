use crate::JsAny;
use crate::JsValue;

impl<T: JsAny + 'static> From<T> for JsValue {
    fn from(value: T) -> Self {
        value.into_value()
    }
}

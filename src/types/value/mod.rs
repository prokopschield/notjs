use crate::JsAny;

pub enum JsValue {
    Unknown(Box<dyn JsAny>),
}

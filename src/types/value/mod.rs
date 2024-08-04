use crate::JsAny;

use super::Undefined;

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Unknown(Box<dyn JsAny>),
}

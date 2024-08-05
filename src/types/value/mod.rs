use crate::JsAny;

use super::{Boolean, Null, String, Undefined};

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Null(Null),
    Boolean(Boolean),
    String(String),
    Unknown(Box<dyn JsAny>),
}

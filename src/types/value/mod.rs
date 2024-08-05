use crate::JsAny;

use super::{Boolean, Null, Undefined};

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Null(Null),
    Boolean(Boolean),
    Unknown(Box<dyn JsAny>),
}

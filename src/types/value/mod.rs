use crate::JsAny;

use super::{Null, Undefined};

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Null(Null),
    Unknown(Box<dyn JsAny>),
}

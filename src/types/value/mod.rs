use crate::JsAny;

use super::{Boolean, Null, String, Symbol, Undefined};

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Null(Null),
    Boolean(Boolean),
    String(String),
    Symbol(Symbol),
    Unknown(Box<dyn JsAny>),
}

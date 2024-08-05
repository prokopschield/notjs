use crate::JsAny;

use super::{Boolean, Null, Number, String, Symbol, Undefined};

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Null(Null),
    Boolean(Boolean),
    String(String),
    Symbol(Symbol),
    Number(Number),
    Unknown(Box<dyn JsAny>),
}

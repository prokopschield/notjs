use crate::JsAny;

use super::{BigInt, Boolean, Null, Number, String, Symbol, Undefined};

pub mod implementations;

pub enum JsValue {
    Undefined(Undefined),
    Null(Null),
    Boolean(Boolean),
    String(String),
    Symbol(Symbol),
    Number(Number),
    BigInt(BigInt),
    Unknown(Box<dyn JsAny>),
}

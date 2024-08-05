mod methods;

use std::sync::Arc;

use crate::{JsAny, JsValue};

use super::String;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    descriptor: Arc<String>,
}

impl JsAny for Symbol {
    fn into_value(self) -> crate::JsValue
    where
        Self: Sized + 'static,
    {
        JsValue::Symbol(self)
    }
}

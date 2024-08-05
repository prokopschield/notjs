use std::sync::Arc;

use crate::{String, Symbol};

impl Symbol {
    pub fn descriptor(&self) -> &Arc<String> {
        &self.descriptor
    }
}

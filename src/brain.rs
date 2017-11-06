//! Brain

///
#[derive(Copy, Clone)]
pub struct Brain {
    ///
    pub is_human: bool,
}

///
impl Brain {
    /// creates a new brain
    pub fn new(is_human: bool) -> Brain {
        Brain {
            is_human: is_human,
        }
    }
}
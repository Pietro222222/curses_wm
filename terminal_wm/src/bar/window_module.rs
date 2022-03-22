
use super::module::DrawableModule;
use super::bar::Bar;

pub struct WindowModule {
    offset: i32
}

impl WindowModule {
    pub fn new(offset: i32) -> Self {
        Self {
            offset
        }
    }
}


impl DrawableModule for WindowModule {
    fn get_module(&self, b: &Bar) -> (i32, String) {
        (self.offset, format!("Windows Opened: {}", b.windows))
    }
}
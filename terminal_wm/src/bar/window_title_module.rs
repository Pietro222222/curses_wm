
use super::module::DrawableModule;
use super::bar::Bar;

pub struct WindowTitleModule {
    offset: i32
}

impl WindowTitleModule {
    pub fn new(offset: i32) -> Self {
        Self {
            offset
        }
    }
}


impl DrawableModule for WindowTitleModule {
    fn get_module(&self, b: &Bar) -> (i32, String) {
        (self.offset, format!("Window Title: {}", b.focused_window))
    }
}
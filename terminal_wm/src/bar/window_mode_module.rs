
use super::module::DrawableModule;
use super::bar::Bar;

pub struct WindowModeModule {
    offset: i32
}

impl WindowModeModule {
    pub fn new(offset: i32) -> Self {
        Self {
            offset
        }
    }
}


impl DrawableModule for WindowModeModule {
    fn get_module(&self, b: &Bar) -> (i32, String) {
        (self.offset, format!("Window Mode: {:?}", b.tiling_mode))
    }
}

use super::module::DrawableModule;
use super::bar::Bar;
use chrono::{self, Datelike};


pub struct BarDateModule {
    offset: i32
}

impl BarDateModule {
    pub fn new(offset: i32) -> Self {
        Self {
            offset
        }
    }
}


impl DrawableModule for BarDateModule {
    fn get_module(&self, b: &Bar) -> (i32, String) {
        let now = chrono::offset::Local::now();
        let module_str = format!("DATE: {}/{}/{}", now.day(), now.month(), now.year());
        (self.offset, module_str)
    }
}

use super::module::DrawableModule;
use super::bar::Bar;
use chrono::{self, Datelike, Timelike};


pub struct BarTimeModule {
    offset: i32
}

impl BarTimeModule {
    pub fn new(offset: i32) -> Self {
        Self {
            offset
        }
    }
}


impl DrawableModule for BarTimeModule {
    fn get_module(&self, b: &Bar) -> (i32, String) {
        let now = chrono::offset::Local::now();
        let module_str = format!("TIME: {}:{}:{}", now.hour(), now.minute(), now.second());
        (self.offset, module_str)
    }
}
use super::bar::Bar;
pub trait DrawableModule {
    fn get_module(&self, b: &Bar) -> (i32, String);
}
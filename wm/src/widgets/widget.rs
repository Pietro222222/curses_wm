use pancurses::Window;
use crate::window;
pub trait Drawable {
    fn draw(&self, window: &Window, parent: Option<&window::Window>); 
}
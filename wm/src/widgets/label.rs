use super::widget::Drawable;
use crate::window::Window;

pub struct Label {
    pub text: String,
    pub pos: (i32, i32)
}

impl Label {
    pub fn new(text: String, pos: (i32, i32)) -> Self {
        Self { text, pos }
    }
} 

impl Drawable for Label {
    fn draw(&self, window: &pancurses::Window, parent: Option<&Window>) {
        if let Some(parent) = parent {
            if self.pos.0 < 0 || self.pos.1 < 0 || self.pos.0 >= parent.size.0 {
                return;
            }
            for i in self.text.chars().into_iter().enumerate() {
               if self.pos.1 + i.0 as i32 + parent.offset.1 < parent.size.1 + parent.offset.1 {
                   window.mvaddch(self.pos.0 + parent.offset.0, self.pos.1 + i.0 as i32 + parent.offset.1, i.1);
               } 
            }
                       
        }else {
            panic!("invalid use of Label Widget, it can't be used without a parent!");
        }
    }
}
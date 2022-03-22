use crate::{widgets::widget::Drawable, screen::Screen};
pub struct Window {
    widgets: Vec<Box<dyn Drawable>>,
    pub title: String,
    //        y    x
    pub size:   (i32, i32),
    pub offset: (i32, i32) //the distance from the borders of the screen LoL
}

impl Window {
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            offset: (5, 5),
            size: (20, 20),
            title: String::from("sex 2"),
        }
    }
    pub fn inc_height(&mut self, screen: &Screen) {
        self.size.0 += 1;   
        screen.clean_up();
    }
    pub fn dec_height(&mut self, screen: &Screen) {
        if self.size.0 > 8 {
            self.size.0 -= 1;    
            screen.clean_up();
        }
    }
    pub fn inc_width(&mut self, screen: &Screen) {
        self.size.1 += 1;    
        screen.clean_up();
    }
    pub fn dec_width(&mut self, screen: &Screen) {
        if self.size.1 > 1 {
            self.size.1 -= 1;    
            screen.clean_up();
        }
    }
    pub fn inc_offset_height(&mut self, screen: &Screen) {
        self.offset.0 += 1;   
        screen.clean_up();
    }
    pub fn dec_offset_height(&mut self, screen: &Screen) {
        if self.offset.0 > screen.top_limits.0 {
            self.offset.0 -= 1;    
            screen.clean_up();
        }
    }
    pub fn inc_offset_width(&mut self, screen: &Screen) {
        self.offset.1 += 1;    
        screen.clean_up();
    }
    pub fn dec_offset_width(&mut self, screen: &Screen) {
        if self.offset.1 > screen.top_limits.1 {
            self.offset.1 -= 1;    
            screen.clean_up();
        }
    }
    pub fn add_widget(&mut self, widget: Box<dyn Drawable>) {
        self.widgets.push(widget);
    }

}


impl Drawable for Window {
    fn draw(&self, window: &pancurses::Window, parent: Option<&Window>) {
        window.mvprintw(self.offset.0, self.offset.1+1, String::from("─").repeat((self.size.1 as usize)-1));
        window.mvprintw(self.offset.0, self.offset.1 + self.size.1, "╮");
        window.mvprintw(self.offset.0, self.offset.1, "╭");
        for i in self.offset.0+1..self.size.0+self.offset.0 {
            window.mvprintw(i, self.offset.1, "│");
            window.mvprintw(i, self.size.1 + self.offset.1, "│");
        }
        for i in &self.widgets {
            i.draw(window, Some(self));
        }
        window.refresh();
        window.mvprintw(self.size.0 + self.offset.0, self.offset.1+1, String::from("─").repeat((self.size.1 as usize)-1)); 
        window.mvprintw(self.size.0 + self.offset.0, self.offset.1, "╰");
        window.mvprintw(self.size.0 + self.offset.0, self.offset.1 + self.size.1, "╯");
    }

}
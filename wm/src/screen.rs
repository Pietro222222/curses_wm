use pancurses::{Window, initscr, curs_set};

use crate::widgets::widget::Drawable;

pub struct Screen {
    pub size: (i32, i32),
    pub window: Window,
    pub top_limits: (i32, i32), // basically, how close the windows can be it to the border
}

impl Screen {
    pub fn new(limits: (i32, i32)) -> Self {
        let window = initscr();
        curs_set(0);
        pancurses::raw();
        pancurses::noecho();
        window.keypad(true);
        window.nodelay(true);
        Self {
            size: window.get_max_yx(),
            window,
            top_limits: limits
        }
    }
    pub fn update(&mut self) {
        self.size = self.window.get_max_yx();
    }
    pub fn clean_up(&self) {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                self.window.mvaddch(i, j, ' ');
            }    
        }
    }
    pub fn draw_window(&self, w: &mut crate::window::Window) {
        w.draw(&self.window, None);
        self.window.refresh();
    } 
    pub fn print_debug_msg(&self, str: String) {
        self.window.mvprintw(0, 0, str);
        self.window.refresh();
    }
    pub fn draw_widget<T: Drawable> (&self, widget: &mut T, parent: Option<&crate::window::Window>) {
        widget.draw(&self.window, parent);
    }
}
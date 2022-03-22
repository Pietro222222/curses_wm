use curses_wm::{window::Window, widgets::widget::Drawable};

pub struct WindowTitleBar;

impl WindowTitleBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Drawable for WindowTitleBar {
    fn draw(&self, window: &pancurses::Window, parent: Option<&Window>) {
        if let Some(parent) = parent {
            let mut titlebar_pos = parent.offset.0 - 2;
            window.mvprintw(titlebar_pos, parent.offset.1, "╭");
            window.mvprintw(titlebar_pos, parent.size.1 + parent.offset.1, "╮");
            
            window.mvprintw(titlebar_pos, parent.offset.1+1, String::from("─").repeat(parent.size.1 as usize - 1));
            
            titlebar_pos += 1;
            window.mvprintw(titlebar_pos, parent.offset.1, "│");
            window.mvprintw(titlebar_pos, parent.size.1 + parent.offset.1, "│");
            for i in parent.title.chars().into_iter().enumerate() {
                if i.0 as i32 + parent.offset.1 <= parent.size.1 - 1 {
                    window.mvaddch(titlebar_pos, i.0 as i32 + 2 + parent.offset.1, i.1);
                }
            }
        }else {
            panic!("invalid usage for TitleBar widget, it requires a parent");
        }
    }
}
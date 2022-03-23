use curses_wm::{widgets::widget::Drawable, window::Window};
use super::module::DrawableModule;

pub struct Bar {
    pub windows: u32,
    pub focused_window: String,
    pub size: (i32, i32),
    pub tiling_mode: crate::tiling::WindowsMode,
    pub modules: Vec<Box<dyn DrawableModule>>
}

impl Bar {
    pub fn new(w: u32, fw: String, size: (i32, i32), mode: crate::tiling::WindowsMode) -> Bar {
        Self {
            focused_window: fw,
            windows: w,
            size,
            tiling_mode: mode,
            modules: vec![]
        }
    }
    pub fn update(&mut self, w: u32, fw: String, size: (i32, i32), mode: crate::tiling::WindowsMode) {
        self.focused_window = fw;
        self.windows = w;
        self.size = size;
        self.tiling_mode = mode;
    }
    pub fn add_module(&mut self, module: Box<dyn DrawableModule>) {
        self.modules.push(module);
    }
}

impl Drawable for Bar {
    fn draw(&self, window: &pancurses::Window, parent: Option<&Window>) {
        window.mvprintw(0, 0+1, String::from("─").repeat(self.size.1 as usize - 1));
        window.mvprintw(0, 0, "╭");
        window.mvprintw(0, self.size.1 -1, "╮");
        let mut pos = 3;
        window.mvprintw(1, 0, "│");
        window.mvprintw(1, self.size.1-1, "│");
        for i in &self.modules {
            let module_info = i.get_module(&self);
            let module = format!("{} │ ", module_info.1);
            if pos + (module.len() as i32) + module_info.0 < self.size.1 - 2 {
                window.mvprintw(1, pos + module_info.0, module.clone());
                pos += module.len() as i32;
            }
            
        }

        window.refresh();
        window.mvprintw(2, 1, String::from("─").repeat(self.size.1 as usize - 1));
        window.mvprintw(2, 0, "╰");
        window.mvprintw(2, self.size.1 - 1, "╯");
    }
}
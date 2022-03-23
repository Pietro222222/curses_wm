use std::{process::exit, thread::{self, spawn}, time::Duration, collections::HashMap};
mod bar;
mod titlebar;
use bar::{bar::Bar, window_module::WindowModule, window_title_module::WindowTitleModule, date_module::BarDateModule, time_module::BarTimeModule};

use bar::window_module;

use curses_wm::{screen::Screen, window::Window, widgets::label::Label};
use pancurses::{Input, endwin};
use titlebar::titlebar::WindowTitleBar;

fn spawn_window(ws: &mut HashMap<u32, Window>, fwindow: &mut u32) {
    *fwindow += 1;
    let mut window = Window::new();
    window.add_widget(Box::new(WindowTitleBar::new()));
    ws.insert(*fwindow, window);
}

fn get_focused_window<'a>(ws: &'a mut HashMap<u32, Window>, fwindow: &u32) -> Option<&'a mut Window> {
    ws.get_mut(fwindow)
}
fn change_focused_window<'a>(ws: &'a mut HashMap<u32, Window>, fwindow: &mut u32) {
    *fwindow += 1;
    if ws.len() == 0 {
        return;
    }
    while let None = ws.get(fwindow) {
        if let Some(last) = ws.keys().last() {
            if *fwindow >= *last {
                *fwindow = 0;
            }
        }else {
            break;
        }
        *fwindow += 1;
    }
}

fn delete_focused_window<'a>(ws: &'a mut HashMap<u32, Window>, fwindow: &mut u32) {
    ws.remove(fwindow);
    change_focused_window(ws, fwindow);
}

fn get_focused_window_title<'a>(ws: &'a mut HashMap<u32, Window>, fwindow: &u32) -> String {
    if let Some(d) = get_focused_window(ws, fwindow) {
        d.title.clone()
    } else {
        String::new()
    }
}

fn tile_windows<'a>(ws: &'a mut HashMap<u32, Window>, screen_pos: (i32, i32), top_limits: (i32, i32)) {
    if ws.len() < 0 {
        return;
    }
    let x_size = (screen_pos.1 - top_limits.1) / ws.len() as i32;
    let mut offset = x_size;

    for (pos, i) in ws.iter_mut().enumerate() {
        i.1.offset.1 = x_size * (pos as i32);
        i.1.size.1 = x_size - 1;
        i.1.size.0 = (screen_pos.0 - top_limits.0) - 1;
    }
}


fn main() {
    let mut screen = Screen::new((5, 0));
    let mut focused_window: u32 = 0;
    let mut windows: HashMap<u32, Window> = HashMap::new();
    let mut bar = Bar::new(windows.len() as u32, get_focused_window_title(&mut windows, &focused_window), screen.size);
    let mut state_changed = true;
    bar.add_module(Box::new(WindowModule::new(0)));
    bar.add_module(Box::new(WindowTitleModule::new(0)));
    bar.add_module(Box::new(BarDateModule::new(0)));
    bar.add_module(Box::new(BarTimeModule::new(0)));
  
    spawn_window(&mut windows, &mut focused_window);
    {
        let mut fwindow = get_focused_window(&mut windows, &focused_window).unwrap();
        fwindow.add_widget(Box::new(Label::new("text widget".into(), (5, 5))));
    }
    tile_windows(&mut windows, screen.size, screen.top_limits);
    'wmloop: loop {
        if let Some(ch) = screen.window.getch() {
            match ch {
                Input::KeyUp => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_height(&screen);
                        state_changed = true;
                    }
                }   
                Input::KeyDown => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_height(&screen);
                        state_changed = true;
                    }
                }
                Input::KeyRight => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_width(&screen);
                        state_changed = true;
                    }
                }
                Input::KeyLeft => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_width(&screen);
                        state_changed = true;
                    } 
                },
                Input::KeySR => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_offset_height(&screen);
                        state_changed = true;
                    }
                }   
                Input::KeySF => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_offset_height(&screen);
                        state_changed = true;
                    }
                }
                Input::KeySRight => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_offset_width(&screen);
                        state_changed = true;
                    }
                }
                Input::KeySLeft => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_offset_width(&screen);
                        state_changed = true;
                    }
                }
                Input::KeySTab => {
                    spawn_window(&mut windows, &mut focused_window);
                    screen.print_debug_msg("spawning window".into());
                }
                Input::Character(c) => {
                    if c == 'Q' || c == 'q' {
                        break 'wmloop
                    }
                    if c == 'T' {
                        spawn_window(&mut windows, &mut focused_window);
                        tile_windows(&mut windows, screen.size, screen.top_limits);
                        state_changed = true;
                        screen.clean_up();
                    }
                    if c == 'F' { 
                        change_focused_window(&mut windows, &mut focused_window);
                        //state_changed = true;
                    }
                    if c == 'W' {
                        delete_focused_window(&mut windows, &mut focused_window);
                        
                        tile_windows(&mut windows, screen.size, screen.top_limits);
                        screen.clean_up();
                        state_changed = true;
                    }
                }
                _ => {
                }
            }
        }
        bar.update(windows.len() as u32, get_focused_window_title(&mut windows, &focused_window), screen.size);
        if state_changed {
            screen.draw_widget(&mut bar, None);
            screen.window.refresh();
            for window in windows.iter_mut() {
                if *window.0 != focused_window {
                    screen.draw_window(window.1);
                }
            } 
            if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                screen.draw_window(w);
            }
            state_changed = false;
        }
        
        thread::sleep(Duration::from_millis(40));
    }
    endwin();
}

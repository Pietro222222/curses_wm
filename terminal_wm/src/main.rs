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


fn main() {
    let mut screen = Screen::new((5, 5));
    let mut focused_window: u32 = 0;
    let mut windows: HashMap<u32, Window> = HashMap::new();
    let mut bar = Bar::new(windows.len() as u32, get_focused_window_title(&mut windows, &focused_window), screen.size);
    bar.add_module(Box::new(WindowModule::new(0)));
    bar.add_module(Box::new(WindowTitleModule::new(0)));
    bar.add_module(Box::new(BarDateModule::new(0)));
    bar.add_module(Box::new(BarTimeModule::new(0)));
    
    spawn_window(&mut windows, &mut focused_window);
    {
        let mut fwindow = get_focused_window(&mut windows, &focused_window).unwrap();
        fwindow.add_widget(Box::new(Label::new("text widget".into(), (5, 5))));
    }
    'wmloop: loop {
        if let Some(ch) = screen.window.getch() {
            match ch {
                Input::KeyUp => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_height(&screen);
                    }
                }   
                Input::KeyDown => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_height(&screen);
                    }
                }
                Input::KeyRight => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_width(&screen);
                    }
                }
                Input::KeyLeft => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_width(&screen);
                    } 
                },
                Input::KeySR => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_offset_height(&screen);
                    }
                }   
                Input::KeySF => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_offset_height(&screen);
                    }
                }
                Input::KeySRight => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.inc_offset_width(&screen);
                    }
                }
                Input::KeySLeft => {
                    if let Some(w) = get_focused_window(&mut windows, &focused_window) {
                        w.dec_offset_width(&screen);
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
                    }
                    if c == 'F' { 
                        change_focused_window(&mut windows, &mut focused_window);
                    }
                    if c == 'W' {
                        delete_focused_window(&mut windows, &mut focused_window);
                        screen.clean_up();
                    }
                }
                _ => {
                }
            }
        }
        bar.update(windows.len() as u32, get_focused_window_title(&mut windows, &focused_window), screen.size);
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
        
        thread::sleep(Duration::from_millis(20));
    }
    endwin();
}

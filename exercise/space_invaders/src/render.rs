use std::io::{Stdout, Write};
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, val) in col.iter().enumerate() {
            if force || *val != last_frame[x][y] {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *val);
            }
        }
    }
    stdout.flush().unwrap();
}

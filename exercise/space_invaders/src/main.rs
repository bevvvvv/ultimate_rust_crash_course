use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{KeyCode, Event};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // Audio Setup
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal Setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    // alternate screen is the same way vim and emacs render data
    stdout.execute(EnterAlternateScreen)?;
    // hiding cursor
    stdout.execute(Hide)?;

    // Game Loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

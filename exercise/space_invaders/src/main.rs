use std::error::Error;
use std::{io, thread, sync::mpsc};
use std::time::{Duration, Instant};
use crossterm::{event, ExecutableCommand, terminal, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{KeyCode, Event};
use rusty_audio::Audio;
use space_invaders::render;
use space_invaders::frame::{Drawable, new_frame};
use space_invaders::player::Player;
use space_invaders::invaders::Army;

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

    // Render Loop in a separate thread
    let (render_transmitter, render_receiver) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_receiver.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut army = Army::new();
    'gameloop: loop {
        // Init
        let mut curr_frame = new_frame();
        let delta = instant.elapsed();
        instant = Instant::now();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if army.update(delta) {
            audio.play("move");
        };

        // Draw and Render
        let drawables : Vec<&dyn Drawable> = vec![&player, &army];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_transmitter.send(curr_frame); // ignores any errors
        thread::sleep(Duration::from_millis(10));
    }

    // Cleanup
    drop(render_transmitter);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

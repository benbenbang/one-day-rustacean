#![allow(unused_variables, special_module_name)]
mod lib;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use lib::frame::Drawable;
use lib::invador::Invaders;
use lib::player::Player;
use lib::{frame, new_frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render Loop in a seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Init player
    let mut player = Player::new();
    let mut instant = Instant::now();

    // Init invadors
    let mut invadors = Invaders::new();

    // Game loop
    'gameloop: loop {
        // Per-frame init
        let mut curr_frame = new_frame();
        let delta = instant.elapsed();
        instant = Instant::now();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Right => player.move_right(),
                    KeyCode::Left => player.move_left(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew")
                        }
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invadors.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invadors) {
            audio.play("explode");
        }

        // Draw & render
        // player.draw(&mut curr_frame);
        // invadors.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invadors];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose
        if invadors.all_killed() {
            audio.play("win");
            break 'gameloop;
        }

        if invadors.reached_botton() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Clean up
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

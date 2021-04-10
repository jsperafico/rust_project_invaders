use std::{error::Error, sync::mpsc, thread, time::{Duration, Instant}};
use rust_project_invaders::{frame::{self, Drawable, new_frame}, invaders::Invaders, player::Player, render};
use rusty_audio::Audio;
use std::io;
use std::io::Stdout;
use crossterm::{ExecutableCommand, event::{self, Event, KeyCode}, terminal};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio_setup(&mut audio);
    play_audio(&mut audio, "startup", false);

    let mut stdout = io::stdout();
    setup_terminal(&mut stdout).unwrap();

    let (render_sender, render_receiver) = mpsc::channel();
    let render_thread = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        'render: loop {
            let curr_frame = match render_receiver.recv() {
                Ok(x) => x,
                Err(_) => break 'render,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders =  Invaders::new();
    
    'gameloop : loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read() ? {
                match key_event.code {
                    KeyCode::Left | KeyCode::Char('a') => player.left(),
                    KeyCode::Right | KeyCode::Char('d') => player.right(),
                    KeyCode::Char(' ') | KeyCode::Char('s') => {
                        if player.shoot() {
                            play_audio(&mut audio, "pew", false);
                        }
                    },
                    KeyCode::Esc | KeyCode::Char('q') => {
                        play_audio(&mut audio, "lose", true);
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }

        player.update(delta);
        if invaders.update(delta) {
            play_audio(&mut audio, "move", false);
        }
        if player.detect_hits(&mut invaders) {
            play_audio(&mut audio, "explode", false);
        }

        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_sender.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        if invaders.all_killed() {
            play_audio(&mut audio, "win", true);
            break 'gameloop;
        } else if invaders.reached_bottom() {
            play_audio(&mut audio, "lose", true);
            break 'gameloop;
        }
    }

    drop(render_sender);
    render_thread.join().unwrap();
    teardown_terminal(&mut stdout).unwrap();
    Ok(())
}

fn audio_setup(audio: &mut Audio) {
    audio.add("explode","resource/sound/explode.wav");
    audio.add("lose","resource/sound/lose.wav");
    audio.add("move","resource/sound/move.wav");
    audio.add("pew","resource/sound/pew.wav");
    audio.add("startup","resource/sound/startup.wav");
    audio.add("win","resource/sound/win.wav");
}

fn play_audio(audio: &mut Audio, name: &str, wait: bool) {
    audio.play(name);
    if wait {
        audio.wait();
    }
}

fn setup_terminal(stdout: &mut Stdout) -> Result <(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    Ok(())
}

fn teardown_terminal(stdout: &mut Stdout) -> Result <(), Box<dyn Error>> {
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode","resource/sound/explode.wav");
    audio.add("lose","resource/sound/lose.wav");
    audio.add("move","resource/sound/move.wav");
    audio.add("pew","resource/sound/pew.wav");
    audio.add("startup","resource/sound/startup.wav");
    audio.add("win","resource/sound/win.wav");
    audio.play("startup");

    audio.wait();
    Ok(())
}

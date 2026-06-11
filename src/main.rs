use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::io::{self, Write};
use std::time::Duration;

pub struct Note {
    pub frequency: f32,
    pub amplitude: f32,
}

fn main() {
    let duration = 1.0;

    // _stream must live as long as the sink
    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Player::connect_new(handle.mixer());

    println!("Welcome to Sonus.");

    print!("Enter a frequency value to generate a note (default is 440 Hz). ");
    io::stdout().flush().unwrap();

    let mut freq_str: String = String::new();
    io::stdin()
        .read_line(&mut freq_str)
        .expect("FAIL, digita certo aí");

    let trimmed = freq_str.trim();

    let freq: f32 = if !trimmed.is_empty() {
        trimmed.parse().expect("Valid number, please")
    } else {
        440.0
    };

    let _notes = Note {
        frequency: freq, // A4, A standard
        amplitude: 0.5,
    };

    let source = SineWave::new(_notes.frequency)
        .take_duration(Duration::from_secs_f32(duration))
        .amplify(_notes.amplitude);
    player.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the
    // player has finished playing all its queued sounds.
    player.sleep_until_end();

    print!("What note was played? ");
    io::stdout().flush().unwrap();

    let mut answer: String = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("FAIL, digita certo aí");

    answer = answer.trim().to_string();

    if freq == 440.0 {
        if answer.to_uppercase() == "A4" || answer.to_uppercase() == "A" {
            println!("You're right!");
        } else {
            println!("You miss, the note was A4.");
        }
    }
}

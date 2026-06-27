use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::io::{self, Write};
use std::time::Duration;

pub struct Note {
    pub frequency: f32,
    pub amplitude: f32,
    pub name: String,
    pub semitons_to_a: i32,
}

impl Note {
    pub fn new(name: String, semitons_to_a: i32, amplitude: f32, base_freq: f32) -> Self {
        let frequency = base_freq * 2.0_f32.powf(semitons_to_a as f32 / 12.0);

        Self {
            frequency,
            amplitude,
            name,
            semitons_to_a,
        }
    }
}

fn main() {
    let duration = 1.0;

    // _stream must live as long as the sink
    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Player::connect_new(handle.mixer());

    println!("Welcome to Sonus.");

    /*
     ******************************************
     ** select the basic tone
     ******************************************
     */
    print!("Enter the default frequency in heartz to represent the A4 (default is 440 Hz). ");
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

    /*
     ******************************************
     ** create note
     ******************************************
     */
    let notes_pool = [
        Note::new(String::from("C4"), -9, 0.3, freq),
        Note::new(String::from("D4"), -7, 0.3, freq),
        Note::new(String::from("E4"), -5, 0.3, freq),
        Note::new(String::from("F4"), -4, 0.3, freq),
        Note::new(String::from("G4"), -2, 0.3, freq),
        Note::new(String::from("A4"), 0, 0.3, freq),
        Note::new(String::from("B4"), 2, 0.3, freq),
    ];

    println!(
        "{:<6} | {:<12} | {:<14}",
        "Note", "Semitones (n)", "Frequency (Hz)"
    );
    println!("{:-<40}", "");

    for note in &notes_pool {
        println!(
            "{:<6} | {:<12} | {:.2} Hz",
            note.name, note.semitons_to_a, note.frequency
        );
    }

    println!("Create a new note.");

    /*
     ******************************************
     ** play note
     ******************************************
     */
    let mut _notes = &notes_pool[0];
    let n1 = SineWave::new(_notes.frequency)
        .take_duration(Duration::from_secs_f32(duration))
        .amplify(_notes.amplitude);

    _notes = &notes_pool[2];
    let n2 = SineWave::new(_notes.frequency)
        .take_duration(Duration::from_secs_f32(duration))
        .amplify(_notes.amplitude);

    _notes = &notes_pool[4];
    let n3 = SineWave::new(_notes.frequency)
        .take_duration(Duration::from_secs_f32(duration))
        .amplify(_notes.amplitude);

    let chord = n1.mix(n2).mix(n3);

    player.append(chord);

    // The sound plays in a separate thread. This call will block the current thread until the
    // player has finished playing all its queued sounds.
    player.sleep_until_end();

    /*
     ******************************************
     ** guess note
     ******************************************
     */
    print!("What note was played? ");
    io::stdout().flush().unwrap();

    let mut answer: String = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("FAIL, digita certo aí");

    answer = answer.trim().to_string();

    /*
     ******************************************
     ** validate guessed note
     ******************************************
     */
    if freq == 440.0 {
        if answer.to_uppercase() == "A4" || answer.to_uppercase() == "A" {
            println!("You're right!");
        } else {
            println!("You miss, the note was A4.");
        }
    }
}

use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use rust_i18n::t;
use std::io::{self, Write};
use std::time::Duration;

rust_i18n::i18n!("../../locales/", fallback = "pt_BR");

pub fn render_welcome_message() -> String {
    t!("hello").to_string()
}

pub fn check_answer(user_guess: &str, correct_answer: &str) -> String {
    if user_guess == correct_answer {
        t!("theory.correct").to_string()
    } else {
        // Nested lookup with variable injection
        t!("theory.incorrect", answer = correct_answer).to_string()
    }
}

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
    rust_i18n::set_locale("pt_BR");
    let duration = 1.0;

    // _stream must live as long as the sink
    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Player::connect_new(handle.mixer());

    println!("{}", t!("hello"));

    /*
     ******************************************
     ** select the basic tone
     ******************************************
     */
    print!("{}", t!("interface.base_freq"));
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
        "{:^6} | {:^12} | {:^14}",
        t!("interface.note"),
        t!("interface.semitons"),
        t!("interface.frequency")
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
    print!("{}", t!("chords.question"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_switching() {
        // Force locale to Spanish for testing
        rust_i18n::set_locale("pt_BR");
        assert_eq!(render_welcome_message(), "Bem-vindo ao Sonus!");

        // Switch back to English
        rust_i18n::set_locale("en");
        assert_eq!(render_welcome_message(), "Welcome to Sonus!");
    }
}

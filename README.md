# sonus

An interactive Command Line / Terminal User Interface (CLI/TUI) tool for
relative ear training and music theory practice, written in Rust.

`sonus` generates real-time audio waves directly through Linux's ALSA
architecture to train your relative ear, tests your music theory knowledge, and
dynamically adapts its difficulty based on your historical performance.

##  Key Features

- **Procedural Audio Generation:** Pure sine waves generated mathematically via ALSA (`libasound2`), no heavy external audio files needed.
- **Comprehensive Music Theory Quiz:** Algorithmic generation of intervals, scales, church modes, harmonic progressions, and rhythm reading.
- **Adaptive Learning Engine:** Automatically tracks your mistakes and builds a custom spaced-repetition schedule focusing on your weaknesses.
- **Multi-language Support:** Core application in English with fully internationalized (i18n) quiz modules (including Portuguese).

## 󱌣 Tech Stack & Architecture

- **Language:** Rust (2021 Edition)
- **Audio Output:** `alsa-sys` / `alsa` crates (Linux native PCM)
- **TUI Framework:** TBD
- **Data Persistence:** TBD

##  Prerequisites (Linux )

Before building, you need the ALSA development headers installed on your system.
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel

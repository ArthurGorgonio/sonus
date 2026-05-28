# Sonus Roadmap
This roadmap is tailored for a development pace of **~3 hours per week** (approximately 12 hours per month), accounting for the learning curve of a developer transitioning into **Rust** from scratch.

## :chart_with_upwards_trend: Summary Timeline Matrix

| **Milestone**                      | **Core Focus**                                                 | **Est. Weeks** | **Release**                  |
|------------------------------------|----------------------------------------------------------------|----------------|------------------------------|
| M1: Audio Engine Basic Synthesis   | Sine waves, sequential/chord playback, ALSA bindings           | 4 weeks        | 0.1.0-alpha                  |
| M2: Music Theory Domain Structures | Notes, Scales, Intervals, Equal Temperament formulas           | 3 weeks        | 0.1.0-alpha                  |
| M3: Advanced Audio Playback        | Thread mixing, chord rendering, metronome clicks               | 2 weeks        | 0.1.0-alpha                  |
| M4: Advanced Music Theory Content  | Modal shifting, chord progressions, functional harmony weights | 2 weeks        | 0.1.0-alpha                  |
| M5: CLI Quiz Framework & i18n      | stdin loop, Portuguese/English localized asset files           | 3 weeks        | 0.1.0-alpha (Feature Freeze) |
| M6: Ear Training Interaction Loop  | Melodic/harmonic guessing loops, concurrency channels          | 2 weeks        | 0.2.0-beta                   |
| M7: Adaptive Learning Engine       | User history persistence (serde), spaced repetition math       | 3 weeks        | 0.2.0-beta                   |
| M8: Rhythm Training Engine         | Precision millisecond input capture via Instant                | 2 weeks        | 0.2.0-beta (Feature Freeze)  |
| M9: Ratatui TUI Dashboard Layout   | Visual widget layouts, tab navigation, rendering loop          | 4 weeks        | 1.0.0-stable                 |
| M10: Advanced Features & Backlog   | ALSA MIDI input integration, curriculum dynamic packs          | TBD            | 1.x.x-post-launch            |

## :mag: Detailed Milestones Breakdown

### Phase 1 — Foundations & Rust Learning curve
_**Focus**: Write a sine wave generation with Alsa playback, Rust memory ownership, unsafe blocks, and raw frequency math._

#### Milestone 1: Core Audio Engine (Sine Wave Generation & ALSA Playback)
* **Objective:** Produce a pure, procedurally generated sine wave directly to the Linux sound card via ALSA without external file dependencies.
* **Key Tasks:**
  * Implement the synthesis function: $f(t) = A \cdot \sin(2\pi \cdot \text{freq} \cdot t)$.
  * Interface with the `alsa` crate, opening a PCM playback stream.
  * Learn to manage audio buffer chunks without underruns.
* **Rust Learning Hurdle:** Understanding Rust's compilation rules, dependencies management, and wrapping raw C bindings (`libasound2`).
* **Estimated Time:** 12 hours (4 weeks)

#### Milestone 2: Music Theory Domain Structures
* **Objective:** Code the algorithmic "brain" of music theory that knows how intervals, scales, and chords are structured.
* **Key Tasks:**
  * Define strongly-typed `Note`, `Interval`, `Scale`, and `Mode` structures using Rust's powerful `enums`.
  * Map frequencies using the equal temperament formula: $f = 440 \cdot 2^{n/12}$.
  * Write pure unit tests to verify intervals (e.g., ensuring `C` + `Perfect 5th` yields `G`).
* **Rust Learning Hurdle:** Learning how to design data models without falling into nested reference loops that trigger the borrow checker.
* **Estimated Time:** 15 hours (5 weeks)

---

### Phase 2 — Core Engine Assembly & Audio Expansion
_**Focus**: Advance the capabilities of the audio engine to support multi-note playback and model core music scales/modes._

#### Milestone 3: Advanced Audio Playback & Synthesis
* **Objective:** Upgrade the procedural audio generator to support complex musical structures like chords and timed clicks.
* **Key Tasks:**
  * Implement sequential note playback by streaming consecutive audio buffers seamlessly.
  * Mix multiple sine wave frequencies mathematically into a single output buffer to achieve chord playback.
  * Create a periodic metronome audible click matching a target BPM.
* **Rust Learning Hurdle:** Managing mutable state and data type conversion (casting floats to raw audio sample bytes like `i16` or `f32`).
* **Estimated Time:** 6 hours (2 weeks)

#### Milestone 4: Advanced Music Theory Content
* **Objective:** Expand the logic brain to construct advanced scales, harmonic progressions, and modal structures programmatically.
* **Key Tasks:**
  * Implement scale step formulas ($W-W-H-W-W-W-H$) starting from any root note.
  * Shift step offsets dynamically to generate all 7 Greek modes (Dorian, Phrygian, Lydian, etc.).
  * Map Roman numeral chord formulas (e.g., I-V-vi-IV) and assign functional harmony weights to generated chords.
* **Rust Learning Hurdle:** Working with collections (`Vec`), iterators, and mapping structures cleanly without cloning errors.
* **Estimated Time:** 6 hours (2 weeks)

---

### Phase 3 — Interactive CLI Quiz & Localization
_**Focus**: Manage terminal input/output, build console-based interactive modules, and handle localization._

#### Milestone 5: CLI Quiz Framework & i18n
* **Objective:** Build a text-only console quiz loop that prints questions dynamically, handles multi-language configurations, and reads input.
* **Key Tasks:**
  * Build a robust execution loop using standard input (`std::io::stdin`).
  * Create an internationalization (i18n) setup utilizing localized JSON/TOML files to support English and Portuguese text.
  * Generate algorithmic questions like *"What is the 3rd minor of D?"* / *"Qual a terça menor de Ré?"*.
* **Rust Learning Hurdle:** Mastering explicit string manipulation (`String` vs `&str`) and handling console I/O result safety.
* **Estimated Time:** 9 hours (3 weeks)

#### Milestone 6: Ear Training Interaction Loop
* **Objective:** Connect the audio synthesizer to the quiz engine so the application tests your hearing before asking for keyboard answers.
* **Key Tasks:**
  * Play interval notes ordered lowest to highest (ascending) or highest to lowest (descending).
  * Stream mixed chord buffers (harmonic playback) for user identification prompts.
  * Keep the terminal interaction loop live and responsive while audio threads execute.
* **Rust Learning Hurdle:** Introducing basic concurrency, non-blocking audio handling, or channel messaging (`std::sync::mpsc`).
* **Estimated Time:** 6 hours (2 weeks)

---

### Phase 4 — Persistence & Smart Tutor Integration
_**Focus**: Local user file storage, statistical data modeling, and automated difficulty adjusting algorithms._

#### Milestone 7: Adaptive Learning Engine
* **Objective:** Convert the application into a smart teacher that automatically targets your personal blind spots.
* **Key Tasks:**
  * Implement file serialization via the standard `serde` crate to save metrics to a local file (`history.json`).
  * Code weakness detection logic to flag specific scales, modes, or intervals dipping below targeted accuracy rates.
  * Implement a simple spaced repetition algorithm to schedule poorly reviewed topics into upcoming quiz streams.
* **Rust Learning Hurdle:** Learning file system I/O, error propagation handling (`Result` / `Option`), and macro attributes (`#[derive(Serialize, Deserialize)]`).
* **Estimated Time:** 9 hours (3 weeks)

---

### Phase 5 — Terminal User Interface (TUI) & Rhythm
_**Focus**: Replace text lines with a rich, responsive layout engine and add precise millisecond keyboard interaction._

#### Milestone 8: Rhythm Training Engine
* **Objective:** Add rhythm generators and evaluate the accuracy of user rhythmic responses.
* **Key Tasks:**
  * Generate procedural note values, subdivisions, and rest patterns.
  * Render rhythm notations visually using blocks or custom Unicode/ASCII text.
  * Capture spacebar key presses to check input precision down to the millisecond against the metronome.
* **Rust Learning Hurdle:** Measuring precise time elapsed using standard library `Instant` and handling microsecond event timestamps.
* **Estimated Time:** 6 hours (2 weeks)

#### Milestone 9: Ratatui TUI Dashboard Layout
* **Objective:** Build a full visual terminal dashboard featuring tabs, layout divisions, and styling elements.
* **Key Tasks:**
  * Integrate `ratatui` and `crossterm` crates for terminal rendering and alternative screen management.
  * Design layout boards split into menus, a quiz interaction zone, and history analytics gauges.
  * Bind app state to widgets to show live error rates or historical progression curves.
* **Rust Learning Hurdle:** Learning immediate-mode UI rendering paradigms and implementing a unified application state event-loop.
* **Estimated Time:** 12 hours (4 weeks)

---

### Phase 6 — Advanced Horizons (Post-MVP Backlog)
_**Focus**: Future enhancements, integrations, and extensions outside the core launch timeline._

#### Milestone 10: Advanced Features & Integrations
* **Objective:** Extend the software to support external hardware inputs, third-party analytics, and custom modules.
* **Key Tasks:**
  * Connect a physical MIDI keyboard controller via Linux ALSA-MIDI to accept played musical answers.
  * Support external configuration files containing customized curriculum streams or lesson expansion packs.
  * Implement alternative synthesis architectures (square wave, triangle wave, or basic FM synthesis plugins).
* **Rust Learning Hurdle:** Handling advanced cross-compilation configurations, raw pointer callbacks from external C-libraries, and dynamic trait dispatching.
* **Estimated Time:** Backlog (TBD)

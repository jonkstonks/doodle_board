## Doodle Board

Minimal drawing app built in Rust, using egui GUI library with eframe framework.

This is a personal learning project to explore Rust GUI development. 
Runs in a native desktop window.\
Currently missing file saving and theme changing functionalities.

### Features

- Freehand canvas
- Digital clock
- File export *(in progress)*
- Dark / Light theme *(in progress)*

### Getting started
```bash
git clone https://github.com/jonkstonks/doodle_board
cd doodle_board
cargo run
```

### Dependencies
| Crate | Purpose |
|---|---|
| `egui` | Immediate-mode GUI widgets |
| `eframe` | Native window + app loop |
| `serde` | State serialisation / persistence |
| `chrono` | System time for the clock |
| `env_logger` | Debug output to terminal |

### Built with

[egui](https://github.com/emilk/egui) · [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
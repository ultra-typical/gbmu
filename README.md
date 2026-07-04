# GBMU
GBMU is a desktop Nintendo Game Boy (DMG) and Game Boy Color (CGB) emulator written in Rust, with graphical interface ([egui](https://www.egui.rs/)), sound and debugger.

Starting as a school project, we decided to continue developing new features, and to push our emulator in the goal of a result that is both solid and precise in its way of emulating a Game Boy, and pleasant to use for a player.

## Screenshots
<img width="1280" height="716" alt="image" src="https://github.com/user-attachments/assets/d90bc655-45c6-42e7-94fa-be78493a6cf6" />
<img width="1152" height="643" alt="image" src="https://github.com/user-attachments/assets/21a07b7b-158e-464d-925a-ed5af887918d" />
<img width="1276" height="717" alt="image" src="https://github.com/user-attachments/assets/a7517759-8171-4aa4-ab7f-b8728bdf3c3d" />


## Features
### Emulation
- DMG and CGB emulation
- Lets you choose the model you want to emulate regardless of ROM
- Faithful audio emulation of all 4 channels, played through [cpal](https://github.com/RustAudio/cpal)
- M-cycle accurate CPU
- T-cycle accurate PPU
- Support for MBC1, MBC2, MBC3 and MBC5

### Tools and quality of life
- Integrated debugger:
    - Step-by-step execution
    - Memory address watchpoints
    - Live CPU state (registers, flags)
    - Instruction disassembly
- In-game saves and save states
- Adjustable Speed (1x to 16x)
- Customizable keymapping
- Drag and drop

## Dependencies

GBMU is written in Rust and uses [egui](https://github.com/emilk/egui) for its
interface and [cpal](https://github.com/RustAudio/cpal) for audio.

### Build requirements

- **Rust 1.85 or newer.** The project uses the 2024 edition, which was
  stabilized in Rust 1.85. If you installed Rust with `rustup`, update with:
```sh
  rustup update stable
```

- **Linux audio libraries.** `cpal` builds against ALSA, so you need the ALSA
  development headers installed:
```sh
  # Debian / Ubuntu
  sudo apt install libasound2-dev

  # Fedora
  sudo dnf install alsa-lib-devel

  # Arch
  sudo pacman -S alsa-lib
```

### Supported platforms

GBMU currently targets **Linux and macOS**. Windows support is planned but not
available yet (see [Known Bugs and TODO](#known-bugs-and-todo)).

## Building from source

Make sure you have the [dependencies](#dependencies) installed, then:

```sh
git clone https://github.com/ultra-typical/gbmu.git
cd gbmu
cargo build --release
```

The compiled binary will be at `target/release/gbmu`.

> **Always build in `--release` mode to play.** Debug builds are far too slow to
> run games at full speed.

## Usage

Launch the emulator with no arguments to open the graphical menu, where you can
drag and drop a ROM or pick one from the file browser:

```sh
cargo run --release
```

You can also pass a ROM directly on the command line:

```sh
cargo run --release -- path/to/game.gb
```

Once built, you can run the binary directly instead of going through cargo:

```sh
./target/release/gbmu path/to/game.gb
```

### Command-line options

| Option | Description |
|--------|-------------|
| `<ROM_PATH>` | Path to the ROM to launch. If omitted, the menu opens instead. |
| `-b`, `--boot_rom` | Boot the original boot ROM before the game (see note below). |
| `-t`, `--type <TYPE>` | Force the emulated model (`Cgb` or `Dmg`), regardless of the ROM. |
| `-h`, `--help` | Print help. |
| `-V`, `--version` | Print version. |

Examples:

```sh
# Force Game Boy Color mode
cargo run --release -- -t Cgb path/to/game.gb

# Boot with the original boot ROM first
cargo run --release -- -b path/to/game.gb
```

### Boot ROM

GBMU does **not** ship with any Nintendo boot ROM. When you use `-b`, the
emulator reads a boot ROM from disk, relative to the current directory:

- `boot-roms/dmg.bin` for the original Game Boy
- `boot-roms/cgb.bin` for the Game Boy Color

You must provide these files yourself. If the file is missing, the emulator will
currently crash.

## Tests
There are many well-known test ROMs used to measure how accurately an emulator reproduces real Game Boy hardware. Here's how GBMU does on them:
- acid: 2/3
- blargg: ?/?
- mooneye: ?/?
- samesuite: ?/?
- mealybug: ?/?

## Known Bugs and TODO list
- See the [issue tracker](https://github.com/ultra-typical/gbmu/issues).

## Suggestions
Have an idea for a feature, or feedback on how the emulator plays? Open an issue
on the [issue tracker](https://github.com/ultra-typical/gbmu/issues), suggestions
are welcome.

## Contributors

<a href="https://github.com/ultra-typical/gbmu/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ultra-typical/gbmu" />
</a>

### How to contribute
Contributions are welcome! To get started:

1. **Find or open an issue.** Check the
   [issue tracker](https://github.com/ulta-typical/gbmu/issues) first. If an issue
   already describes what you want to work on, use it; otherwise open a new one
   describing the feature or bug.
2. **Claim it.** Comment on the issue or assign it to yourself to let everyone
   know you're working on it.
3. Fork the repository and clone your fork.
4. Create a branch for your change (`git checkout -b my-feature`).
5. Make sure the project still builds (`cargo build`) and that the tests pass
   (`cargo test`).
6. Commit your work and open a pull request that **references the issue** (for
   example `Closes #42`) and describes what you changed or added and why.

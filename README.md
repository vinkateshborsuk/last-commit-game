# The Last Commit: Chronicles of the Latte Jungle

**Tactical survival** in an abandoned open space.  
You are the last surviving developer. Fight your way through the floors of a derelict tech park, battle bugs, assemble your team, and reach the legendary server to perform a `git push --force` and reboot reality.

---

## Project Structure

The project is **multi‑language** – each engine/language lives in its own folder.  
Currently, the **Rust + Bevy** version is ready (`rust_engine`).  
You can add implementations in other languages (Godot, Unity, Python, etc.) – just create a new folder and describe it below.

### Rust (Bevy 0.19) — `rust_engine`

#### Requirements
- Rust 1.77 or newer ([installation](https://www.rust-lang.org/tools/install))

#### Run
```bash
cd rust_engine
cargo run
Build Release Version
bash
cd rust_engine
cargo build --release
# binary: rust_engine/target/release/last-commit-game
```
### Controls (Rust version)
Key	Action
WASD / Arrow keys	Move
E	Pick up item (nearby)
F	Talk to NPC
ESC	Pause menu
Game Objects (Rust version)
Blue square – Player

Red squares – Bugs (drain health on contact)

Green square – Cookie (+5 health)

Yellow square – Coffee (increases speed)

Cyan square – Sysadmin (dialogue)

Orange square – Tester (dialogue)

Grey squares – Walls (obstacles)

### How to Contribute
Fork the repository.

If you are working in Rust, develop inside the rust_engine folder. For other languages, create a separate folder (e.g., godot_engine, python_pygame).

Work in your own branch, then open a Pull Request to main.

Discuss ideas in the Issues section.

License
MIT – see the LICENSE file for details.

# Bevy Template

This project serves as a template for Bevy projects. It is configured alongside Rapier2D, and includes a preconfigured
basic character controller.

## Structure

- **World** - Contains entities relevant to the global environment of the game
- **Player** - Contains components, systems and resources relevant to the player, including a plugin. This should serve
  as an example for implementing other standalone entities.

# Running
```bash
cargo run # Default development mode
cargo run --features debug # development mode with debug features
cargo run --release # Release mode 
```
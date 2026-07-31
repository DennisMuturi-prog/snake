# 🐍 Procedural IK Snake Game

A modern 2D Snake game built with **Rust**, **Bevy Engine 0.17**, and **Avian 2D Physics**. 

Unlike classic grid-bound snake games, this project features **continuous procedural movement** powered by **FABRIK (Forward And Backward Reaching Inverse Kinematics)**. The snake moves smoothly with realistic joint bending, dynamic body growth, proximity sensors, reactive sprite animations, and 2D physics collisions.

---

## ✨ Features

- 🦴 **FABRIK Inverse Kinematics**: Smooth, organic, continuous snake body physics using inverse kinematics algorithms instead of discrete grid turns.
- ⚡ **Bevy 0.17 ECS Architecture**: Implemented cleanly using Bevy's Data-Driven Entity Component System (ECS).
- 🎯 **Avian 2D Physics Integration**: Custom collision layers for the snake head, body parts, boundary walls, apple pickup sensor, and proximity fields.
- 😮 **Proximity Sensors & Reactive Animations**: 
  - **Apple Field Proximity**: The snake's mouth opens dynamically when approaching an apple.
  - **Animated Sprites**: Texture atlas-based animations for flickering tongue, blinking eyes, and mouth states.
  - **Collision Impact FX**: Custom hit explosion effect (`snake_hit.png`) and sound effect on impact.
- 🍎 **Dynamic Body Growth**: Eating apples triggers crunch sound effects and seamlessly inserts new IK joints into the snake's body queue.
- 🏆 **Score & High Score Tracking**: Real-time HUD displaying current score and persisting high score across sessions.
- 🔄 **Game State Machine**: Integrated game state flow (`Start` → `GameOver` → `Restart`) complete with an interactive UI restart button.

---

## 🕹️ Controls

| Key / Input | Action |
| :--- | :--- |
| **Arrow Keys** (`Up`, `Down`, `Left`, `Right`) | Steer Snake Direction |
| **WASD** (`W`, `A`, `S`, `D`) | Alternative Movement Keys |
| **Mouse Click (UI)** | Click **Restart** button on Game Over screen |

---

## 🔬 Technical Architecture

### 1. FABRIK Inverse Kinematics (`src/fabrik.rs`)
The snake's skeleton consists of `Segment` nodes forming a `Limb`. When the head target moves:
1. **Forward Pass**: Moves the tail segment to target position and resolves segment constraints backwards through the chain.
2. **Visual Sync**: Updates `Transform` positions and rotational quaternions of joint meshes and segment colliders (`LimbSegment`) in real-time.
3. **Dynamic Joint Spawning**: Adding parts inserts new segments into `VecDeque<Segment>` and spawns kinematic rigid bodies and colliders into the Bevy ECS world.

### 2. Physics Layers & Collision System (`src/main.rs`)
Powered by `avian2d`, collision layers maintain strict physical interaction rules:
- `GameLayer::SnakeHead` $\rightarrow$ Detects collisions with `Apple`, `AppleField`, `Boundary`, and `SnakePart`.
- `GameLayer::AppleField` $\rightarrow$ Sensor trigger zone around the apple that notifies the snake's head to open its mouth when close.
- `GameLayer::SnakePart` $\rightarrow$ Enables self-collision detection when the snake head loops back into its own body.
- `GameLayer::Boundary` $\rightarrow$ Outer wall colliders enclosing the play area.

---

## 📁 Project Structure

```
snake/
├── assets/
│   ├── sounds/
│   │   ├── crunch.wav      # Sound played when eating an apple
│   │   └── hit.wav         # Sound played on crash/game over
│   └── sprites/
│       ├── apple.png                 # Apple item sprite
│       ├── snake_eye_sprite.png      # Animated eye texture atlas
│       ├── snake_hit.png             # Animated impact explosion atlas
│       ├── snake_mouth_sprite.png    # Animated mouth texture atlas
│       └── snake_tounge.png          # Animated tongue texture atlas
├── src/
│   ├── fabrik.rs           # Inverse kinematics solver & segment component logic
│   ├── lib.rs              # Library exports (`fabrik` module)
│   └── main.rs             # Game loop, Bevy plugins, systems, UI, & collision listeners
├── Cargo.toml              # Dependencies (bevy, avian2d, rand, bevy_asset_loader)
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites
- **Rust Toolchain**: [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2024 compiler support, 1.85+)
- **Linux System Dependencies** (required for Bevy & Wayland/X11 graphics/sound):
  ```bash
  # Ubuntu / Debian
  sudo apt-get install pkg-config libwayland-dev libxkbcommon-dev libasound2-dev libudev-dev
  ```

### Running the Game
Clone the repository and run:

```bash
cargo run --release
```

> **Note**: Running with `--release` enables compiler optimizations, providing smooth 60+ FPS physics and inverse kinematics simulation.

---

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - Safe, fast systems programming language
- **[Bevy Engine](https://bevyengine.org/)** (v0.17.3) - Data-driven Rust game engine
- **[Avian 2D](https://github.com/JONDAB/avian)** (v0.4) - 2D physics engine for Bevy
- **[rand](https://crates.io/crates/rand)** (v0.9.2) - Random number generation for apple spawning

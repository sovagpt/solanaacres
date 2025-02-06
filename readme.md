<div align="center">
<h1>🌐 Hello,World?: An AI-Driven Village Simulation</h1>
<p>Powered by Advanced AI Systems for Dynamic NPC Interactions</p>

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/Bevy-0.12-yellow)](https://bevyengine.org/)
</div>

## 🚀 What is HelloWorld?

HelloWorld is a sophisticated AI village simulation that creates a living, breathing world of NPCs with varying levels of self-awareness. Inspired by "The Truman Show," some NPCs are aware they're in a simulation while one live their virtual lives unaware, creating unique and complex social dynamics.

### Key Features

- 🧠 **Advanced AI Systems**: Complex personality and behavior patterns
- 🤝 **Dynamic Social Interactions**: Realistic relationship and group dynamics
- 💭 **Memory Systems**: Short and long-term memory with natural decay
- 🎭 **Awareness Levels**: Mixed population of aware and unaware NPCs
- 🗳️ **Community Influence**: User voting system for town events

## 🛠️ Quick Start

### Prerequisites
- Rust 1.70+
- Cargo package manager
- PostgreSQL database

### Installation

```bash
# Install using cargo
cargo install helloworld

# Or build from source
git clone https://github.com/devforsolana/helloworld.git
cd helloworld
cargo build --release
```

### Basic Usage

```rust
use helloworld::Town;

fn main() {
    let mut town = Town::new();
    
    // Add NPCs with different awareness levels
    town.add_npc(true);  // Aware NPC
    town.add_npc(false); // Unaware NPC
    
    town.run();
}
```

## 🧠 AI Systems

- **Cognitive System**: Decision making and reasoning
- **Memory System**: Experience storage and recall
- **Social System**: Relationships and group dynamics
- **Personality System**: Traits and behavior patterns
- **Dialogue System**: Contextual conversations

## ⚙️ Configuration

```rust
let config = Config {
    simulation: SimulationConfig {
        tick_rate: 60.0,
        world_size: Vec2::new(1000.0, 1000.0),
        max_entities: 1000,
    },
    ai: AiConfig {
        max_npcs: 100,
        memory_decay_rate: 0.1,
        interaction_radius: 50.0,
        awareness_threshold: 0.8,
    },
};
```

## 🎮 Controls

- Left Click: Select NPC/Interact/Vote
- MORE TO INTERACTIVITY TO COME (P2E)

## 📊 Features

### NPC Behaviors
- Daily routines
- Social interactions
- Memory-based decisions
- Emotional responses
- Dynamic relationships

### Town Events
- Community voting
- Social gatherings
- Economic activities
- Environmental changes

## 🤝 Contributing

Contributions are welcome! Please check our [Contributing Guidelines](CONTRIBUTING.md) for details.

## 📝 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 📫 Contact

For questions, issues, or contributions, please visit our [GitHub repository](https://github.com/yourusername/helloworld).

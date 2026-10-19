# Conway's Game of Life (Rust + Macroquad)

A fast, interactive implementation of Conway's Game of Life written in Rust using the `macroquad` 2D engine.

## ✨ Features
- **Two boundary modes:** Bounded (edges) and Toroidal (wrap-around).
- **Interactive camera:** Pan and zoom smoothly.
- **Built-in patterns:** Glider, Pulsar, and Gosper Glider Gun.
- **Real-time controls:** Adjust simulation speed on the fly.

## 🎮 Controls
| Input | Action |
| :--- | :--- |
| **Left Click** | Toggle cell alive/dead |
| **Right / Middle Click** | Pan camera |
| **Mouse Wheel** | Zoom in/out |
| **Space** | Pause / Resume |
| **N** | Step forward one generation |
| **R** | Clear the board |
| **F** | Fill with random cells |
| **B** | Toggle boundary mode |
| **1 / 2 / 3** | Spawn Glider / Pulsar / Gosper Gun |
| **Up / Down** | Increase / Decrease speed |
| **H** | Toggle on-screen help |

## 🚀 How to Run

Make sure you have [Rust](https://rustup.rs/) installed, then run:

```bash
cargo run --release
```
*(Using `--release` is highly recommended for smooth performance).*

## 📦 Dependencies
- `macroquad` (for rendering and input)
- `rand` (for random board generation)
```

### Why this works well:
1. **Scannable:** Uses tables and bullet points so users can find controls instantly.
2. **Concise:** No unnecessary fluff, just what a user/developer needs to know.
3. **Actionable:** Tells them exactly how to run it with the optimal `--release` flag.

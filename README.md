# Mood Mosaic — AI Capstone (Rust + Warp)

## Overview

**Mood Mosaic** is an interactive Rust web app built with Warp. Users pick a mood and receive a personalized micro-experience (message, color accent, quick tip), combined with animated visuals and a hidden surprise endpoint.

This project demonstrates: async Rust web servers, JSON APIs, a dynamic single-file frontend, and a polished developer workflow.

## Features

- Interactive homepage with mood tiles
- API endpoints: `/api/mood?mood=...` and `/api/surprise`
- Visual flourishes: confetti, share/copy button, and an easter-egg keyboard shortcut
- Clean code structure and clear documentation for capstone grading

## System Requirements

- OS: Linux, macOS, or Windows
- Install Rust via `rustup` (recommended)
- Tools: `cargo`, `git`, a modern browser

## Install & Run

1. Install Rust (if needed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```
2. Run the app:
```bash
git clone git@github.com:wachira567/ai_capstone_rust.git
cd ai_capstone_rust
cargo run
# open http://127.0.0.1:3000/
```

## Endpoints

- `/` — Frontend UI (Mood tiles)
- `/api/mood?mood=happy|curious|stressed|calm` — JSON with `mood`, `message`, `color`, `tip`
- `/api/surprise` — small randomized surprise payload

## Development notes

- Source: `src/main.rs` contains the full server and inline HTML frontend.
- Dependencies are declared in `Cargo.toml` (includes `warp`, `tokio`, `serde`, `rand`).

## Testing (quick)

Run these commands while the server is running:
```bash
curl -i http://127.0.0.1:3000/       # HTML homepage
curl -sS 'http://127.0.0.1:3000/api/mood?mood=happy' | jq
curl -sS http://127.0.0.1:3000/api/surprise | jq
```

## Capstone deliverables covered

- Functional web app with backend and frontend
- README with setup, usage, and references
- Source code with comments and clear structure
- Interactive, 'wow' features (confetti, easter egg)

---


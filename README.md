# Prompt-Powered Kickstart: Beginner’s Toolkit for Rust (Warp)

## 1. Title & Objective

**Title:**
Prompt-Powered Kickstart: Beginner’s Toolkit for Rust (Warp)

**Objective:**
Create a beginner-friendly toolkit to help anyone get started with Rust and the Warp web framework. The end goal is a runnable "Hello World" web server and a clear, replicable guide for others.

---

## 2. Quick Summary of the Technology

**What is Rust?**
Rust is a modern systems programming language focused on safety, speed, and concurrency. **Warp** is a web framework for building APIs and web servers in Rust.

**Where is it used?**
Rust is used for CLI tools, web servers, and performance-critical applications. Warp is used for building fast, type-safe web APIs in Rust.

**Real-world example:**
Cloudflare uses Rust for edge services; Discord uses Rust for performance-critical backend services.

---

## 3. System Requirements

- OS: Linux, macOS, or Windows
- Tools: Rust (via rustup), Cargo, VS Code (recommended)
- Packages: warp, tokio, serde, serde_json

---

## 4. Installation & Setup Instructions

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # Restart your terminal after install
   rustc --version
   cargo --version
   ```
2. Clone this repo and run:
   ```bash
   git clone git@github.com:wachira567/ai_capstone_rust.git
   cd ai_capstone_rust
   cargo run
   ```
3. Open http://127.0.0.1:3000/ in your browser.

---

## 5. Minimal Working Example

This project runs a minimal Warp web server with two endpoints:

- `/` — Serves an HTML page with a greeting and a button
- `/hello` — Returns a JSON greeting

**Example code:** See `src/main.rs` for full code with comments.

**Expected output:**
Visit http://127.0.0.1:3000/ to see the greeting and test the button.

---

## 6. AI Prompt Journal

See `AI_PROMPT_JOURNAL.md` for all prompts used, responses, and reflections.

---

## 7. Common Issues & Fixes

- **Error:** `error: could not compile warp`
  - **Fix:** Run `rustup update` and ensure Rust is up to date.
- **Error:** `cargo: command not found`
  - **Fix:** Ensure Rust is installed and your PATH is set correctly.
- **Port already in use:**
  - **Fix:** Change the port in `main.rs` or stop the other process.

---

## 8. References

- [Rust Official Docs](https://doc.rust-lang.org/book/)
- [Warp Framework](https://github.com/seanmonstar/warp)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

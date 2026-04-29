# AI Prompt Journal

This journal documents all AI prompts used, curriculum links, responses, and reflections for the capstone.

---

## 6. Mood Mosaic Design & Implementation

**Prompt:**
"Design an original, interactive Rust web app that demonstrates async Warp servers and a polished frontend. Include ideas for 'wow' features."

**AI Response Summary:**
The AI proposed "Mood Mosaic": a mood-driven micro-experience web app with mood tiles, personalized messages, visual confetti, a surprise endpoint, and a small inline frontend to keep the project self-contained.

**Reflection:**
Implemented the app as `src/main.rs`. Added unit tests, CI workflow, and README updates. Committed changes incrementally with meaningful messages.

---

## 1. Project Planning & Technology Selection

**Prompt:**
"Suggest a modern, beginner-friendly Rust web framework for a capstone project. It should be easy to set up and support a minimal web server."

**Curriculum Link:**
Learning with AI → Deepening Knowledge of Your Current Programming Language

**AI Response Summary:**
The AI recommended Warp for its simplicity, async support, and strong documentation. It also mentioned Actix and Rocket but highlighted Warp as best for beginners.

**Reflection:**
Warp was easy to set up and matched the project goals.

---

## 2. Minimal Working Example (Hello World)

**Prompt:**
"Create a minimal Rust Warp web server that serves an HTML page and a JSON endpoint."

**Curriculum Link:**
Learning with AI → AI-Powered Code Suggestions

**AI Response Summary:**
The model provided a `main.rs` example using `warp`, `tokio`, and `serde_json`, with `/` for HTML and `/hello` for JSON. This was adapted directly.

**Reflection:**
The AI code worked with minor tweaks. Reading the docs helped clarify async and filter usage.

---

## 3. Error Handling & Troubleshooting

**Prompt:**
"I get 'error: could not compile warp' when running cargo build. How do I fix this?"

**Curriculum Link:**
Using AI to debug errors → Tracing Error Messages and Stack Traces

**AI Response Summary:**
The AI explained this is often due to an outdated Rust toolchain. Suggested running `rustup update` and checking Cargo.toml for typos.

**Reflection:**
Updating Rust fixed the issue. The AI's troubleshooting steps were clear and actionable.

---

## 4. Documentation & Best Practices

**Prompt:**
"How should I structure a beginner-friendly Rust project README and what sections are essential for a toolkit?"

**Curriculum Link:**
Generating and Improving Documentation with AI → Project README Generation

**AI Response Summary:**
The AI suggested a structure matching the capstone rubric: Title, Objective, Tech Summary, Requirements, Setup, Example, Prompts, Issues, References.

**Reflection:**
Following this structure made the project easy to follow and grade.

---

## 5. Reflection on AI Use

**Prompt:**
"How can I use AI most effectively for learning Rust and documenting my process?"

**Curriculum Link:**
Principles when using AI

**AI Response Summary:**
The AI emphasized: communicate clearly, verify all code, use guided practice, and document learning. It warned against copy-pasting without understanding.

**Reflection:**
Actively engaging with the AI and verifying each step led to deeper understanding and a better toolkit.

---

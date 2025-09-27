# ⚠️ Disclaimer

> This entire project—including game logic, rendering, and documentation was produced via extensive interaction with an LLM agent.

# Bigger Spoon RL

An experiment in "vibe coding"—pairing precise prompts with large language models to sketch out a minimal, cross-platform roguelike. This project aims to be a clean, approachable example for anyone exploring text-based games that run both in a terminal and in a browser.

## Goals

- Demonstrate how to share game logic between a native text console (Crossterm + Ratatui) and a web canvas (Webatui + Yew/WASM).
- Keep the example approachable: a single room, a player `@`, a staircase `>`, and a dog `d` that chases the player.
- Showcase how much of a project can be iterated via targeted LLM prompting and manual verification.

## Features

- **Text UI:** Ratatui renders the map, player, items, monsters, and a single-line message log.
- **Web UI:** Webatui mirrors the same layout in the browser, using Trunk for build/serve.
- **Shared Logic:** Movement, collision, stairs, messages, and monster updates live in `game.rs` and feed both front-ends.
- **Theme Support:** Rendering is driven by a `Theme` that picks Base16 palette colours, making it easy to tweak appearance.
- **LLM-Assisted Workflow:** Every module—`game`, `render`, `web`, `main`, even this README—was produced through large amounts of tightly-targeted prompts, with manual reviews and edits after each iteration.

## Tech Stack

- **[Rust](https://rust-lang.org/)**
- **[Ratataui](https://ratatui.rs/)** (`crossterm` backend)
- **[Webatui](https://docs.rs/webatui/latest/webatui/) + [Yew](https://yew.rs/)** (browser rendering & keyboard handling)
- **[Trunk](https://trunkrs.dev/)** (WASM bundling + static asset copying)
- **[Base16 Palettes](https://github.com/chriskempson/base16)** (theme colours)
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** (native terminal control)

# Demo Video Clips

Text console:

https://github.com/user-attachments/assets/bfd75c12-8e9f-4c62-8619-f3f937b90fa0

Web:

https://github.com/user-attachments/assets/a781fbf9-6182-422f-aec8-d501fa39dd2e



## Workflow Notes

The entire codebase—architecture, refactors, and documentation—was driven by iterative LLM prompts and human review. The typical loop was:
1. Draft a change via the agent (with explicit goals & constraints).
2. Run `cargo fmt`, `cargo check`, and targeted tests.
3. Inspect the output manually, refine the prompt, repeat.
4. Commit once nicely formatted and validated.

Even documentation follows the same loop: this entire README was drafted by the agent from a rough outline, then reviewed and polished manually.

## Getting Started

```bash
# Native terminal build
cargo run

# Web build
./web.sh   # or: trunk serve --no-default-features --features web
```

## Some gameplay highlights

- Descending the staircase in the browser redirects you to another web page
- Descending in the terminal exits the app.
- The dog bites if it reaches you, but never kills you

## TODO / Future Notes

See `IDEAS.md` for future refactor thoughts: better message logging, theme switching, smarter pathfinding, and turn queues.

 Getting Started with Rust – A Beginner’s “Hello, World!” Toolkit

- *Author:* Collins Kiprotich Korir 
- *Date:* November 2025  
- *Technology:* Rust (stable toolchain via `rustup`)

*Documentation reference:* built for the **Moringa AI Capstone – Beginner’s Toolkit with GenAI**, following the prompt-powered workflow and deliverable checklist.

Rust remains the most-loved language on Stack Overflow for nearly a decade thanks to its blend of C/C++ performance, fearless concurrency, and compile-time memory safety without a garbage collector. I wanted a systems language that felt modern, let me ship fast CLI tools, and opened doors in infrastructure, blockchain, and emerging ecosystems—Rust checks all of those boxes.

---

 1. Quick Summary of the Technology

*What is Rust?*  
Rust is a modern systems programming language that enforces memory safety and data-race freedom at compile time. Ownership, borrowing, and lifetimes replace the need for a garbage collector while still delivering low-level control.

*Where is it used?*  
Microsoft Windows components, Mozilla Firefox, Discord’s backend, Dropbox sync, Cloudflare Workers, AWS Firecracker, Meta infra, Solana, Deno, Turbopack, and many more high-performance stacks.

*Real-world example:* Deno (the TypeScript/JavaScript runtime) and npm’s Turbopack bundler are written entirely in Rust.

---

 2. System Requirements

- *OS:* Windows 10+, macOS 10.12+, or most modern Linux distros  
- *RAM:* ≥ 4 GB (8 GB recommended)  
- *Disk:* ~2–3 GB for the Rust toolchain and caches  
- *Network:* Needed during installation to pull the standard library

*Required tools*
- `rustup` (installer + toolchain manager)
- Code editor (VS Code + `rust-analyzer` extension recommended, but any editor works)

---

#
 3. Installation & Setup

 macOS / Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```
 Windows
1. Visit https://rustup.rs  
2. Download and run `rustup-init.exe`  
3. Accept the default installation (press `1` then Enter)

 Verify install
```bash
rustc --version
cargo --version
```
You should see versions similar to `rustc 1.82.0` and `cargo 1.82.0`.

 Recommended VS Code extensions
`rust-analyzer`, `CodeLLDB`, `Error Lens`, `Better TOML`

---

 4. Minimal Working Example – Hello World

 Create & run the project
```bash
cargo new hello_rust
cd hello_rust
cargo run
```

 `src/main.rs`
The project ships with an interactive greeter:

```rust
fn main() {
    println!("Hello, World! 👋");
    println!("What is your name?");

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();
    let display_name = if name.is_empty() { "Stranger" } else { name };

    println!("Hello, {display_name}! Welcome to Rust! 🚀");
}
```

Run `cargo run` and you’ll be prompted for your name:
```
Hello, World! 👋
What is your name?
Alice
Hello, Alice! Welcome to Rust! 🚀
```

---

## 5. Bonus Program – `cargo run --bin joke`

I added a second binary at `src/bin/joke.rs` that fetches a programming joke using `reqwest` and `tokio`.

```bash
cargo run --bin joke
```

Sample output:
```
Fetching a programming joke...

Why do programmers prefer dark mode?
Because light attracts bugs!
```

Feel free to customize the endpoint or print multiple jokes.

---

 6. AI Prompt Journal *(prompts issued via [ai.moringaschool.com](https://ai.moringaschool.com))*

| # | Prompt | AI Response Summary | Helpfulness |
|---|--------|---------------------|-------------|
| 1 | “Give me the simplest possible way to install Rust on Windows, macOS, and Linux in 2025” | Delivered the `rustup.rs` installer scripts plus Windows EXE workflow | 10/10 |
| 2 | “Create a step-by-step beginner guide to make first Rust project using cargo new” | Covered `cargo new`, project layout, `cargo run` | 9/10 |
| 3 | “How do I read a line of input from the user in Rust without external crates?” | Demonstrated `std::io::stdin().read_line()` pattern | 10/10 |
| 4 | “Explain Rust’s String vs &str in simple terms with examples” | Used a great ownership/borrowing analogy (house vs guest room) | 10/10 |
| 5 | “Why am I getting ‘cannot borrow as mutable’ error when using read_line?” | Pointed out the missing `mut` binding | 10/10 |
| 6 | “Best VS Code extensions for Rust in 2025” | Recommended `rust-analyzer`, `CodeLLDB`, `Even Better TOML` | 9/10 |

*Reflection:* Generative AI shrank my learning curve from ~6 hours of manual doc-diving to ~90 minutes. It felt like pairing with a senior Rustacean who could unblock me instantly, especially on ownership and tooling errors.

---

 7. Common Issues & Fixes

| Issue | Error Message | Fix |
|-------|---------------|-----|
| `rustc`/`cargo` not found | `command not found: rustc` | Restart terminal or `source $HOME/.cargo/env` (Windows: reboot) |
| Missing `mut` | `cannot borrow as mutable` | Declare `let mut name = String::new();` |
| VS Code squiggles everywhere | rust-analyzer stuck | Reload VS Code (`Ctrl+Shift+P` → Reload Window) or reinstall extension |
| First build is slow | Compiling stdlib | Normal on fresh installs; subsequent builds are fast |
| Antivirus blocks `rustup-init.exe` | “Threat detected” | Temporarily disable or whitelist; `rustup.rs` is trusted |

---

 8. References

- The Rust Book: https://doc.rust-lang.org/book/  
- Rust by Example: https://doc.rust-lang.org/rust-by-example/  
- Rust Playground: https://play.rust-lang.org  
- “Rust Programming Course for Beginners” – Let’s Get Rusty (2024/2025)  
- “Rust in 100 Seconds” – Fireship  
- Official installer: https://rustup.rs

---

 9. Repository & Extras

- GitHub (placeholder):   
- Screenshots & Loom walkthrough live in `screenshots/` and the linked Loom share (replace with your own).  
- Bonus idea: experiment with more bins such as `cargo run --bin timer` or add tests with `cargo test`.

 10. Screenshots & Loom Proof

 - All assets live in `screenshots/` (already created with a placeholder file so it’s tracked in Git).  
 - Suggested filenames:  
   - `01-rustup-install.png` – installer output / verification  
   - `02-cargo-run-hello.png` – prompt + greeting sample  
   - `03-cargo-run-joke.png` – joke API output  
   - `04-vscode-setup.png` – editor + extensions panel  
 - Update the Loom URL (if recorded) and keep it alongside these screenshots for quick reviewer access.

*Final thoughts:* Rust’s learning curve is real, but partnering with GenAI as a just-in-time tutor made the process fun instead of frustrating. I’m now confident enough to start shipping real CLI utilities. 🚀


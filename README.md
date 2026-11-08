<p align="center">
  <img src="assets/logo.svg" alt="BufReader in Rust — Podcast Companion Examples" width="600" />
</p>

<p align="center">
  <b>🇬🇧 English</b> | <a href="README.ru.md">🇷🇺 Русский</a>
</p>

# 🦀 BufReader in Rust — Podcast Companion Examples (Episode 1)

This repository is the practical code companion for Episode 1 of the engineering podcast series **«Rust in Details»** (*«Rust в деталях»*).  
Here you will find runnable benchmarks and minimal, clear examples demonstrating the massive performance difference between raw operating system syscalls and buffered I/O in Rust.

---

## 🎙 Podcast & Blog Article

* 🎧 **Listen on YouTube:** [Episode 1 — BufReader \| How to accelerate file & stream reading](https://www.youtube.com/watch?v=gATf436M6ow)
* 📖 **Read the deep-dive article:** [BufReader in Rust: Accelerating I/O on eagle2000.space](https://eagle2000.space/en/rust/bufreader/) *(or in [Russian](https://eagle2000.space/rust/bufreader/))*

---

## 💡 The Core Idea: Real-World Analogies

Whenever an application reads data (from an SSD, a network socket, or standard input), the OS handles the request through **system calls** (syscalls like `read(2)` on Unix/Linux/macOS). Every single syscall requires the CPU to switch context from user mode to kernel mode and back again — hundreds of CPU cycles for just one operation.

* 🚰 **Reading 1 byte at a time (unbuffered `Read`):**  
  Imagine having no plumbing at home. Every time you want a sip of water, you have to walk outside down the street to the water pump. Running the task is simple, but doing it 8 million times a day will leave you exhausted with zero energy for anything else.
* 🏺 **Reading the entire file into memory at once:**  
  This is like hauling a massive 20-liter water jug home. It's fast, but heavy to carry around, and if the file is 10 GB, your system crashes with an Out Of Memory (*OOM*) panic.
* ☕ **`BufReader`:**  
  This is a regular coffee mug. It fetches a sensible chunk of water from the pump once (**8 KB / 8192 bytes** by default) and stores it in your application's RAM buffer. Your code drinks comfortably from the mug. Instead of 8,000,000 expensive OS round-trips, you make only ~1,000!

---

## 📂 What's Inside `src/bin`

| Binary | Approach | Notes |
| :--- | :--- | :--- |
| **`01_unbuffered_read.rs`** | Raw 1-byte reading with `std::io::Read` | Extremely slow (~8–10s on an 8 MB file due to millions of syscalls) |
| **`02_bufreader_lines.rs`** | Standard `BufReader::new(file).lines()` | Idiomatic and fast (~10ms), allocates a new `String` per line |
| **`03_bufreader_reuse_buffer.rs`** | `BufReader + .read_line(&mut line)` | Even faster (~6–8ms), reuses a single `String` buffer (**0 heap allocations**) |
| **`04_read_entire_file.rs`** | `fs::read_to_string` | Record speed (~2ms), but requires allocating RAM equal to file size |
| **`05_benchmark_comparison.rs`** | **All-in-one comparative benchmark** | Runs all 4 approaches sequentially and prints a formatted benchmark table |

---

## 📊 Benchmark Results (8 MB Sample File, 100,000 Lines)

Measured in release mode (`cargo run --release`):

| Approach | Execution Time | OS Syscalls | Memory Footprint |
| :--- | :--- | :--- | :--- |
| **1. Unbuffered byte-by-byte (`Read`)** | ~9.84 s | ~8,000,000 syscalls | Minimal |
| **2. `BufReader + .lines()`** | ~10.4 ms | ~1,000 syscalls | 8 KB buffer + per-line strings |
| **3. `BufReader + reuse String`** | **~7.1 ms** | ~1,000 syscalls | **8 KB buffer, 0 allocations** 🏆 |
| **4. Entire file into memory (`read_to_string`)** | ~2.1 ms | 1 syscall | 8 MB allocated in RAM |

> **Takeaway:** `BufReader` with string reuse delivers a **>1000x speedup** compared to raw unbuffered reads, all while keeping memory consumption constant and strictly bounded.

---

## ⚠️ When is `BufReader` Redundant?

1. **The data is already in RAM:** (`&[u8]`, `Vec<u8>`, `Cursor`, `String`) — no OS syscalls occur anyway; an extra wrapper buffer only adds overhead.
2. **Reading in large chunks:** if your application already reads 64 KB or 1 MB blocks at a time, syscall overhead is negligible.
3. **Tens of thousands of concurrent connections:** each `BufReader` allocates an 8 KB internal buffer. For 20,000 concurrent network sockets, that equals ~160 MB of memory just for read buffers.

---

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/eaglebk/rust-bufreader-examples.git
cd rust-bufreader-examples

# Run the complete comparative benchmark
cargo run --release --bin 05_benchmark_comparison

# Or run individual examples:
cargo run --release --bin 02_bufreader_lines
cargo run --release --bin 03_bufreader_reuse_buffer
```

> **Note:** A realistic ~8 MB text file (`sample_data.txt`) will be automatically generated on the first run.

---

## 🔗 Useful Links

* 📖 Rust Docs: [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
* 📖 Rust Docs: [`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)
* 🌐 Blog: **[eagle2000.space](https://eagle2000.space/en/)** — interactive tutorials on Rust, mobile engineering, and visual arts
* ✈️ Telegram: **[@prog_lighthouse](https://t.me/prog_lighthouse)** — «Маяк Программиста» `[RU]`
* 📺 YouTube: **[@prog_lighthouse](https://www.youtube.com/@prog_lighthouse)** — Programmer's Lighthouse `[RU]`

---

## 📄 License

MIT © Konstantin ([@eaglebk](https://github.com/eaglebk))

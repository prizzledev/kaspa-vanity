# ⚡ Kaspa Vanity Address Generator

This is a high-performance, multi-threaded Rust tool for generating **Kaspa addresses** that match a custom pattern of your choice — whether they start with, contain, or end with specific characters.

Built with performance in mind, this tool leverages the power of parallel processing to help you generate your unique Kaspa identity quickly and efficiently.

---

## 🛠 Features

- 🔁 Supports matching:
  - `starts with`
  - `contains`
  - `ends with` (default)
- 🚀 Super fast multithreaded search using [`rayon`](https://docs.rs/rayon)
- 🧠 Smart progress logs every 100,000 attempts
- 💾 Automatically saves matching address and private key to `address.json`

- 🏗️ Cross-platform builds (macOS, Windows)

---


## 📦 Binaries / Releases


Don't want to compile? No worries — precompiled binaries for macOS, and Windows are available on the [Releases page](https://github.com/prizzledev/kaspa-vanity/releases).


Just download, extract, and run:


```bash

./kaspa-vanity-macos

kaspa-vanity-windows.exe

```


> Make sure you run the binary in a terminal to see prompts and progress.



---

## ▶️ Usage

```bash
cargo run --release
```

You'll be asked:

- How many threads to use (defaults to 8)
- What type of match you want (`starts`, `contains`, `ends`)
- The pattern you'd like to match

Example:
```
Enter number of threads (default 8): 12
Match type [starts|contains|ends] (default ends): ends
Enter pattern to match: KAS
```

Once a match is found, the tool will:

- Show the **Kaspa address**
- Show the **private key (hex)**
- Save both to a file named `address.json`

---

## 🧱 Building from Source


Make sure you have [Rust](https://rustup.rs) and Docker installed and run:


```bash

./build-all.sh

```


Binaries will be placed in the `/bin` folder, ready to be distributed or uploaded.


---

## ☕ Support & Donations

If this project helped you find a cool Kaspa address and you'd like to say thanks — a small Kaspa tip would be super appreciated ❤️

**Kaspa donation address:**  
`kaspa:qr8yusem6h0x4wmcty39jueegdc9q67jwtr6uhswcskt33fljy89jlgekaspa`

No pressure at all — just happy you're using it!

---

## 📄 License

MIT License — feel free to use, modify, and share.

---

## 💬 Questions?

Open an issue or start a discussion — happy to help.
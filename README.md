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

[//]: # (- 🏗️ Cross-platform builds &#40;Linux, macOS, Windows&#41;)

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

[//]: # (## 📦 Binaries / Releases)

[//]: # ()
[//]: # (Don't want to compile? No worries — precompiled binaries for Linux, macOS, and Windows are available on the [Releases page]&#40;https://github.com/your-username/kaspa-vanity/releases&#41;.)

[//]: # ()
[//]: # (Just download, extract, and run:)

[//]: # ()
[//]: # (```bash)

[//]: # (./van-linux)

[//]: # (./van-macos)

[//]: # (van-windows.exe)

[//]: # (```)

[//]: # ()
[//]: # (> Make sure you run the binary in a terminal to see prompts and progress.)

[//]: # ()
[//]: # (---)

[//]: # (## 🧱 Building from Source)

[//]: # ()
[//]: # (Make sure you have [Rust installed]&#40;https://rustup.rs&#41; and run:)

[//]: # ()
[//]: # (```bash)

[//]: # (./build-all.sh)

[//]: # (```)

[//]: # ()
[//]: # (Binaries will be placed in the `/bin` folder, ready to be distributed or uploaded.)

[//]: # ()
[//]: # (---)

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
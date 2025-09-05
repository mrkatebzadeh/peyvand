# Peyvand 🕸️
A **[Surf][surf]-inspired web browser** built on top of **[wry][wry]** and **[tao][tao]** with **Vim-style modal key bindings** in mind.
It aims to provide the efficiency of a focused and keyboard-driven browsing experience.

---

## ⚡ How to Build

Make sure you have Rust installed (https://rustup.rs).

```bash
git clone https://github.com/mrkatebzadeh/peyvand.git
cd peyvand
cargo build --release
```

## ▶️ How to Run
```bash
cargo run --release -- https://google.com
```

---
## 🚀 Features So Far

- ✅ Window and WebView integration
- ✅ Cookie Manager
- ✅ User Agent
- ✅ Basic Vim navigation keys: `h`, `j`, `k`, `l`
- ✅ History stack for back/forward navigation
- ✅ Customizable keybindings
- ✅ Status bar for mode & command display
- ✅ URL manipulation (change, copy, paste, reload)
- ✅ Search text within page

---

## 🎯 Upcoming

- ⬜ Follow links with hints (like Vimium-style navigation)
- ⬜ Configurable homepage
- ⬜ Multi tab

---

[surf]: https://surf.suckless.org/
[wry]: https://docs.rs/wry/latest/wry/
[tao]: https://docs.rs/tao/latest/tao/

# Quill

A native desktop screenplay writing application built with Rust.

## About

Quill is a local-first screenplay editor with industry-standard formatting. No cloud, no subscriptions—your stories stay on your machine.

## Status

🚧 **Early Development** - Basic structure only.

## Building

**Prerequisites:**
- Rust 1.70+
- System dependencies for your platform (see below)

**Arch Linux:**
```bash
sudo pacman -S rust base-devel pkg-config fontconfig freetype2 libxcb
```

**Ubuntu/Debian:**
```bash
sudo apt install build-essential pkg-config libfontconfig1-dev libfreetype6-dev libxcb-render0-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**macOS:**
```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Build:**
```bash
cargo run
```

## License

See [LICENSE](LICENSE)
# Noci

![Version 0.1](https://img.shields.io/badge/Version%200.1-FFC832?style=for-the-badge&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000?style=for-the-badge&logo=rust&logoColor=white)
[![MIT License](https://img.shields.io/badge/MIT%20License-004772?style=for-the-badge&logo=license&logoColor=white)](https://github.com/lnB51/spark/blob/master/LICENSE)

---

Noci is a desktop music controller app built with [SvelteKit](https://kit.svelte.dev/) and [Tauri](https://tauri.app/). It provides a minimal, always-on-top UI for controlling Spotify playback, displaying track info, and visualizing audio progress.

---

## ✅ Features

- Minimal, always-on-top window with transparent background and notch support
- Spotify playback controls (play/pause, next, previous)
- Displays current track info and album art
- Animated audio bars with color extracted from album art
- Responsive design for desktop and compact modes
- Built with SvelteKit (frontend) and Rust/Tauri (backend)

## ✨ Planned
- Apple music support
- Air drop in notch support

---

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+ recommended)
- [Rust](https://www.rust-lang.org/tools/install)
- [npm](https://www.npmjs.com/) / [pnpm](https://pnpm.io/) / [yarn](https://yarnpkg.com/)
- Spotify Desktop App (for playback control)

### Installation

```sh
git clone https://github.com/lnb51/noci.git
cd noci
```

---

## 🛠 Development

To start the development server and Tauri app (runs both frontend and backend):

```sh
cargo tauri dev
# or
npm run tauri dev
# or
pnpm tauri dev
# or
yarn tauri dev
```

---

## 📦 Building for Production

To build the app for production:

```sh
cargo tauri build
# or
npm run build
npm run tauri build
# or
pnpm build && pnpm tauri build
# or
yarn build && yarn tauri build
```

The final binaries will be in the `src-tauri/target/release/` directory.

---

## 📁 Project Structure

```
src/         # SvelteKit frontend (UI, routes, assets)
src-tauri/   # Tauri backend (Rust, window management, Spotify integration)
static/      # Static assets
build/       # Production build output
```

---

## 📜 Scripts

- `npm run dev` / `pnpm dev` / `yarn dev` - Start SvelteKit dev server
- `cargo tauri dev / npm run tauri dev` / `pnpm tauri dev` / `yarn tauri dev` - Start Tauri app in dev mode
- `npm run build` / `pnpm build` / `yarn build` - Build SvelteKit frontend
- `cargo tauri build / npm run tauri build` / `pnpm tauri build` / `yarn tauri build` - Build Tauri app for release

---

## 💻 Showcase

### Default view

![default](https://github.com/lnb51/noci/showcase/default_view.png)

### Wide 

![wide](https://github.com/lnb51/noci/showcase/wide_view.png)


---

## 📝 License

MIT

---

Made with ❤️ using SvelteKit and Tauri.
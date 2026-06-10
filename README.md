<div align="center">
<br/>

# wayland-shell

### The visual layer of Sorabbyngo OS — AI-native desktop shell, VM window management, and voice-first UI.

<br/>

[![Division](https://img.shields.io/badge/Division-Sorabbyngo_OS-6C3FC4?style=for-the-badge)](https://github.com/sorabbyngo-os)
[![Lang](https://img.shields.io/badge/Lang-Rust-AA66FF?style=for-the-badge)](https://github.com/sorabbyngo-os/wayland-shell)
[![Status](https://img.shields.io/badge/Status-In_Development-FF6B35?style=for-the-badge)](https://github.com/sorabbyngo-os/wayland-shell)

</div>

---

## What It Does

wayland-shell is the Sorabbyngo OS Wayland compositor and desktop shell for byngo-os. It is AI-native from the ground up — Byngox agents are first-class citizens of the window system, Chameha voice commands control the entire desktop, and the UI adapts to context using the HAL ambient awareness feed.

---

## Features

| Feature | Description |
|---|---|
| **Wayland Compositor** | wlroots-based compositor with custom Sorabbyngo protocol extensions |
| **AI Agent Windows** | Byngox agents rendered as native windows with transparent protocol |
| **Voice-first UI** | Full desktop control via Chameha — open apps, manage windows, execute tasks |
| **Adaptive Layout** | Desktop adapts based on context: focus mode, presentation mode, split |
| **VM Window Management** | Seamless VM windows via hypervisor-engine integration |
| **Dark-first Design** | Deep dark palette (#080608) — designed for low-light African environments |
| **Multi-language UI** | English and Swahili — additional African languages via l10n system |

---

## Architecture

```
wayland-shell/
├── src/
│   ├── compositor/   # wlroots Wayland compositor core
│   ├── shell/        # Desktop shell — panels, workspaces, launchers
│   ├── agents/       # Byngox agent window protocol
│   ├── voice/        # Chameha voice command integration
│   ├── vm/           # VM window seamless embedding
│   ├── adaptive/     # Context-aware layout engine (HAL feed)
│   ├── render/       # wgpu rendering pipeline
│   └── l10n/         # Localisation (English, Swahili, ...)
├── Cargo.toml
└── README.md
```

---

## Design Philosophy

The shell gets out of the way. The AI agents, the voice interface, and the ambient context do the heavy lifting. When you sit down at a Sorabbyngo desktop, the system already knows where you are, what you were doing, and what you probably need next.

The visual language is dark, precise, and purple. Not because it looks cool — because it works in Nairobi at 2am and in a bright Kisii office at noon.

---

## Stack

- **Language:** Rust
- **Compositor:** wlroots (via smithay)
- **Rendering:** wgpu
- **Voice:** Chameha (sorabbyngo-os/chameha)
- **AI:** Byngox (sorabbyngo-os/byngox)
- **Sensors:** HAL ambient feed (sorabbyngo-os/hal)
- **OS target:** byngo-os

---

*Sorabbyngo OS · Kisii, Kenya · 2026*


# 💬 messenger-wrapper

![Status](https://img.shields.io/badge/Status-Unofficial-red) 
![Rust](https://img.shields.io/badge/Made_with-Rust-orange?logo=rust) 
![Tauri](https://img.shields.io/badge/Framework-Tauri-blue?logo=tauri) 
![License](https://img.shields.io/badge/License-MIT-green)

A lightweight, open-source application that wraps the Messenger website into a standalone executable. **Built using Tauri.**

---
> 
> **⚠️ IMPORTANT DISCLAIMER**
>
> This project is **100% Open Source** and is a strictly **UNOFFICIAL** community project.
> It is **NOT** associated with, endorsed by, or affiliated with Meta Platforms, Inc. (Facebook) in any way.
> This software is a web wrapper created for educational purposes and to restore desktop functionality for the community.
---
## Features

- **Privacy Focused:** Sandboxed environment with no extra telemetry.
- **Native Notifications:** Detects incoming messages via window title monitoring and pushes native system alerts, even when the app is in the background.
- **Lightweight & Fast:** Uses system webview instead of a bundled browser (unlike Electron), saving RAM.



## Prerequisites
- Rust (latest stable)
- Node.js (for Tauri CLI)

## How to build

1. Clone
```bash
git clone https://github.com/floyare/messenger-wrapper.git
cd messenger-wrapper
```

2. Install dependencies
`pnpm install`
    
3. Run (Development)
`pnpm tauri dev`

4. Build (Production)
`pnpm tauri build`
## License

This project is licensed under the [MIT License](https://choosealicense.com/licenses/mit/) - see the LICENSE file for details.

"Messenger" is a registered trademark of Meta Platforms, Inc. This project uses the name for descriptive purposes only.


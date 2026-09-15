# 🧬 Helix Backbone
### Ultra-High-Velocity 32-Bit Lock-Free Ingestion & Routing Engine

The **Helix Backbone** is a zero-allocation, first-principles concurrent engine designed to eliminate data routing bottlenecks entirely. Built natively on a fully lock-free `DashMap` pipeline, the architecture strips away traditional soft-locking context stalls to process high-intensity compressed data streams at the absolute execution floor of modern server hardware.

---

## ⚖️ Dual-Licensing Disclosure & Enterprise Terms

This repository is distributed under a strict **Dual-Licensing Model** to promote open scientific collaboration while firmly protecting proprietary intellectual property:

1. **Public/Evaluation Track (AGPLv3):** This software is open-source under the GNU Affero General Public License v3. Independent developers, systems researchers, and technical auditors are completely free to clone, locally compile, modify, inspect, and test this engine free of charge.
2. **Commercial/Corporate Track:** The AGPLv3 license strictly forbids the integration of this lock-free engine into any closed-source enterprise software, proprietary corporate platforms, or commercial network services without opening the parent platform's source code.

If a commercial entity or corporation wishes to import, link, or deploy this 158 KB engine inside a proprietary infrastructure without being bound by copyleft disclosure laws, you **must purchase a commercial license directly from the author.**

*For corporate licensing tracks or commercial advisory terms, contact the author directly.*

---

## 📦 How to Compile the Helix Backbone Engine

This engine compiles into an ultra-lean, stripped dynamic library. Before building, ensure you have the native Rust toolchain installed from [rustup.rs](https://rustup.rs).

### 1. Building Locally on Windows (`.dll`)
If you are running your validation or staging tests locally on a Windows environment, execute the following command in your terminal:
```powershell
cargo build --release
```
The optimized dynamic link library will be emitted natively at:  
`target/release/helix_stature.dll`

### 2. Cross-Compiling for Linux Targets from Windows (`.so`)
To compile the portable Linux shared object library directly from a Windows host machine without an external virtual environment, configure and run:
```powershell
# Pull down the explicit Linux target component architecture
rustup target add x86_64-unknown-linux-gnu

# Compile the optimized release shared library object
cargo build --release --target x86_64-unknown-linux-gnu
```
The resulting un-throttled production binary will be located at:  
`target/x86_64-unknown-linux-gnu/release/libhelix_stature.so`

### 3. Building Natively on Linux (`.so`)
If you are working directly inside a native Linux terminal environment, simply execute:
```bash
cargo build --release
```
The compiled Linux shared object will be emitted directly at:  
`target/release/libhelix_stature.so`

---

## 🛠️ Repository Architecture

* `/src` — Core lock-free 32-bit Rust ingestion loops and static register routing matrices.
* `/daemons` — Platform-specific handshake routing drivers.
* `Cargo.toml` — Metal-optimized aggressive release profiles.
* `LICENSE` — Official AGPLv3 copyleft firewall tracking text.

---
*Copyright (c) 2026 Mark Pratt. All Rights Reserved. For corporate licensing inquiries or advisory validation checks, contact the author directly.*

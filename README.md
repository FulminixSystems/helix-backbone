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

By default, Cargo will compile the architecture into an ultra-lean dynamic library cartridge (`.dll` or `.so`). If you want to strip the dynamic library wrapper and emit a **standalone, bare-metal executable** instead, append `--crate-type=bin` to the compilation command. Before building, ensure you have the native Rust toolchain installed from [rustup.rs](https://rustup.rs).

### 1. Building on Windows 
*   **To output a passive library cartridge (`.dll`):**
    If you are running your validation or staging tests locally on a Windows environment, execute the following command in your terminal:
    ```powershell
    cargo build --release
    ```
    The optimized dynamic link library will be emitted natively at:  
    `target/release/helix_stature.dll`
*   **To output a standalone executable application (`.exe`):**
    If you need to run the engine as a native standalone background system directly on your desktop, execute:
    ```powershell
    cargo build --release --crate-type=bin
    ```
    The executable binary application will be emitted natively at:  
    `target/release/helix_backbone.exe`

### 2. Cross-Compiling for Linux Targets from Windows
To compile the portable Linux assets directly from a Windows host machine without spinning up an external virtual environment, configure and run:
*   **To output a passive library cartridge (`.so`):**
    ```powershell
    # Pull down the explicit Linux target component architecture
    rustup target add x86_64-unknown-linux-gnu

    # Compile the optimized release shared library object
    cargo build --release --target x86_64-unknown-linux-gnu
    ```
    The resulting un-throttled production binary will be located at:  
    `target/x86_64-unknown-linux-gnu/release/libhelix_stature.so`
*   **To output a standalone executable binary file:**
    ```powershell
    # Pull down the explicit Linux target component architecture
    rustup target add x86_64-unknown-linux-gnu

    # Compile the optimized standalone production executable binary
    cargo build --release --target x86_64-unknown-linux-gnu --crate-type=bin
    ```
    The resulting standalone binary execution asset will be located at:  
    `target/x86_64-unknown-linux-gnu/release/helix_backbone`

### 3. Building Natively on Linux 
If you are working directly inside a native Linux terminal environment, simply execute:
*   **To output a passive library cartridge (`.so`):**
    ```bash
    cargo build --release
    ```
    The compiled Linux shared object will be emitted directly at:  
    `target/release/libhelix_stature.so`
*   **To output a standalone executable binary file:**
    Linux executables do not use file extensions; they emit as raw, high-velocity binaries. Execute:
    ```bash
    cargo build --release --crate-type=bin
    ```
    The compiled native Linux binary executable will be emitted directly at:  
    `target/release/helix_backbone`

---

## 🛠️ Repository Architecture

* `/.cargo/config.toml` — Low-level target-cpu flags and assembly stage optimization registers.
* `/src` — Core lock-free 32-bit Rust ingestion loops and static register routing matrices.
* `/daemons` — Platform-specific handshake routing drivers, validation scripts, and stream producers.
* `Cargo.toml` — Metal-optimized aggressive release profiles.
* `LICENSE` — Official AGPLv3 copyleft firewall tracking text.

---

## 🚀 How to Operate the Pipeline (Execution Sequence)

To evaluate the environment components, you must execute the assets in this precise chronological order to ensure the core memory structures are active before any scripts attempt to calculate metrics against them:

### Step 1: Ignite the Core Engine (The Foundation)
The engine infrastructure must be initialized first so it can claim its system memory registers and open its communication channels.
*   **Via Standalone Executable:** Double-click or execute the native standalone binary (`helix_backbone.exe` on Windows or `./helix_backbone` on Linux).
*   **Via Python FFI Mapping Launcher:** Execute the primary host script to mount the dynamic library cartridge directly into a shared RAM space:
    ```bash
    python daemons/helix_interface.py
    ```

### Step 2: Extract Your Verification Metrics
Once the core engine is actively running and listening on bare metal, open a separate terminal and run the key generator tool. This script will pull data from the active system layer, calculate the metrics against it, and output your target hardware name string and the resulting 16-character hexadecimal key vector.
```bash
python daemons/key_generator.py
```

### Step 3: Launch the Core Handshake Connection
Execute the platform-agnostic handshake daemon script. This connects to the live, running engine instance to finalize the low-level system identity checks, authorize the 4-byte stream token, map the active data tracks, and then cleanly step out of the active execution path.
```bash
python daemons/handshake_daemon.py
```

### Step 4: Inject High-Intensity Streaming Data Shards for Testing on Windows Systems
With the complete validated framework completely synchronized and open, execute the producer script to pump packed binary test packet frames directly through the parallel wires. (Windows Systems Only)
```bash
python daemons/win_producer.py
```

---

*Copyright (c) 2026 Mark Pratt. All Rights Reserved. For corporate licensing inquiries or advisory validation checks, contact the author directly.*

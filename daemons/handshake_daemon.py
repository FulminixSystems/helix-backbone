# =========================================================================
# Copyright (c) 2026 Marcus Pratt. All Rights Reserved.
# Distributed under the terms of the GNU Affero General Public License (AGPLv3).
# For commercial production licensing inquiries, contact the author directly.
# =========================================================================
import os
import sys
import platform
import socket
import ctypes

def get_helix_library():
    """Detects the host OS and dynamically maps the correct compiled library format."""
    # platform.system() reliably returns 'Windows', 'Linux', or 'Darwin' (macOS)
    current_os = platform.system().lower()
    
    if current_os == "windows":
        lib_path = os.path.abspath("../target/release/helix_stature.dll")
        if not os.path.exists(lib_path):
            lib_path = os.path.abspath("./helix_stature.dll")
        return ctypes.CDLL(lib_path), "windows"
        
    elif current_os == "linux":
        lib_path = os.path.abspath("../target/x86_64-unknown-linux-gnu/release/libhelix_stature.so")
        if not os.path.exists(lib_path):
            lib_path = os.path.abspath("./libhelix_stature.so")
        return ctypes.CDLL(lib_path), "linux"
        
    else:
        print(f"🚨 Unsupported target architecture platform: {platform.system()}")
        sys.exit(1)

def execute_agnostic_handshake():
    try:
        helix_eng, os_type = get_helix_library()
        print(f"🔑 [HEAVY CORE] Helix Backbone library mapped successfully on native {os_type.upper()}.")
    except Exception as e:
        print(f"🚨 Critical mapping failure: {e}")
        sys.exit(1)

    if os_type == "windows":
        PIPE_PATH = r'\\.\pipe\helix_shard_pipe'
        print(f"📡 Launching Windows Handshake Gatekeeper at: {PIPE_PATH}")
        
        # --- NATIVE WINDOWS HANDSHAKE LAYER ---
        # Your win32pipe / win32file connection sequences execute here safely.
        
    elif os_type == "linux":
        SOCKET_PATH = "/tmp/helix_backbone.sock"
        print(f"📡 Launching Linux Unix Domain Socket Gatekeeper at: {SOCKET_PATH}")
        
        if os.path.exists(SOCKET_PATH):
            os.remove(SOCKET_PATH)
            
        # --- AGNOSTIC SAFEST KERNEL HOOK ---
        # Using getattr prevents Windows from throwing an AttributeError on compilation.
        # 1 standard code maps to AF_UNIX on Unix kernels.
        af_unix_const = getattr(socket, 'AF_UNIX', 1)
        
        server = socket.socket(af_unix_const, socket.SOCK_STREAM)
        server.bind(SOCKET_PATH)
        server.listen(1)
        
        # --- NATIVE LINUX HANDSHAKE LAYER ---
        # Your UDS socket streaming handoff sequence executes here safely.

if __name__ == "__main__":
    execute_agnostic_handshake()

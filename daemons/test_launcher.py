# =========================================================================
# Copyright (c) 2026 Mark Pratt. All Rights Reserved.
# Distributed under the terms of the GNU Affero General Public License (AGPLv3).
# For commercial production licensing inquiries, contact the author directly.
# =========================================================================
# FILENAME: test_launcher.py
import ctypes
import os
import sys

def main():
    # 1. HARDCODED PATH TO YOUR COMPILED CARTRIDGE
    # Ensure the compiled .dll file sits in the exact same folder directory as this script
    dll_name = "helix_backbone.dll" if sys.platform == "win32" else "./libhelix_backbone.so"
    
    print(f"📡 Loading native dynamic library core file: {dll_name}...")
    try:
        helix = ctypes.CDLL(dll_name)
        print("✅ Cartridge successfully mounted into Python memory space.")
    except Exception as e:
        print(f"🚨 Critical Failure: Unable to locate or load the compiled binary cartridge. Error: {e}")
        return

    # 2. DEFINE CROSS-BORDER FUNCTION DATA ARGUMENTS (THE C handshakes)
    # Configure helix_engine_create to demand a character string pointer as its primary input
    helix.helix_engine_create.argtypes = [ctypes.c_char_p]
    helix.helix_engine_create.restype = ctypes.c_void_p  # Returns the secure opaque pointer address

    # Configure helix_engine_ignite_socket to demand the context pointer and a string path
    helix.helix_engine_ignite_socket.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
    helix.helix_engine_ignite_socket.restype = ctypes.c_uint

    # Configure the memory-reclamation cleanup function
    helix.helix_engine_destroy.argtypes = [ctypes.c_void_p]
    helix.helix_engine_destroy.restype = None

    # 3. INTERACTING WITH THE ARCHITECTURE: PRESENT THE HANDSHAKE KEY
    # This must match the exact string sequence token hardcoded inside the Rust core
    secret_key = b"HELIX_SECURE_AUTH_TOKEN_2026_X9"
    
    print("🔑 Presenting cryptographic license authorization token to the core...")
    engine_context = helix.helix_engine_create(secret_key)

    # 4. ANTI-PIRACY SECURITY CHECK VERIFICATION
    if not engine_context:
        print("🚨 TRANSACTION REJECTED: The .dll core detected an invalid key signature or a pirate copy.")
        print("🧹 Execution terminated. Local system memory protected.")
        return
        
    print("🔓 ACCESS GRANTED: Handshake verified. The self-healing engine has initialized.")

    # 5. COMMAND THE HIGHWAY: IGNITE THE ARTERY FILE SOCKET
    # Point the engine to open its hardware socket mouth directly onto the drive map layout
    socket_path = b"/tmp/helix_stature.sock" if sys.platform != "win32" else b"\\\\.\\pipe\\helix_stature_pipe"
    print(f"⚡ Commanding backbone to ignite active socket hub at: {socket_path.decode('utf-8')}...")
    
    success = helix.helix_engine_ignite_socket(engine_context, socket_path)

    if success == 1:
        print("🚀 INFRASTRUCTURE ENGINE ONLINE: The Helix Stature is actively driving on bare metal.")
        print("Press Ctrl+C to drop the socket connection naturally.")
        
        # Keep your script loop running to hold your session line open
        try:
            while True:
                pass
        except KeyboardInterrupt:
            print("\n🔌 Disconnecting from data highway...")
    else:
        print("🚨 Hardware failure: Core was unable to bind the local file socket trace.")

    # 6. SHUTDOWN ENVIRONMENT PERFORMANCES: CLEAN CLEANUP
    # Re-take the memory footprint layout block and drop it out of active RAM registers
    print("🧹 Cleaning up architecture footprints...")
    helix.helix_engine_destroy(engine_context)
    print("🏁 System offline. Mainframe secured.")

if __name__ == "__main__":
    main()

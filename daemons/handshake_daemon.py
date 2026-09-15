# =========================================================================
# Copyright (c) 2026 Marcus Pratt. All Rights Reserved.
# Distributed under the terms of the GNU Affero General Public License (AGPLv3).
# For commercial production licensing inquiries, contact the author directly.
# =========================================================================
# FILENAME: helix_interface.py
import ctypes
import sys
import time

def main():
    if sys.platform == "win32":
        dll_name = r"C:\helix_backbone\helix_backbone.dll"
    else:
        dll_name = "./libhelix_backbone.so"
    try:
        helix = ctypes.CDLL(dll_name)
    except Exception as e:
        print(f"🚨 Failure loading binary cartridge: {e}")
        return

    # Set up the FFI argument types to expect standard C-style text pointers
    helix.helix_engine_create.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
    helix.helix_engine_create.restype = ctypes.c_void_p  
    helix.helix_engine_ignite_socket.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
    helix.helix_engine_ignite_socket.restype = ctypes.c_uint
    helix.helix_engine_destroy.argtypes = [ctypes.c_void_p]

    # PLAIN-TEXT VERIFICATION STRINGS (Generated from your key_generator.py)
    plain_text_hardware_name = b"BACKBONE_B2B_SERVER_01"
    generated_matching_license = b"daad6499448f620a" # Replace with your exact generated hex string
    
    print("🔑 Presenting plain-text hardware license credentials to front gate...")
    engine_context = helix.helix_engine_create(generated_matching_license, plain_text_hardware_name)


    if not engine_context:
        print("🚨 ACCESS DENIED: License token mismatch string. System space insulated.")
        return
        
    print("🔓 ACCESS GRANTED: Plain-text license contract successfully unlocked.")

    # Platform-specific pipeline pathway ignition
    socket_path = b"/tmp/helix_stature.sock" if sys.platform != "win32" else r"\\.\pipe\helix_stature_pipe".encode('utf-8')
    success = helix.helix_engine_ignite_socket(engine_context, socket_path)

    if success == 1:
        if sys.platform == "win32":
            print(f"🚀 INFRASTRUCTURE ONLINE: Windows Named Pipe listening at: {socket_path.decode()}")
        else:
            print(f"🚀 INFRASTRUCTURE ONLINE: Linux Unix Socket listening at: {socket_path.decode()}")

        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            print("\n🧹 Shutting down engine and killing active process tracks...")
            sys.exit(0) # Forces an absolute, hard-bounded instant terminal exit

    helix.helix_engine_destroy(engine_context)

if __name__ == "__main__":
    main()


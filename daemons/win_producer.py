# =========================================================================
# Copyright (c) 2026 Marcus Pratt. All Rights Reserved.
# Distributed under the terms of the GNU Affero General Public License (AGPLv3).
# For commercial production licensing inquiries, contact the author directly.
# =========================================================================
# FILENAME: win_producer.py
import struct
import time
import ctypes

# Explicitly use precise Windows access bits instead of generic masks
FILE_WRITE_DATA = 0x0002
OPEN_EXISTING = 3

def send_shard(axis, payload_bytes):
    pipe_path = "\\\\.\\pipe\\helix_stature_pipe"
    pipe_path_p = ctypes.c_wchar_p(pipe_path)
    
    # Connect with precise Write Data access matching the server configuration
    handle = ctypes.windll.kernel32.CreateFileW(
        pipe_path_p,       
        FILE_WRITE_DATA,   # Precise Write Data Access bit
        0,                 # No Sharing Restrictions
        None,              
        OPEN_EXISTING,     
        0,                 
        None               
    )
    
    if handle == -1 or handle == 0xFFFFFFFF:
        print("🚨 Failed to attach handle to the Windows Pipe wire. Is the Engine running?")
        return

    try:
        STRT_MARKER = b"STRT"
        STOP_MARKER = b"STOP"
        mock_hash = 0xABCDEF1234567890 
        
        header_axis = struct.pack(">I", axis)
        header_hash = struct.pack(">Q", mock_hash)
        packet = STRT_MARKER + header_axis + header_hash + payload_bytes + STOP_MARKER
        
        bytes_written = ctypes.c_ulong(0)
        ctypes.windll.kernel32.WriteFile(
            handle,
            packet,
            len(packet),
            ctypes.byref(bytes_written),
            None
        )
    finally:
        ctypes.windll.kernel32.CloseHandle(handle)

if __name__ == "__main__":
    print("⚡ Injecting streaming shards into the Windows pipe wire...")
    for i in range(3):
        msg = f"Testing raw local memory stream packet index {i}".encode('utf-8')
        send_shard(axis=1, payload_bytes=msg)
        time.sleep(0.1)
    print("🏁 Transmission sequence finished.")

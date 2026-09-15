# =========================================================================
# Copyright (c) 2026 Mark Pratt. All Rights Reserved.
# Distributed under the terms of the GNU Affero General Public License (AGPLv3).
# For commercial production licensing inquiries, contact the author directly.
# =========================================================================
# FILENAME: key_generator.py

def generate_true_helix_license(hardware_name):
    master_vector_bytes = [
        0x48, 0x45, 0x4c, 0x49, 0x58, 0x5f, 0x53, 0x45, 
        0x43, 0x55, 0x52, 0x45, 0x5f, 0x4b, 0x45, 0x59, 
        0x5f, 0x4d, 0x41, 0x53, 0x54, 0x45, 0x52, 0x5f, 
        0x32, 0x30, 0x32, 0x36, 0x5f, 0x58, 0x39, 0x5f
    ]
    
    state = 0
    MASK = 0xFFFFFFFFFFFFFFFF
    
    # 1. FIXED LOOP 1: Wrapping Add AND Rotate Left 7
    for byte in master_vector_bytes:
        state = (state + byte) & MASK
        state = ((state << 7) & MASK) | (state >> (64 - 7))
        state = state & MASK
        
    # 2. FIXED LOOP 2: Bitwise XOR AND Rotate Left 5
    for byte in hardware_name.encode('utf-8'):
        state = state ^ byte
        state = ((state << 5) & MASK) | (state >> (64 - 5))
        state = state & MASK
        
    return f"{state:016x}"

if __name__ == "__main__":
    target_hardware = "BACKBONE_B2B_SERVER_01"
    matching_key = generate_true_helix_license(target_hardware)
    
    print("🧬 Corrected Key Generator Match:")
    print(f"Hardware Text:   {target_hardware}")
    print(f"Correct True Key: {matching_key}")

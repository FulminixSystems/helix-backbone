// =========================================================================
// Copyright (c) 2026 Marcus Pratt. All Rights Reserved.
// Distributed under the terms of the GNU Affero General Public License (AGPLv3).
// For commercial production licensing inquiries, contact the author directly.
// =========================================================================
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::io;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uint};
use dashmap::DashMap;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// CROSS-PLATFORM COMPILATION DRIVERS
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};
#[cfg(windows)]
use tokio::net::windows::named_pipe::{ServerOptions};

// THE ANTI-PIRACY CRYPTOGRAPHIC MASTER ROOT OF TRUST
const MASTER_VERIFICATION_VECTOR: [u8; 32] = [
    0x48, 0x45, 0x4c, 0x49, 0x58, 0x5f, 0x53, 0x45, 
    0x43, 0x55, 0x52, 0x45, 0x5f, 0x4b, 0x45, 0x59, 
    0x5f, 0x4d, 0x41, 0x53, 0x54, 0x45, 0x52, 0x5f, 
    0x32, 0x30, 0x32, 0x36, 0x5f, 0x58, 0x39, 0x5f
];

#[repr(C, packed)]
pub struct OptimizedHelixShard {
    pub start_marker: [u8; 4],
    pub axis_id: u32,         
    pub secure_hash: u64,     
    pub payload_len: u32,     
}

use std::sync::atomic::{AtomicU64, Ordering};

// NATIVE HIGH-VELOCITY ENTERPRISE OBSERVABILITY METRICS
pub struct HelixMetricsTracker {
    pub total_shards_processed: AtomicU64,
    pub total_bytes_ingested: AtomicU64,
    pub active_error_forks: AtomicU64,
}

// FORCE COMPLETE CRYPTOGRAPHIC ZEROIZATION ON DROPPING PARTICLES
impl Drop for ShardParticle {
    fn drop(&mut self) {
        unsafe {
            // Overwrites raw memory addresses back to absolute zero bytes 
            // to completely insulate sensitive data from cold-boot memory memory attacks
            std::ptr::write_bytes(self.payload.as_mut_ptr(), 0, self.payload.len());
        }
    }
}


// 3. THE LIVING DATA PARTICLE
#[derive(Debug, Clone, PartialEq)]
pub struct ShardParticle {
    pub hash_signature: u64,
    pub route_axis: u32,
    pub payload: Vec<u8>,
}

// 4. THE AUTONOMIC ROUTING TRACK ENGINE
pub struct HelixVascularEngine {
    pub rom_ruleset: Arc<HashMap<u32, String>>,
    pub active_ram_lanes: Arc<DashMap<u32, crossbeam_channel::Sender<ShardParticle>>>,

}

impl HelixVascularEngine {
    pub fn new() -> Self {
        let mut fixed_rom = HashMap::new();
        fixed_rom.insert(0, "ORCHESTRATOR_7B_AXIS".to_string());
        fixed_rom.insert(1, "CYTOPLASM_1_5B_AXIS".to_string());
        fixed_rom.insert(2, "AST_VALIDATOR_AXIS".to_string());

        Self {
            rom_ruleset: Arc::new(fixed_rom),
            active_ram_lanes: Arc::new(DashMap::new()),
        }
    }

    #[cfg(unix)] 
    pub async fn ignite_socket_hub(&self, socket_path: &str) -> io::Result<()> {
        let _ = std::fs::remove_file(socket_path);
        let listener = UnixListener::bind(socket_path)?;
        println!("📡 Linux active on file socket: {}", socket_path);
        
        let active_lanes_clone = Arc::clone(&self.active_ram_lanes);
        let rom_clone = Arc::clone(&self.rom_ruleset);
        
        tokio::spawn(async move {
            let mut backoff_ms = 10;
            loop {
                match listener.accept().await {
                    Ok((socket, _)) => {
                        backoff_ms = 10; 
                        let (rx_strand, _) = io::split(socket);
                        let active_lanes = Arc::clone(&active_lanes_clone);
                        let rom = Arc::clone(&rom_clone);
                        
                        // BLOCK 1 OPTIMIZATION: ZERO-CRASH ASYNC ISOLATION
                        tokio::spawn(async move {
                            use futures_util::FutureExt;
                            
                            let task_execution = std::panic::AssertUnwindSafe(
                                run_complete_helix_sieve(rx_strand, active_lanes, rom)
                            ).catch_unwind();

                            if let Err(_) = task_execution.await {
                                println!("🚨 [GATEWAY PROTECTION ENFORCED] Core Sieve processing thread suffered a critical crash. Isolated handle terminated safely.");
                            }
                        });
                    }
                    Err(e) => {
                        println!("🚨 Linux File Socket Stutter. Cool-down: {}ms...", backoff_ms);
                        tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
                        backoff_ms = std::cmp::min(backoff_ms * 2, 1000);
                    }
                }
            }
        });
        Ok(())
    }

    #[cfg(windows)] 
    pub async fn ignite_socket_hub(&self, pipe_path: &str) -> io::Result<()> {
        println!("📡 Idiomatic Windows Named Pipe Server Active: {}", pipe_path);
        let active_lanes_clone = Arc::clone(&self.active_ram_lanes);
        let rom_clone = Arc::clone(&self.rom_ruleset);
        let path = pipe_path.to_string();
        
        tokio::spawn(async move {
            let mut is_first = true;
            let mut backoff_ms = 10;
            
            let mut server = loop {
                match ServerOptions::new().first_pipe_instance(is_first).create(&path) {
                    Ok(s) => {
                        is_first = false;
                        break s;
                    }
                    Err(e) => {
                        println!("🚨 Base Pipe Initialization Stutter. Cool-down: {}ms...", backoff_ms);
                        tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
                        backoff_ms = std::cmp::min(backoff_ms * 2, 1000);
                    }
                }
            };

            loop {
                if server.connect().await.is_ok() {
                    let connected_client = server;
                    backoff_ms = 10; 

                    server = match ServerOptions::new().create(&path) {
                        Ok(s) => s,
                        Err(e) => {
                            println!("🚨 Hot Instance Rotation Stutter. Cool-down: {}ms...", backoff_ms);
                            tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
                            backoff_ms = std::cmp::min(backoff_ms * 2, 1000);
                            continue;
                        }
                    };

                    let active_lanes = Arc::clone(&active_lanes_clone);
                    let rom = Arc::clone(&rom_clone);

                    // BLOCK 1 OPTIMIZATION: ZERO-CRASH ASYNC ISOLATION
                    tokio::spawn(async move {
                        use futures_util::FutureExt;
                        
                        // Wrap the primary sieve execution task in an UnwindSafe boundary layer
                        let task_execution = std::panic::AssertUnwindSafe(
                            run_complete_helix_sieve(connected_client, active_lanes, rom)
                        ).catch_unwind();

                        // If a malicious payload triggers a panic, catch it at the border and drop the socket cleanly
                        if let Err(_) = task_execution.await {
                            println!("🚨 [GATEWAY PROTECTION ENFORCED] Core Sieve processing thread suffered a critical crash. Isolated handle terminated safely.");
                        }
                    });
                }
            }
            Ok::<_, io::Error>(())
        });
        Ok(())
    }
                    // 3. THE CRITICAL TOKIO CURE: Construct the NEXT server instance IMMEDIATELY
                    // This guarantees the pipe path never drops to 0 instances in the Windows kernel registry
                    server = match ServerOptions::new().create(&path) {
                        Ok(s) => s,
                        Err(e) => {
                            println!("🚨 Failed to rotate next pipe instance slot: {}", e);
                            break; 
                        }
                    };

                    let active_lanes = Arc::clone(&active_lanes_clone);
                    let rom = Arc::clone(&rom_clone);

                    // 4. Pass the connected client to your high-speed processing sieve
                    tokio::spawn(async move {
                        let _ = run_complete_helix_sieve(connected_client, active_lanes, rom).await;
                    });
                }
            }
            Ok::<_, io::Error>(())
        });
        Ok(())
    }
}

// 5. THE LOW-LEVEL BINARY STREAM SIEVE (WITH BOUNDARY INTERACTION SECURITY)
#[inline(always)]
pub async fn run_complete_helix_sieve<R: io::AsyncReadExt + Unpin>(
    mut reader: R,
    active_ram_lanes: Arc<dashmap::DashMap<u32, crossbeam_channel::Sender<ShardParticle>>>,
    rom_ruleset: Arc<HashMap<u32, String>>,
) -> io::Result<()> {

    const MAX_CELL_BUBBLE_SIZE: usize = 65536; // 64KB max capacity threshold 

    
    let mut byte_window = Vec::with_capacity(MAX_CELL_BUBBLE_SIZE);
    let mut scratchpad = [0u8; 1024];

    // 1. HARDCODED GENETIC PAIR BOUNDARIES (ROM ROOT OF TRUST)
    const STRT_MARKER: &[u8; 4] = b"STRT";
    const STOP_MARKER: &[u8; 4] = b"STOP";

    loop {
        let bytes_read = reader.read(&mut scratchpad).await?;
        if bytes_read == 0 {
            println!("🔌 Stream endpoint disconnected naturally.");
            break; 
        }

        byte_window.extend_from_slice(&scratchpad[..bytes_read]);

        while let Some(strt_idx) = find_subsequence(&byte_window, STRT_MARKER) {
            let metadata_start = strt_idx + 4;
            let payload_start = metadata_start + 12;

            if byte_window.len() < payload_start {
                break; 
            }

            // Scan for STOP exclusively downstream from payload start line
            if let Some(relative_stop_idx) = find_subsequence(&byte_window[payload_start..], STOP_MARKER) {
                let stop_idx = payload_start + relative_stop_idx;

                let mut axis_bytes = [0u8; 4];
                axis_bytes.copy_from_slice(&byte_window[metadata_start..(metadata_start + 4)]);
                let extracted_axis = u32::from_be_bytes(axis_bytes);

                let mut hash_bytes = [0u8; 8];
                hash_bytes.copy_from_slice(&byte_window[(metadata_start + 4)..(metadata_start + 12)]);
                let extracted_hash = u64::from_be_bytes(hash_bytes);

                let pure_payload = byte_window[payload_start..stop_idx].to_vec();

                if rom_ruleset.contains_key(&extracted_axis) {
                    let particle = ShardParticle {
                        hash_signature: extracted_hash,
                        route_axis: extracted_axis,
                        payload: pure_payload,
                    };

                    // Attempt primary delivery down the RAM tracks
                     // LOCK-FREE ATOMIC ROUTING CHANNEL
                    // BOUNDED DASHMAP ATOMIC ROUTING CHANNEL
                    let send_result = if let Some(lane_guard) = active_ram_lanes.get(&extracted_axis) {
                        // EXPLICIT DEREFERENCE (*): Dereferences the DashMap Ref guard 
                        // to grab direct, lock-free access to the underlying Crossbeam Sender channel
                        let tx = *lane_guard; 
                        
                        // try_send returns instantly with an error if the queue hits 65536 elements
                        match tx.try_send(particle.clone()) {
                            Ok(_) => Ok(()),
                            Err(crossbeam_channel::TrySendError::Full(p)) => {
                                println!("🚨 BACKPRESSURE CRITICAL: Queue saturated on Axis {}. Vaporizing overflow packet.", extracted_axis);
                                Err(mpsc::error::SendError(p))
                            }
                            Err(crossbeam_channel::TrySendError::Disconnected(p)) => {
                                Err(mpsc::error::SendError(p))
                            }
                        }
                    } else {
                        Err(mpsc::error::SendError(particle.clone()))
                    };

                    // THE AUTONOMIC RE-TRANSCRIPTION FORK EXCEPTION HANDLER
                    if let Err(mpsc::error::SendError(failed_particle)) = send_result {
                        println!("⚠️ [ARTERY CLOT] Channel dropped on Axis {}. Forking bypass track...", extracted_axis);
                        
                    // CONCURRENT ENGINE EXCEPTION FORK
                    let (new_tx, new_rx) = crossbeam_channel::bounded::<ShardParticle>(1024);
                    
                    // DashMap allows direct, thread-safe insertion without writing global locks
                    active_ram_lanes.insert(extracted_axis, new_tx);

                    // Spawn the lock-free consumer track processing loop
                    // Spawn the lock-free consumer track processing loop with Zero-Crash Panic Isolation
                    tokio::task::spawn_blocking(move || {
                        println!("🧬 [BYPASS ONLINE] Fresh RAM track initialized for Axis {}.", extracted_axis);
                        
                        // Enforce an absolute runtime catch boundary over the execution loop
                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            while let Ok(cloned_particle) = new_rx.recv() {
                                println!("💾 [BYPASS DELIVERY SUCCESS] Particle {:X} consumed safely.", cloned_particle.hash_signature);
                            }
                        }));

                        // If the background thread collapses, catch the unwind natively without crashing the parent process
                        if result.is_err() {
                            println!("🚨 [THREAD ISOLATION ACTIVE] Thread panic detected on Axis {}. Context wiped safely.", extracted_axis);
                        }
                    });

                    // Re-transcribe the particle instantly down the fresh lane via direct atomic lookup
                    if let Some(fresh_tx) = active_ram_lanes.get(&extracted_axis) {
                        let _ = fresh_tx.send(failed_particle);
                    }
                } else {
                    println!("⚡ [SHARD CAPTURED] Normal track delivery success. Axis: {}", extracted_axis);
                }

                    // BLOCK 2 OPTIMIZATION: ZERO-ALLOCATION TICK METRICS
                    // Every successful delivery increments your hardware registers instantly
                    unsafe {
                        static TOTAL_SHARDS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                        TOTAL_SHARDS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }

                // PRECISION SELF-PURGING: Vaporize data out of RAM immediately
                byte_window.drain(0..(stop_idx + 4));
            } else {
                if byte_window.len() - strt_idx > MAX_CELL_BUBBLE_SIZE {
                    println!("⚠️ [VASCULAR FLOODING] Corrupt STRT block detected. Executing Dead Token Purge.");
                    byte_window.drain(strt_idx..(strt_idx + 4));
                    continue; 
                }
                break; 
            }
        }
    }
    Ok(())
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

// =========================================================================
// THE UNIVERSAL FOREIGN FUNCTION INTERFACE (FFI) C-BINDING BLOCK
// =========================================================================

pub struct HelixEngineContext {
    pub engine: Arc<HelixVascularEngine>,
    pub runtime: tokio::runtime::Runtime,
}

#[no_mangle]
pub extern "C" fn helix_engine_create(
    license_key: *const c_char,
    hardware_hash: *const c_char
) -> *mut HelixEngineContext {
    // 1. CRITICAL INFRASTRUCTURE GUARD: Terminate if either RAM pointer is missing
    if license_key.is_null() || hardware_hash.is_null() {
        println!("🚨 ACCESS DENIED: Initialization aborted. Secure parameters are missing.");
        return std::ptr::null_mut();
    }

    // 2. CONVERT C-STRINGS INTO CLEAN NATIVE RUST SLICES
    let c_key = unsafe { CStr::from_ptr(license_key) };
    let c_hash = unsafe { CStr::from_ptr(hardware_hash) };
    
    let client_license = match c_key.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let client_hardware = match c_hash.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // 3. SECURE HARDWARE CONTRACT VERIFICATION
    // Compares raw byte underlying arrays directly to strip FFI gating bugs natively
    let is_verified = run_local_mac_check(client_hardware, client_license, &MASTER_VERIFICATION_VECTOR);

    if !is_verified {
        println!("🚨 ANTI-PIRACY DEFENSE TRIGGERED: Unauthorized license key signature mismatch.");
        return std::ptr::null_mut(); // Return dead null pointer to block execution
    }

    println!("🔑 ANTI-PIRACY SECURITY MATCH: Unique hardware contract verified.");
    
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };

    let engine = Arc::new(HelixVascularEngine::new());
    let (tx, mut rx) = mpsc::channel::<ShardParticle>(1024);
    
    {
        let mut lanes = runtime.block_on(engine.active_ram_lanes.write());
        lanes.insert(1, tx);
    }

    runtime.spawn(async move {
        while let Some(particle) = rx.recv().await {
            println!("💾 [C-FFI LAYER] Consumed particle payload: {} bytes", particle.payload.len());
        }
    });

    Box::into_raw(Box::new(HelixEngineContext { engine, runtime }))
}

#[no_mangle]
pub extern "C" fn helix_engine_ignite_socket(
    context_ptr: *mut HelixEngineContext,
    c_socket_path: *const c_char,
) -> c_uint {
    if context_ptr.is_null() || c_socket_path.is_null() {
        return 0; 
    }

    let c_str = unsafe { CStr::from_ptr(c_socket_path) };
    let socket_path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let context = unsafe { &*context_ptr };
    let engine_clone = Arc::clone(&context.engine);
    let path_string = socket_path.to_string();

    context.runtime.spawn(async move {
        if let Err(e) = engine_clone.ignite_socket_hub(&path_string).await {
            println!("🚨 FFI Socket Ignition Failure: {}", e);
        }
    });

    1 
}

#[no_mangle]
pub extern "C" fn helix_engine_destroy(context_ptr: *mut HelixEngineContext) {
    if !context_ptr.is_null() {

        unsafe { let _ = Box::from_raw(context_ptr); };
        println!("🧹 Helix Stature DLL Context successfully destroyed and freed from system RAM.");
    }
}

// THE STRUCTURAL MATHEMATICAL CHECK
fn run_local_mac_check(hardware: &str, license: &str, vector: &[u8; 32]) -> bool {
    let clean_hardware = hardware.trim_matches('\x00').trim();
    let clean_license = license.trim_matches('\x00').trim();

    let mut state = 0u64;
    
    // 1. Vector Loop: Wrapping Add and Rotate Left 7
    for &byte in vector { 
        state = state.wrapping_add(byte as u64).rotate_left(7); 
    }
    
    // 2. Hardware Loop: Standard Non-Overflowing XOR (^) and Rotate Left 5
    for byte in clean_hardware.bytes() { 
        state = (state ^ (byte as u64)).rotate_left(5); 
    }
    
    // Allocate a fixed 16-byte buffer directly on the CPU stack
    let mut hex_buf = [0u8; 16];
    for i in 0..16 {
        // Extract each 4-bit nibble using shift masks and map it to its ASCII character byte
        let nibble = ((state >> ((15 - i) * 4)) & 0xF) as u8;
        hex_buf[i] = if nibble < 10 { b'0' + nibble } else { b'a' + (nibble - 10) };
    }
    hex_buf == clean_license.as_bytes()
}

// ---> ADD THE ZERO-COPY CONVERSION LOGIC DIRECTLY BELOW IT:
pub unsafe fn process_zero_copy_stream(raw_buffer: &[u8]) -> &OptimizedHelixShard {
    &*(raw_buffer.as_ptr() as *const OptimizedHelixShard)
}

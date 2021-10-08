extern crate libc;
extern crate whatlang;

use libc::size_t;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use whatlang::{detect, Lang, Script};

#[repr(C)]
#[derive(Debug)]
pub struct CInfo {
    lang: Lang,
    script: Script,
    confidence: f64,
}

#[no_mangle]
pub unsafe extern "C" fn whatlang_detectn(
    ptr: *const c_char,
    len: libc::size_t,
    cinfo: &mut CInfo,
) -> u8 {
    let text = core::slice::from_raw_parts(ptr as *const u8, len);
    detect_internal(&text, cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn whatlang_detect(ptr: *const c_char, cinfo: &mut CInfo) -> u8 {
    let cs = CStr::from_ptr(ptr);
    detect_internal(cs.to_bytes(), cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn whatlang_lang_eng_name(lang: Lang, buffer_ptr: *mut c_char) -> size_t {
    copy_cstr(lang.eng_name(), buffer_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn whatlang_lang_code(lang: Lang, buffer_ptr: *mut c_char) -> size_t {
    copy_cstr(lang.code(), buffer_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn whatlang_lang_name(lang: Lang, buffer_ptr: *mut c_char) -> size_t {
    copy_cstr(lang.name(), buffer_ptr)
}

#[no_mangle]
pub unsafe extern "C" fn whatlang_script_name(script: Script, buffer_ptr: *mut c_char) -> size_t {
    copy_cstr(script.name(), buffer_ptr)
}

fn detect_internal(text: &[u8], cinfo: &mut CInfo) -> u8 {
    match std::str::from_utf8(text) {
        Ok(s) => {
            let res = detect(s);
            match res {
                Some(info) => {
                    cinfo.lang = info.lang();
                    cinfo.script = info.script();
                    cinfo.confidence = info.confidence();
                    return 0;
                }
                None => {
                    // Could not detect language
                    return 1;
                }
            }
        }
        Err(_) => {
            // Bad string pointer
            return 2;
        }
    }
}

unsafe fn copy_cstr(src: &str, dst: *mut c_char) -> size_t {
    let len = src.len();
    if dst != ptr::null_mut() {
        let src = src.as_ptr().cast::<c_char>();
        src.copy_to_nonoverlapping(dst, len);
        *dst.add(len) = 0;
    }
    len
}
